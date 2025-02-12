use std::error::Error;
use std::fs::{metadata, OpenOptions};
use std::path::Path;
use std::thread;
use std::time::Duration;

use anyhow::Result;
use clap::{arg, ArgGroup, ArgMatches, Command};
use csv::Writer;
use ibapi::contracts::Contract;
use ibapi::market_data::realtime::{BarSize, WhatToShow};
use ibapi::Client;
use time::{format_description::well_known::Rfc3339};

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let matches = Command::new("stream_bars")
        .version("1.0")
        .author("Wil Boayue <wil@wsbsolutions.com>")
        .about("Streams realtime bars, writes them incrementally to CSV, and attempts reconnection on failure")
        .arg(arg!(--connection_string <VALUE>)
            .default_value("127.0.0.1:7497")
            .help("IB connection string"))
        .arg(arg!(--stock <SYMBOL>).help("Stock symbol (e.g., AAPL)"))
        .arg(arg!(--futures <SYMBOL>).help("Futures symbol (e.g., ES)"))
        .arg(arg!(--bar_size <VALUE>)
            .default_value("Sec5")
            .help("Bar size (only 'Sec5' is supported)"))
        .arg(arg!(--output <FILE>)
            .default_value("bars.csv")
            .help("Output CSV file"))
        .arg(arg!(--timeout <SECONDS>)
            .default_value("10")
            .help("Timeout (in seconds) for the bar stream"))
        .arg(arg!(--client_id <VALUE>)
            .default_value("100")
            .help("Client ID for IB connection"))
        .group(
            ArgGroup::new("contract")
                .args(&["stock", "futures"])
                .required(true),
        )
        .get_matches();

    let connection_string = matches
        .get_one::<String>("connection_string")
        .expect("connection_string is required");
    let client_id: i32 = matches.get_one::<String>("client_id").unwrap().parse()?;
    let timeout_secs: u64 = matches.get_one::<String>("timeout").unwrap().parse()?;
    let timeout_duration = Duration::from_secs(timeout_secs);

    let contract = extract_contract(&matches)
        .expect("Error parsing --stock or --futures argument. One of them is required.");
    let bar_size_str = matches
        .get_one::<String>("bar_size")
        .expect("bar_size is required");
    let bar_size = parse_bar_size(bar_size_str)?;

    let output_file = matches
        .get_one::<String>("output")
        .expect("output is required");

    println!("Using connection_string: {:?}", connection_string);
    println!("Using contract: {:?}", contract);
    println!("Using bar_size: {:?}", bar_size);
    println!("Writing output to: {:?}", output_file);
    println!("Timeout: {} seconds", timeout_secs);
    println!("Client ID: {}", client_id);

    // Open (or create) the CSV file in append mode.
    let output_path = Path::new(output_file);
    let write_header = if output_path.exists() {
        metadata(output_path)?.len() == 0
    } else {
        true
    };
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(output_path)?;
    let mut csv_writer = Writer::from_writer(file);

    // Write the header if needed.
    if write_header {
        csv_writer.write_record(&["date", "open", "high", "low", "close", "volume", "wap", "count"])?;
        csv_writer.flush()?;
    }

    // Main loop: continuously (re)connect, subscribe, and write data incrementally.
    loop {
        println!("Attempting to connect and subscribe for realtime bars...");

        // Attempt to connect.
        let client = match Client::connect(connection_string, client_id) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Failed to connect: {}. Retrying in 5 seconds...", e);
                thread::sleep(Duration::from_secs(5));
                continue;
            }
        };

        // Print some server info.
        println!("Connected! server_version: {}", client.server_version());
        println!("server_time: {:?}", client.connection_time());
        println!("next_order_id: {}", client.next_order_id());

        // Attempt to subscribe to realtime bars.
        let bars = match client.realtime_bars(&contract, bar_size, WhatToShow::Trades, false) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("Failed to subscribe to realtime bars: {}. Retrying in 5 seconds...", e);
                thread::sleep(Duration::from_secs(5));
                continue;
            }
        };

        println!("Subscribed to realtime bars. Streaming data...");

        // Process the stream of bars.
        for (i, bar) in bars.timeout_iter(timeout_duration).enumerate() {
            // Format the bar's date as an RFC3339 string.
            let time_str = bar
                .date
                .format(&Rfc3339)
                .unwrap_or_else(|_| "invalid_date".into());

            // Write the bar data to CSV incrementally.
            if let Err(e) = csv_writer.write_record(&[
                time_str,
                bar.open.to_string(),
                bar.high.to_string(),
                bar.low.to_string(),
                bar.close.to_string(),
                bar.volume.to_string(),
                bar.wap.to_string(),
                bar.count.to_string(),
            ]) {
                eprintln!("Error writing to CSV: {}", e);
            }
            if let Err(e) = csv_writer.flush() {
                eprintln!("Error flushing CSV writer: {}", e);
            }
            println!("Bar {}: {:?}", i, bar);
        }

        // Check if the stream ended due to an error.
        if let Some(err) = bars.error() {
            eprintln!("Bar stream error: {}. Attempting to reconnect...", err);
        } else {
            eprintln!("Bar stream ended unexpectedly. Attempting to reconnect...");
        }

        // Wait briefly before attempting to reconnect.
        thread::sleep(Duration::from_secs(5));
    }
}

/// Extracts a contract from the CLI arguments. Returns a stock contract if --stock is provided,
/// otherwise, if --futures is provided, returns a futures contract.
fn extract_contract(matches: &ArgMatches) -> Option<Contract> {
    if let Some(symbol) = matches.get_one::<String>("stock") {
        Some(Contract::stock(&symbol.to_uppercase()))
    } else {
        matches.get_one::<String>("futures")
            .map(|symbol| Contract::futures(&symbol.to_uppercase()))
    }
}

/// Parses a bar size string (e.g., "Sec5") into a BarSize enum.
/// Currently, only "sec5" (case-insensitive) is supported.
fn parse_bar_size(s: &str) -> Result<BarSize, Box<dyn Error>> {
    match s.to_lowercase().as_str() {
        "sec5" => Ok(BarSize::Sec5),
        _ => Err(format!("Invalid bar size: {}. Only 'Sec5' is supported.", s).into()),
    }
}


# IBKR Real Time Bars 📊🚀

A Rust-based tool to stream real‑time bar data from IBKR, write it incrementally to a CSV file, and automatically reconnect if the connection is lost. This project is perfect for capturing live market data for analysis, ML/RL training, or simply monitoring market activity.

## Features ✨

- **Real-time Streaming:** Connect to IBKR and subscribe to live bar data (OHLCV, WAP, and count).
- **Incremental CSV Writing:** Write each bar to a CSV file immediately to ensure data isn’t lost 💾.
- **Auto-Reconnect:** If the connection or data stream fails, the tool will automatically attempt to reconnect 🔄.
- **Configurable via CLI:** Easily customize your connection parameters, instrument type (stock or futures), bar size, output file, and more using command-line arguments 🔧.
- **Logging:** View connection details and streaming activity with clear log messages using `env_logger` 📜.

## Requirements ⚙️

- **Rust:** Install Rust via [rustup](https://rustup.rs/).
- **IBKR Gateway/TWS:** Ensure your IBKR Gateway or Trader Workstation (TWS) is running and accessible on the specified connection string.
- **Cargo:** Comes with Rust for building and running the project.

## Installation 📦

Clone the repository and build the project:

```bash
git clone https://github.com/yourusername/ibkr_real_time_bars.git
cd ibkr_real_time_bars
cargo build --release
```

## Usage 🚀

Run the application using Cargo. Below are some example commands:

### Streaming Stock Data

```bash
cargo run -- --stock AAPL --connection_string 127.0.0.1:4002 --bar_size Sec5 --output bars.csv --timeout 10 --client_id 100
```

### Streaming Futures Data

```bash
cargo run -- --futures ES --connection_string 127.0.0.1:4002 --bar_size Sec5 --output bars.csv --timeout 10 --client_id 100
```

### Command-Line Arguments Overview

- `--connection_string <VALUE>`  
  The IBKR connection string (Default: `127.0.0.1:4002`).

- `--stock <SYMBOL>`  
  The stock symbol to subscribe to (e.g., `AAPL`). Use either `--stock` **or** `--futures`.

- `--futures <SYMBOL>`  
  The futures symbol to subscribe to (e.g., `ES`). Use either `--stock` **or** `--futures`.

- `--bar_size <VALUE>`  
  Bar size to use (currently only supports `"Sec5"`) (Default: `Sec5`).

- `--output <FILE>`  
  The output CSV file where data will be written (Default: `bars.csv`).

- `--timeout <SECONDS>`  
  Timeout (in seconds) for the bar stream (Default: `10`).

- `--client_id <VALUE>`  
  Client ID for the IBKR connection (Default: `100`).

## How It Works 🛠️

1. **Connection & Subscription:**  
   The program connects to IBKR using the specified connection string and client ID. It then subscribes to real‑time bar data based on your chosen contract (stock or futures) and bar size.

2. **Incremental Data Writing:**  
   Each bar (including date, open, high, low, close, volume, wap, and count) is written to the CSV file immediately and flushed. This ensures minimal data loss even if the program terminates unexpectedly.

3. **Auto-Reconnect:**  
   If the connection or subscription fails, the tool waits 5 seconds and then attempts to reconnect and resume streaming.

## Logging & Debugging 🐛

This application uses `env_logger` to log useful connection information and streaming data details. To see log messages, set the `RUST_LOG` environment variable:

```bash
export RUST_LOG=info
cargo run -- --stock AAPL
```

## Troubleshooting 🔍

- **Connection Issues:**  
  Ensure your IBKR Gateway/TWS is running and accessible on the specified connection string.

- **Data Loss Concerns:**  
  Because data is written incrementally to the CSV file, you’ll retain all received data even if the application crashes. However, note that IBKR’s realtime bars API only streams current data. Missing data during a disconnect will not be recovered.

- **Bar Size Limitations:**  
  Currently, only the `"Sec5"` bar size is supported. If you need additional bar sizes, you may need to update the code and ensure your IBKR API version supports them.

## Contributing 🤝

Contributions are welcome! Feel free to fork the repository, submit pull requests, or open issues on GitHub.  
See the [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## License 📄

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

Happy streaming! 📈🚀📝
# ibkr_real_time_bars

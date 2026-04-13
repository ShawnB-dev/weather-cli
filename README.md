# Weather CLI

A simple Rust weather application with both command-line and GUI modes.

## Features

- CLI mode: `cargo run -- <city>`
- GUI mode: `cargo run`
- Current weather and 5-day forecast
- ASCII weather art in CLI and GUI
- Satellite imagery link in GUI
- Fetches weather from `wttr.in`

## Usage

- Launch the GUI:

  ```sh
  cargo run
  ```

- Get weather in the terminal:

  ```sh
  cargo run -- seattle
  ```

## Requirements

- Rust and Cargo installed
- Internet access for weather data

## Files

- `src/main.rs` — app entry point, CLI and GUI dispatch
- `src/weather.rs` — weather fetch and JSON parsing
- `Cargo.toml` — dependencies and package configuration

## License

This project is licensed under the MIT License.

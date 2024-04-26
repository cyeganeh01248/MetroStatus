# WMATA Metro Status

This Rust program is designed to retrieve information about upcoming trains arriving at a specified station. It utilizes
the Washington Metropolitan Area Transit Authority (WMATA) API to fetch real-time train predictions.

## Installation

Ensure you have Rust installed on your system. If not, you can install it
from [Rust's official website](https://www.rust-lang.org/).

Clone this repository to your local machine and navigate into it:

```bash
git clone https://github.com/cyeganeh01248/MetroStatus.git
cd MetroStatus
```

### Local Install

In order to install locally, run the following script.

```bash
cargo build --release
```

### Docker

In order to install via Docker, run the following script.

```bash
docker build -t metro_status .
```

## Configuration

Before running the program, you need to obtain an API key from WMATA. You can register for an API
key [here](https://developer.wmata.com/).

Once you have the API key, you can pass it as a command-line argument.

If you do not obtain one, you may leave the argument out. The default public demo key will be used. This key has
significantly low limits and is more used for testing purposes.

## Running the Program

To run the program locally, execute the following command:

```bash
cargo run -- [--api-key YOUR_API_KEY] [--target-station TARGET_STATION_NAME]
```

To then run the program with docker, execute the following command:

```bash
docker run --rm -it metro_status [--api-key YOUR_API_KEY] [--target-station TARGET_STATION_NAME]
```

If desired, replace `YOUR_API_KEY` with your WMATA API key and `TARGET_STATION_NAME` with the name of the station you
want to fetch train schedules for. Leaving off the `TARGET_STATION_NAME` will default to Huntington Station.

### Command-line Options

- `--api-key` / `-a`: The WMATA API key to use. Defaults to a public demo key if not provided.
- `--target-station` / `-t`: The name of the station to look for. Defaults to "Huntington" if not provided.

## Dependencies

This program utilizes the following external crates:

- `clap`: A command-line argument parsing library for parsing command-line options.
- `tokio`: An async framework for rust for running async functions such as the reqwest lib.
- `reqwest`: An Async Rust HTTP client for making requests to the WMATA API.
- `serde`: A serialization/deserialization library for converting JSON responses into Rust structs.

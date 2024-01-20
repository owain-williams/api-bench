# API Benchmark

This program is designed to benchmark API endpoints. It measures the performance of the APIs by sending requests to them and observing the responses. The program respects rate limiting, meaning it won't send requests at a rate that exceeds the limit set by the API server. It provides a command-line interface, and users can get usage information by running ./api-bench --help. The program is built with Rust and uses the cargo build system.

## Installation

Clone the repository and navigate to the project directory. Run the following command to build the project:

```sh
cargo build
```

## Usage
After building the project, you can run it with:
```sh
cargo run
```

To run the executable:
```sh
./api-bench
```

Further information can be found using:
```sh
./api-bench --help
```
This should show Usage information
```sh
Simple program to benchmark API endpoints, respecting rate limiting

Usage: api-bench [OPTIONS]

Options:
  -u, --url <URL>            The URL of the API endpoint you want to benchmark [default: https://jsonplaceholder.typicode.com/todos/1]
  -r, --requests <REQUESTS>  The number of requests to make [default: 10]
  -d, --delay <DELAY>        The delay between requests (in ms) [default: 1000]
  -h, --help                 Print help
  -V, --version              Print version
```

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License

[MIT](https://choosealicense.com/licenses/mit/)
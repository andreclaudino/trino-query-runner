# trino-query-runner

A Rust application for running Trino queries based on a template file and provided parameters.

## Features

- **Command Line Interface (CLI)**: Easily run Trino queries using various configurable parameters.
- **Template Based Queries**: Define query templates in separate files, making it easier to manage complex queries.
- **Parameterized Queries**: Use command line arguments to inject values into query templates at runtime.
- **Connection String Handling**: Connect to a Trino server using a connection string formatted as `trino://<user>:<password>@<host>:<port>/<catalog>`.

## Dependencies

The application relies on several Rust libraries, including:

- `clap`: For parsing command line arguments.
- `serde`: For serialization and deserialization tasks.
- `anyhow`: A library for error handling.
- `minijinja`: Provides template rendering capabilities.
- `trino-rust-client`: A client library for interacting with the Trino server.
- `tokio`: An asynchronous runtime, used for managing asynchronous tasks.

## Usage

To use the trino-query-runner application, follow these steps:

1. **Compile and Run**:
   ```bash
   cargo run -- \
     --template-path=path/to/query_template.txt \
     --connection-string=trino://user:password@host:port/catalog \
     -p param1=value1 -p param2=value2

docker run andreclaudino/trino-query-runner trino-query-runner \
  --template-path=/path/to/query_template.txt \
  --connection-string=trino://user:password@host:port/catalog \
  -p param1=value1 -p param2=value2
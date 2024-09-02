# Template CLI

This Rust project is a command-line tool designed to interact with Microsoft services, specifically for obtaining an OAuth2 access token using client credentials. The tool authenticates with Microsoft Azure and retrieves an access token that can be used to interact with Microsoft Graph API or other Azure services. The project is structured with a main program and an authentication module.

## Features

- Retrieve access tokens via Microsoft OAuth2.
- Supports customizable logging levels (trace, debug, info, warn, error).
- Environment variable support for storing sensitive information like client credentials.
- Detailed logging for development and debugging.

## Prerequisites

Before running the tool, ensure you have the following:

- Rust installed on your system.
- A Microsoft Azure app registration with a `client_id`, `client_secret`, and `tenant_id`.
- Any Microsoft Graph API permission granted to your Azure app registration
- A `.env` file containing your Azure credentials.

## Installation

1. **Clone the Repository**:
    ```bash
    git clone git@github.com:whitefox82/microsoft_services_tools.git
    cd microsoft_services_tools/template_cli/
    ```

2. **Build the Project**:
    Make sure you have Rust installed on your machine. You can build the project with Cargo:
    ```bash
    cargo build --release
    ```

3. **Set Up Environment Variables**:
    Create a `.env` file in the root of the project and add the following variables:
    ```env
    TENANT_ID=<your-tenant-id>
    CLIENT_ID=<your-client-id>
    CLIENT_SECRET=<your-client-secret>
    ```

## Usage

To run the tool and retrieve an access token, use the following command:

```sh
./target/release/template_cli
```

### Options

The tool supports different logging levels, which can be controlled with command-line flags:

- `--trace`: Enables trace-level logging.
- `--debug`: Enables debug-level logging.
- `--info`: Enables info-level logging (default).
- `--warn`: Enables warn-level logging.
- `--error`: Enables error-level logging.
- `--off`: Disables logging.

Example with debug logging:

```sh
./target/release/template_cli --debug
```

## Logging

The tool uses the `env_logger` crate for logging. By default, the log level is set to `info`, but you can change it using command-line options or by setting the `RUST_LOG` environment variable:

```sh
RUST_LOG=debug ./target/release/template_cli
```

## License

This project is licensed under the GNU General Public License v3.0. See the [LICENSE](https://github.com/whitefox82/microsoft_services_tools/blob/main/LICENSE) file for details.

# API Template

`api_template` is a Rust-based command-line tool that retrieves an OAuth 2.0 access token from the Microsoft identity platform using the client credentials flow. 
This tool is designed to help developers interact with Microsoft APIs by providing an easy way to obtain an access token.

## Features

- **Retrieve Access Token**: Obtain an OAuth 2.0 access token from the Microsoft identity platform using client credentials.
- **Verbose Logging**: Enable detailed logging for in-depth debugging and monitoring.

## Prerequisites

- Rust installed on your system.
- A Microsoft Azure app registration with a `client_id`, `client_secret`, and `tenant_id`.
- The following Microsoft Graph API permission granted to your Azure app registration:
  - **`Application.Read.All`**: Allows the application to read the app registration details, which is necessary for this tool to operate.
- A `.env` file containing your Azure credentials.

## Installation

1. **Clone the Repository**:
    ```bash
    git clone git@github.com:whitefox82/api_template.git
    cd api_template/
    ```

2. **Build the Project**:
    Ensure you have Rust installed on your machine. You can build the project using Cargo:
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

To use `api_template`, run the following command:

```bash
./target/release/api_template [-v]
```

### Example

```bash
./target/release/api_template -v
```
this command will retrieve an access token using the provided credentials and print detailed logs if the --verbose flag is enabled.

## Logging

api_template uses the env_logger crate for logging. You can control the log output by setting the RUST_LOG environment variable:

```bash
RUST_LOG=info ./target/release/api_template
```
Set `RUST_LOG=debug` for more detailed logging.

If you prefer to see all logs, including the debug-level logs, you can set the environment variable like this:

```bash
RUST_LOG=debug ./target/release/api_template -v
```

This will ensure that even the debug-level logs are printed, providing in-depth insights into the internal operations of the tool.

## License

This project is licensed under the GNU General Public License v3.0. See the [LICENSE](https://github.com/whitefox82/microsoft_services_tools/blob/main/LICENSE) file for details.
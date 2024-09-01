# Email Send Service

This Rust project is a command-line tool that allows you to send emails using the Microsoft Graph API. The tool authenticates with Microsoft Azure using OAuth2 and sends emails to specified recipients. The project is structured with a main program and an authentication module.

## Features

- Send emails via Microsoft Graph API.
- Customizable email recipient, subject, body, and sender.
- Verbose logging for detailed debugging information.
- Environment variable support for storing sensitive information like client credentials.

## Prerequisites

Before running the tool, ensure you have the following:

- Rust installed on your system.
- A Microsoft Azure app registration with a `client_id`, `client_secret`, and `tenant_id`.
- A `.env` file containing your Azure credentials.

## Installation

1. Clone the repository:

    ```sh
    git clone https://github.com/yourusername/send_email.git
    cd send_email
    ```

2. Add the necessary dependencies to your `Cargo.toml`:

    ```toml
    [dependencies]
    anyhow = "1.0"
    clap = { version = "4.0", features = ["derive"] }
    dotenv = "0.15.0"
    env_logger = "0.11.5"
    log = "0.4.11"
    regex = "1.5"
    reqwest = { version = "0.12.7", features = ["json"] }
    serde = { version = "1.0", features = ["derive"] }
    serde_derive = "1.0"
    serde_json = "1.0"
    tokio = { version = "1", features = ["full"] }
    ```

3. Create a `.env` file in the root directory with your Azure credentials:

    ```env
    TENANT_ID=your_tenant_id
    CLIENT_ID=your_client_id
    CLIENT_SECRET=your_client_secret
    ```

4. Build the project:

    ```sh
    cargo build --release
    ```

## Usage

To run the email sending tool, use the following command:

```sh
./target/release/send_email --email recipient@example.com --subject "Your Subject" --body "Your Email Body" --sender sender@example.com
```

### Options

- `--email` or `-e`: The recipient's email address.
- `--subject` or `-s`: The subject of the email.
- `--body` or `-b`: The body of the email.
- `--sender` or `-u`: The sender's email address.
- `--verbose` or `-v`: Enable verbose logging.

Example with verbose logging:

```sh
./target/release/send_email --email recipient@example.com --subject "Test Email" --body "Hello, this is a test." --sender sender@example.com --verbose
```

## Logging

The tool uses the env_logger crate for logging. You can control the log level using the RUST_LOG environment variable:

```sh
RUST_LOG=debug ./target/release/send_email --email recipient@example.com --subject "Test Email" --body "Hello, this is a test." --sender sender@example.com
```

## License

This project is licensed under the GNU General Public License v3.0. See the [LICENSE](https://github.com/whitefox82/microsoft_services_tools/blob/main/LICENSE) file for details.

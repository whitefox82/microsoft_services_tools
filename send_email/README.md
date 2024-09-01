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
- The following Microsoft Graph API permissions granted to your Azure app registration:
  - **`Mail.Send**: Allows the application to send emails as any user.
- A `.env` file containing your Azure credentials.

## Installation

1. **Clone the Repository**:
    ```bash
    git clone git@github.com:whitefox82/microsoft_services_tools.git
    cd microsoft_services_tools/send_email/
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

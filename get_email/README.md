# Get Email

`get_email` is a Rust-based command-line tool designed to retrieve basic email information from a user's mailbox within a Microsoft 365 tenant. The tool interacts with the Microsoft Graph API to search for emails based on a specific subject and provides options to log information, debug details, and output potential spoofed email addresses.

## Features

- **Email Search**: Search for emails in a user's mailbox based on the specified subject.
- **Access Token Retrieval**: Automatically retrieve the required access token using a local authentication service.
- **Logging Options**: Enable informational and debug logging for better insight into the tool's operation.
- **Spoofed Email Detection**: Optionally detect and highlight spoofed email addresses in the results.

## Prerequisites

- Rust installed on your system.
- A Microsoft Azure app registration with a `client_id`, `client_secret`, and `tenant_id`.
- The following Microsoft Graph API permission granted to your Azure app registration:
  - **`Mail.Read`**: Allows the application to read email in user mailboxes.
- A local authentication service script named `ms_auth_service` for retrieving the access token.
- The `ms_auth_service` script should be placed in the same directory as the compiled `get_email` binary.

## Installation

1. **Clone the Repository**:
    ```bash
    git clone git@github.com:whitefox82/microsoft_services_tools.git
    cd microsoft_services_tools/get_email/
    ```

2. **Build the Project**:
    Ensure that Rust is installed on your machine. You can build the project with Cargo:
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

To use the `get_email` tool, run the following command:

```bash
./target/release/get_email --upn <user-upn> --subject <email-subject> [--info] [--debug] [--spoofed]
```

### Arguments

- `--upn` or `-u`: Specifies the User Principal Name (UPN) of the mailbox to search.
- `--subject` or `-s`: Specifies the subject of the email to search for.
- `--info`: Enables informational logging.
- `--debug`: Enables debug logging for more detailed output.
- `--spoofed` or `-p`: Outputs potential spoofed email addresses.

### Example Command

```bash
./target/release/get_email --upn john.doe@contoso.com --subject "Quarterly Report" --info --spoofed
```

This command searches for emails in the mailbox of john.doe@contoso.com with the subject "Quarterly Report" and outputs potential spoofed email addresses while enabling informational logging.

## Example Output

The tool will output the search results directly to the terminal. If the --spoofed flag is used, it will highlight spoofed email addresses in red and matching ReplyTo and From addresses in green.

## Logging

`get_email` uses the `env_logger` crate for logging. You can control the log output by setting the `RUST_LOG` environment variable:

```bash
RUST_LOG=info ./target/release/get_email --upn <user-upn> --subject <email-subject>
```

Set RUST_LOG=debug for more detailed logging.

## License

This project is licensed under the GNU General Public License v3.0. See the [LICENSE](https://github.com/whitefox82/microsoft_services_tools/blob/main/LICENSE) file for details.

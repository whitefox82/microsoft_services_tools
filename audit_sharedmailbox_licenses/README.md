# Audit Shared Mailbox Licenses

`audit_sharedmailbox_licenses` is a Rust-based command-line tool designed to audit shared mailboxes within a Microsoft 365 tenant. The tool interacts with the Microsoft Graph API to retrieve all shared mailboxes that have a license.

## Features

- **Fetch User Data**: Retrieve all users in the tenant, including their assigned licenses.
- **Concurrent Mailbox Settings Fetch**: Efficiently fetch mailbox settings for users with licenses using asynchronous concurrent requests.
- **Identify Shared Mailboxes**: Determine which users have mailbox settings indicating a "shared" purpose and report on those who also have assigned licenses.

## Prerequisites

- Rust installed on your system.
- A Microsoft Azure app registration with a `client_id`, `client_secret`, and `tenant_id`.
- The following Microsoft Graph API permission granted to your Azure app registration:
  - **`User.Read.All`**: Allows the application to read user profiles.
  - **`MailboxSettings.Read`**: Allows the application to read user mailbox settings.
- A `.env` file containing your Azure credentials.

## Installation

1. **Clone the Repository**:
    ```bash
    git clone git@github.com:whitefox82/microsoft_services_tools.git
    cd microsoft_services_tools/audit_sharedmailbox_licenses/
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

To use the `audit_sharedmailbox_licenses` tool, run the following command:

```bash
./target/release/audit_sharedmailbox_licenses
```

This command will start the audit process, fetching all users, checking their assigned licenses, and identifying those with a "shared" mailbox purpose.

## Example Output

The tool will output to the terminal the users who have both assigned licenses and a mailbox purpose of "shared." The output will include the User Principal Name (UPN) of each identified user.

## Logging

audit_sharedmailbox_licenses uses the env_logger crate for logging. You can control the log output by setting the RUST_LOG environment variable:

```bash
RUST_LOG=info ./target/release/audit_sharedmailbox_licenses
```

Set RUST_LOG=debug for more detailed logging.

## License

This project is licensed under the GNU General Public License v3.0. See the [LICENSE](https://github.com/whitefox82/microsoft_services_tools/blob/main/LICENSE) file for details.

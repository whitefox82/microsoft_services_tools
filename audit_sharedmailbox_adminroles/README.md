# Audit Shared Mailbox Admin Roles

`audit_sharedmailbox_adminroles` is a Rust-based command-line tool designed to audit directory roles within a Microsoft 365 tenant and identify shared mailboxes that have been assigned administrative roles. The tool interacts with the Microsoft Graph API to retrieve directory roles, their members, and mailbox settings.

## Features

- **Fetch Directory Roles**: Retrieve all directory roles within the tenant.
- **Fetch Role Members**: Identify members of each directory role.
- **Concurrent Mailbox Settings Fetch**: Efficiently fetch mailbox settings for role members using asynchronous concurrent requests.
- **Identify Shared Mailboxes**: Determine which role members have mailbox settings indicating a "shared" purpose and report on those who have been assigned administrative roles.

## Prerequisites

- Rust installed on your system.
- A Microsoft Azure app registration with a `client_id`, `client_secret`, and `tenant_id`.
- The following Microsoft Graph API permissions granted to your Azure app registration:
  - **`Directory.Read.All`**: Allows the application to read directory data, including roles and users.
  - **`MailboxSettings.Read`**: Allows the application to read user mailbox settings.
- A `.env` file containing your Azure credentials.

## Installation

1. **Clone the Repository**:
    ```bash
    git clone git@github.com:whitefox82/microsoft_services_tools.git
    cd microsoft_services_tools/audit_sharedmailbox_adminroles/
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

To use the `audit_sharedmailbox_adminroles` tool, run the following command:

```bash
./target/release/audit_sharedmailbox_adminroles
```

This command will start the audit process, fetching directory roles, checking their members, and identifying those with a "shared" mailbox purpose who also have administrative roles.

## Example Output

The tool will output to the terminal the User Principal Names (UPNs) of users who have both administrative roles and a mailbox purpose of "shared."

## Logging

audit_sharedmailbox_adminroles uses the env_logger crate for logging. You can control the log output by setting the RUST_LOG environment variable:

```bash
RUST_LOG=info ./target/release/audit_sharedmailbox_adminroles
```

Set `RUST_LOG=debug` for more detailed logging.

## License

This project is licensed under the GNU General Public License v3.0. See the [LICENSE](https://github.com/whitefox82/microsoft_services_tools/blob/main/LICENSE) file for details.

# Revoke Session Service

RevokeSessionService is a Rust-based command-line tool that revokes Microsoft 365 sign-in sessions for users based on their User Principal Name (UPN). The tool uses the Microsoft Graph API to revoke these sessions securely and efficiently.

## Features

- **Revoke Sign-In Sessions**: Revoke all active sign-in sessions for a specified user using their UPN.
- **Verbose Logging**: Enable detailed logging to debug and monitor the process.

## Prerequisites

- Rust installed on your system.
- A Microsoft Azure app registration with a `client_id`, `client_secret`, and `tenant_id`.
- The following Microsoft Graph API permission granted to your Azure app registration:
  - **`User.ReadWrite.All`**: Allows the application to read and write user profiles, including revoking sign-in sessions.
- A `.env` file containing your Azure credentials.

## Installation

1. **Clone the Repository**:
    ```bash
    git clone git@github.com:whitefox82/microsoft_services_tools.git
    cd microsoft_services_tools/revoke_sessions/
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

To use the RevokeSessionService, run the following command:

```bash
./target/release/revoke_sessions -u <user_principal_name> [-v]
```

- `-u, --upn`: The User Principal Name (UPN) of the user whose sessions you want to revoke.
- `-v, --verbose`: Enable verbose logging for detailed output.

### Example

```bash
./target/release/revoke_sessions -u john.doe@contoso.com -v
```

This command will revoke all sign-in sessions for john.doe@contoso.com and print detailed logs.

## Logging

RevokeSessionService uses the env_logger crate for logging. You can control the log output by setting the RUST_LOG environment variable:

```bash
RUST_LOG=info ./target/release/revoke_sessions -u john.doe@contoso.com
```

Set RUST_LOG=debug for more detailed logging.

## License

This project is licensed under the GNU General Public License v3.0. See the [LICENSE](https://github.com/whitefox82/microsoft_services_tools/blob/main/LICENSE) file for details.
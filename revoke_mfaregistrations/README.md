# Revoke MFA Registrations Service

`revoke_mfaregistrations` is a Rust-based command-line tool designed to enforce Multi-Factor Authentication (MFA) re-registration for users in Microsoft 365. The tool interacts with the Microsoft Graph API to delete specific authentication methods, requiring users to set up MFA again.

## Features

- **Enforce MFA Re-Registration**: Delete existing software OATH authentication methods, requiring users to re-register for MFA.
- **Verbose Logging**: Enable detailed logging to aid in debugging and monitoring the operation.

## Prerequisites

- **Rust** installed on your system.
- A **Microsoft Azure app registration** with a `client_id`, `client_secret`, and `tenant_id`.
- The following **Microsoft Graph API permissions** granted to your Azure app registration (all should be application permissions):
  - **`UserAuthenticationMethod.ReadWrite.All`**: Allows the application to read and update user authentication methods.
  - **`User.Read.All`**: Allows the application to read the profile of every user in the directory.
  - **`User.ReadWrite.All`**: Allows the application to read and write data in user profiles.
  - **`User.ManageIdentities.All`**: Allows the application to manage user identities, including resetting MFA or other authentication methods.
- A **`.env` file** containing your Azure credentials with the following keys:
  - `TENANT_ID`: Your Azure Active Directory tenant ID.
  - `CLIENT_ID`: The client ID of your Azure app registration.
  - `CLIENT_SECRET`: The client secret of your Azure app registration.


## Installation

1. **Clone the Repository**:
    ```bash
    git clone git@github.com:whitefox82/microsoft_services_tools.git
    cd microsoft_services_tools/revoke_mfaregistrations/
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

To use the `revoke_mfaregistrations` tool, run the following command:

```bash
./target/release/revoke_mfaregistrations -u <user_principal_name> [-v]
```
- `-u, --upn`: The User Principal Name (UPN) of the user for whom you want to enforce MFA re-registration.
- `-v, --verbose`: Enable verbose logging for more detailed output.

### Example

```bash
./target/release/revoke_mfaregistrations -u john.doe@contoso.com -v
```

This command will delete the software OATH MFA method for `john.doe@contoso.com`, requiring them to re-register their MFA method. Detailed logs will be printed due to the verbose flag.

## Logging

`revoke_mfaregistrations` uses the `env_logger` crate for logging. You can control the log output by setting the `RUST_LOG` environment variable:

```bash
RUST_LOG=info ./target/release/revoke_mfaregistrations -u john.doe@contoso.com
```

Set RUST_LOG=debug for more detailed logging.

## License

This project is licensed under the GNU General Public License v3.0. See the [LICENSE](https://github.com/whitefox82/microsoft_services_tools/blob/main/LICENSE) file for details.

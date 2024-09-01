# Microsoft 365 License Removal Tool

`licenseRemoval` is a Go-based command-line application designed to remove assigned licenses from a specified user in Microsoft 365. The tool interacts with the Microsoft Graph API to fetch the assigned licenses for a user and remove them.

## Features

- **Fetch and Remove Licenses**: Retrieve all licenses assigned to a user and remove them using the Microsoft Graph API.
- **Error Handling**: Comprehensive error handling ensures that issues during the license removal process are properly reported.

## Prerequisites

- Go installed on your system.
- A Microsoft Azure app registration with a `client_id`, `client_secret`, and `tenant_id`.
- The following Microsoft Graph API permission granted to your Azure app registration:
  - **`Directory.ReadWrite.All`**: Allows the application to read and write directory data, including assigned licenses.
- A `.env` file containing your Azure credentials.

## Installation

1. **Clone the Repository**:
    ```bash
    git clone git@github.com:whitefox82/microsoft_services_tools.git
    cd microsoft_services_tools/licenseRemoval/
    ```

2. **Build the Project**:
    Ensure that Go is installed on your machine. You can build the project with the following command:
    ```bash
    go build -o licenseRemoval
    ```

3. **Set Up Environment Variables**:
    Create a `.env` file in the root of the project and add the following variables:
    ```env
    TENANT_ID=<your-tenant-id>
    CLIENT_ID=<your-client-id>
    CLIENT_SECRET=<your-client-secret>
    ```

## Usage

To use the `licenseRemoval`, run the following command:

```bash
./licenseRemoval <user_principal_name>
```

### Example

```bash
./license_removal_tool john.doe@contoso.com
```

This command will remove all licenses assigned to `john.doe@contoso.com` using the Microsoft Graph API.

## Logging

The `license_removal_tool` uses Go's built-in `log` package for logging. Errors and important information will be logged to the console. Ensure that the terminal output is monitored during execution for any error messages.

## License

This project is licensed under the GNU General Public License v3.0. See the [LICENSE](https://github.com/whitefox82/microsoft_services_tools/blob/main/LICENSE) file for details.

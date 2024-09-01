# SKU Availability Checker

This Go project is a command-line tool that retrieves the availability of SKUs (Stock Keeping Units) from Microsoft Azure using the Azure SDK for Go. The tool authenticates with Microsoft Azure using client credentials and fetches the remaining units of specified SKUs.

## Features

- Retrieve SKU availability via the Azure Commerce API.
- Filter results based on specified SKU part numbers.
- Support for checking all available SKUs with a wildcard.
- JSON output for easy integration with other tools.

## Prerequisites

Before running the tool, ensure you have the following:

- Go installed on your system.
- A Microsoft Azure app registration with a `client_id`, `client_secret`, and `tenant_id`.
- The following Microsoft Azure API permissions granted to your Azure app registration:
  - **`Commerce Usage`**: Allows the application to access usage data for Azure resources.
- A `.env` file containing your Azure credentials.

## Installation

1. **Clone the Repository**:
    ```bash
    git clone git@github.com:<your-username>/sku-availability-checker.git
    cd sku-availability-checker/
    ```

2. **Install Dependencies**:
    Make sure you have Go installed on your machine. You can install the required dependencies using `go mod`:
    ```bash
    go mod tidy
    ```

3. **Set Up Environment Variables**:
    Create a `.env` file in the root of the project and add the following variables:
    ```env
    TENANT_ID=<your-tenant-id>
    CLIENT_ID=<your-client-id>
    CLIENT_SECRET=<your-client-secret>
    ```

## Usage

To run the SKU availability checker tool, use the following command:

```sh
go run . <SKU_PART_NUMBER_1> <SKU_PART_NUMBER_2> ... | *
```
### Options

- `SKU_PART_NUMBER`: The part number of the SKU to check. You can specify multiple SKU part numbers.
- `*`: Check availability for all SKUs.

Example to check availability for specific SKUs:

```sh
go run . STANDARD_DS1_V2 STANDARD_DS2_V2
```
Example to check availability for all SKUs:

```sh
go run . *
```

## Output

The tool outputs the availability data in JSON format, including the SKU part number and the remaining units:

```sh
[
    {
        "skuPartNumber": "STANDARD_DS1_V2",
        "remainingUnits": 100
    },
    {
        "skuPartNumber": "STANDARD_DS2_V2",
        "remainingUnits": 150
    }
]
```

## License

This project is licensed under the GNU General Public License v3.0. See the [LICENSE](https://github.com/whitefox82/microsoft_services_tools/blob/main/LICENSE) file for details.

# BootlegEditor3000

BootlegEditor3000 is a Command Line Interface (CLI) tool written in Rust for working with CSV files. The tool allows users to perform various operations on a CSV file, such as displaying the entire file, paginating, deleting and modifying rows/fields, and saving the data to a new or existing CSV file.

## Features

- **Display Entire File:** View the entire content of the CSV file in the terminal.

- **Paginate:** Display a specific range of rows from the CSV file.

- **Delete Row:** Remove a specific row from the CSV file while maintaining its dimensions. It will display the full file to see the modification afterward.

- **Modify Field:** Change the value of a field in a particular row while preserving the CSV file's structure. It will print the modified line to see the modification afterward.

- **Save to CSV:** Save the modified CSV data to a new or existing CSV file.

- **Exit:** Exit the program.

## Usage

```bash
git clone https://github.com/DocteurPing/PeaqTest.git
cd PeaqTest
cargo run
```

## Notes

- The CSV file "testdata.csv" is assumed to be at the root of the project directory. The file have been placed in the project directory for convenience.

- The list of feature is displayed after each command termination to help the user.

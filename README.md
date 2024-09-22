# Hexdump Utility

## Overview

This project is a command-line utility written in Rust that reads a file and outputs its contents in little-endian hexadecimal format (hexdump). The program allows an optional argument to limit the number of bytes read from the file. It is modeled after the Linux implementation of hexdump.

## Features
- Outputs the content of a file in a hexadecimal format, similar to the Linux `hexdump` command.
- Supports reading the full file or a specified number of bytes using the `-n` flag.
- Handles common errors such as invalid arguments or file errors gracefully.
- Includes unit tests for argument parsing and hexdump output.

## Usage

```bash
./hexdump [-n LEN] FILE
```

- `FILE`: Path to the file to be read.
- `-n LEN`: Optional flag to specify the number of bytes to read from the file.

### Examples

1. **Read the entire file:**
    ```bash
    ./hexdump file.txt
    ```
   This will print the entire contents of `file.txt` in hexadecimal format.

2. **Read only the first 100 bytes:**
    ```bash
    ./hexdump -n 100 file.txt
    ```
   This will limit the output to the first 100 bytes of `file.txt`.

### Error Handling
- If incorrect usage is detected (e.g., missing file or `-n` flag without a valid length), an error message is printed and the program exits with status code `1`.
- If an invalid length is provided for the `-n` flag, an error message is shown.

## Example Output

For a file with the following content in bytes: `00 01 02 03`:
```bash
00000000 0100 0302
```

For larger files, the output will be formatted in 16-byte chunks per line.

## Dependencies

This utility depends on:
- `std::env` for argument handling.
- `std::fs::File` for file operations.
- `std::io::{self, Read}` for reading files and handling I/O.

## Installation

1. Install Rust if you haven't already: https://www.rust-lang.org/tools/install
2. Clone the repository:
    ```bash
    git clone https://github.com/Gamacore/rust-hexdump.git
    ```
3. Navigate to the directory:
    ```bash
    cd rust-hexdump
    ```
4. Build the project:
    ```bash
    cargo build --release
    ```
5. Run the executable:
    ```bash
    ./target/release/hexdump [-n LEN] FILE
    ```

## Running Tests

This project includes a suite of unit tests to ensure the correctness of the `hexdump` function and argument parsing. To run the tests, use:

```bash
cargo test
```

## License

This project is licensed under the MIT License.

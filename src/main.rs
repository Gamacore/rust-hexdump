use std::env; // Environment
use std::fs::File; // File Handling
use std::io::{self, Read}; // I/O operations

#[derive(Debug, PartialEq)]
// Defining custom errors to handle argument parsing errors
enum ArgError {
    InvalidUsage,  // Error for incorrect usage of CLI arguments
    InvalidLength, // Error for invalid length argument
}

fn main() -> io::Result<()> {
    // Collect CLI args
    let args: Vec<String> = env::args().collect();

    // Parse the args and handle errors
    let (filename, max_bytes) = match parse_args(&args) {
        Ok(result) => result, // On success, return parsed result
        Err(ArgError::InvalidUsage) => {
            // Display usage message if the argument format is incorrect
            eprintln!("Usage: {} [-n LEN] FILE", args[0]);
            std::process::exit(1);
        }
        Err(ArgError::InvalidLength) => {
            // Display error if the length argument is invalid
            eprintln!("Invalid length argument");
            std::process::exit(1);
        }
    };

    // Open the file based on the parsed filename
    let mut file = File::open(filename)?;

    // Buffer to hold file content
    let mut buffer = Vec::new();

    // Determine whether to read the whole file to limit by 'max_bytes'
    let bytes_read = match max_bytes {
        Some(len) => file.take(len as u64).read_to_end(&mut buffer)?, // Limit bytes
        None => file.read_to_end(&mut buffer)?, // Read entire file if no length is provided
    };

    // Call 'hexdump' function to convert the file content to hexadecimal format
    let output = hexdump(&buffer[..bytes_read])?;
    print!("{}", output);

    Ok(())
}

// Function to parse CLI arguments
fn parse_args(args: &[String]) -> Result<(&str, Option<usize>), ArgError> {
    match args.len() {
        2 => Ok((&args[1], None)), // If only filename is provided, no byte limit
        4 if args[1] == "-n" => {
            // Parse length arguemnt and ensure it's a valid number
            let len = args[2].parse().map_err(|_| ArgError::InvalidLength)?;
            Ok((&args[3], Some(len))) // Return filename and length if valid
        }
        _ => Err(ArgError::InvalidUsage), // Error for incorrect usage
    }
}

// Function to convert the file content into a hexadecimal dump format
fn hexdump<R: Read>(mut reader: R) -> io::Result<String> {
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?; // Read the content into buffer

    let mut output = String::new(); // String to store the final output

    // Process buffer in chunks of 16 bytes
    for (i, chunk) in buffer.chunks(16).enumerate() {
        output.push_str(&format!("{:08x}", i * 16)); // Print address offset

        // Handle bytes in pairs for better readability. Can extend function to include big_endian format
        for pair in chunk.chunks(2) {
            output.push(' ');
            match pair.len() {
                2 => output.push_str(&format!("{:02x}{:02x}", pair[1], pair[0])), // Reverse byte order for little_endian format
                1 => output.push_str(&format!("{:02x}", pair[0])), // Handle single bytes
                _ => unreachable!(),                               // Sanity check
            }
        }

        output.push('\n'); // Formatting (newline after each 16-byte chunk)
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_hexdump_empty() {
        // Test case for an empty input file
        let input = Cursor::new(vec![]);
        assert_eq!(hexdump(input).unwrap(), ""); // Expect empty string
    }

    #[test]
    fn test_hexdump_single_line() {
        // Test case for a small file that fits on a single line of output
        let input = Cursor::new(vec![0x00, 0x01, 0x02, 0x03]);
        assert_eq!(hexdump(input).unwrap(), "00000000 0100 0302\n"); // Expected hex format
    }

    #[test]
    fn test_hexdump_multiple_lines() {
        // Test case for a file with multiple lines of hexdump
        let input = Cursor::new((0..32).collect::<Vec<u8>>());
        let expected = "\
            00000000 0100 0302 0504 0706 0908 0b0a 0d0c 0f0e\n\
            00000010 1110 1312 1514 1716 1918 1b1a 1d1c 1f1e\n";
        assert_eq!(hexdump(input).unwrap(), expected); // Expected hex format
    }

    #[test]
    fn test_hexdump_partial_line() {
        // Test case for a file with a partial final line
        let input = Cursor::new((0..20).collect::<Vec<u8>>());
        let expected = "\
            00000000 0100 0302 0504 0706 0908 0b0a 0d0c 0f0e\n\
            00000010 1110 1312\n";
        assert_eq!(hexdump(input).unwrap(), expected); // Expected hex format
    }

    #[test]
    fn test_parse_args_file_only() {
        // Test case for argument parsing with only a file
        let args = vec!["program".to_string(), "file.txt".to_string()];
        assert_eq!(parse_args(&args), Ok(("file.txt", None)));
    }

    #[test]
    fn test_parse_args_with_length() {
        // Test case for argument parsing with a length argument
        let args = vec![
            "program".to_string(),
            "-n".to_string(),
            "100".to_string(),
            "file.txt".to_string(),
        ];
        assert_eq!(parse_args(&args), Ok(("file.txt", Some(100))));
    }

    #[test]
    fn test_parse_args_invalid_usage() {
        // Test case for invalid usage of arguments
        let args = vec![
            "program".to_string(),
            "-n".to_string(),
            "file.txt".to_string(),
        ];
        assert_eq!(parse_args(&args), Err(ArgError::InvalidUsage)); // Expect usage error
    }

    #[test]
    fn test_parse_args_invalid_length() {
        // Test case for invalid length argument
        let args = vec![
            "program".to_string(),
            "-n".to_string(),
            "not_a_number".to_string(),
            "file.txt".to_string(),
        ];
        assert_eq!(parse_args(&args), Err(ArgError::InvalidLength)); // Expect length error
    }
}

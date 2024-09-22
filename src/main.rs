use std::env;
use std::fs::File;
use std::io::{self, Read};

#[derive(Debug, PartialEq)]
enum ArgError {
    InvalidUsage,
    InvalidLength,
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let (filename, max_bytes) = match parse_args(&args) {
        Ok(result) => result,
        Err(ArgError::InvalidUsage) => {
            eprintln!("Usage: {} [-n LEN] FILE", args[0]);
            std::process::exit(1);
        }
        Err(ArgError::InvalidLength) => {
            eprintln!("Invalid length argument");
            std::process::exit(1);
        }
    };

    let mut file = File::open(filename)?;

    let mut buffer = Vec::new();
    let bytes_read = match max_bytes {
        Some(len) => file.take(len as u64).read_to_end(&mut buffer)?,
        None => file.read_to_end(&mut buffer)?,
    };

    let output = hexdump(&buffer[..bytes_read])?;
    print!("{}", output);

    Ok(())
}

fn parse_args(args: &[String]) -> Result<(&str, Option<usize>), ArgError> {
    match args.len() {
        2 => Ok((&args[1], None)),
        4 if args[1] == "-n" => {
            let len = args[2].parse().map_err(|_| ArgError::InvalidLength)?;
            Ok((&args[3], Some(len)))
        }
        _ => Err(ArgError::InvalidUsage),
    }
}

fn hexdump<R: Read>(mut reader: R) -> io::Result<String> {
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let mut output = String::new();
    for (i, chunk) in buffer.chunks(16).enumerate() {
        output.push_str(&format!("{:08x}", i * 16));

        for pair in chunk.chunks(2) {
            output.push(' ');
            match pair.len() {
                2 => output.push_str(&format!("{:02x}{:02x}", pair[1], pair[0])),
                1 => output.push_str(&format!("{:02x}", pair[0])),
                _ => unreachable!(),
            }
        }

        output.push('\n');
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_hexdump_empty() {
        let input = Cursor::new(vec![]);
        assert_eq!(hexdump(input).unwrap(), "");
    }

    #[test]
    fn test_hexdump_single_line() {
        let input = Cursor::new(vec![0x00, 0x01, 0x02, 0x03]);
        assert_eq!(hexdump(input).unwrap(), "00000000 0100 0302\n");
    }

    #[test]
    fn test_hexdump_multiple_lines() {
        let input = Cursor::new((0..32).collect::<Vec<u8>>());
        let expected = "\
            00000000 0100 0302 0504 0706 0908 0b0a 0d0c 0f0e\n\
            00000010 1110 1312 1514 1716 1918 1b1a 1d1c 1f1e\n";
        assert_eq!(hexdump(input).unwrap(), expected);
    }

    #[test]
    fn test_hexdump_partial_line() {
        let input = Cursor::new((0..20).collect::<Vec<u8>>());
        let expected = "\
            00000000 0100 0302 0504 0706 0908 0b0a 0d0c 0f0e\n\
            00000010 1110 1312\n";
        assert_eq!(hexdump(input).unwrap(), expected);
    }

    #[test]
    fn test_parse_args_file_only() {
        let args = vec!["program".to_string(), "file.txt".to_string()];
        assert_eq!(parse_args(&args), Ok(("file.txt", None)));
    }

    #[test]
    fn test_parse_args_with_length() {
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
        let args = vec![
            "program".to_string(),
            "-n".to_string(),
            "file.txt".to_string(),
        ];
        assert_eq!(parse_args(&args), Err(ArgError::InvalidUsage));
    }

    #[test]
    fn test_parse_args_invalid_length() {
        let args = vec![
            "program".to_string(),
            "-n".to_string(),
            "not_a_number".to_string(),
            "file.txt".to_string(),
        ];
        assert_eq!(parse_args(&args), Err(ArgError::InvalidLength));
    }
}

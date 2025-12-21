use std::error::Error;
use std::fmt::Display;

// 1. Finish the definition

#[derive(Debug, PartialEq, Eq)]
pub enum ParsePercentageError {
    InvalidInput,
    OutOfRange,
}

// 2. Implement the `Error` trait
impl Error for ParsePercentageError {}

impl Display for ParsePercentageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid input")
    }
}

pub fn parse_percentage(input: &str) -> Result<u8, ParsePercentageError> {
    // 3. Implement this function
    match input.parse::<u8>() {
        Ok(num) => {
            if num > 100 {
                Err(ParsePercentageError::OutOfRange)
            } else {
                Ok(num)
            }
        }
        Err(_) => Err(ParsePercentageError::InvalidInput),
    }
}

// Example usage
pub fn main() {
    let result = parse_percentage("50");
    println!("{:?}", result); // Should print: Ok(50)

    let result = parse_percentage("101");
    println!("{:?}", result); // Should print: Err(ParsePercentageError::OutOfRange)

    let result = parse_percentage("abc");
    println!("{:?}", result); // Should print: Err(ParsePercentageError::InvalidInput)
}

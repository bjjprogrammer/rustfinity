pub fn parse_percentage(input: &str) -> Result<u8, String> {
    // TODO: Implement the function here
    match input.parse::<u8>() {
        Ok(num) => {
            if num > 100 {
                Err("Percentage out of range".to_string())
            } else {
                Ok(num)
            }
        }
        Err(_) => Err("Invalid input".to_string()),
    }
}

// Example usage
pub fn main() {
    let result = parse_percentage("50");
    assert_eq!(result, Ok(50));

    let result = parse_percentage("101");
    assert_eq!(result, Err("Percentage out of range".to_string()));

    let result = parse_percentage("abc");
    assert_eq!(result, Err("Invalid input".to_string()));
}

// src/cli.rs

fn parse_args(args: Vec<String>) -> Option<u32> {
    // TODO: Consider using a defaults module or struct for default values
    match args.len() {
        1 => Some(600), // Default to 10 minutes if no duration provided
        2 => args[1].parse::<u32>().ok(), // Parse 2nd argument as u32
        _ => None, // Invalid number of arguments
    }
}

// ============ Unit Tests =============
#[cfg(test)]
mod tests {
    #[test]
    fn parse_args_extracts_second_duration() {
        // Test: prase_args should extract duration from CLI args
        let args = vec!["timeterm".to_string(), "30".to_string()];
        assert_eq!(super::parse_args(args), Some(30));
        let args = vec!["timeterm".to_string(), "4294967295".to_string()];
        assert_eq!(super::parse_args(args), Some(4294967295));
    }

    #[test]
    fn parse_args_defaults_10min() {
        // Test: parse_args should default to 10 minutes if no args
        let args = vec!["timeterm".to_string()];
        assert_eq!(super::parse_args(args), Some(600));
    }
}

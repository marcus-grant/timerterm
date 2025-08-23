// src/cli.rs

fn parse_args(args: Vec<String>) -> Option<u32> {
    // Check if we have exactly 2 arguments (program name + duration)
    if args.len() == 2 {
        // Try to parse the 2nd arg as number
        args[1].parse().ok()
    }
    else {
        None
    }
}

// ============ Unit Tests =============
#[cfg(test)]
mod tests {
    #[test]
    fn parse_args_extracts_duration() {
        // Test: prase_args should extract duration from CLI args
        let args = vec!["timeterm".to_string(), "30".to_string()];
        assert_eq!(super::parse_args(args), Some(30));
    }
}

// src/cli.rs

fn parse_time_fmt(time_str: &str) -> Option<u32> {
    // Handle ss format
    if !time_str.contains(':') { return time_str.parse().ok(); }

    let time_units: Vec<&str> = time_str.split(':').collect();

    match time_units.len() {
        2 => { // handle mm:ss format
            let mins = time_units[0].parse::<u32>().ok()?;
            let secs = time_units[1].parse::<u32>().ok()?;
            Some(mins * 60 + secs)
        }
        3 => { // handle hh:mm:ss format
            let hrs = time_units[0].parse::<u32>().ok()?;
            let mins = time_units[1].parse::<u32>().ok()?;
            let secs = time_units[2].parse::<u32>().ok()?;
            Some(hrs * 3600 + mins * 60 + secs)
        }
        _ => None, // Invalid format
    }
}

pub fn parse_args(args: Vec<String>) -> Option<u32> {
    // TODO: Consider using a defaults module or struct for default values
    match args.len() {
        1 => Some(600), // Default to 10 minutes if no duration provided
        2 => parse_time_fmt(&args[1]), // Parse 2nd argument as u32
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

    #[test]
    fn parse_time_fmt_handles_secs_only() {
        // Test: strings of ss only returns that number of seconds in u32
        assert_eq!(super::parse_time_fmt("69420"), Some(69420));
    }

    #[test]
    fn parse_time_fmt_handles_mins_secs() {
        // Test: "mm:ss" format should return (60 * mm) + ss seconds
        assert_eq!(super::parse_time_fmt("1:36"), Some(96));
        assert_eq!(super::parse_time_fmt("100:01"), Some(6001));
    }

    #[test]
    fn parse_time_fmt_handles_hrs_mins_secs() {
    // Test: "1:30:45" should parse to 5445 seconds (1*3600 + 30*60 + 45)
    assert_eq!(super::parse_time_fmt("1:30:45"), Some(5445));
    // Test: "0:00:30" should parse to 30 seconds  
    assert_eq!(super::parse_time_fmt("0:00:30"), Some(30));
    // Test: "2:15:00" should parse to 8100 seconds (2*3600 + 15*60)
    assert_eq!(super::parse_time_fmt("2:15:00"), Some(8100));
    }

    // TODO: Need leading zero tests for ss, mm:ss, hh:mm:ss formats
}

use chrono::Duration;
use std::{sync::Arc, thread};

use crate::profile::Profile;

pub fn run(duration: Duration, profile: Arc<Profile>) {
    thread::spawn(move || {
        tracing::info!("spawned scheduler thread");
        loop {
            thread::sleep(duration.to_std().unwrap());
            profile.move_ram_to_disk();
        }
    });
}

pub fn parse_duration(every: &str) -> Option<Duration> {
    let mut total_seconds = 0;

    for part in every.split(' ') {
        let (t, u) = if let Some(index) = part.find(|c: char| !c.is_numeric()) {
            part.split_at(index)
        } else {
            return None;
        };

        let value: i64 = t.parse().unwrap();

        match u {
            "hour" | "hours" | "h" => total_seconds += value * 3600,
            "mins" | "min" | "m" => total_seconds += value * 60,
            "secs" | "sec" | "s" => total_seconds += value,
            _ => return None,
        }
    }

    assert!(
        total_seconds > 60,
        "total seconds need to be over 60 seconds"
    );

    tracing::debug!(?total_seconds, "parsed duration");
    Some(Duration::seconds(total_seconds))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_duration_hours_minutes_seconds() {
        let input = "1h 15mins 30secs";
        let expected_duration = Duration::hours(1) + Duration::minutes(15) + Duration::seconds(30);
        assert_eq!(parse_duration(input), Some(expected_duration));
    }

    #[test]
    fn test_parse_duration_minutes_seconds() {
        let input = "10mins 45secs";
        let expected_duration = Duration::minutes(10) + Duration::seconds(45);
        assert_eq!(parse_duration(input), Some(expected_duration));
    }

    #[test]
    fn test_parse_duration_seconds() {
        let input = "120secs";
        let expected_duration = Duration::seconds(120);
        assert_eq!(parse_duration(input), Some(expected_duration));
    }

    #[test]
    fn test_parse_duration_invalid_format() {
        let input = "1hour 15mins 30seconds"; // Invalid unit
        assert_eq!(parse_duration(input), None);
    }

    #[test]
    fn test_parse_duration_empty_input() {
        let input = "";
        assert_eq!(parse_duration(input), None);
    }

    #[test]
    fn test_parse_duration_invalid_value() {
        let input = "1hour 15mins 30";
        assert_eq!(parse_duration(input), None); // Missing unit for last part
    }
}

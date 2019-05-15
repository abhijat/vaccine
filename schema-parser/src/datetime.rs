use chrono::Utc;
use chrono_tz::Tz;
use humantime::parse_duration;

use crate::datetime::Direction::{Ahead, Behind, Here};

#[derive(Debug, PartialEq)]
enum Direction {
    Here,
    Ahead,
    Behind,
}

pub fn datetime_from_now(date_string: &str, tzname: &str) -> chrono::DateTime<Tz> {
    if is_now(date_string) {
        let now = get_now(tzname);
        let direction = get_direction(date_string);
        match direction {
            None => panic!(format!("invalid date string {}", date_string)),
            Some(Here) => now,
            _ => shift_date(now, get_offset(date_string), direction.unwrap()),
        }
    } else {
        panic!("string passed in was not nowable!");
    }
}

pub fn is_now(s: &str) -> bool {
    s.starts_with("now")
}

fn get_direction(s: &str) -> Option<Direction> {
    if s == "now" {
        Some(Here)
    } else if s.starts_with("now + ") {
        Some(Ahead)
    } else if s.starts_with("now - ") {
        Some(Behind)
    } else {
        None
    }
}

fn get_now(tzname: &str) -> chrono::DateTime<Tz> {
    let tz: Tz = tzname.parse().expect(&format!("failed to parse timezone {}", tzname));
    Utc::now().with_timezone(&tz)
}

fn get_offset(s: &str) -> chrono::Duration {
    let duration: String = s.chars().skip("now . ".len()).collect();

    let duration = parse_duration(&duration)
        .expect(&format!("failed to parse duration {}", duration));

    chrono::Duration::from_std(duration)
        .expect("failed to parse chrono duration from std::duration")
}

fn shift_date(d: chrono::DateTime<Tz>, offset: chrono::Duration, direction: Direction)
              -> chrono::DateTime<Tz> {
    match direction {
        Direction::Here => d,
        Direction::Ahead => d + offset,
        Direction::Behind => d - offset,
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Datelike, Timelike, Utc};
    use chrono_tz::{Asia, US};

    use super::*;

    #[test]
    fn test_get_now_has_correct_timezone() {
        let d = get_now("Asia/Kolkata");
        assert_eq!(d.timezone(), Asia::Kolkata);

        let d = get_now("US/Pacific");
        assert_eq!(d.timezone(), US::Pacific);
    }

    #[test]
    fn test_get_now_has_correct_date_and_time() {
        let d = get_now("Asia/Kolkata");
        let now = Utc::now().with_timezone(&Asia::Kolkata);

        assert_eq!(d.date(), now.date());
        assert_eq!(d.hour(), now.hour());
        assert_eq!(d.minute(), now.minute());
    }

    #[test]
    fn test_get_offset() {
        let duration = get_offset("now + 1h");
        assert_eq!(duration.num_hours(), 1);

        let duration = get_offset("now + 1h 1m");
        assert_eq!(duration.num_minutes(), 61);

        let duration = get_offset("now - 1s");
        assert_eq!(duration.num_seconds(), 1);
    }

    #[test]
    fn test_directions() {
        let d = get_direction("now + foo");
        assert_eq!(d.unwrap(), Ahead);

        let d = get_direction("now - bar");
        assert_eq!(d.unwrap(), Behind);

        let d = get_direction("now");
        assert_eq!(d.unwrap(), Here);

        assert!(get_direction("foobar").is_none());
    }

    #[test]
    fn parse_now_plus_one_offset() {
        let now = Utc::now().with_timezone(&Asia::Kolkata);
        let d = datetime_from_now("now + 1h", "Asia/Kolkata");

        assert_eq!(now.date(), d.date());
        assert_eq!(now.hour(), d.hour() - 1);

        let d = datetime_from_now("now + 1month", "Asia/Kolkata");
        assert_eq!(d.month(), now.month() + 1);

        let d = datetime_from_now("now", "Asia/Kolkata");
        assert_eq!(now.date(), d.date());
    }

    #[test]
    fn parse_now() {
        let now = Utc::now().with_timezone(&Asia::Kolkata);
        let d = datetime_from_now("now", "Asia/Kolkata");
        assert_eq!(now.date(), d.date());
    }
}

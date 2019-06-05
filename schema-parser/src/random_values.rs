use chrono::{TimeZone, Utc};
use chrono_tz::Tz;
use rand::distributions::Alphanumeric;
use rand::Rng;
use rand::seq::{SliceChooseIter, SliceRandom};

pub fn generate_random_string() -> String {
    let mut r = rand::thread_rng();
    let size = r.gen_range(5, 20);
    r.sample_iter(&Alphanumeric)
        .take(size)
        .collect()
}

pub fn generate_random_number() -> i64 {
    let mut r = rand::thread_rng();
    r.gen()
}

pub fn generate_random_float() -> f64 {
    let mut r = rand::thread_rng();
    r.gen()
}

pub fn generate_random_boolean() -> bool {
    let mut r = rand::thread_rng();
    r.gen::<bool>()
}

fn generate_number_in_range(start: i64, stop: i64) -> i64 {
    let mut r = rand::thread_rng();
    r.gen_range(start, stop)
}

pub fn generate_random_datetime(format_string: &str, timezone: &str) -> String {
    let tz: Tz = timezone.parse().expect("failed to parse timezone!");
    let now = Utc::now().with_timezone(&tz);
    let delta = chrono::Duration::days(generate_number_in_range(-500000, 50000));
    let random_datetime = now + delta;
    random_datetime.format(format_string).to_string()
}

pub fn random_elements<T>(items: &Vec<T>) -> SliceChooseIter<[T], T> {
    let mut r = rand::thread_rng();
    let count = r.gen_range(1, items.len() + 1);
    items.choose_multiple(&mut r, count)
}


#[cfg(test)]
mod tests {
    use chrono::{Duration, NaiveDateTime, TimeZone, Utc};
    use chrono_tz::Tz;

    use super::*;

    #[test]
    fn size_of_random_choices_from_vector() {
        let names = vec!["foo", "bar", "baz"];
        for _i in 0..100 {
            let p: Vec<&str> = random_elements(&names)
                .map(|v| *v)
                .collect();
            assert!(p.len() >= 1 && p.len() <= names.len());
        }
    }

    #[test]
    fn randomized_datetimes_have_expected_timezone() {
        let r = generate_random_datetime("%Z", "Asia/Kolkata");
        assert_eq!(r, "IST");

        let r = generate_random_datetime("%Z", "US/Pacific");
        assert_eq!(r, "PDT");
    }
}

use rand::distributions::Alphanumeric;
use rand::Rng;

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

pub fn generate_random_boolean() -> bool {
    let mut r = rand::thread_rng();
    r.gen::<bool>()
}

use rand::distributions::Alphanumeric;
use rand::Rng;
use rand::seq::{SliceRandom, SliceChooseIter};

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

pub fn random_elements_from_collection<T>(items: &Vec<T>) -> SliceChooseIter<[T], T> {
    let mut r = rand::thread_rng();
    let count = r.gen_range(1, items.len() + 1);
    items.choose_multiple(&mut r, count)
}

use std::sync::OnceLock;

use rand::seq::SliceRandom;

const RAW: &str = include_str!("../assets/en.txt");

fn dictionary() -> &'static [&'static str] {
    static DICT: OnceLock<Vec<&'static str>> = OnceLock::new();
    DICT.get_or_init(|| RAW.lines().filter(|line| !line.is_empty()).collect())
}

pub fn sample(count: usize) -> Vec<&'static str> {
    let dict = dictionary();
    let mut rng = rand::thread_rng();
    (0..count)
        .map(|_| *dict.choose(&mut rng).expect("dictionary is not empty"))
        .collect()
}

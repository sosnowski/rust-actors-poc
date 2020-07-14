use rand::{ Rng, thread_rng };
use rand::distributions::Alphanumeric;

pub fn uuid() -> String {
    return thread_rng()
        .sample_iter(&Alphanumeric)
        .take(20)
        .collect();
}
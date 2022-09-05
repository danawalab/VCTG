use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

pub fn generate_address() -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();

    rand_string.to_string()
}

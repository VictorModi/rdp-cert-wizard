use rand::distr::Alphanumeric;
use rand::Rng;

pub fn generate_strong_password(len: usize) -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

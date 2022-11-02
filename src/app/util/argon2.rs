use argon2::Config;
use rand::{distributions::Alphanumeric, Rng};

pub fn hash_password(password: &str) -> String {
    let salt = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect::<String>();
    let config = Config::default();
    let hash = argon2::hash_encoded(password.as_bytes(), &salt.as_bytes(), &config).unwrap();

    return hash;
}

pub fn matches(hash: &str, password: &str) -> bool {
    let matches = argon2::verify_encoded(hash, password.as_bytes()).unwrap();
    return matches;
}

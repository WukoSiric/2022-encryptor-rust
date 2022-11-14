use bcrypt::{DEFAULT_COST, hash, verify, hash_with_salt};
struct User {
    username: String,
    password: String,
    salt: [u8; 16],
}

impl User {
    fn new(username: String, password: String) -> User {
        let salt = generate_salt();
        let cost = DEFAULT_COST;
        let password = hash_with_salt(password.as_bytes(), cost, salt).unwrap().to_string();
        User {
            username,
            password,
            salt,
        }
    }
}
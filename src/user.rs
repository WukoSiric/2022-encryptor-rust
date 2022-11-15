use bcrypt::{DEFAULT_COST, hash, verify, hash_with_salt};
use hash_utils;
struct User {
    username: String,
    password: String,
    salt: [u8; 16],
}

impl User {
    fn new(username: String, password: String) -> User {
        let cost = DEFAULT_COST;
        let salt = generate_salt();
        let password = hash_with_salt(password.as_bytes(), cost, salt).unwrap().to_string();
        User {
            username,
            password,
            salt,
        }
    }
}

#[test]
fn can_create_user() {
    let username = String::from("test_user");
    let password = String::from("password");
    let user = User::new(username.clone(), password.clone());
    assert_eq!(user.username, username);
    assert_eq!(user.salt.len(), 16);
}
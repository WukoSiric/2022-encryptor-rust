use bcrypt::{DEFAULT_COST, hash, verify, hash_with_salt};
use rand::Rng; //Choosing random word 
// use std::collections::HashSet; 

fn main() {
    println!("Hello, world!");
}

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

fn generate_salt() -> [u8; 16] {
    let mut salt: [u8; 16] = [0; 16];
    // Generating random numbers 
    for n in 0..15 {
        let rng = rand::thread_rng().gen_range(0..16);
        salt[n] = rng;
    }

    return salt
}

fn login(username: &String, password: &String, user: &User) -> bool {
    let salt = user.salt;
    let password = hash_with_salt(password.as_bytes(), DEFAULT_COST, salt).unwrap().to_string();

    if (username == &user.username) && (password == user.password) {
        return true
    }

    return false;
}

#[test]
fn can_hash() {
    let cost: u32 = 5;
    let test_password = "password";
    let hashed = hash(test_password, cost).unwrap();
    let valid = verify(test_password, &hashed);
    assert!(valid.unwrap());
}

#[test]
fn can_hash_different_cost() {
    let cost: u32 = 5;
    let test_password = "password";
    let hashed = hash(test_password, cost).unwrap();
    let cost: u32 = 10;
    let hashed2 = hash(test_password, cost).unwrap();
    assert_ne!(hashed, hashed2);
}

#[test]
fn can_generate_unique_salts_with_proper_lengths() {
    let salt = generate_salt();
    let salt2 = generate_salt();
    // println!("salt1: {:?}", salt);
    // println!("salt2: {:?}", salt2);
    assert_ne!(salt, salt2);
    assert_eq!(salt.len(), 16);
    assert_eq!(salt2.len(), 16);
}

#[test]
fn can_create_user() {
    let username = String::from("test_user");
    let password = String::from("password");
    let user = User::new(username.clone(), password.clone());
    // println!("User: {}", user.username);
    // println!("Password: {}", user.password);
    // println!("salt: {:?}", user.salt);
    assert_eq!(user.username, username);
    assert_eq!(user.salt.len(), 16);
}

#[test]
fn can_login_correctly() {
    let username = String::from("test_user");
    let password = String::from("password");
    let user = User::new(username.clone(), password.clone());

    let login_result = login(&username, &password, &user);
    assert_eq!(login_result, true);
}

#[test]
fn can_login_incorrectly() {
    let username = String::from("test_user");
    let password = String::from("password");
    let incorrect_password = String::from("incorrect_password");
    let user = User::new(username.clone(), password.clone());

    let login_result = login(&username, &incorrect_password, &user);
    assert_eq!(login_result, false);
}

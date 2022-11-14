use rand::Rng; //Choosing random word 
// use std::collections::HashSet; 

fn main() {
    println!("Hello, world!");
}





// #[test]
// fn can_hash() {
//     let cost: u32 = 5;
//     let test_password = "password";
//     let hashed = hash(test_password, cost).unwrap();
//     let valid = verify(test_password, &hashed);
//     assert!(valid.unwrap());
// }

// #[test]
// fn can_hash_different_cost() {
//     let cost: u32 = 5;
//     let test_password = "password";
//     let hashed = hash(test_password, cost).unwrap();
//     let cost: u32 = 10;
//     let hashed2 = hash(test_password, cost).unwrap();
//     assert_ne!(hashed, hashed2);
// }

// #[test]
// fn can_generate_unique_salts_with_proper_lengths() {
//     let salt = generate_salt();
//     let salt2 = generate_salt();
//     // println!("salt1: {:?}", salt);
//     // println!("salt2: {:?}", salt2);
//     assert_ne!(salt, salt2);
//     assert_eq!(salt.len(), 16);
//     assert_eq!(salt2.len(), 16);
// }

// #[test]
// fn can_create_user() {
//     let username = String::from("test_user");
//     let password = String::from("password");
//     let user = User::new(username.clone(), password.clone());
//     assert_eq!(user.username, username);
//     assert_eq!(user.salt.len(), 16);
// }

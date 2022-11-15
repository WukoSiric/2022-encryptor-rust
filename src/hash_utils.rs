use rand::Rng; //Choosing random word 

pub fn generate_salt() -> [u8; 16] {
    let mut salt: [u8; 16] = [0; 16];

    for n in 0..15 {
        let rng = rand::thread_rng().gen_range(0..16);
        salt[n] = rng;
    }

    return salt
}

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};

pub fn hash_password(password: &String) -> Result<String, String> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    match argon2.hash_password(password.as_bytes(), &salt) {
        Ok(password) => Ok(password.to_string()),
        Err(err) => Err(format!("Couldn't hash password, error: {:?}", err)),
    }
}

pub fn verify_password(password: &String, password_hash: &String) -> bool {
    let parsed_hash = match PasswordHash::new(&password_hash) {
        Ok(password) => password,
        Err(_) => return false,
    };

    match Argon2::default().verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => true,
        Err(_) => false,
    }
}

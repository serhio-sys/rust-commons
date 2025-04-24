use pwhash::bcrypt::{ self, BcryptSetup };

pub fn verify_password(hash: &str, password: &str) -> bool {
    bcrypt::verify(password, hash)
}

pub fn hash_password(password: &str) -> Result<String, pwhash::error::Error> {
    bcrypt::hash_with(
        BcryptSetup {
            variant: Some(bcrypt::BcryptVariant::V2b),
            cost: Some(5),
            ..Default::default()
        },
        password
    )
}

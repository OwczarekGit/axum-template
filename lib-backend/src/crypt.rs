use bcrypt::BcryptError;

pub trait Hashable {
    fn hash(&self, cost: u32) -> Result<String, BcryptError>;
    fn verify(&self, provided: &str) -> Result<bool, BcryptError>;
}

impl Hashable for String {
    fn hash(&self, cost: u32) -> Result<String, BcryptError> {
        hash(&self, cost)
    }

    fn verify(&self, provided: &str) -> Result<bool, BcryptError> {
        verify(&self, provided)
    }
}

pub fn hash(value: &str, cost: u32) -> Result<String, BcryptError> {
    bcrypt::hash(value, cost)
}

pub fn verify(value: &str, hash: &str) -> Result<bool, BcryptError> {
    bcrypt::verify(value, hash)
}

#[cfg(test)]
mod test {
    use super::*;
    const COST: u32 = 8;

    #[test]
    fn string_hash_and_hash_fn_are_the_same() {
        let password = "My_secret-pa$$word:1234".to_string();

        let hash = hash(&password, COST).unwrap();
        let hash2 = password.hash(COST).unwrap();

        assert_eq!(
            password.verify(&hash).unwrap(),
            password.verify(&hash2).unwrap()
        );
    }
}

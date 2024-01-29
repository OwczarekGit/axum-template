pub fn hash<E: From<bcrypt::BcryptError>>(value: &str, cost: u32) -> Result<String, E> {
    Ok(bcrypt::hash(value, cost)?)
}

pub fn verify<E: From<bcrypt::BcryptError>>(value: &str, hash: &str) -> Result<bool, E> {
    Ok(bcrypt::verify(value, hash)?)
}

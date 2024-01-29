pub fn get_env(key: &str) -> String {
    dotenvy::var(key).unwrap_or_else(|_| panic!("The variable '{key}' was not set in env."))
}

pub trait FromEnvVar
where
    Self: Sized,
{
    fn var(key: &str) -> String {
        get_env(key)
    }

    fn from_env(key: &str) -> Self;
}

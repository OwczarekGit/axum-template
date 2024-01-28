pub fn read_env(key: &str) -> String {
    dotenvy::var(key).expect(&format!("{key} was not set in env."))
}

pub trait FromEnvVar
where
    Self: Sized
{
    fn var(key: &str) -> String {
        read_env(key)
    }

    fn from_env(key: &str) -> Self;
}
use std::env;

pub fn get_env_var(name: &str) -> String {
    env::var(name).unwrap_or(String::from(""))
}

use std::env;

pub fn get_env_var(name: &str) -> String {
    if let Ok(value) = env::var(name) {
        String::from(value)
    } else {
        String::from("")
    }
}

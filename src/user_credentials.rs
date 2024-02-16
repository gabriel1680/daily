use crate::utils::get_env_var;

pub struct UserCredentials {
    pub password: String,
    pub username: String,
}

pub fn make_credentials() -> UserCredentials {
    let username = get_env_var("API_USERNAME");
    let password = get_env_var("API_PASSWORD");
    UserCredentials { username, password }
}

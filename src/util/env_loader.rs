use dotenv::dotenv;
use std::env;
pub fn load_env() {
    //load env
    let env_mode = env::var("ENV_MODE").unwrap_or_else(|_| "development".to_string());

    match env_mode.as_str() {
        "production" => dotenv::from_filename(".env.production").ok(),
        "development" => dotenv::from_filename(".env.development").ok(),
        _ => dotenv().ok(),
    };
}

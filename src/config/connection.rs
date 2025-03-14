use dotenv::dotenv;
use log::{info, warn};
use std::env;

pub fn set_environment_variable(env_var: &str, default_value: &str) -> String {
    dotenv().ok();
    match env::var(env_var) {
        Ok(var_in_env_file) => {
            info!("Loaded {} env var", &env_var);
            var_in_env_file
        }
        Err(e) => {
            warn!("Error reading variable {}: {}. Using default.", &env_var, e);
            default_value.to_string()
        }
    }
}

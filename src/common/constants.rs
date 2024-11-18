use std::env;

pub fn get_base_directory() -> String {
    env::var("XDG_CONFIG_HOME").unwrap_or(env::var("HOME").unwrap())
}

pub fn get_param_prefix() -> String {
    "--".to_string()
}

use std::{collections::BTreeMap, env};

pub fn get_base_directory() -> String {
    env::var("XDG_CONFIG_HOME").unwrap_or(env::var("HOME").unwrap())
}

pub fn get_param_prefix() -> String {
    "--".to_string()
}

pub fn get_alias_prefix() -> String {
    "-".to_string()
}

pub fn get_param_aliases() -> BTreeMap<String, String> {
    BTreeMap::from([
        ("f".to_string(), "file".to_string()),
        ("k".to_string(), "key".to_string()),
        ("o".to_string(), "output".to_string()),
        ("op".to_string(), "output-path".to_string()),
    ])
}

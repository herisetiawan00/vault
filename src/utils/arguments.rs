use std::{collections::BTreeMap, env};

use crate::common::constants;

pub fn get_arguments() -> BTreeMap<String, String> {
    let args: Vec<String> = env::args().collect();
    let mut mapped_args: BTreeMap<String, String> = BTreeMap::new();
    let param_prefix = constants::get_param_prefix();
    let alias_prefix = constants::get_alias_prefix();
    let aliases = constants::get_param_aliases();

    fn get_arg_value(args: &Vec<String>, index: &usize, prefix: &String) -> String {
        args.get(index + 1).map_or("".to_string(), |v| {
            if v.starts_with(prefix) {
                "".to_string()
            } else {
                v.to_string()
            }
        })
    }

    for (index, arg) in args.iter().enumerate() {
        let value = get_arg_value(&args, &index, &param_prefix);
        if arg.starts_with(&param_prefix) {
            mapped_args.insert(arg[param_prefix.len()..].to_string(), value.to_string());
        } else if arg.starts_with(&alias_prefix) {
            let alias = &arg[1..];
            let key: String = match aliases.get(alias) {
                Some(value) => value.to_string(),
                None => alias.to_string(),
            };
            mapped_args.insert(key, value);
        }
    }

    mapped_args
}

pub fn get_action() -> String {
    let args: Vec<String> = env::args().collect();
    args.get(1).map_or("".to_string(), |v| v.to_string())
}

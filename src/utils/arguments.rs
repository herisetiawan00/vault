use std::{collections::BTreeMap, env};

use crate::common::constants;

pub fn get_arguments() -> BTreeMap<String, String> {
    let args: Vec<String> = env::args().collect();
    let mut mapped_args: BTreeMap<String, String> = BTreeMap::new();
    let param_prefix = constants::get_param_prefix();

    for (index, arg) in args.iter().enumerate() {
        if arg.starts_with(&param_prefix) {
            let value = args.get(index + 1).map_or(r#""#, |v| {
                if v.starts_with(&param_prefix) {
                    r#""#
                } else {
                    v
                }
            });

            mapped_args.insert(arg[param_prefix.len()..].to_string(), value.to_string());
        }
    }

    mapped_args
}

pub fn get_action() -> String {
    let args: Vec<String> = env::args().collect();
    args.get(1).map_or("".to_string(), |v| v.to_string())
}

use env_logger::Env;
use log::{debug, info};
use std::collections::HashMap;

use serde_json::Value;

use keyfile::Field;

fn main() {
    let env = Env::default()
        .filter_or("RS_AUTOKEY_LEVEL", "info")
        .write_style_or("RS_AUTOKEY_STYLE", "always");

    env_logger::init_from_env(env);

    let kwd = include_str!("../resources/kwd.json");

    let kwd: Value = serde_json::from_str(kwd).unwrap();

    for (key, value) in kwd {
        create_struct(key, value);
    }

    info!("io: kwd.json deserialized");
    debug!("{kwd}")
}

fn create_struct(key: String, value: HashMap<String, String>) {}

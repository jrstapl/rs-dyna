use env_logger::Env;
use log::{debug, error, info};
use std::fs;

use serde_json::Value;

fn main() {
    let env = Env::default()
        .filter_or("RS_AUTOKEY_LEVEL", "info")
        .write_style_or("RS_AUTOKEY_STYLE", "always");

    env_logger::init_from_env(env);

    let kwd = include_str!("../resources/kwd.json");

    let kwd: Value = serde_json::from_str(kwd).unwrap();

    debug!("io: kwd.json deserialized");
}

use env_logger::Env;
use log::{debug, info};
use std::collections::HashMap;

use keyfile::Field;

fn main() {
    let env = Env::default()
        .filter_or("RS_AUTOKEY_LEVEL", "info")
        .write_style_or("RS_AUTOKEY_STYLE", "always");

    env_logger::init_from_env(env);

    let kwd = include_str!("../resources/kwd.json").replace("null", "\"\"");

    let kwd: HashMap<String, Vec<HashMap<String, Vec<Field>>>> =
        serde_json::from_str(kwd.as_str()).unwrap();

    let keys: Vec<&String> = kwd.keys().collect();

    // for (key, value) in kwd {
    //     create_struct(key, value);
    // }

    info!("io: kwd.json deserialized");
    debug!("{:?}", keys)
}

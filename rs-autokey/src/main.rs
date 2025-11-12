use env_logger::Env;
use log::{debug, info};
use std::collections::HashMap;

use keyfile::Field;

fn main() {
    let env = Env::default()
        .filter_or("RS_AUTOKEY_LEVEL", "info")
        .write_style_or("RS_AUTOKEY_STYLE", "always");

    env_logger::init_from_env(env);

    let kwd = include_str!("../resources/kwd.json");

    let kwd =
        serde_json::from_str::<HashMap<String, Vec<HashMap<String, Vec<Field>>>>>(kwd).unwrap();

    let keys: Vec<&String> = kwd.keys().collect();

    info!("io: kwd.json deserialized");
    debug!("{:?}", keys);

    for (key, value) in kwd.into_iter() {
        create_keyword_struct(&key, &value)
            .expect(format!("Unable to create keyword card for {}", key).as_str());
    }
}

fn create_keyword_struct(
    keyword: &str,
    data: &Vec<HashMap<String, Vec<Field>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

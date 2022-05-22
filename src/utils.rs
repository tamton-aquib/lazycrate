use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub dependencies: toml::value::Map<String, toml::Value>,
}

pub fn get_crates_from_toml() -> Vec<String> {
    let config: Config = toml::from_str(include_str!("../Cargo.toml")).unwrap();
    // println!("{:?}", config.dependencies);
    config
        .dependencies
        .iter()
        .map(|s| s.0.to_string())
        .collect::<Vec<String>>()
}

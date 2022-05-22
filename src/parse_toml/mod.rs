use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub dependencies: toml::value::Map<String, toml::Value>,
}

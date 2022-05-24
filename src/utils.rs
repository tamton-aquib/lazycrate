use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Package {
    name: String,
    description: String,
    version: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub dependencies: toml::value::Map<String, toml::Value>,
    pub package: Package,
}

pub fn get_crates_from_toml() -> Vec<String> {
    let config: Config = toml::from_str(include_str!("../Cargo.toml")).unwrap();
    // println!("{:?}", config.dependencies);
    config
        .dependencies
        .iter()
        .map(|s| s.0.to_string())
        .collect()
}

pub fn get_packages() -> Vec<String> {
    let cfg: Config = toml::from_str(include_str!("../Cargo.toml")).unwrap();
    format!(
        r#"
    Name: {}
    Description: {}
    Version: {}
    "#,
        cfg.package.name, cfg.package.description, cfg.package.version
    )
    .trim()
    .split('\n')
    .map(|s| s.to_owned())
    .collect()
}

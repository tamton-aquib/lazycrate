use crate::parse_toml::Config;

pub fn get_crates_from_toml() -> Vec<String> {
    let config: Config = toml::from_str(include_str!("../Cargo.toml")).unwrap();
    // println!("{:?}", config.dependencies);
    config
        .dependencies
        .iter()
        .map(|s| format!("{}", s.0))
        .collect::<Vec<String>>()
}

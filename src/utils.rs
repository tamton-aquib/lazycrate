// use crate::app::App;
// use crate::panel::PanelName;
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

pub fn update_status() -> Vec<String> {
    // let item = match app.panels.get(&app.current_panel).unwrap().panel_name {
    // PanelName::Crates => "Crates",
    // PanelName::Package => "Info",
    // PanelName::Commands => "Commands",
    // _ => "Undefined",
    // };
    vec!["test".to_string()]
}

pub fn get_output() -> Vec<String> {
    vec!["output line example".to_owned()]
}

pub fn get_commands() -> Vec<String> {
    let nice = ["clippy", "format", "search", "doc", "tree"];
    nice.map(|s| s.to_owned()).to_vec()
}

pub fn get_crates_from_toml() -> Vec<String> {
    let cfg: Config = toml::from_str(include_str!("../Cargo.toml")).unwrap();
    cfg.dependencies.iter().map(|s| s.0.to_string()).collect()
}

pub fn get_package_info() -> Vec<String> {
    let cfg: Config = toml::from_str(include_str!("../Cargo.toml")).unwrap();
    vec![
        format!("Name: {}", cfg.package.name),
        format!("Description: {}", cfg.package.description),
        format!("Version: {}", cfg.package.version),
    ]
}

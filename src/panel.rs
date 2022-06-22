const CRATES_HELP: &str = r#"
a                       cargo add
"#;

#[derive(PartialEq, Clone, Eq, Hash, Copy)]
pub enum PanelName {
    Status,
    Package,
    Crates,
    Commands,
    Output,
}

pub struct Panel {
    pub index: u8,
    pub panel_name: PanelName,
    pub content: Vec<String>,
}

impl Panel {
    pub fn get_help(&self) -> String {
        let [name, page]: [&str; 2] = match self.panel_name {
            PanelName::Package => ["packages", CRATES_HELP],
            PanelName::Crates => ["crates", CRATES_HELP],
            PanelName::Status => ["status", CRATES_HELP],
            PanelName::Commands => ["commands", CRATES_HELP],
            PanelName::Output => ["output", CRATES_HELP],
        };
        format!("This is a help page for `{}` module!\n{}", name, page)
    }
}

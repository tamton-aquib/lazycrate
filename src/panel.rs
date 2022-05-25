#[derive(PartialEq, Clone)]
pub enum PanelName {
    Status,
    Package,
    Crates,
    Commands,
}

pub struct Panel {
    pub index: u8,
    pub panel_name: PanelName,
    pub content: Vec<String>,
}

impl Panel {
    pub fn get_help(&self) -> String {
        "Noice".to_string()
    }
}

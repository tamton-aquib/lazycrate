#[derive(PartialEq, Clone)]
pub enum PanelName {
    Package,
    Crates,
}

pub struct Panel {
    pub index: u8,
    pub panel_name: PanelName,
    pub content: Vec<String>,
}

impl Panel {
    pub fn get_help(&self) -> String {
        // not a reference to thor+loki
        "Noice".to_string()
    }
}

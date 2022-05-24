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

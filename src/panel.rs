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
        let tp = match self.panel_name {
            PanelName::Package => "packages",
            PanelName::Crates => "crates",
            PanelName::Status => "status",
            PanelName::Commands => "commands",
            PanelName::Output => "output",
        };
        format!("This is a help page for {} module!", tp)
    }
}

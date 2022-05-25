use crate::panel::{Panel, PanelName};

pub enum Mode {
    Normal,
    Insert,
}

pub struct App {
    pub panel: Panel,
    pub cursor: u8,
    pub mode: Mode,
    pub show_popup: bool,
}

impl App {
    pub fn new() -> App {
        App {
            panel: Panel {
                index: 0,
                panel_name: PanelName::Package,
                content: vec![],
            },
            cursor: 0,
            mode: Mode::Normal,
            show_popup: false,
        }
    }

    // TODO: clean this func.
    pub fn cycle_panels(&mut self, dir: bool) {
        let panel_names = [PanelName::Package, PanelName::Crates, PanelName::Commands];
        let pos = panel_names
            .iter()
            .position(|r| r == &self.panel.panel_name)
            .unwrap();
        let direction = if dir { 1 } else { 2 };
        let new_index = (pos + direction) % panel_names.len();
        self.panel.index = new_index as u8;
        self.panel.panel_name = panel_names.get(new_index).unwrap().to_owned();
        self.cursor = 0;
    }
}

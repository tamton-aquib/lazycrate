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

    // FIX: cycling is broken.
    pub fn cycle_panels(&mut self) {
        let panel_names = [PanelName::Crates, PanelName::Package, PanelName::Commands];
        let new_index = (self.panel.index + 1) % (panel_names.len() as u8);
        // let new_index = panel_names[self.panel.index + 1];
        // let new_index = panel_names.get((self.panel.index + 1) as usize).unwrap();
        // if new_index <= (panel_names.len() as u8) {
        let panelname = &panel_names[self.panel.index as usize];
        self.panel.index = new_index;
        self.panel.panel_name = panelname.clone();
        self.cursor = 0;
        // }
    }
}

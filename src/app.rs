use crate::panel::{Panel, PanelName};

pub enum Mode {
    Normal,
    Insert,
}

pub struct App {
    pub panel: Panel,
    pub cursor: u8,
    pub mode: Mode,
    pub crates: Vec<String>,
    pub packages: Vec<String>,
    // pub content: Vec<char>,
}

impl App {
    pub fn new() -> App {
        App {
            packages: vec!["".to_string()],
            panel: Panel {
                index: 0,
                panel_name: PanelName::Crates,
                // content: vec![],
            },
            crates: vec!["".to_string()],
            cursor: 0,
            mode: Mode::Normal,
        }
    }

    pub fn cycle_panels(&mut self) {
        let panel_names = [PanelName::Crates, PanelName::Package];
        let new_index = (self.panel.index + 1) % (panel_names.len() as u8);
        if new_index <= (panel_names.len() as u8) {
            let panelname = &panel_names[self.panel.index as usize];
            self.panel = Panel {
                index: new_index,
                // panel_name: panel_names[self.panel.index as usize],
                panel_name: panelname.clone(),
            };
            self.cursor = 0;
        }
    }
}

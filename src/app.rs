use crate::panel::{Panel, PanelName};
use crate::utils;
use std::collections::HashMap;

pub enum Mode {
    Normal,
    Insert,
}

pub struct App {
    pub panels: HashMap<PanelName, Panel>,
    pub current_panel: PanelName,
    pub cursor: u8,
    pub mode: Mode,
    pub show_popup: bool,
}

impl App {
    pub fn new() -> App {
        let panel_crates = Panel {
            index: 0,
            panel_name: PanelName::Crates,
            content: utils::get_crates_from_toml(),
        };
        let panel_package = Panel {
            index: 0,
            panel_name: PanelName::Package,
            content: utils::get_package_info(),
        };
        let panel_status = Panel {
            index: 0,
            panel_name: PanelName::Status,
            content: utils::update_status(),
        };
        let panel_commands = Panel {
            index: 0,
            panel_name: PanelName::Commands,
            content: utils::get_commands(),
        };
        let panel_output = Panel {
            index: 0,
            panel_name: PanelName::Output,
            content: utils::get_output(),
        };

        let panels = HashMap::from([
            (PanelName::Crates, panel_crates),
            (PanelName::Package, panel_package),
            (PanelName::Status, panel_status),
            (PanelName::Commands, panel_commands),
            (PanelName::Output, panel_output),
        ]);

        App {
            panels,
            current_panel: PanelName::Package,
            cursor: 0,
            mode: Mode::Normal,
            show_popup: false,
        }
    }

    pub fn get_panel(&mut self) -> &mut Panel {
        self.panels.get_mut(&self.current_panel).unwrap()
    }
    pub fn get_specific(&mut self, name: PanelName) -> &mut Panel {
        self.panels.get_mut(&name).unwrap()
    }

    pub fn cycle_panels(&mut self, dir: bool) {
        let panel_names = [
            PanelName::Status,
            PanelName::Package,
            PanelName::Crates,
            PanelName::Commands,
        ];

        let direction = if dir { 1 } else { panel_names.len() - 1 };
        let new_index = (panel_names
            .iter()
            .position(|r| r == &self.current_panel)
            .unwrap()
            + direction)
            % panel_names.len();
        self.current_panel = panel_names[new_index];

        self.cursor = 0;
    }
}

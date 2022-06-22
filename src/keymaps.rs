use crate::app::App;
use crate::panel::PanelName;
// use std::io::{BufRead, BufReader};
use crate::commands;

pub fn pressed_enter(app: &mut App) {
    // TODO beautify (currently ugly)
    let item = &app.get_panel().content.clone()[app.cursor as usize];
    app.panels.get_mut(&PanelName::Status).unwrap().content = vec![item.to_owned()];

    match app.current_panel {
        PanelName::Commands => {
            match item.as_str() {
                "clippy" => commands::do_command(app, "clippy"),
                "fmt" => commands::do_command(app, "fmt"),
                _ => {}
            }
            // println!("{}", item);
        }
        _ => {}
    };
}

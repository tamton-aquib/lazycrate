use crate::app::App;
use crate::panel::PanelName;
// use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

fn do_command(app: &mut App, cmd: &str) {
    Command::new("cargo")
        .args(["clippy"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap()
        .stdout;
    // let mut reader = BufReader::new(out).lines();

    // let reader = BufReader::new(out);
    // let mut total_lines = vec![];
    // for s in reader.lines() {
    // total_lines.push(s.unwrap());
    // }
    let fake_out = format!("Command {} successfully completed!", cmd);
    app.get_specific(PanelName::Output).content = vec![fake_out];
}

pub fn pressed_enter(app: &mut App) {
    // TODO beautify (currently ugly)
    let item = &app.get_panel().content.clone()[app.cursor as usize];
    app.panels.get_mut(&PanelName::Status).unwrap().content = vec![item.to_owned()];

    match app.current_panel {
        PanelName::Commands => {
            match item.as_str() {
                "clippy" => do_command(app, "clippy"),
                _ => {}
            }
            // println!("{}", item);
        }
        _ => {}
    };
}

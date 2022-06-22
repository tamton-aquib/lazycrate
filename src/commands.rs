use crate::app::App;
use crate::panel::PanelName;
use std::process::{Command, Stdio};

// TODO: later cleanup
pub fn do_command(app: &mut App, cmd: &str) {
    let mut out = Command::new("cargo")
        .args(["clippy"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    // .stdout;
    // let mut reader = BufReader::new(out).lines();
    // let status = out.wait();

    // let reader = BufReader::new(out);
    // let mut total_lines = vec![];
    // for s in reader.lines() {
    // total_lines.push(s.unwrap());
    // }
    let status = out.wait();
    let code = status.unwrap().code().unwrap();
    let fake_out: String;

    if code == 0 {
        fake_out = format!("Cmd: {} | Code: {}", cmd, code);
    } else {
        fake_out = format!("Cmd: {} | Code: {}", cmd, code);
    }
    app.get_specific(PanelName::Output).content = vec![fake_out];
}

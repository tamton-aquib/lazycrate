use crate::app::App;

pub fn idk(app: &mut App, c: char) {
    // println!("Pressed: {c}");
    // TODO: does not work so need to refactor
    let item = &app.panel.content[app.panel.index as usize - 1];
    println!("{}", item);
}

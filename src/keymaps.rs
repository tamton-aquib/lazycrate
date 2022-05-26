use crate::app::App;
use crate::panel::PanelName;

pub fn pressed_enter(app: &mut App) {
    // TODO: does not work so need to refactor
    // let item = &app.panels.get(&app.current_panel).unwrap().content
    // [app.panels.get(&app.current_panel).unwrap().index as usize + 1];

    let item = &app.panels.get(&app.current_panel).unwrap().content[app.cursor as usize];
    app.panels.get_mut(&PanelName::Status).unwrap().content = vec![item.to_owned()];
    // vec!["Noice".to_string()]
    // app.panels[app.current_panel]
}

use crate::app::App;
use crate::panel::PanelName;

pub fn pressed_enter(app: &mut App) {
    // let get_content = |panel: PanelName| {
    // app.panels.get_mut
    // };
    // TODO beautify (currently ugly)
    let item = &app.panels.get(&app.current_panel).unwrap().content[app.cursor as usize];
    app.panels.get_mut(&PanelName::Status).unwrap().content = vec![item.to_owned()];

    match app.current_panel {
        PanelName::Commands => {
            // println!("{}", item);
        }
        _ => {}
    };

    // vec!["Noice".to_string()]
    // app.panels[app.current_panel]
}

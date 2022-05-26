use crate::app::App;
use crate::keymaps;
use crossterm::event::{KeyCode, KeyEvent};

pub fn parse_keys(app: &mut App, key: KeyEvent) -> Option<()> {
    match key.code {
        KeyCode::Char('q') => return Some(()),
        KeyCode::Char('?') => app.show_popup = !app.show_popup,
        KeyCode::Left | KeyCode::Char('h') => app.cycle_panels(false),
        KeyCode::Right | KeyCode::Char('l') => app.cycle_panels(true),
        KeyCode::Enter => keymaps::pressed_enter(app),

        KeyCode::Up | KeyCode::Char('k') => {
            if app.cursor != 0 {
                app.cursor -= 1;
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if app.cursor < (app.panels.get(&app.current_panel).unwrap().content.len() as u8) {
                app.cursor += 1;
            }
        }
        _ => {}
    }
    None
}

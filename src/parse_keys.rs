use crate::app::{App, Mode};
use crossterm::event::{KeyCode, KeyEvent};

pub fn parse_keys(app: &mut App, key: KeyEvent) -> Option<()> {
    match key.code {
        KeyCode::Char('q') => return Some(()),
        KeyCode::Char('i') => app.mode = Mode::Insert,
        KeyCode::Char('?') => app.show_popup = !app.show_popup,
        KeyCode::Esc => app.mode = Mode::Normal,
        KeyCode::Up | KeyCode::Char('k') => {
            if app.cursor != 0 {
                app.cursor -= 1;
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if app.cursor < (app.panel.content.len() as u8) {
                app.cursor += 1;
            }
        }
        KeyCode::Left | KeyCode::Char('h') => {
            app.cycle_panels(false);
        }
        KeyCode::Right | KeyCode::Char('l') => {
            app.cycle_panels(true);
        }
        // KeyCode::Char(c) => {
        // app.content.push(c);
        // }
        _ => {}
    }
    None
}

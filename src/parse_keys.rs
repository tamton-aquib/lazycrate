use crate::app::{App, Mode};
use crossterm::event::{KeyCode, KeyEvent};

pub fn parse_keys(app: &mut App, key: KeyEvent) -> Option<()> {
    match key.code {
        KeyCode::Char('q') => {
            return Some(());
        }
        KeyCode::Char('i') => app.mode = Mode::Insert,
        KeyCode::Up | KeyCode::Char('k') => {
            app.cursor -= 1;
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.cursor += 1;
        }
        KeyCode::Left | KeyCode::Char('h') => {
            app.cycle_panels();
        }
        KeyCode::Right | KeyCode::Char('l') => {
            app.cycle_panels();
        }
        KeyCode::Char('?') => {
            // popup("help".to_string());
            unimplemented!("Show toggleable help menu for the current panel.");
        }
        // KeyCode::Char(c) => {
        // app.content.push(c);
        // }
        _ => {}
    }
    None
}

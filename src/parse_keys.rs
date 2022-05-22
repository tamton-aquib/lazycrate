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
        KeyCode::Char('?') => {
            unimplemented!("Show help")
        }
        // KeyCode::Char(c) => {
        // app.content.push(c);
        // }
        _ => {}
    }
    None
}

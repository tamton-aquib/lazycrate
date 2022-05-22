pub enum Mode {
    Normal,
    Insert,
}

pub struct App {
    pub panel: String,
    pub cursor: u8,
    pub mode: Mode,
    pub content: Vec<char>,
    pub crates: Vec<String>,
}

impl App {
    pub fn new() -> App {
        App {
            content: "Dummy text here".chars().collect::<Vec<char>>(),
            crates: vec!["".to_string()],
            panel: "crates".to_string(),
            cursor: 0,
            mode: Mode::Normal,
        }
    }
}

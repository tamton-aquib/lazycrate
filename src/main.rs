use anyhow::{Error, Result};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use lazycrate::app::{App, Mode};
use lazycrate::utils;
use std::{
    // error::Error,
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame, Terminal,
};

fn main() -> Result<(), Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let tick_rate = Duration::from_millis(250);
    let app = App::new();
    let res = run_app(&mut terminal, app, tick_rate);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    KeyCode::Char('i') => {
                        app.mode = Mode::Insert;
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        app.cursor -= 1;
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        app.cursor += 1;
                    }
                    // KeyCode::Char(c) => {
                    // app.content.push(c);
                    // }
                    _ => {}
                };
            }
        }
        if last_tick.elapsed() >= tick_rate {
            // app.on_tick() if any
            last_tick = Instant::now();
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();
    let tokyodark = Color::Rgb(0x11, 0x12, 0x1D);

    // Words made "loooong" to demonstrate line breaking.
    // let s = "Veeeeeeeeeeeeeeeery    loooooooooooooooooong   striiiiiiiiiiiiiiiiiiiiiiiiiing.   ";
    // let mut long_line = s.repeat(usize::from(size.width) / s.len() + 4);
    // long_line.push('\n');

    let block = Block::default().style(Style::default().bg(tokyodark));
    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(5)
        .constraints(
            [
                Constraint::Percentage(100),
                // Constraint::Percentage(80),
            ]
            .as_ref(),
        )
        .split(size);

    // let nice = &app.content;
    // let noice = nice.into_iter().collect::<String>();
    // let all_crates = &get_crates_from_toml();
    let value_stuff = &utils::get_crates_from_toml();
    app.crates = value_stuff.to_owned();
    let text: Vec<Spans> = app
        .crates
        .iter()
        .enumerate()
        .map(|(i, c)| {
            Spans::from(Span::styled(
                c,
                Style::default()
                    .fg(Color::Green)
                    .bg(tokyodark)
                    .add_modifier(if app.cursor == (i as u8) {
                        Modifier::BOLD
                    } else {
                        Modifier::empty()
                    }),
            ))
        })
        .collect();

    // let text = vec![
    // Spans::from(Span::styled(
    // all_crates,
    // Style::default().fg(Color::Green).bg(tokyodark),
    // )),
    // // Spans::from(Span::styled(&long_line, Style::default().bg(Color::Green))),
    // ];

    let create_block = |title| {
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Rgb(17, 18, 29)).fg(Color::Black))
            .title(Span::styled(
                title,
                Style::default().add_modifier(Modifier::BOLD).fg(Color::Red),
            ))
    };
    // let paragraph = Paragraph::new(text.clone())
    // .style(Style::default().bg(Color::White).fg(Color::Black))
    // .block(create_block("Left, no wrap"))
    // .alignment(Alignment::Left);
    // f.render_widget(paragraph, chunks[0]);
    let paragraph = Paragraph::new(text.clone())
        .style(Style::default().bg(Color::White).fg(Color::Black))
        .block(create_block("Crates"))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, chunks[0]);
    // let paragraph = Paragraph::new(text.clone())
    // .style(Style::default().bg(Color::White).fg(Color::Black))
    // .block(create_block("Center, wrap"))
    // .alignment(Alignment::Center)
    // .wrap(Wrap { trim: true })
    // .scroll((app.scroll, 0));
    // f.render_widget(paragraph, chunks[2]);
}

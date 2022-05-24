use crate::app::App;
use crate::panel::PanelName;
use crate::utils;

use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

fn add_crates(app: &mut App) -> Vec<Spans> {
    let value_stuff = &utils::get_crates_from_toml();
    app.crates = value_stuff.to_owned();
    app.crates
        .iter()
        .enumerate()
        .map(|(i, c)| {
            Spans::from(Span::styled(
                c,
                Style::default()
                    .fg(Color::Green)
                    .bg(Color::Rgb(0x11, 0x12, 0x1D))
                    .add_modifier(
                        if app.cursor == (i as u8) && app.panel.panel_name == PanelName::Crates {
                            Modifier::BOLD
                        } else {
                            Modifier::empty()
                        },
                    ),
            ))
        })
        .collect::<Vec<Spans>>()
}
fn add_package(app: &mut App) -> Vec<Spans> {
    let value_stuff = &utils::get_packages();
    app.packages = value_stuff.to_owned();
    app.packages
        .iter()
        .enumerate()
        .map(|(i, c)| {
            Spans::from(Span::styled(
                c,
                Style::default()
                    .fg(Color::Green)
                    .bg(Color::Rgb(0x11, 0x12, 0x1D))
                    .add_modifier(
                        if app.cursor == (i as u8) && app.panel.panel_name == PanelName::Package {
                            Modifier::BOLD
                        } else {
                            Modifier::empty()
                        },
                    ),
            ))
        })
        .collect::<Vec<Spans>>()
}
pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();
    let tokyodark = Color::Rgb(0x11, 0x12, 0x1D);
    let create_block = |title| {
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Rgb(17, 18, 29)).fg(Color::Black))
            .title(Span::styled(
                title,
                Style::default().add_modifier(Modifier::BOLD).fg(Color::Red),
            ))
    };

    // Words made "loooong" to demonstrate line breaking.
    // let s = "Veeeeeeeeeeeeeeeery    loooooooooooooooooong   striiiiiiiiiiiiiiiiiiiiiiiiiing.   ";
    // let mut long_line = s.repeat(usize::from(size.width) / s.len() + 4);
    // long_line.push('\n');

    let block = Block::default().style(Style::default().bg(tokyodark));
    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(5)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(size);

    // let nice = &app.content;
    // let noice = nice.into_iter().collect::<String>();
    // let all_crates = &get_crates_from_toml();
    // add_crates(app);
    // let text = vec![
    // Spans::from(Span::styled(
    // all_crates,
    // Style::default().fg(Color::Green).bg(tokyodark),
    // )),
    // // Spans::from(Span::styled(&long_line, Style::default().bg(Color::Green))),
    // ];

    // let paragraph = Paragraph::new(text.clone())
    // .style(Style::default().bg(Color::White).fg(Color::Black))
    // .block(create_block("Left, no wrap"))
    // .alignment(Alignment::Left);
    // f.render_widget(paragraph, chunks[0]);
    let paragraph = Paragraph::new(add_crates(app).clone())
        .style(Style::default().bg(Color::White).fg(Color::Black))
        .block(create_block("Crates"))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, chunks[0]);

    let paragraph = Paragraph::new(add_package(app).clone())
        .style(Style::default().bg(Color::White).fg(Color::Black))
        .block(create_block("Package Details"))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, chunks[1]);
    // let paragraph = Paragraph::new(text.clone())
    // .style(Style::default().bg(Color::White).fg(Color::Black))
    // .block(create_block("Center, wrap"))
    // .alignment(Alignment::Center)
    // .wrap(Wrap { trim: true })
    // .scroll((app.scroll, 0));
    // f.render_widget(paragraph, chunks[2]);
}

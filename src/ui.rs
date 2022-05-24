use crate::app::App;
use crate::panel::PanelName;
use crate::utils;

use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

fn get_content(app: &mut App, panel: PanelName) -> Vec<Spans> {
    let value_stuff = match panel {
        PanelName::Package => utils::get_package_info(),
        PanelName::Crates => utils::get_crates_from_toml(),
    };
    app.panel.content = value_stuff.to_owned();
    app.panel
        .content
        .iter()
        .enumerate()
        .map(|(i, c)| {
            Spans::from(Span::styled(
                c,
                Style::default()
                    .fg(Color::Green)
                    .bg(Color::Rgb(0x11, 0x12, 0x1D))
                    .add_modifier(
                        if app.cursor == (i as u8) && app.panel.panel_name == panel {
                            Modifier::BOLD
                        } else {
                            Modifier::empty()
                        },
                    ),
            ))
        })
        .collect::<Vec<Spans>>()
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
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

    let block = Block::default().style(Style::default().bg(tokyodark));
    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(size);

    let paragraph = Paragraph::new(get_content(app, PanelName::Package).clone())
        .style(Style::default().bg(Color::White).fg(Color::Black))
        .block(create_block("Info"))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, chunks[0]);

    let paragraph = Paragraph::new(get_content(app, PanelName::Crates).clone())
        .style(Style::default().bg(Color::White).fg(Color::Black))
        .block(create_block("Crates"))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, chunks[1]);

    if app.show_popup {
        let area = centered_rect(80, 80, size);
        let para = Paragraph::new(app.panel.get_help())
            .block(create_block("Help menu!"))
            .style(Style::default().fg(Color::White));
        f.render_widget(Clear, area);
        f.render_widget(para, area)
    }

    // let paragraph = Paragraph::new(text.clone())
    // .style(Style::default().bg(Color::White).fg(Color::Black))
    // .block(create_block("Center, wrap"))
    // .alignment(Alignment::Center)
    // .wrap(Wrap { trim: true })
    // .scroll((app.scroll, 0));
    // f.render_widget(paragraph, chunks[2]);
}

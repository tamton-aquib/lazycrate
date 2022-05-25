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
    app.panel.content = match panel {
        PanelName::Package => utils::get_package_info(),
        PanelName::Crates => utils::get_crates_from_toml(),
        PanelName::Status => utils::get_status(&app),
        PanelName::Commands => utils::get_commands(),
    };
    app.panel
        .content
        .iter()
        .enumerate()
        .map(|(i, c)| {
            Spans::from(Span::styled(
                c,
                Style::default().fg(Color::Green).add_modifier(
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
    let create_block = |title| {
        Block::default().borders(Borders::ALL).title(Span::styled(
            title,
            Style::default().add_modifier(Modifier::BOLD).fg(Color::Red),
        ))
    };
    let create_noheader_block = || {
        Block::default().borders(Borders::ALL)
        // .style(Style::default().fg(Color::Black))
    };

    let block = Block::default().style(Style::default());
    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(size);
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(20),
            Constraint::Percentage(35),
            Constraint::Percentage(35),
        ])
        .split(chunks[0]);
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(chunks[1]);

    // Left panels
    let paragraph = Paragraph::new(get_content(app, PanelName::Status))
        .block(create_block("Status"))
        .scroll((1, 1))
        .alignment(Alignment::Left);
    f.render_widget(paragraph, left_chunks[0]);
    let paragraph = Paragraph::new(get_content(app, PanelName::Package).clone())
        .block(create_block("Info"))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, left_chunks[1]);

    let paragraph = Paragraph::new(get_content(app, PanelName::Crates).clone())
        .block(create_block("Crates"))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, left_chunks[2]);

    let paragraph = Paragraph::new(get_content(app, PanelName::Commands).clone())
        .block(create_block("Commands"))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, left_chunks[3]);

    // Right panels
    let paragraph = Paragraph::new("Status stuff example")
        .block(create_noheader_block())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, right_chunks[0]);
    let paragraph = Paragraph::new("Current output example")
        .block(create_noheader_block())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, right_chunks[1]);

    if app.show_popup {
        let area = centered_rect(80, 80, size);
        let para = Paragraph::new(app.panel.get_help()).block(create_block("Help menu!"));
        f.render_widget(Clear, area);
        f.render_widget(para, area)
    }
}

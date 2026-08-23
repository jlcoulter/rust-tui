pub mod example;

use ratatui::{layout::Constraint, layout::Layout, layout::Rect, Frame};

use crate::app::{App, Tab};

/// Top-level render function. Dispatches to the active tab.
pub fn render(app: &App, f: &mut Frame) {
    let area = f.area();

    // Layout: top bar + main content
    let chunks = Layout::vertical([
        Constraint::Length(1), // tab bar
        Constraint::Min(0),    // main content
    ])
    .split(area);

    // Render tab bar
    let tabs = ratatui::widgets::Tabs::new(vec!["Home", "Example"])
        .select(match app.tab {
            Tab::Home => 0,
            Tab::Example => 1,
        })
        .style(ratatui::style::Style::default().fg(ratatui::style::Color::White))
        .highlight_style(ratatui::style::Style::default().fg(ratatui::style::Color::Yellow));
    f.render_widget(tabs, chunks[0]);

    // Render active tab content
    match app.tab {
        Tab::Home => render_home(app, f, chunks[1]),
        Tab::Example => example::render(app, f, chunks[1]),
    }
}

fn render_home(app: &App, f: &mut Frame, area: Rect) {
    let text = ratatui::text::Line::from(format!(
        "Counter: {} | Press ↑/↓ to change, Tab to switch, q to quit",
        app.counter
    ));
    let paragraph = ratatui::widgets::Paragraph::new(text)
        .style(ratatui::style::Style::default().fg(ratatui::style::Color::Cyan));
    f.render_widget(paragraph, area);
}

use ratatui::{layout::Rect, widgets::Paragraph, Frame};

use crate::app::App;

/// Example tab — replace this with your own widget.
pub fn render(app: &App, f: &mut Frame, area: Rect) {
    let text = ratatui::text::Line::from(format!("Example tab — counter is {}", app.counter));
    let paragraph = Paragraph::new(text)
        .style(ratatui::style::Style::default().fg(ratatui::style::Color::Green));
    f.render_widget(paragraph, area);
}

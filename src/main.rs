mod app;
mod event;
mod key;
mod ui;

use std::io;

use clap::Parser;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use tracing_subscriber::EnvFilter;

/// A Rust TUI application template — replace this description.
#[derive(Parser)]
#[command(name = "rust-tui-template", version, about)]
struct Cli {
    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> anyhow::Result<()> {
    // Parse CLI args before touching the terminal
    let cli = Cli::parse();

    // Setup logging to stderr so it doesn't pollute the TUI
    let level = if cli.verbose { "debug" } else { "warn" };
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(level)),
        )
        .with_writer(std::io::stderr)
        .init();

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run app
    let app = app::App::new();
    let result = run_app(&mut terminal, app);

    // Restore terminal regardless of outcome
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    mut app: app::App,
) -> anyhow::Result<()> {
    loop {
        // Render current state
        terminal.draw(|f| ui::render(&app, f))?;

        // Handle events
        let event = event::read_event()?;
        if !app.handle_event(event) {
            break; // App requested quit
        }
    }
    Ok(())
}

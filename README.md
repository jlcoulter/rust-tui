# Rust TUI Template

A GitHub template for Rust terminal UI applications — ratatui for rendering, crossterm for input, tokio for async events, with a clean event loop architecture.

## Why not rust-cli?

`rust-cli` is for command-line tools that parse args and exit. TUIs are different:
- They take over the terminal with a full-screen interface
- They run an event loop (keyboard, mouse, async updates)
- They need graceful terminal restore on exit (alternate screen, raw mode)
- They render frames at ~60fps with ratatui's widget system
- They handle async data updates (file watching, network, timers)

## Features

- **ratatui** — immediate-mode TUI rendering with widgets
- **crossterm** — cross-platform terminal input (keyboard + mouse)
- **tokio** — async runtime for background tasks and timers
- **Event loop** — clean architecture with typed events
- **App state** — separate state from rendering
- **Key bindings** — configurable keyboard shortcuts
- **Error handling** — anyhow for errors, proper terminal restore on panic
- **Tests** — unit tests for app logic (no terminal needed)
- **CI** — test, clippy, fmt, build
- **Docker** — multi-stage build with musl (~3MB)

## Usage

1. Click **"Use this template"** on GitHub to create a new repo
2. Run the setup script:
   ```sh
   ./setup.sh mytui
   ```
3. Add your widgets in `src/ui/`
4. Add your events in `src/event.rs`
5. Add your business logic in `src/app.rs`

## Project Structure

```
.
├── src/
│   ├── main.rs              # Entry point, terminal setup, event loop
│   ├── lib.rs               # Re-exports for integration tests
│   ├── app.rs               # Application state and logic
│   ├── event.rs             # Event types and async event reader
│   ├── ui/
│   │   ├── mod.rs            # Top-level render function
│   │   └── example.rs        # Example widget (delete me)
│   └── key.rs               # Key binding definitions
├── tests/
│   └── integration_test.rs  # Integration tests
├── .github/
│   └── workflows/
│       └── ci.yml            # Test + clippy + fmt
├── Cargo.toml
├── Dockerfile
├── Makefile
├── setup.sh
└── README.md
```

## Quick Start

```sh
# Run locally
make run

# Run tests
make test

# Build release binary
make build

# Lint
make lint
```

## Container Images

CI builds and pushes a container image to GHCR on every push to any branch.

```sh
# Pull the latest image
docker pull ghcr.io/<owner>/rust-tui-template:latest

# Pull a specific commit
docker pull ghcr.io/<owner>/rust-tui-template:<sha>

# Run
docker run ghcr.io/<owner>/rust-tui-template:latest
```

Replace `<owner>` with your GitHub username or org. Images are tagged with both `latest` and the commit SHA.

## Architecture

The app follows a simple loop:

```
main() → install terminal
  ↓
loop {
  event = read_key()          ← crossterm (blocking)
  app = app.handle_event(event) ← update state
  render(app)                  ← ratatui widgets
}
  ↓
main() → restore terminal
```

Key principles:
- **App owns all state** — no global mutable state
- **Events are typed** — `Event::Key`, `Event::Tick`, `Event::Message`
- **Rendering is pure** — `ui::render(app, frame)` only reads state
- **Logic is testable** — `app.handle_event()` takes `&mut self`, easy to unit test

## Adding a Widget

```rust
// src/ui/mypage.rs
use ratatui::{Frame, layout::Rect, widgets::Paragraph};

pub fn render(f: &mut Frame, area: Rect, app: &App) {
    let paragraph = Paragraph::new(app.my_data.clone());
    f.render_widget(paragraph, area);
}
```

Then call it from `src/ui/mod.rs` in the `render` function.

## License

MIT
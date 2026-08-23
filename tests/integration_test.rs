// Integration tests for the TUI binary.
// Note: full TUI interaction can't be tested in headless environments.
// Unit tests in src/app.rs test the app logic without a terminal.

use assert_cmd::Command;

#[test]
fn version_flag() {
    // --version doesn't need a terminal, so this is safe to test
    Command::cargo_bin("rust-tui-template")
        .unwrap()
        .arg("--version")
        .assert()
        .success();
}

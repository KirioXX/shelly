//! CLI Smoke Tests - Minimal verification that commands exist and work

use std::process::Command;

/// Test that --help works
#[test]
fn test_cli_help() {
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "--help should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("shelly"), "Help should mention shelly");
    assert!(stdout.contains("Commands:"), "Help should list commands");
}

/// Test that cmds subcommand lists commands
#[test]
fn test_cli_cmds_subcommand() {
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--", "cmds"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "cmds should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("setup"), "cmds should list setup");
    assert!(stdout.contains("generate"), "cmds should list generate");
}

/// Test that version works
#[test]
fn test_cli_version() {
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--", "--version"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "--version should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.is_empty(), "Version should output something");
}

/// Test that generate dry-run has a short alias
#[test]
fn test_generate_dry_run_short_alias() {
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--", "generate", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "generate --help should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("-d, --dry-run"),
        "generate help should show -d as an alias for --dry-run; got:\n{}",
        stdout
    );
}

/// Test that completions zsh works (smoke test)
#[test]
fn test_cli_completions() {
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--", "completions", "zsh"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "completions should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.is_empty(), "Completions should output something");
}

/// Test that config --help lists the --show flag (smoke test)
#[test]
fn test_cli_config_help() {
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--", "config", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "config --help should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("--show"),
        "config help should list --show; got:\n{}",
        stdout
    );
}

/// Test that config --show works with default/empty config (smoke test)
#[test]
fn test_cli_config_show() {
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--", "config", "--show"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "config --show should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Current configuration"),
        "config --show should display config; got:\n{}",
        stdout
    );
}

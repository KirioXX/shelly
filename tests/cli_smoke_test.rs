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

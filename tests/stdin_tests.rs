use std::process::{Command, Stdio};
use std::io::Write;

#[test]
fn test_stdin_functionality() {
    // Test basic stdin functionality
    let mut child = Command::new("cargo")
        .args(&["run", "--", "--stdin"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn process");

    // Write to stdin
    if let Some(stdin) = child.stdin.as_mut() {
        stdin.write_all(b"Test stdin entry\n").expect("Failed to write to stdin");
    }

    let output = child.wait_with_output().expect("Failed to read output");
    
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("Logged"));
}

#[test]
fn test_stdin_with_time_override() {
    // Test stdin with time override
    let mut child = Command::new("cargo")
        .args(&["run", "--", "--stdin", "-t", "14:30"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn process");

    // Write to stdin
    if let Some(stdin) = child.stdin.as_mut() {
        stdin.write_all(b"Test stdin entry with time override\n").expect("Failed to write to stdin");
    }

    let output = child.wait_with_output().expect("Failed to read output");
    
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("Logged"));
}

#[test]
fn test_stdin_empty_input() {
    // Test stdin with empty input (should fail)
    let mut child = Command::new("cargo")
        .args(&["run", "--", "--stdin"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn process");

    // Write empty content to stdin
    if let Some(stdin) = child.stdin.as_mut() {
        stdin.write_all(b"\n").expect("Failed to write to stdin");
    }

    let output = child.wait_with_output().expect("Failed to read output");
    
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("No content read from stdin"));
} 
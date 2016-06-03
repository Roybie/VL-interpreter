#![allow(non_snake_case)]
use std::process::Command;

#[test]
fn w() {
    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("w")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert_eq!(String::from_utf8_lossy(&output.stdout), "0");

    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("iHi;w")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert_eq!(String::from_utf8_lossy(&output.stdout), "Hi");
}

#[test]
fn l() {
    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("l")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "0\n");

    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("iHi;l")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "Hi\n");
}

#[test]
fn W() {
    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("W")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "1");

    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("IHi;W")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "Hi");
}

#[test]
fn L() {
    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("L")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "1\n");

    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("IHi;L")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "Hi\n");
}

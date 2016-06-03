use std::process::Command;

#[test]
fn int_to_string() {
    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("i9;=w")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "9");
}

#[test]
fn string_to_int() {
    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("i9;==w")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "9");

    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("iHi;=")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() > 0, "stderr should exist");
}

#[test]
fn string_length() {
    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("istring;@W")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "6");

    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("i;@W")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "0");

    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("i99;@W")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() > 0, "stderr should exist");
}


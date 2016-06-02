#![allow(non_snake_case)]
use std::process::Command;

#[test]
fn p_and_y() {
    let output = Command::new("vl")
        .arg("-s")
        .arg("p")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "");

    let output = Command::new("vl")
        .arg("-s")
        .arg("y")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "");

    let output = Command::new("vl")
        .arg("-s")
        .arg("a]p]yw[yw")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "01");
}

#[test]
fn P_and_Y() {
    let output = Command::new("vl")
        .arg("-s")
        .arg("P")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "");

    let output = Command::new("vl")
        .arg("-s")
        .arg("Y")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "");

    let output = Command::new("vl")
        .arg("-s")
        .arg("3P1]YW1[YW")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "03");
}

#[test]
fn i() {
    let output = Command::new("vl")
        .arg("-s")
        .arg("i99;wyw")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "9999");
}

#[test]
fn I() {
    let output = Command::new("vl")
        .arg("-s")
        .arg("I99;Wyw")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "9999");
}

#[test]
fn a() {
    let output = Command::new("vl")
        .arg("-s")
        .arg("awyw")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "11");

    let output = Command::new("vl")
        .arg("-s")
        .arg("10aw")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "10");
}

#[test]
fn x() {
    let output = Command::new("vl")
        .arg("-s")
        .arg("xwyw")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "-1-1");

    let output = Command::new("vl")
        .arg("-s")
        .arg("10xw")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "-10");
}

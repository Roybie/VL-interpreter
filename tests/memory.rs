use std::process::Command;

#[test]
fn mark() {
    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("i7;'byw'ayw")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "07");

    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("'a'b'c'd'e'f'g'h'i'j'k'l'm'n'o'p'q'r's't'u'v'w'x'y'z")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "");

    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("'0")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() > 0, "stderr should exist");
}

#[test]
fn index() {
    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("i7;1`ayw0`ayw")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "07");

    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("500`a")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "");

    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("1`6")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() > 0, "stderr should exist");
}

#[test]
fn next_mark() {
    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("]")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "");

    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("'di7;'ayw3]yw")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "07");

    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("i7;25]yw]yw")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "07");
}

#[test]
fn prev_mark() {
    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("[")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "");

    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("i7;'dyw3[yw")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "07");

    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("i7;[yw25[yw")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "07");
}

#[test]
fn next_index() {
    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("}")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "");

    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("10`ai7;'ayw10}yw")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "07");
}

#[test]
fn prev_index() {
    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("}{")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "");

    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("{")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() > 0, "stderr should exist");

    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("i7;10`ayw10{yw")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "07");
}

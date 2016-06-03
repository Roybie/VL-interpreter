use std::process::Command;

#[test]
fn line_down() {
    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("j")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "");

    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("j99W\n7W")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "7");

    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("2j99W\n99W\n7W")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "7");
}

#[test]
fn line_up() {
    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("2j\n2j\nk")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "");

    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("2j\n7W2j\n0W1k99W")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "07");

    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("3j\n0W\n7W2j\n2k99W")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "07");
}

#[test]
fn line_begin() {
    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("a5!^w")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "5");
}

#[test]
fn jump_forward() {
    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("fw")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "");

    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("fwaw")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "0");

    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("5fwawawawawaw")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "0");
}

#[test]
fn jump_back() {
    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("Fw")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "");

    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("fFawjFwa")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "0");

    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("f3waaaaaj3Faaw\nw")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "3");
}

#[test]
fn con_equals() {
    // Test int comparing
    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("7a7?j^")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "");

    // Test string comparing
    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("iHi;V?j^")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "");
}

#[test]
fn con_not_equals() {
    // Test int comparing
    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("7a0!j^")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "");

    // Test string comparing
    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("iHi;IHello;!j^")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "");
}

#[test]
fn con_less() {
    // Test int comparing
    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("7a0<j^")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "");

    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("7a10<^")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "");

    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("7a7<^")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "");

    // Test string comparing
    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("iHi;IHello;<j^")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "");

    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("iHa;IHello;<^")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "");

    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("iHi;IHi;<^")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "");
}

#[test]
fn con_more() {
    // Test int comparing
    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("7a0>^")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "");

    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("7a10>j^")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "");

    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("7a7>^")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "");

    // Test string comparing
    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("iHi;IHello;>^")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "");

    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("iHa;IHello;>j^")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "");

    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("iHi;IHi;>^")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "");
}

#[test]
fn repeat() {
    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("w.")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "00");

    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("w9.")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "0000000000");
}

#[test]
fn groups() {
    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("a(w)")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "1");

    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("10(w)")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "0000000000");

    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("2(W)")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "11");

    let output = Command::new("./target/debug/vl")
        .arg("-s")
        .arg("(j)w")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "0");
}

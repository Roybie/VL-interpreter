use std::process::Command;

#[test]
fn int_plus() {
    let output = Command::new("vl")
        .arg("-s")
        .arg("i2;2+w")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "4");
}

#[test]
fn int_minus() {
    let output = Command::new("vl")
        .arg("-s")
        .arg("i5;2-w")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "3");
}

#[test]
fn int_times() {
    let output = Command::new("vl")
        .arg("-s")
        .arg("i3;2*w")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "6");
}

#[test]
fn int_divide() {
    let output = Command::new("vl")
        .arg("-s")
        .arg("i7;2/wW")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "31");
}

#[test]
fn string_plus() {
    let output = Command::new("vl")
        .arg("-s")
        .arg("iHel;Ilo!;+w")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "Hello!");
}

#[test]
fn string_minus() {
    let output = Command::new("vl")
        .arg("-s")
        .arg("iHello!;3-lW")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "Hel\nlo!");
}

#[test]
fn string_times() {
    let output = Command::new("vl")
        .arg("-s")
        .arg("iH;}ie;}il;}il;}io;*w")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "Hello");

    let output = Command::new("vl")
        .arg("-s")
        .arg("iH;}ie;}il;}il;}io;'bI.;'a*w")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "H.e.l.l.o");
}

#[test]
fn string_divide() {
    let output = Command::new("vl")
        .arg("-s")
        .arg("iHello!;/yl}yl}yl")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "H\ne\nl\n");

    let output = Command::new("vl")
        .arg("-s")
        .arg("iHello there!;I ;/yl}yl")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    assert!(output.stderr.len() == 0, "stderr shouldn't exist");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "Hello\nthere!\n");
}


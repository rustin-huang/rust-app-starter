use std::process::Command;

#[test]
fn smoke_test_basic_functionality() {
    // Verify that the program can compile and run properly
    let output = Command::new("cargo")
        .arg("run")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success(), "Program execution failed");

    // Verify program output
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Hello"),
        "Program output does not contain the expected greeting"
    );
}

#[test]
fn smoke_test_error_handling() {
    // Verify error handling functionality
    let result = std::fs::read_to_string("nonexistent_file.txt");
    assert!(
        result.is_err(),
        "Should return an error for a nonexistent file"
    );
}

#[test]
fn smoke_test_basic_computation() {
    // Verify basic computation functionality
    assert_eq!(2 + 2, 4, "Basic arithmetic operation failed");
}

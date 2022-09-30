use std::fs;
use std::path::Path;
use std::process::Command;
use test_finder::test_finder;

fn run_system_test(name: &Path) {
    let output = Command::new("./target/debug/crush")
        .args(&[name.to_str().unwrap()])
        .output()
        .expect("failed to execute process");
    let output_name = name.clone().with_extension("crush.output");
    let expected_output = fs::read_to_string(output_name.to_str().unwrap())
        .expect(format!("failed to read output file {}", output_name.to_str().unwrap()).as_str());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        expected_output,
        "\n\nError while running file {}",
        name.to_str().unwrap()
    );
}

#[test]
fn test_nothing() {
    // This empty test only exists to make sure IDEs will realize that there are tests to run in
    // this file. All the real tests are generated by the test_finder macro below.
}

test_finder!();

use std::fs;

use common::{get_test_case, TestBinary, TestCase};

mod common;

fn run_test_case(mut test_case: TestCase) {
    if let Ok(stdin) = fs::File::open(test_case.stdin) {
        test_case.command.stdin(stdin);
    }

    let output = test_case
        .command
        .arg(test_case.input)
        .output()
        .expect("Unable to sucessfully run executable");

    let (actual, expected) = if let Ok(expected_stderr) = fs::read(test_case.stderr) {
        (output.stderr, expected_stderr)
    } else {
        (output.stdout, fs::read(test_case.expected).expect("Unable to read expected output"))
    };

    assert_eq!(actual, expected);
}

mod interpreter {
    use super::*;

    test_cases!(Interpreter, run_test_case);
}

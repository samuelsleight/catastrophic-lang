use std::fs;

use common::{get_test_case, TestBinary, TestCase};

mod common;

fn run_test_case(mut test_case: TestCase) {
    let output = test_case
        .command
        .arg(test_case.input)
        .output()
        .expect("Unable to sucessfully run executable");

    assert_eq!(output.stdout, fs::read(test_case.expected).expect("Unable to read expected output"))
}

mod interpreter {
    use super::*;

    test_cases!(Interpreter, run_test_case);
}

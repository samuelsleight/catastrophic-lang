#![cfg(not(tarpaulin))]

use std::{fs, process::Command};

use common::{get_test_case, TestBinary, TestCase};
use tempfile::tempdir;

mod common;

fn run_test_case(mut test_case: TestCase) {
    let dir = tempdir().unwrap();
    let output = dir.path().join("output");

    // First, run the `catastrophicc` compiler
    let compiler_output = test_case
        .command
        .args(["--opt", "none", "-o"])
        .arg(&output)
        .arg(test_case.input)
        .output()
        .expect("Unable to sucessfully run compiler");

    if let Ok(expected_stderr) = fs::read(test_case.stderr) {
        // If we are testing for an error, check that now and return if so
        assert_eq!(compiler_output.stderr, expected_stderr);
    } else if !compiler_output.status.success() {
        panic!("Failed running compiler: {}", String::from_utf8(compiler_output.stderr).unwrap());
    } else {
        // Otherwise, run the compiled output
        let mut output_command = Command::new(output);

        if let Ok(stdin) = fs::File::open(test_case.stdin) {
            output_command.stdin(stdin);
        };

        let output = output_command
            .output()
            .expect("Unable to sucessfully run output executable");

        assert_eq!(output.stdout, fs::read(test_case.expected).expect("Unable to read expected output"));
    }
}

mod compiler {
    use super::*;

    test_cases!(Compiler, run_test_case);
}

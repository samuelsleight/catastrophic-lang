use std::fs;

use bintest::Stdio;
use common::{get_test_case, TestBinary, TestCase};

use crate::common::get_llvm_binary;

mod common;

fn run_test_case(mut test_case: TestCase) {
    let compiler_output = test_case
        .command
        .args(["--opt", "none"])
        .arg(test_case.input)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Unable to sucessfully run executable");

    let output = get_llvm_binary("lli")
        .stdin(Stdio::from(compiler_output.stdout.unwrap()))
        .output()
        .expect("Unable to sucessfully run lli");

    assert_eq!(output.stdout, fs::read(test_case.expected).expect("Unable to read expected output"))
}

mod compiler {
    use super::*;

    test_cases!(Compiler, run_test_case);
}

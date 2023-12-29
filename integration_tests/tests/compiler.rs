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

#[macro_export]
macro_rules! test_case {
    ($case_name:ident) => {
        #[test]
        fn $case_name() {
            run_test_case(get_test_case(TestBinary::Compiler, std::stringify!($case_name)))
        }
    };
}

mod compiler {
    use super::*;

    test_case!(simple_addition);
    test_case!(simple_subtraction);
}

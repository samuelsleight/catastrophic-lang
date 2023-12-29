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

#[macro_export]
macro_rules! test_case {
    ($case_name:ident) => {
        #[test]
        fn $case_name() {
            run_test_case(get_test_case(TestBinary::Interpreter, std::stringify!($case_name)))
        }
    };
}

mod interpreter {
    use super::*;

    test_case!(simple_addition);
    test_case!(simple_subtraction);
}

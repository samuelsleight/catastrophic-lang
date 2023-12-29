use std::{
    fs,
    path::{Path, PathBuf},
};

use bintest::{BinTestBuilder, Command};
use once_cell::sync::Lazy;

#[derive(Clone, Copy)]
enum TestBinary {
    Compiler,
    Interpreter,
}

fn test_binary_name(binary: TestBinary) -> &'static str {
    match binary {
        TestBinary::Compiler => "catastrophicc",
        TestBinary::Interpreter => "catastrophici",
    }
}

fn get_test_binary(binary: TestBinary) -> Command {
    let bintest = BinTestBuilder::new()
        .build_workspace(true)
        .build_executable(test_binary_name(binary))
        .build();

    bintest.command(test_binary_name(binary))
}

fn run_interpreter_test_case(test_case_path: &Path) {
    let input = test_case_path.join("input.cat");
    let expected = test_case_path.join("output.txt");

    let mut command = get_test_binary(TestBinary::Interpreter);

    let output = command
        .arg(input)
        .output()
        .expect("Unable to sucessfully run executable");

    assert_eq!(output.stdout, fs::read(expected).expect("Unable to read expected output"))
}

static TEST_CASE_DIR: Lazy<PathBuf> = Lazy::new(|| Path::new(std::env!("CARGO_MANIFEST_DIR")).join("test_cases"));

macro_rules! interpreter_test_case {
    ($case_name:ident) => {
        #[test]
        fn $case_name() {
            run_interpreter_test_case(&TEST_CASE_DIR.join(std::stringify!($case_name)))
        }
    };
}

mod integration_tests {
    use super::*;

    interpreter_test_case!(simple_addition);
    interpreter_test_case!(simple_subtraction);
}

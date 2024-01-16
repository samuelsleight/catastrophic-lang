#![allow(dead_code)]

use std::path::{Path, PathBuf};

use bintest::{BinTest, Command};
use once_cell::sync::Lazy;

#[derive(Clone, Copy)]
pub enum TestBinary {
    Compiler,
    Interpreter,
}

pub struct TestCase {
    pub command: Command,
    pub input: PathBuf,
    pub expected: PathBuf,
    pub stdin: PathBuf,
    pub stderr: PathBuf,
}

#[macro_export]
macro_rules! test_cases {
    ($binary:ident, $runner:ident) => {
        test_cases!(simple_addition, $binary, $runner);
        test_cases!(simple_subtraction, $binary, $runner);
        test_cases!(simple_multiplication, $binary, $runner);
        test_cases!(simple_division, $binary, $runner);
        test_cases!(simple_inequality, $binary, $runner);

        test_cases!(ite_equality, $binary, $runner);
        test_cases!(ite_inequality, $binary, $runner);
        test_cases!(ite_less_than, $binary, $runner);
        test_cases!(ite_greater_than, $binary, $runner);
        test_cases!(ite_not_less_than, $binary, $runner);
        test_cases!(ite_not_greater_than, $binary, $runner);

        test_cases!(string_simple, $binary, $runner);
        test_cases!(string_emoji, $binary, $runner);

        test_cases!(input_char, $binary, $runner);
        test_cases!(input_loop, $binary, $runner);
        test_cases!(input_string, $binary, $runner);

        test_cases!(nested_symbol_names, $binary, $runner);

        test_cases!(fib_divergent, $binary, $runner);
        test_cases!(fib_tail_recursive, $binary, $runner);

        test_cases!(error_unexpected_char, $binary, $runner);
        test_cases!(error_unmatched_open_brace, $binary, $runner);
        test_cases!(error_unmatched_close_brace, $binary, $runner);
        test_cases!(error_unterminated_string, $binary, $runner);
        test_cases!(error_undefined_symbol, $binary, $runner);
        test_cases!(error_duplicate_symbol, $binary, $runner);
        test_cases!(error_missing_arrow, $binary, $runner);
    };

    ($name:ident, $binary:ident, $runner:ident) => {
        #[test]
        #[serial_test::file_serial]
        fn $name() {
            $runner(get_test_case(TestBinary::$binary, std::stringify!($name)))
        }
    };
}

static TEST_CASE_DIR: Lazy<PathBuf> = Lazy::new(|| Path::new(std::env!("CARGO_MANIFEST_DIR")).join("test_cases"));
static LLVM_DIR: Lazy<PathBuf> = Lazy::new(|| Path::new(std::env!("LLVM_SYS_140_PREFIX")).join("bin"));

pub fn get_test_case(binary: TestBinary, name: &str) -> TestCase {
    let test_case_path = TEST_CASE_DIR.join(name);

    TestCase {
        command: get_test_binary(binary),
        input: test_case_path.join("input.cat"),
        expected: test_case_path.join("output.txt"),
        stdin: test_case_path.join("stdin.txt"),
        stderr: test_case_path.join("stderr.txt"),
    }
}

pub fn get_llvm_binary(bin: &str) -> Command {
    Command::new(LLVM_DIR.join(bin))
}

fn test_binary_name(binary: TestBinary) -> &'static str {
    match binary {
        TestBinary::Compiler => "catastrophicc",
        TestBinary::Interpreter => "catastrophici",
    }
}

fn get_test_binary(binary: TestBinary) -> Command {
    let bintest = BinTest::with()
        .build_workspace(true)
        .build_executable(test_binary_name(binary))
        .quiet(true)
        .build();

    bintest.command(test_binary_name(binary))
}

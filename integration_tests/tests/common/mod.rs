#![allow(dead_code)]

use std::path::{Path, PathBuf};

use bintest::{BinTestBuilder, Command};
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
        test_cases!(fib_divergent, $binary, $runner);
        test_cases!(fib_tail_recursive, $binary, $runner);
    };

    ($name:ident, $binary:ident, $runner:ident) => {
        #[test]
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
    let bintest = BinTestBuilder::new()
        .build_workspace(true)
        .build_executable(test_binary_name(binary))
        .quiet(true)
        .build();

    bintest.command(test_binary_name(binary))
}

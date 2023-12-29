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
        .build();

    bintest.command(test_binary_name(binary))
}

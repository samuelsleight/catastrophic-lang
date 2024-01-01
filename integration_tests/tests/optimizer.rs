use std::fs;

use common::{get_llvm_binary, get_test_case, TestBinary, TestCase};

mod common;

fn run_test_case(mut test_case: TestCase) {
    let llvm_output_path = test_case
        .input
        .with_file_name("optimizer_llvm_output");

    let mut lli_command = get_llvm_binary("lli");

    if let Ok(stdin) = fs::File::open(test_case.stdin) {
        lli_command.stdin(stdin);
    }

    test_case
        .command
        .args(["--opt", "all"])
        .arg(test_case.input)
        .stdout(fs::File::create(&llvm_output_path).expect("Unable to open llvm output file"))
        .output()
        .expect("Unable to sucessfully run executable");

    let output = lli_command
        .arg(&llvm_output_path)
        .output()
        .expect("Unable to sucessfully run lli");

    assert_eq!(output.stdout, fs::read(test_case.expected).expect("Unable to read expected output"));

    fs::remove_file(llvm_output_path).expect("Unable to delete temporary file");
}

mod optimizer {
    use super::*;

    test_cases!(Compiler, run_test_case);
}

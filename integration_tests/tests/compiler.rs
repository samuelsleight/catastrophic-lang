use std::fs;

use common::{get_llvm_binary, get_test_case, TestBinary, TestCase};

mod common;

fn run_test_case(mut test_case: TestCase) {
    let llvm_output_path = test_case
        .input
        .with_file_name("compiler_llvm_output");

    let mut lli_command = get_llvm_binary("lli");

    if let Ok(stdin) = fs::File::open(test_case.stdin) {
        lli_command.stdin(stdin);
    }

    // First, run the `catastrophicc` compiler
    let compiler_output = test_case
        .command
        .args(["--opt", "none"])
        .arg(test_case.input)
        .stdout(fs::File::create(&llvm_output_path).expect("Unable to open llvm output file"))
        .output()
        .expect("Unable to sucessfully run executable");

    if let Ok(expected_stderr) = fs::read(test_case.stderr) {
        // If we are testing for an error, check that now and return if so
        assert_eq!(compiler_output.stderr, expected_stderr);
    } else {
        // Otherwise, run `lli` on the `catastrphicc` output
        let output = lli_command
            .arg(&llvm_output_path)
            .output()
            .expect("Unable to sucessfully run lli");

        assert_eq!(output.stdout, fs::read(test_case.expected).expect("Unable to read expected output"));
    }

    fs::remove_file(llvm_output_path).expect("Unable to delete temporary file");
}

mod compiler {
    use super::*;

    test_cases!(Compiler, run_test_case);
}

use bintest::BinTestBuilder;

#[test]
fn does_this_work() {
    let bintest = BinTestBuilder::new().build_workspace(true).build_executable("catastrophici").build();
    println!("{:?}", bintest.list_executables());

    let manifest_dir = std::env!("CARGO_MANIFEST_DIR");
    println!("MANIFEST: {:?}", manifest_dir);

    let mut filename = std::path::Path::new(manifest_dir).parent().unwrap().to_path_buf();
    filename.push("random.cat");
    println!("Filename: {}", filename.display());

    let mut command = bintest.command("catastrophici");
    let result = command.arg(filename).output().unwrap();
    println!("Result: {:?}", result);
}

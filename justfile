alias i := interpret
alias c := compile

llvm := "${LLVM_SYS_211_PREFIX}"
opt := "all"

# Run the project tests
test:
    cargo test

# Build a given executable
build bin:
    cargo build --bin {{bin}}

# Run the interpreter on a given example in the `examples` directory
interpret example: (build "catastrophici")
    cargo run --bin catastrophici -- examples/{{example}}.cat

_output-ll example: (build "catastrophicc")
    @mkdir -p out
    cargo run --bin catastrophicc -- --opt={{opt}} examples/{{example}}.cat > out/{{example}}.ll

_output-s example: (_output-ll example)
    {{llvm}}/bin/llc out/{{example}}.ll > out/{{example}}.s

# Run the compiler on a given example in the `examples` directory, producing `out/{example}` as a binary
compile example: (_output-s example)
    cc out/{{example}}.s -o out/{{example}}

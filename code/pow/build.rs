fn main() {
    tonic_build::configure()
        .out_dir("src/proto")
        .compile(&["abi.proto"], &["."])
        .unwrap();
}

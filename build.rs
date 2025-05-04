fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_file = "proto/crossplane/apis/apiextensions/fn/proto/v1/run_function.proto";
    tonic_build::configure()
        .build_server(true)
        .out_dir("src")
        .compile_protos(&[proto_file], &["proto"])?;
    Ok(())
}

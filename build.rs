fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_file = "schema/crossplane/apis/apiextensions/fn/proto/v1/run_function.proto";
    tonic_build::configure()
        .extern_path(".google.protobuf.Struct", "::prost_wkt_types::Struct")
        .build_server(true)
        .out_dir("src")
        .compile_protos(&[proto_file], &[".", "build.rs"])?;
    Ok(())
}

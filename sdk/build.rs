use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_file = "schema/crossplane/proto/fn/v1/run_function.proto";
    fs::create_dir_all("src/generated")?;
    tonic_build::configure()
        // use prost_wkt_types to have serializable structs (used in ../example/src/lib.rs to map types)
        .extern_path(".google.protobuf.Struct", "::prost_wkt_types::Struct")
        .build_server(true)
        .out_dir("src/generated")
        .compile_protos(&[proto_file], &[".", "build.rs"])?;
    Ok(())
}

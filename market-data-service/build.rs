fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Tell Cargo to re-run this script if any .proto file changes
    println!("cargo:rerun-if-changed=protos/controller.proto");

    tonic_build::configure()
        .build_server(true)          // generate server traits
        .compile_protos(&["../protos/finam/marketdata/marketdata_service.proto"], &["../protos/finam"])?;

    Ok(())
}
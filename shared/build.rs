fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Tell Cargo to re-run this script if any .proto file changes
    tonic_prost_build::configure()
        .include_file("mod.rs")
        .build_server(true) // generate server traits
        .compile_protos(
            &["protos/finam/grpc/tradeapi/v1/marketdata/marketdata_service.proto"],
            &["protos/finam"],
        )?;

    Ok(())
}

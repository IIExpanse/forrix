fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Tell Cargo to re-run this script if any .proto file changes
    tonic_build::configure()
        .build_server(true) // generate server traits
        .compile_protos(
            &["protos/finam/grpc/tradeapi/v1/marketdata/marketdata_service.proto"],
            &["protos/finam"],
        )?;

    Ok(())
}

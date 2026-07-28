fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure()
        .include_file("mod.rs")
        .build_client(true)
        .compile_protos(
            &[
                "protos/finam/grpc/tradeapi/v1/marketdata/marketdata_service.proto",
                "protos/finam/grpc/tradeapi/v1/auth/auth_service.proto"
            ],
            &["protos/finam"],
        )?;

    Ok(())
}

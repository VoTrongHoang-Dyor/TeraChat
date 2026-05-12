fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_dir = "../proto";

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile(
            &[
                format!("{}/terachat.proto", proto_dir),
                format!("{}/errors.proto", proto_dir),
                format!("{}/signals.proto", proto_dir),
                format!("{}/commands.proto", proto_dir),
            ],
            &[proto_dir],
        )?;

    Ok(())
}

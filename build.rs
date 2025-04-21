fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .out_dir("src/message")
        .compile_protos(
            &["proto/message.proto","proto/list.proto"],
            &["proto/"],
        )?;
    Ok(())
}
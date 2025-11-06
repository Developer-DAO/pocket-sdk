fn main() -> Result<(), Box<dyn std::error::Error>> {
    // if !std::env::var("SHANNON_PROTO_REBUILD").is_ok() {
    //     return Ok(());
    // }

    let mut config = prost_build::Config::new();
    config.out_dir("src");
    config.include_file("_includes.rs");
    config.enable_type_names();
    let path = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
    tonic_buf_build::compile_from_buf_workspace_with_config(
        tonic_build::configure().build_server(false),
        Some(config),
        tonic_buf_build::TonicBufConfig {
            buf_dir: Some(path),
        },
    )?;

    Ok(())
}

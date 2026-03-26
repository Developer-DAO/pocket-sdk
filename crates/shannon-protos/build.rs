use std::error::Error;
use std::process::{Command, Stdio};

fn main() -> Result<(), Box<dyn Error>> {
    if std::env::var("SHANNON_PROTO_REBUILD").is_err() {
        return Ok(());
    }

    // Export the entire protobuf dependency tree into exploded directories.
    let path = std::path::PathBuf::from(std::env::var("OUT_DIR")?);
    let path = path.to_str().ok_or("invalid path")?;

    Command::new("buf")
        .args(["export", "--output", &path])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .spawn()?
        .wait()
        .map_err(|_| "failed to buf export")?;

    // Get the list of protobufs filenames in the exploded directories.
    let mut command = Command::new("buf");
    let output = command
        .args(["ls-files", path])
        .stdout(Stdio::piped())
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .spawn()
        .map_err(|_| "failed to buf ls-files")?;

    // Split the command output into a vector of filename strings.
    let output = output.wait_with_output()?;
    let files = String::from_utf8(output.stdout).unwrap();
    let files = files
        .split('\n')
        .filter(|entry| !entry.is_empty())
        .collect::<Vec<&str>>();

    // Compile protobufs and generate Rust server and client code (gRPC).
    tonic_prost_build::configure()
        .include_file("_includes.rs")
        .out_dir("src")
        .build_server(false)
        .build_client(true)
        .build_transport(true)
        .compile_protos(&files, &[path])?;

    Ok(())
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=proto/model_service.proto");
    println!("cargo:warning=Building proto files...");

    // Compile proto files using default OUT_DIR
    tonic_build::compile_protos("proto/model_service.proto")?;

    println!("cargo:warning=Proto build completed");
    Ok(())
}
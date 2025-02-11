fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../llmd-server/proto/model_service.proto")?;
    Ok(())
} 
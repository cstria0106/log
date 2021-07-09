use std::error::Error;

fn compile_protos() -> std::io::Result<()> {
    tonic_build::compile_protos("../proto/logger.proto")?;
    tonic_build::compile_protos("../proto/ping.proto")?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    compile_protos()?;
    Ok(())
}

use std::io::Result;
fn main() -> Result<()> {
    prost_build::Config::new()
    .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
    .out_dir("src/proto")
    .compile_protos(&["../proto/temperature.proto"], &["../proto"])?;
    Ok(())
}
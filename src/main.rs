use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    dxf_ref_line::run()?;
    Ok(())
}

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    stdr_refline::run()?;
    Ok(())
}

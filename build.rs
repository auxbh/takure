use std::error::Error;
use vergen::EmitBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    EmitBuilder::builder()
        .git_describe(false, true, None)
        .git_sha(false)
        .build_date()
        .emit()?;
    Ok(())
}
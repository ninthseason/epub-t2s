use anyhow::{Ok, Result};
use std::env;
use std::path::Path;
mod utils;

fn main() -> Result<()> {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        return Err(anyhow::anyhow!("Usage: {} <epub-file>", args[0]));
    }
    let file_path = Path::new(&args[1]);
    if !file_path.exists() {
        return Err(anyhow::anyhow!("File not found: {}", file_path.display()));
    }
    utils::process_epub(file_path)?;
    Ok(())
}

use std::path::Path;

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    let dir = Path::new(".").canonicalize()?;
    println!("Current directory: {}", dir.display());
    let rel = dir.join("target").join("release").canonicalize()?;
    println!("Release directory: {}", rel.display());

    Ok(())
}

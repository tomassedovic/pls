use std::path::Path;

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    let dir = Path::new(".").canonicalize()?;
    println!("Current directory: {}", dir.display());
    let rel = dir.join("target").join("release").canonicalize()?;
    println!("Release directory: {}", rel.display());

    let extension = "";

    let release_file = rel.join("pls").with_extension(extension);
    println!("Release file: {}", release_file.display());

    Ok(())
}

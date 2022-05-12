use std::path::Path;

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    let dir = Path::new(".").canonicalize()?;
    println!("Current directory: {}", dir.display());
    let rel = dir.join("target").join("release").canonicalize()?;
    println!("Release directory: {}", rel.display());

    #[cfg(windows)]
    let extension = "exe";
    #[cfg(not(windows))]
    let extension = "";

    let release_file = rel.join("pls").with_extension(extension);
    println!("Release file: {}", release_file.display());

    let release_suffix = std::env::var("TARGET_TRIPLE").unwrap_or_default();

    let mut target_file_name = String::from("pls");
    if !release_suffix.is_empty() {
        target_file_name.push('-');
        target_file_name.push_str(&release_suffix);
    }
    if !extension.is_empty() {
        target_file_name.push('.');
        target_file_name.push_str(extension);
    }

    let target_file = release_file.with_file_name(target_file_name);
    println!("Target file: {}", target_file.display());

    std::fs::copy(&release_file, &target_file)?;

    Ok(())
}

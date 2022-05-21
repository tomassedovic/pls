use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

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

    let target_file = release_file
        .with_file_name(target_file_name)
        .with_extension(extension);
    println!("Target file: {}", target_file.display());
    let source_file_name = release_file
        .file_name()
        .expect("Release file name is `None`")
        .to_str()
        .expect("Could not convert file name to a str.");

    #[cfg(target_os = "linux")]
    {
        use std::process::Command;

        let publish_dir = std::path::PathBuf::from("target/publish");

        if publish_dir.exists() {
            println!(
                "The publish directory exists, deleting: {}",
                publish_dir.display()
            );
        }

        println!("Creating the publish directory: {}", publish_dir.display());
        std::fs::create_dir_all(&publish_dir)?;

        println!(
            "Copying release file \"{}\" to \"{}\"",
            release_file.display(),
            publish_dir.display()
        );
        std::fs::copy(&release_file, publish_dir.join(source_file_name))?;
        println!("Copying readme to the publish dir.");
        std::fs::copy("README.md", publish_dir.join("README.md"))?;
        println!("Copying the license file to the publish dir.");
        std::fs::copy("COPYING.txt", publish_dir.join("COPYING.txt"))?;

        println!("Compressing the publish dir.");
        let mut tarfile = target_file.clone();
        tarfile.set_extension("tar.gz");
        let tarfile = tarfile
            .file_name()
            .expect("Target file name is `None`.")
            .to_str()
            .expect("Could not convert the target file name to str.");

        println!("Destination: {}", publish_dir.join(tarfile).display());
        Command::new("tar")
            .args([
                "-czf",
                tarfile,
                "README.md",
                "COPYING.txt",
                source_file_name,
            ])
            .current_dir(publish_dir)
            .output()
            .expect("Failed to create the tar archive.");
    }

    // TODO: move this to the macos/windows target
    {
        use zip::{write::FileOptions, ZipWriter};
        let method = zip::CompressionMethod::Stored;
        let options = FileOptions::default()
            .compression_method(method)
            .unix_permissions(0o755);

        let mut zip_path = target_file;
        zip_path.set_extension("zip");
        dbg!(&zip_path);
        let zipfile = File::create(&zip_path)?;
        let mut zip = zip::ZipWriter::new(zipfile);

        {
            use std::io::Read;
            zip.start_file("README.md", options)?;
            zip.write_all(std::fs::read_to_string("README.md")?.as_bytes())?;

            zip.start_file("COPYING.txt", options)?;
            zip.write_all(std::fs::read_to_string("COPYING.txt")?.as_bytes())?;

            zip.start_file(source_file_name, options)?;
            let mut f = File::open(release_file)?;
            let mut buf = Vec::new();
            f.read_to_end(&mut buf)?;
            zip.write_all(&buf)?;

            zip.finish()?;
        }
    }

    #[cfg(any(target_os = "macos", target_os = "windows"))]
    {
        // TODO: create a zip file
        println!("Compressing on windows or macos");
    }

    Ok(())
}

use std::{
    fs::File,
    io::{Read, Seek, Write},
    path::{Path, PathBuf},
};

use zip::ZipWriter;

fn zip_add_directory<T: Write + Seek>(
    zip: &mut ZipWriter<T>,
    p: &Path,
    dirname: &str,
    options: zip::write::FileOptions,
) -> anyhow::Result<()> {
    println!("Adding directory: {} as: {}", p.display(), dirname);
    let mut buffer = vec![];
    for entry in walkdir::WalkDir::new(p) {
        let entry = entry?;
        if entry.path() == p {
            continue;
        }
        let stripped = entry.path().strip_prefix(p)?;
        let stripped = PathBuf::from(dirname).join(stripped);
        if entry.file_type().is_file() {
            println!("Adding file: {}", stripped.display());
            let mut f = File::open(entry.path())?;
            f.read_to_end(&mut buffer)?;
            let pname = format!("{}", stripped.display());
            zip.start_file(&pname, options)?;
            zip.write_all(&buffer)?;
            buffer.clear();
        }
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let release_dir = Path::new("target").join("release").canonicalize()?;
    println!("Release directory: {}", release_dir.display());

    let executable_extension = std::env::var("EXECUTABLE_EXTENSION").unwrap_or_default();
    let archive_extension = std::env::var("ARCHIVE_EXT").unwrap_or_else(|_| String::from("zip"));
    let release_suffix = std::env::var("TARGET_TRIPLE").unwrap_or_default();
    let cargo_bundle = std::env::var("CARGO_BUNDLE").unwrap_or_default() == "true";
    let version = {
        let v = std::env::var("VERSION").unwrap_or_default();
        if v.contains('/') {
            println!("Warning: unknown version format: '{v}'. Keeping the version empty.");
            String::new()
        } else {
            v
        }
    };

    let app_build = if cargo_bundle {
        // TODO: handle all bundles, not just the macos app one
        release_dir.join("bundle/osx/pls.app")
    } else {
        release_dir
            .join("pls")
            .with_extension(&executable_extension)
    };
    let files_to_package = vec![
        PathBuf::from("COPYING.txt"),
        PathBuf::from("README.md"),
        app_build,
    ];

    let archive_filename = {
        let mut s = String::from("pls");
        if !version.is_empty() {
            s.push('-');
            s.push_str(&version);
        }
        if !release_suffix.is_empty() {
            s.push('-');
            s.push_str(&release_suffix);
        }
        s.push('.');
        s.push_str(&archive_extension);
        s
    };
    println!("Archive filename: {}", &archive_filename);

    if archive_extension == "zip" {
        let method = zip::CompressionMethod::Stored;
        let options = zip::write::FileOptions::default()
            .compression_method(method)
            .unix_permissions(0o755);

        let zipfile = File::create(&archive_filename)?;
        let mut zip = ZipWriter::new(zipfile);

        let mut buf = Vec::new();
        for p in &files_to_package {
            println!("Adding file to the archive: {}", p.display());
            let filename = p
                .file_name()
                .expect("No file name in path.")
                .to_str()
                .expect("Cannot convert target file to str.");

            // NOTE: A macos app file is in fact a directory
            if p.is_file() {
                let mut f = File::open(p)?;
                zip.start_file(filename, options)?;
                f.read_to_end(&mut buf)?;
                zip.write_all(&buf)?;
            } else if p.is_dir() {
                zip_add_directory(&mut zip, p, filename, options)?;
            } else {
                anyhow::bail!("Path is neither file nor directory: '{}'", p.display());
            }
        }
        zip.finish()?;
    } else if archive_extension == "tar.gz" {
        use flate2::{write::GzEncoder, Compression};

        let tarfile = File::create(&archive_filename)?;
        let encoder = GzEncoder::new(tarfile, Compression::default());
        let mut tar = tar::Builder::new(encoder);
        for p in &files_to_package {
            println!("Adding file to the archive: {}", p.display());
            let filename = p
                .file_name()
                .expect("No file name in path.")
                .to_str()
                .expect("Cannot convert target file to str.");

            // TODO: handle directories.
            let mut f = File::open(p)?;
            tar.append_file(filename, &mut f)?;
        }
    } else {
        anyhow::bail!("Unsupported archive extension: '{archive_extension}'");
    }

    let archive_path = Path::new(&archive_filename);
    if archive_path.is_file() {
        println!("Created archive in: {}", archive_path.display());
    } else {
        anyhow::bail!(
            "Archive '{}' was not created successfully.",
            archive_path.display()
        )
    }

    Ok(())
}

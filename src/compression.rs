use anyhow::Context;
use zip::ZipArchive;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};
use zip::{result::ZipError, write::SimpleFileOptions, ZipWriter};
use std::fs::File;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

pub fn zip_dir<T>(
    it: &mut dyn Iterator<Item = DirEntry>,
    prefix: &Path,
    writer: Arc<Mutex<ZipWriter<T>>>,
    method: zip::CompressionMethod,
) -> anyhow::Result<()>
where
    T: Write + Seek,
{
    let options = SimpleFileOptions::default()
        .compression_method(method)
        .unix_permissions(0o755);

    let prefix = Path::new(prefix);
    let mut buffer = Vec::new();

    // Process each entry in parallel
    it.for_each(|entry| {
        let path = entry.path();
        let name = path.strip_prefix(prefix).unwrap();
        let path_as_string = name
            .to_str()
            .map(str::to_owned)
            .with_context(|| format!("{name:?} is a Non UTF-8 Path"))
            .unwrap();

        if path.is_file() {
            println!("Adding file {path:?} as {name:?}...");
            let mut f = File::open(path).unwrap();
            f.read_to_end(&mut buffer).unwrap();

            // Lock writer to safely access ZipWriter
            let mut zip = writer.lock().unwrap();
            zip.start_file(path_as_string, options).unwrap();
            zip.write_all(&buffer).unwrap();
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            println!("Adding dir {path_as_string:?} as {name:?}...");
            let mut zip = writer.lock().unwrap();
            zip.add_directory(path_as_string, options).unwrap();
        }
    });

    Ok(())
}

pub fn start_compressing(src_dir: &Path, dst_file: &Path, method: zip::CompressionMethod) -> anyhow::Result<()> {
    if !src_dir.is_dir() {
        return Err(ZipError::FileNotFound.into());
    }

    let path = Path::new(dst_file);
    let file = File::create(path).unwrap();
    let zip_writer = ZipWriter::new(file);
    let writer = Arc::new(Mutex::new(zip_writer));

    let walkdir = WalkDir::new(src_dir);
    let it = walkdir.into_iter();

    zip_dir(&mut it.filter_map(|e| e.ok()), src_dir, writer, method)?;

    Ok(())
}



pub fn unzip_file(zip_path: &str, output_dir: &str) -> zip::result::ZipResult<()> {
    // Open the ZIP file
    let zip_file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(zip_file)?;

    // Ensure the output directory exists
    std::fs::create_dir_all(output_dir)?;

    // Iterate through the entries in the ZIP file
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;

        // Get the file's path inside the zip
        let outpath = Path::new(output_dir).join(file.name());

        if (*file.name()).ends_with('/') {
            // If it's a directory, create it
            std::fs::create_dir_all(&outpath)?;
        } else {
            // If it's a file, extract it
            let mut out_file = File::create(&outpath)?;
            std::io::copy(&mut file, &mut out_file)?;
        }
        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                std::fs::set_permissions(&outpath, std::fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
    Ok(())
}
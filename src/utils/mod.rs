use anyhow::Context;
use anyhow::Ok;
use anyhow::Result;
use opencc_rust::*;
use std::fs;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use zip::write::SimpleFileOptions;

pub fn new_opencc() -> OpenCC {
    let output_path = "./dicts";
    generate_static_dictionary(output_path, DefaultConfig::TW2SP).unwrap();
    OpenCC::new(Path::join(Path::new(&output_path), DefaultConfig::TW2SP)).unwrap()
}

pub fn process_epub(fname: &Path) -> Result<()> {
    let file =
        fs::File::open(fname).context(format!("Failed to open file: {}", fname.display()))?;
    let mut archive = zip::ZipArchive::new(file).context("Failed to read file data.")?;
    let out_filename = format!(
        "{}-简中.epub",
        fname.to_string_lossy().trim_end_matches(".epub")
    );
    let out_file = fs::File::create(out_filename).context("Can't create output file.")?;
    let mut out_zip = zip::ZipWriter::new(out_file);
    let opencc = new_opencc();
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).context("Can't read file.")?;
        if file.name().starts_with("OEBPS/Text/") {
            println!("processing: {:?}", file.name());
            out_zip.start_file(file.name(), SimpleFileOptions::default())?;
            let mut string_buf = String::new();
            file.read_to_string(&mut string_buf)
                .context(format!("Can't read file({}) data.", file.name()))?;
            let string_buf = opencc.convert(&string_buf);
            out_zip.write_all(string_buf.as_bytes())?;
        } else {
            out_zip.start_file(file.name(), SimpleFileOptions::default())?;
            let mut buf = Vec::new();
            file.read_to_end(&mut buf)
                .context(format!("Can't read file({}) data.", file.name()))?;
            out_zip.write_all(&buf)?;
        }
    }
    Ok(())
}

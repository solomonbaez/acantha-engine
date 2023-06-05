use std::{env, fs, path::Path};

fn main() -> std::io::Result<()> {
    let source = "../engine/index.html";
    let dir_manifest = env::var("CARGO_MANIFEST_DIR").unwrap();
    let dir_destination = Path::new(&dir_manifest).join("src");
    let file_destination = dir_destination.join("index.html");

    if let Ok(_file_exists) = fs::File::open(&file_destination) {
        return Ok(());
    }

    fs::create_dir_all(&dir_destination).expect("Failed to build destination directory");
    std::fs::copy(source, file_destination).expect("Failed to copy index.html");
    Ok(())
}

use std::env;
use std::fs::{self, DirBuilder};
use std::path::{Path, PathBuf};

fn main() {
    let source_index = "../engine/index.html";
    let dir_manifest = env::var("CARGO_MANIFEST_DIR").unwrap();
    let dir_destination = Path::new(&dir_manifest).join("src");
    let file_destination = dir_destination.join("index.html");

    fs::create_dir_all(&dir_destination).expect("Failed to build destination directory");
    std::fs::copy(source_index, file_destination).expect("Failed to copy index.html");

    let source_wasm = PathBuf::from("../engine/target/wasm32-unknown-unknown");
    let folder_destination =
        PathBuf::from(env::var("OUT_DIR").unwrap()).join("wasm32-unknown-unknown");

    DirBuilder::new()
        .recursive(true)
        .create(&folder_destination)
        .unwrap();

    copy_folder(source_wasm, folder_destination).unwrap();
}

fn copy_folder<P: AsRef<Path>>(src: P, dst: P) -> std::io::Result<()> {
    if src.as_ref().is_dir() {
        DirBuilder::new().recursive(true).create(&dst)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let entry_path = entry.path();
            let dst_path = dst.as_ref().join(entry.file_name());
            copy_folder(entry_path, dst_path)?;
        }
    } else {
        fs::copy(src, dst).expect("Couldn't retrieve file.");
    }
    Ok(())
}

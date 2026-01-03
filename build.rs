use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Path dari folder assets di source
    let assets_src = Path::new("assets");

    // Path target untuk copy (target/debug/assets atau target/release/assets)
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    let target_dir = Path::new(&env::var("OUT_DIR").unwrap())
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join(profile)
        .join("assets");

    if assets_src.exists() {
        // Hapus folder assets lama kalau ada
        let _ = fs::remove_dir_all(&target_dir);

        // Buat folder baru
        fs::create_dir_all(&target_dir).unwrap();

        // Copy semua file dari assets ke target
        for entry in fs::read_dir(assets_src).unwrap() {
            let entry = entry.unwrap();
            let dest_path = target_dir.join(entry.file_name());
            fs::copy(entry.path(), dest_path).unwrap();
        }

        // Bilang ke Cargo rebuild kalau assets berubah
        println!("cargo:rerun-if-changed=assets/");
    }
}

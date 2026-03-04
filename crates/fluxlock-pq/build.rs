use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

    // Go up two directories to workspace root
    let workspace_root = manifest_dir.parent().unwrap().parent().unwrap();

    let lib_path = workspace_root.join("lib");

    println!("cargo:rustc-link-search=native={}", lib_path.display());
    println!("cargo:rustc-link-lib=oqs");
}

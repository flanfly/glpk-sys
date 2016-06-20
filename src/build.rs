fn main() {
    // for brew
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-search=native=/usr/local/lib");
    }

    println!("cargo:rustc-flags=-l glpk");
}

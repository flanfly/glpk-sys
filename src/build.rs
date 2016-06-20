fn main() {
    // for brew
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-search=native=/usr/local");
    }

    println!("cargo:rustc-flags=-l glpk");
}

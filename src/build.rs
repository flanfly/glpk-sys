use std::process::Command;

fn main() {
    // for brew
    if cfg!(target_os = "macos") {
        Command::new("brew")
            .args(&vec!["--prefix"])
            .output().ok().and_then(|x| { String::from_utf8(x.stdout).ok() })
            .map(|x| println!("cargo:rustc-link-search=native={}/lib",x));
    }

    println!("cargo:rustc-flags=-l glpk");
}

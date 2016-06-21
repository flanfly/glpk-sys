use std::process::Command;

fn main() {
    // for brew
    if cfg!(target_os = "macos") {
        let maybe_brew_prfx = Command::new("brew")
            .args(&vec!["--prefix","glpk"])
            .output().ok().and_then(|x| { String::from_utf8(x.stdout).ok() });

        match maybe_brew_prfx {
            Some(x) => println!("cargo:rustc-link-search=native={}/lib",x),
            None => {},
        }
    }

    println!("cargo:rustc-flags=-l glpk");
}

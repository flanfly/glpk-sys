use std::process::Command;
use std::path::PathBuf;

fn main() {
    // for brew
    if cfg!(target_os = "macos") {
        let maybe_brew_prfx = Command::new("brew")
            .args(&vec!["--prefix","homebrew/science/glpk"])
            .output().ok().and_then(|x| { String::from_utf8(x.stdout).ok() })
            .and_then(|x| { x.lines().next().map(|x| x.to_string()) })
            .map(|x| { PathBuf::from(&x) });

        match maybe_brew_prfx {
            Some(x) => println!("cargo:rustc-link-search=native={}",x.join("lib").display()),
            None => {},
        }
    }

    println!("cargo:rustc-flags=-l glpk");
}

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn deps() -> Vec<String> {
    // GLPK_DIR override
    if let Ok(prfx) = env::var("GLPK_DIR") {
        let prfx = PathBuf::from(prfx);
        let inc = prfx.join("include").to_string_lossy().to_string();

        if !prfx.join("include/glpk.h").exists() {
            panic!("GLPK_DIR={prfx:?} does not contain include/glpk.h");
        }

        println!(
            "cargo:rustc-link-search=native={}",
            prfx.join("lib").display()
        );

        return vec![inc];
    }

    if let Ok(lib) = pkg_config::probe_library("glpk") {
        return lib
            .include_paths
            .iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect();
    }

    // for brew
    if cfg!(target_os = "macos") {
        let maybe_brew_prfx = Command::new("brew")
            .args(["--prefix", "glpk"])
            .output()
            .ok()
            .and_then(|x| String::from_utf8(x.stdout).ok())
            .and_then(|x| x.lines().next().map(|x| x.to_string()))
            .map(|x| PathBuf::from(&x));

        match maybe_brew_prfx {
            Some(x) => {
                println!("cargo:rustc-link-search=native={}", x.join("lib").display());
                return vec![x.join("include").to_string_lossy().to_string()];
            }
            None => panic!("Failed to locate glpk libs, consider setting GLPK_DIR"),
        }
    }

    Vec::default()
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=GLPK_DIR");
    println!("cargo:rustc-link-lib=dylib=glpk");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_args(deps().into_iter().map(|p| format!("-I{}", p)))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // we want GLP_UP/LO et.al. to be i32
        .default_macro_constant_type(bindgen::MacroTypeVariation::Signed)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

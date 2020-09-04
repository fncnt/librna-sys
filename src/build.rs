extern crate bindgen;
#[cfg(feature = "auto")]
extern crate pkg_config;

use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::collections::HashSet;

#[derive(Debug)]
struct IgnoreMacros(HashSet<String>);

impl bindgen::callbacks::ParseCallbacks for IgnoreMacros {
    fn will_parse_macro(&self, name: &str) -> bindgen::callbacks::MacroParsingBehavior {
        if self.0.contains(name) {
            bindgen::callbacks::MacroParsingBehavior::Ignore
        } else {
            bindgen::callbacks::MacroParsingBehavior::Default
        }
    }
}

fn main() {
    if let Ok(include_dir) = env::var("LIBRNA_INCLUDE_DIR") {
        println!("cargo:include={}", include_dir);
    }

    if let Some(lib_dir) = env::var_os("LIBRNA_LIB_DIR") {
        let lib_dir = Path::new(&lib_dir);

        if lib_dir.join("libRNA.a").exists() {
            println!("cargo:rustc-link-search=native={}", lib_dir.display());
           }
    }

    let ignored_macros = IgnoreMacros(
            vec![
                "FP_INFINITE".into(),
                "FP_NAN".into(),
                "FP_NORMAL".into(),
                "FP_SUBNORMAL".into(),
                "FP_ZERO".into()
            ]
            .into_iter()
            .collect(),
        );
    // Tell cargo to tell rustc to link the system RNA library.
    println!("cargo:rustc-link-lib=RNA");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=src/wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("src/wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        //.parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // https://github.com/rust-lang/rust-bindgen/issues/687#issuecomment-450750547
        .parse_callbacks(Box::new(ignored_macros))
        .rustfmt_bindings(true)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    if !configure_pkg_config() {
        return
    }
}

#[cfg(feature = "auto")]
fn configure_pkg_config() -> bool {
    // For some reason, the viennaRNA people decided to call the config file "RNAlib2.pc"
    // which is kind of consistent with the inconsistent library name itself
    match pkg_config::probe_library("RNAlib2") {
        Ok(info) => {
            for path in info.include_paths {
                println!("cargo:include={}", path.display());
            }
            true
        },
        Err(err) => {
            println!("cargo:warning=pkg_config failed ({}).", err);
            println!("cargo:warning=Consider setting LIBRNA_INCLUDE_DIR/LIBRNA_LIB_DIR instead.");
            false
        }
    }
}

#[cfg(not(feature = "auto"))]
fn configure_pkg_config() -> bool {
    false
}


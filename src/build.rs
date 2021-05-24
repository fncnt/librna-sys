extern crate bindgen;
#[cfg(feature = "auto")]
extern crate pkg_config;

use std::collections::HashSet;
use std::env;
use std::path::Path;
use std::path::PathBuf;

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
    if !configure_pkg_config() {
        if let Ok(include_dir) = env::var("LIBRNA_INCLUDE_DIR") {
            println!("cargo:include={}", include_dir);
        } else {
            println!("cargo:warning=LIBRNA_INCLUDE_DIR not set.");
            println!("cargo:warning=Using LIBRNA_INCLUDE_DIR=/usr/include as default.");

            println!("cargo:include=/usr/include");
        }

        if let Ok(lib_dir) = env::var("LIBRNA_LIB_DIR") {
            let lib_dir = Path::new(&lib_dir);

            if lib_dir.join("libRNA.a").exists() {
                println!("cargo:rustc-link-search=native={}", lib_dir.display());
            } else {
                println!("cargo:warning=libRNA.a not found!");
                println!("cargo:warning=Fallback mode for locally building libRNA.a is not yet implemented.");
            }
        } else {
            println!("cargo:warning=LIBRNA_LIB_DIR not set.");
            println!("cargo:warning=Using LIBRNA_LIB_DIR=/usr/lib as default.");

            println!("cargo:rustc-link-search=native=/usr/lib");
        }
    }

    let ignored_macros = IgnoreMacros(
        vec![
            "FP_INFINITE".into(),
            "FP_NAN".into(),
            "FP_NORMAL".into(),
            "FP_SUBNORMAL".into(),
            "FP_ZERO".into(),
        ]
        .into_iter()
        .collect(),
    );

    println!("cargo:rustc-link-lib=static=RNA");

    println!("cargo:rerun-if-changed=src/wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("src/wrapper.h")
        .parse_callbacks(Box::new(ignored_macros))
        .rustfmt_bindings(true)
        .generate()
        .expect("Unable to generate bindings");

    // Write bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

#[cfg(feature = "auto")]
fn configure_pkg_config() -> bool {
    match pkg_config::probe_library("RNAlib2") {
        Ok(info) => {
            // println!("cargo:warning=pkg_config probing successful.");
            for path in info.include_paths {
                println!("cargo:include={}", path.display());
            }
            for path in info.link_paths {
                println!("cargo:rustc-link-search=native={}", path.display());
            }
            true
        }
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

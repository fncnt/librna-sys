extern crate bindgen;
#[cfg(feature = "auto")]
extern crate pkg_config;

use std::collections::HashSet;
use std::env;
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
    let mut includes: Vec<PathBuf> = vec![];

    if !configure_pkg_config() {
        let mut include_path = PathBuf::from(env::var("LIBRNA_INCLUDE_DIR").unwrap_or_else(|e| {
            println!(
                "cargo:warning={}. Using default {}={}",
                e, "LIBRNA_INCLUDE_DIR", "/usr/include"
            );
            String::from("/usr/include")
        }))
        .canonicalize()
        .expect("cannot canonicalize path");

        includes.push(include_path.clone());
        include_path.push("ViennaRNA");
        includes.push(include_path);

        for include in &includes {
            // metadata for directly depending crates
            println!("cargo:include={}", include.display());
        }

        let lib_path = PathBuf::from(env::var("LIBRNA_LIB_DIR").unwrap_or_else(|e| {
            println!(
                "cargo:warning={}. Using default {}={}",
                e, "LIBRNA_LIB_DIR", "/usr/lib"
            );
            String::from("/usr/lib")
        }))
        .canonicalize()
        .expect("cannot canonicalize path");

        if lib_path.join("libRNA.a").exists() {
            println!("cargo:rustc-link-search=native={}", lib_path.display());
        } else {
            println!("cargo:warning=libRNA.a not found!");
            println!(
                "cargo:warning=Fallback mode for locally building libRNA.a is not yet implemented."
            );
        }

        const LIBS: &[&str] = &["static=RNA", "stdc++", "gsl", "mpfr", "gomp", "gmp"];

        for lib in LIBS {
            println!("cargo:rustc-link-lib={}", lib);
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

    println!("cargo:rerun-if-changed=src/wrapper.h");
    println!("cargo:rerun-if-env-changed=LIBRNA_LIB_DIR");
    println!("cargo:rerun-if-env-changed=LIBRNA_INCLUDE_DIR");

    let bindings = bindgen::Builder::default()
        .header("src/wrapper.h")
        .parse_callbacks(Box::new(ignored_macros))
        .merge_extern_blocks(true)
        .clang_args(includes.iter().map(|dir| {
            "-I".to_string() + dir.to_str().expect("LIBRNA_INCLUDE_DIR is not valid UTF-8")
        }))
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
    match pkg_config::Config::new()
        .atleast_version("2.7.0")
        .statik(true)
        .probe("RNAlib2")
    {
        Ok(info) => {
            // metadata for directly depending crates
            for path in info.include_paths {
                println!("cargo:include={}", path.display());
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

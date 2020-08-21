extern crate bindgen;
#[cfg(feature = "dynamic")]
extern crate pkg_config;

use std::env;
use std::env::consts;
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
        let dylib_name = format!("{}RNA{}", consts::DLL_PREFIX, consts::DLL_SUFFIX);
        if lib_dir.join(dylib_name).exists() ||
           lib_dir.join("libRNA.a").exists() ||
           lib_dir.join("RNA.lib").exists() {
            println!("cargo:rustc-link-search=native={}", lib_dir.display());
            println!("cargo:rustc-link-lib=RNA");
            return;
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
    // Tell cargo to tell rustc to link the system RNA
    // shared library.
    // Not sure about this though. Shouldn't this only be needed for dynamic linking?
    // Need to learn more about pkg-config
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

    let requires_static_only = cfg!(feature = "static") || env::var("LIBRNA_STATIC").is_ok();
    if requires_static_only || (!configure_pkg_config() && cfg!(feature = "static-fallback")) {
        compile_static();
    }
}

#[cfg(feature = "dynamic")]
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
            println!("cargo:warning=pkg_config failed ({}). Falling back to static build.", err);
            false
        }
    }
}

#[cfg(not(feature = "dynamic"))]
fn configure_pkg_config() -> bool {
    false
}

#[cfg(not(any(feature = "static", feature = "static-fallback")))]
fn compile_static() {
    println!("cargo:warning='static' feature of librna-sys is disabled, so this library won't be build, and probably won't work at all.");
    println!("cargo:rustc-link-lib=RNA");
}

#[cfg(any(feature = "static", feature = "static-fallback"))]
fn compile_static() {
    println!("cargo:warning=Linking statically is not yet implemented.");
}

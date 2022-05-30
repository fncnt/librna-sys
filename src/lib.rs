//! `librna` provides (incomplete) high-level bindings to the [ViennaRNA](https://www.tbi.univie.ac.at/RNA/) package
//! for RNA secondary structure bioinformatics.

/// Re-export of unsafe FFI bindings
///
/// Documention is available [upstream][api-docs].
///
/// [api-docs]: https://www.tbi.univie.ac.at/RNA/ViennaRNA/doc/html/
#[allow(missing_docs, dead_code, non_snake_case, non_camel_case_types)]
pub mod ffi {
    pub use librna_sys::*;
}

/// TODO: fold compound struct docs
#[allow(dead_code)]
pub mod fold_compound;

use ffi::VRNA_VERSION;

/// Version string of the statically linked `ViennaRNA` library.
// Safety: the version string of ViennaRNA should always be valid Unicode
pub const VERSION: &str = unsafe { std::str::from_utf8_unchecked(VRNA_VERSION) };

// just a simple test as example
#[cfg(test)]
mod tests {
    /*use super::*;

    use std::ffi::CString;

    pub fn hamming_distance(s1: &str, s2: &str) -> i32 {
        let seq1 = CString::new(s1).expect("CString::new failed");
        let seq2 = CString::new(s2).expect("CString::new failed");

        unsafe { vrna_hamming_distance(seq1.as_ptr(), seq2.as_ptr()) }
    }

    #[test]
    fn hamming() {
        assert_eq!(hamming_distance("ACGUA", "ACGUC"), 1);
    }*/
}

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// TODO: move higher-level wrapper to separate crate "rna-rs"
use std::ffi::CString;

pub fn hamming_distance(s1: &str, s2: &str) -> i32 {
    let seq1 = CString::new(s1).expect("CString::new failed");
    let seq2 = CString::new(s2).expect("CString::new failed");

    unsafe {
        vrna_hamming_distance(seq1.as_ptr(), seq2.as_ptr())
    }
}

// TODO: write tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hamming() {
        assert_eq!(hamming_distance("ACGUA", "ACGUC"), 1);
    }
}

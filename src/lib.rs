#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::{
        vrna_fold_compound, vrna_fold_compound_free, vrna_fold_compound_t, vrna_hamming_distance,
        vrna_md_t, VRNA_OPTION_EVAL_ONLY, VRNA_VERSION,
    };
    use std::ffi::{CStr, CString};

    struct FoldCompound {
        // TODO: this should be wrapped in NonNull<> for proper safe bindings
        fc: *mut vrna_fold_compound_t,
    }

    impl FoldCompound {
        #[allow(unused)]
        fn new(sequence: &str) -> Self {
            let csequence = CString::new(sequence).expect("CString::new failed");
            // SAFETY: TODO
            let fc = unsafe {
                let md = std::ptr::null::<vrna_md_t>();
                vrna_fold_compound(csequence.as_ptr(), md, VRNA_OPTION_EVAL_ONLY)
            };
            Self { fc }
        }
    }

    impl Drop for FoldCompound {
        fn drop(&mut self) {
            // SAFETY: self.fc is non-null, valid by construction, and has not been freed yet.
            unsafe {
                vrna_fold_compound_free(self.fc);
            }
        }
    }

    pub fn hamming_distance(s1: &str, s2: &str) -> i32 {
        let seq1 = CString::new(s1).expect("CString::new failed");
        let seq2 = CString::new(s2).expect("CString::new failed");

        // SAFETY: vrna_hamming_distance() is supplied with valid, non-null, immutable Rust-managed references.
        unsafe { vrna_hamming_distance(seq1.as_ptr(), seq2.as_ptr()) }
    }

    #[test]
    fn test_hamming() {
        assert_eq!(hamming_distance("ACGUA", "ACGUC"), 1);
    }

    // Prior to ViennaRNA 2.7.0, this would fail linking OpenMP which could be worked around using `openmp-sys`.
    // This is not necessary anymore, so this test is always run to verify linking works in future releases.
    #[test]
    fn test_link_openmp() {
        let sequence = "GUACUGAUGUCGUAUACAGGGCUUUUGACAU";
        let _fc = FoldCompound::new(sequence);
    }

    #[test]
    fn test_version_string() {
        // TODO: Ideally, this would be const but handling Result<>/Option<> types is not const.
        // TODO: We could use const unsafe str::from_utf8_unchecked() but this still has the null byte.
        let version_string: Option<&str> = CStr::from_bytes_until_nul(VRNA_VERSION)
            .ok()
            .and_then(|v| v.to_str().ok());

        // We're only checking that `version_string` is valid UTF8.
        // We could check for the specific string:
        // assert_eq!(version_string, Some("2.7.0"));
        // but this changes with every patch release of ViennaRNA.
        assert!(version_string.is_some());
    }
}

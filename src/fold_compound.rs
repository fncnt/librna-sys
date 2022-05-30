//! A [`FoldCompound`] is a struct encapsulating dynamic programming matrices and thermodynamic model
//! details required for secondary structure computations and related tasks.
//!
//! TODO: proper pairtable wrapper instead of simply ArrayView1<i16>? or maybe only require &[i16]?
//! TODO: wrap VRNA_OPTION_* into enum and make FoldCompound generic over this enum to handle different kinds of FC?
//! TODO: proper model details wrapper struct

use crate::ffi::{
    vrna_eval_structure_pt, vrna_fold_compound, vrna_fold_compound_free, vrna_fold_compound_t,
    vrna_md_t, VRNA_OPTION_EVAL_ONLY,
};
use std::ffi::CString;

/// A wrapper struct around `vrna_fold_compound_t` from ViennaRNA.
pub struct FoldCompound {
    fc: *mut vrna_fold_compound_t,
}

impl FoldCompound {
    /// Create a new `FoldCompound` wrapper object from a string representing an RNA sequence.
    pub fn new(sequence: &str) -> Self {
        let csequence = CString::new(sequence).expect("CString::new failed");
        let fc = unsafe {
            let md = std::ptr::null::<vrna_md_t>();

            vrna_fold_compound(csequence.as_ptr(), md, VRNA_OPTION_EVAL_ONLY)
        };

        Self { fc }
    }

    /// Compute the minimum free energy of an RNA secondary structure provided as a pair table.
    /// Pair tables are 1-indexed and contain the structure's length at position 0.
    pub fn evaluate_structure(&self, pairtable: &[i16]) -> i32 {
        assert_eq!(pairtable.len(), self.len() + 1);
        unsafe { vrna_eval_structure_pt(self.fc, pairtable.as_ptr()) }
    }

    /// Compute the minimum free energy of an RNA secondary structure provided as a pair table.
    /// Pair tables are 1-indexed and contain the structure's length at position 0.
    /// **This returns `[evaluate_structure()] * 0.01` (`kcal/mol`)**
    pub fn evaluate_structure_f64(&self, pairtable: &[i16]) -> f64 {
        self.evaluate_structure(pairtable) as f64 * 0.01
    }

    /// Return the length of the underlying sequence of the fold compound.
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        // Safety: fc.length is a u32 field in ViennaRNA
        unsafe { (*self.fc).length as usize }
    }
}

impl Drop for FoldCompound {
    fn drop(&mut self) {
        unsafe {
            vrna_fold_compound_free(self.fc);
        }
    }
}

// TODO: check safety for this
// https://medium.com/dwelo-r-d/wrapping-unsafe-c-libraries-in-rust-d75aeb283c65
// see bindings.rs #[pyclass(unsendable)]
//unsafe impl Send for FoldCompound {}
//unsafe impl Sync for FoldCompound {}

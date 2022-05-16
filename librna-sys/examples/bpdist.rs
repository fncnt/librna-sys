use libc::free;
use librna_sys::{vrna_ptable_from_string, VRNA_BRACKETS_ANY};
use pyo3::prelude::*;
use std::ffi::CString;

fn ptable_pk(structure: &str) -> Option<Vec<i16>> {
    let cstructure = CString::new(structure).expect("CString::new failed");

    let ptable = unsafe { vrna_ptable_from_string(cstructure.as_ptr(), VRNA_BRACKETS_ANY) };
    if ptable.is_null() {
        unsafe {
            free(ptable as *mut core::ffi::c_void);
        }
        return None;
    } else {
        // length of structure is saved at ptable[0], we don't need it, so [1..structure.len()]
        // would be enough but let's keep it like ViennaRNA
        let ptable_vec = unsafe { std::slice::from_raw_parts(ptable, structure.len()) } //[1..structure.len()]
            .as_ref()
            .to_vec();

        unsafe {
            free(ptable as *mut core::ffi::c_void);
        }

        return Some(ptable_vec);
    }
}

pub fn bp_distance_pk(s1: &str, s2: &str) -> Option<i32> {
    // return None for invalid structures
    let pt1 = ptable_pk(s1)?;
    let pt2 = ptable_pk(s2)?;

    fn compare_pair((i, (&p1, &p2)): (usize, (&i16, &i16))) -> i32 {
        let mut dist = 0;

        if p1 != p2 {
            if p1 > i as i16 {
                dist += 1;
            }
            if p2 > i as i16 {
                dist += 1;
            }
        }
        return dist;
    }
    // calculate and return Some(dist)
    Some(
        pt1.iter()
            .zip(pt2.iter())
            .skip(1) // skip first element (contains just the sizes, don't need them)
            .enumerate() // enumerate elements for helper function
            .map(compare_pair)
            .sum(),
    )
}

#[pymodule]
fn libbpdist(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m)]
    #[pyo3(name = "bp_distance_pk")]
    fn bp_distance_pk_py(_py: Python, s1: &str, s2: &str) -> PyResult<i32> {
        Ok(bp_distance_pk(s1, s2).unwrap())
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const S1: &str = ".((..[[[..))..]]].";
    const S2: &str = ".((.[.[[..))..]]].";

    #[test]
    fn pair_table() {
        assert_eq!(
            ptable_pk(S1),
            Some(vec![
                18, 0, 12, 11, 0, 0, 17, 16, 15, 0, 0, 3, 2, 0, 0, 8, 7, 6
            ])
        );
    }

    #[test]
    fn basepair_distance() {
        assert_eq!(bp_distance_pk(S1, S2), Some(2));
    }
}

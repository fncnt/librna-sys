#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// TODO: write tests

/*#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hamming() {
        assert_eq!(hamming_distance("ACGUA", "ACGUC"), 1);
    }
}*/

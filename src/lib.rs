#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

//! Rust bindings for [`libanalogrytm`](https://github.com/bsp2/libanalogrytm).
//!
//! This crate is not meant to be used directly, but rather as a dependency of [rytm-rs](https://github.com/alisomay/rytm-rs).

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl Default for ar_plock_seq_t {
    fn default() -> Self {
        ar_plock_seq_t {
            plock_type: 0xFF,
            track_nr: 0xFF,
            // This is not project default in AR but I think it is more sound.
            // The project default is 0x00 in AR.
            data: [0xFF; 64],
        }
    }
}

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

//! Rust bindings for [`libanalogrytm`](https://github.com/bsp2/libanalogrytm).
//!
//! This crate is not meant to be used directly, but rather as a dependency of [rytm-rs](https://github.com/alisomay/rytm-rs).

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

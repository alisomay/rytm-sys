#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

//! Rust bindings for my fork of [`libanalogrytm`](https://github.com/alisomay/libanalogrytm).
//!
//! This crate is not meant to be used directly, but rather as a dependency of [rytm-rs](https://github.com/alisomay/rytm-rs).
//!
//! Upstream `libanalogrytm` could be found [here](https://github.com/bsp2/libanalogrytm).

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
#![allow(dead_code)]
#![allow(unused)]

use std::{
    env,
    path::{Path, PathBuf},
};

fn main() {
    // Directories
    let project_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let libanalogrytm_dir = project_dir.join("libanalogrytm");

    cc::Build::new()
        .warnings(false)
        .extra_warnings(false)
        .include(&libanalogrytm_dir)
        .files(&[
            libanalogrytm_dir.join("kit.c"),
            libanalogrytm_dir.join("pattern.c"),
            libanalogrytm_dir.join("sound.c"),
            libanalogrytm_dir.join("sysex.c"),
        ])
        .compile("analogrytm");

    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=analogrytm");

    let mut bindings_builder = bindgen::Builder::default()
        .header("wrapper.h")
        // .array_pointers_in_arguments(doit)
        .generate_comments(true)
        .explicit_padding(true)
        .clang_arg(format!("-I{}", &libanalogrytm_dir.to_string_lossy()))
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));

    let bindings = bindings_builder
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

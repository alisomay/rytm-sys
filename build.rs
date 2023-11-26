#![allow(dead_code)]
#![allow(unused)]

use std::{
    env,
    io::BufRead,
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
            libanalogrytm_dir.join("global.c"),
            libanalogrytm_dir.join("settings.c"),
            libanalogrytm_dir.join("sysex.c"),
        ])
        .compile("analogrytm");

    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=analogrytm");

    let mut bindings_builder = bindgen::Builder::default()
        .header("wrapper.h")
        .generate_comments(true)
        .explicit_padding(true)
        .derive_default(true)
        .clang_arg(format!("-I{}", &libanalogrytm_dir.to_string_lossy()))
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));

    let bindings = bindings_builder
        .generate()
        .expect("Unable to generate bindings");

    let bindings_path = out_dir.join("bindings.rs");

    bindings
        .write_to_file(&bindings_path)
        .expect("Couldn't write bindings!");

    // Post process bindings making every struct packed
    // libanalogrytm only uses 1 byte aligned packed structs which bindgen sometimes does not generate properly.
    let mut file = std::fs::File::open(&bindings_path).unwrap();
    let replaced = std::io::BufReader::new(file)
        .lines()
        .map(|line| {
            let line = line.unwrap();
            if line.contains("#[repr(C)]") {
                line.replace("#[repr(C)]", "#[repr(C, packed)]")
            } else {
                line
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    std::fs::write(&bindings_path, replaced).expect("Couldn't post process bindings!");
}

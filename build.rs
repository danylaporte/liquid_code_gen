use std::{env::var, fs::copy, path::PathBuf};

fn main() {
    let out_dir = var("OUT_DIR").expect("OUT_DIR");
    let mut p = PathBuf::from(&out_dir);
    p.push("liquidlib.lib");

    copy("lib/liquidlib.lib", &p).expect("failed copy liquidlib.lib");

    println!("cargo:rustc-link-lib=static=liquidlib");
    println!("cargo:rustc-link-search=native={}", out_dir);
}

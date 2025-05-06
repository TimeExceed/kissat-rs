#![feature(exit_status_error)]

use std::fs;
use std::path::*;
extern crate giputils;

fn main() -> Result<(), String> {
    giputils::build::git_submodule_update()?;
    println!("cargo:rerun-if-changed=./kissat");

    let kissat_cpps = collect_cpps(&[&Path::new("kissat/src")]);
    let mut build = cc::Build::new();
    build.files(kissat_cpps)
        .includes(["kissat/src"])
        .cpp(false)
        .warnings(false)
        .opt_level(3);
    if let Ok(target) = std::env::var("TARGET") {
        if target.contains("musl") {
            if let Ok(wrapper) = std::env::var("RUST_WRAPPER") {
                if wrapper.contains("sccache") {
                    unsafe {std::env::set_var("CC", "sccache clang")};
                }
            }
            build.no_default_flags(true)
                .static_flag(true)
                .flag("-nostdinc")
                .flag("-I/usr/include/")
                .flag("-nostdlib");
        }
    }
    build.compile("kissat");

    // println!(
    //     "cargo:rustc-link-search=native={}",
    //     cb_path.join("build").display()
    // );
    println!("cargo:rustc-link-lib=static=kissat");
    Ok(())
}

fn collect_cpps(verific_dirs: &[&Path]) -> Vec<PathBuf> {
    let mut cpps = vec![];
    for dir in verific_dirs.iter() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let p = entry.path().canonicalize().unwrap();
            match p.extension().map(|x| x.to_str().unwrap()) {
                Some("c") => {
                    cpps.push(p);
                }
                _ => {}
            }
        }
    }
    cpps
}

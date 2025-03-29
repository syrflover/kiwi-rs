use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use bindgen::{RustEdition, RustTarget};

fn main() {
    link_cxx();

    let kiwi_dir = PathBuf::from("Kiwi")
        .canonicalize()
        .expect("can't canonicalize path");
    let header_path = kiwi_dir.join("include/kiwi/capi.h");
    let header_path_str = header_path.to_str().expect("Path is not a valid string");

    let lib_dir = build_kiwi(&kiwi_dir);

    link_kiwi(&lib_dir);

    let bindings = bindgen::Builder::default()
        .header(header_path_str)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .rust_target(RustTarget::stable(84, 0).unwrap_or_default())
        .rust_edition(RustEdition::Edition2021)
        .generate()
        .unwrap();

    // let out_path = PathBuf::from("src").join("bindings.rs");
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("bindings.rs");

    bindings
        .write_to_file(out_path)
        .expect("couldn't write bindings!");
}

fn link_cxx() {
    let cxx = match std::env::var("CXXSTDLIB") {
        Ok(s) if s.is_empty() => None,
        Ok(s) => Some(s),
        Err(_) => {
            let target = std::env::var("TARGET").unwrap();
            if target.contains("msvc") {
                None
            } else if target.contains("apple")
                | target.contains("freebsd")
                | target.contains("openbsd")
            {
                Some("c++".to_string())
            } else {
                Some("stdc++".to_string())
            }
        }
    };
    if let Some(cxx) = cxx {
        println!("cargo:rustc-link-lib={}", cxx);
    }
}

fn link_kiwi(lib_dir: &Path) {
    println!(
        "cargo:rustc-link-search=native={}",
        lib_dir.to_str().unwrap()
    );
    println!("cargo:rustc-link-lib=static=kiwi_static");
}

fn build_kiwi(kiwi_dir: &Path) -> PathBuf {
    let lib_dir = kiwi_dir.join("build");

    fs::create_dir_all(&lib_dir).unwrap();

    let res = Command::new("git")
        .args(["lfs", "pull"])
        .current_dir(kiwi_dir)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("can't pull lfs");

    if !res.status.success() {
        panic!("can't pull lfs");
    }

    let res = Command::new("git")
        .args(["submodule", "sync"])
        .current_dir(kiwi_dir)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("can't sync submodule");

    if !res.status.success() {
        panic!("can't sync submodule");
    }

    let res = Command::new("git")
        .args(["submodule", "update", "--init", "--recursive"])
        .current_dir(kiwi_dir)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("can't update submodule");

    if !res.status.success() {
        panic!("can't update submodule");
    }

    let res = Command::new("cmake")
        .args(["-DCMAKE_BUILD_TYPE=Release", "../"])
        .current_dir(&lib_dir)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("cmake");

    if !res.status.success() {
        panic!("cmake");
    }

    let res = Command::new("make")
        .current_dir(&lib_dir)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("make");

    if !res.status.success() {
        panic!("make");
    }

    lib_dir
}

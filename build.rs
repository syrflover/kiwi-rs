use std::{
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use bindgen::{RustEdition, RustTarget};

const KIWI_GIT: &str = "https://github.com/bab2min/Kiwi.git";
const KIWI_VERSION: &str = "v0.20.4";

fn main() {
    load_kiwi_sources("Kiwi".as_ref());
    load_kiwi_models("Kiwi".as_ref());

    let kiwi_dir = PathBuf::from("Kiwi").canonicalize().unwrap();
    let header_path = kiwi_dir.join("include/kiwi/capi.h");
    let header_path_str = header_path.to_str().expect("Path is not a valid string");

    if cfg!(feature = "static") {
        static_link(&kiwi_dir, true);
    } else if cfg!(feature = "static_prebuilt") {
        static_link(&kiwi_dir, false);
    } else {
        println!("cargo:rustc-link-lib=dylib=kiwi");
    }

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

fn load_kiwi_sources(kiwi_dir: &Path) {
    if kiwi_dir.exists() {
        let res = Command::new("git")
            .args(["fetch", "--depth", "1"])
            .current_dir(kiwi_dir)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .expect("can't fetch Kiwi");

        if !res.status.success() {
            panic!("can't fetch Kiwi");
        }

        let res = Command::new("git")
            .args(["checkout", KIWI_VERSION])
            .current_dir(kiwi_dir)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .expect("can't checkout Kiwi");

        if !res.status.success() {
            panic!("can't checkout Kiwi");
        }
    } else {
        let res = Command::new("git")
            .args(["clone", "--depth", "1", "--branch", KIWI_VERSION, KIWI_GIT])
            .current_dir(".")
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .expect("can't clone Kiwi");

        if !res.status.success() {
            panic!("can't clone Kiwi");
        }
    }
}

fn load_kiwi_models(kiwi_dir: &Path) {
    let res = Command::new("git")
        .args(["lfs", "pull"])
        .current_dir(kiwi_dir)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("can't lfs pull models");

    if !res.status.success() {
        panic!("can't lfs pull models");
    }
}

fn load_kiwi_submodules(kiwi_dir: &Path) {
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
        .args([
            "submodule",
            "update",
            "--init",
            "--recursive",
            "--depth",
            "1",
        ])
        .current_dir(kiwi_dir)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("can't update submodule");

    if !res.status.success() {
        panic!("can't update submodule");
    }
}

fn static_link(kiwi_dir: &Path, with_build: bool) {
    link_cxx();
    link_kiwi(
        with_build
            .then(|| {
                load_kiwi_submodules(kiwi_dir);
                build_kiwi(kiwi_dir)
            })
            .as_deref(),
    );
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

fn link_kiwi(lib_dir: Option<&Path>) {
    match lib_dir {
        Some(lib_dir) => {
            println!("cargo:rustc-link-search={}", lib_dir.display());
        }
        None => {
            println!("cargo:rustc-link-search=/usr/local/lib");
        }
    }
    println!("cargo:rustc-link-lib=static=kiwi_static");
}

fn build_kiwi(kiwi_dir: &Path) -> PathBuf {
    let mut cmake = cmake::Config::new(kiwi_dir);

    cmake
        .define("CMAKE_BUILD_TYPE", "Release")
        .out_dir(kiwi_dir)
        .very_verbose(true);

    cmake.build().join("build")
}

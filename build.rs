extern crate pkg_config;

fn main() {
    assert!(pkg_config::find_library("hunspell").is_ok());
    println!("cargo:rustc-link-lib=hunspell-1.3.0");
}

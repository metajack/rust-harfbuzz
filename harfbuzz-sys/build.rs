extern crate cmake;
extern crate pkg_config;

use std::env;

fn main() {
    if env::var_os("HARFBUZZ_SYS_NO_PKG_CONFIG").is_none() {
        if pkg_config::find_library("harfbuzz").is_ok() {
            return
        }
    }

    let dst = cmake::Config::new("harfbuzz").build_target("INSTALL").build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=harfbuzz");
}

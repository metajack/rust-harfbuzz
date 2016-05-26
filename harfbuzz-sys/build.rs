extern crate cmake;
extern crate pkg_config;

use std::env;

fn main() {
    if env::var_os("HARFBUZZ_SYS_NO_PKG_CONFIG").is_none() {
        if pkg_config::find_library("harfbuzz").is_ok() {
            return
        }
    }

    let dst = cmake::Config::new("harfbuzz").build_target("ALL_BUILD").build();
    println!("cargo:rust-link-search=native={}", dst.display());
    println!("cargo:rust-link-lib=static=harfbuzz");
}

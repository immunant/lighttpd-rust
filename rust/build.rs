#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    // add unix dependencies below
    // println!("cargo:rustc-flags=-l readline");
    println!("cargo:rustc-link-lib=pcre2-8");
    println!("cargo:rustc-link-lib=crypt");
    println!("cargo:rustc-link-arg=-Wl,-export-dynamic");
}

#[cfg(target_os = "macos")]
fn main() {
    // add macos dependencies below
    // println!("cargo:rustc-flags=-l edit");
}

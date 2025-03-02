fn main() {
    let path = std::env::var("CARGO_MANIFEST_DIR").unwrap() + "/lib";
    println!("cargo:rustc-link-lib=dylib=stdc++");
    println!("cargo:rustc-link-search={path}");
}

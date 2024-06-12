fn main() {
    cc::Build::new()
        .cpp(true)
        .file("src/ffi.cpp")
        .compile("rust-ffi");
    println!("cargo::rerun-if-changed=src/ffi.cpp");
}
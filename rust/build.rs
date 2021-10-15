fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs");
    cxx_build::bridge("src/lib.rs")
        .flag_if_supported("-std=c++14")
        .compile("network");
}

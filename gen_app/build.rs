fn main() {
    cc::Build::new()
        .file("src/lfsr.c")
        .include("src")
        .compile("lfsr");

    println!("cargo:rustc-link-lib=static=lfsr");
}

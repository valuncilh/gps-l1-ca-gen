fn main() {
    cc::Build::new()
        .file("src/lfsr.c")
        .include("src")
        .opt_level(3)
        .compile("lfsr");

    println!("cargo:rustc-link-lib=static=lfsr");
}

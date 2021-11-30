fn main() {
    let mut build = cc::Build::new();
    build.opt_level(3);

    build.flag_if_supported("-g");
    build.flag_if_supported("-Wall");
    build.flag_if_supported("-Wextra");
    build.flag_if_supported("-Wno-sign-compare");
    build.flag_if_supported("-Wno-unused-parameter");
    build.flag_if_supported("-Wuninitialized");
    build.flag_if_supported("-Wwrite-strings");
    build.flag_if_supported("-Wchar-subscripts");
    build.flag_if_supported("-funsigned-char");

    build.flag_if_supported("-Wno-array-bounds");
    build.flag_if_supported("-Wno-format-truncation");

    build.flag_if_supported("-Wno-implicit-fallthrough");

    build.file("src/regex.c").compile("regex.a");

    println!("cargo:rerun-if-changed=src/regex.c");
    println!("cargo:rerun-if-changed=src/libregexp.c");
    println!("cargo:rerun-if-changed=src/libunicode.c");
    println!("cargo:rerun-if-changed=src/cutils.c");
    println!("cargo:rerun-if-changed=build.rs");
}

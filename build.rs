fn main() {
    let mut build = cc::Build::new();
    build.file("src/regex.c").compile("regex.a");
    //    build.file("src/quickjs/cutils.c")
    //        .compile("cutils.a");
    //    build.file("src/quickjs/libunicode.c")
    //        .compile("libunicode.a");
    //    build.file("src/quickjs/libregexp.c")
    //        .compile("libregexp.a");
    println!("cargo:rerun-if-changed=src/regex.c");
}

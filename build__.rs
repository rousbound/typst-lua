extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/typst-lua.c")
        .include("/usr/include/lua5.3")
        .compile("typst-lua");
}

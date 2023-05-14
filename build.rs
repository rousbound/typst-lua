extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/luatyp.c")
        .compile("luatyp");
}

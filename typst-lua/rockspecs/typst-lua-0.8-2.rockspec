package = "typst-lua"
version = "0.8-2"

source = {
    url = "git+https://github.com/rousbound/typst-lua",
}

description = {
    summary = "Typst binding for Lua",
    detailed = "Typst-lua is a simple interface from Lua to Typst for Lua 5.3. It enables a Lua program to generate complex and dynamic pdfs, by passing variables directly to typst. ",
    homepage = "https://github.com/rousbound/typst-lua",
    license = "APACHE"
}

dependencies = {
    "lua >= 5.1",
    "luarocks-build-rust-mlua",
}

build = {
    type = "rust-mlua",

    modules = {
        -- Native library expected in `<target_path>/release/libmy_module.so` (linux; uses right name on macos/windows)
        "typst",
        -- More complex case, native library expected in `<target_path>/release/libalt_name.so`
        ["typst-lua"] = "typst",
    },

    -- Optional: target_path if cargo "target" directory not in the module root
    --target_path = "path/to/cargo/target/directory"

    -- Optional: if set to `false` pass `--no-default-features` to cargo
    default_features = false,

    -- Optional: pass additional features
    --features = {"extra", "features"}
}

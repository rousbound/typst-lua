package = "typst-lua"
version = "1.0-0"

source = {
   url = "git+https://github.com/rousbound/typst-lua.git",
   branch = "main"
}

description = {
   summary = "Typst compiler bindings for Lua",
   detailed = [[
      Lua bindings for the Typst compiler
   ]],
   homepage = "https://github.com/rousbound/typst-lua",
   license = "APACHE2.0"
}

dependencies = {
   "lua >= 5.1, <= 5.5",
   "luarocks-build-rust-mlua",
}

build = {
   type = "rust-mlua",
   modules = {
      ["typst"] = "typst",
   },
}

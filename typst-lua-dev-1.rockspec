package = "typst-lua"
version = "dev-1"

source = {
   url = "git+https://github.com/rousbound/typst-lua.git",
   branch = "dev-1.x"
}

description = {
   summary = "Typst compiler bindings for Lua",
   detailed = [[
      Lua bindings for the Typst compiler
   ]],
   homepage = "https://github.com/rousbound/typst-lua",
   license = "MIT"
}

dependencies = {
   "lua >= 5.1",
   "luarocks-build-rust-mlua",
}

build = {
   type = "rust-mlua",
   modules = {
      ["typst"] = "typst",
   },
}

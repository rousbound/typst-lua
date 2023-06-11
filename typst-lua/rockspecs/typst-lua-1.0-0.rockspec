package = "typst-lua"
version = "1.0-0"
source = {
   url = "https://github.com/rousbound/typst-lua/releases/download/v1/typst-lua.tar.gz",
}
description = {
   summary = "Typst binding for Lua",
   detailed = [[
Typst-lua is a simple interface from Lua to Typst. It enables a Lua program to generate complex and dynamic pdfs. 
   ]],
   license = "APACHE",
}
dependencies = {
   "lua >= 5.3",
}
build = {
   type = "none",
   copy_directories = { "lib" },
   modules = {
      ["typst-lua"] = "lib/typst.so",
   },
}

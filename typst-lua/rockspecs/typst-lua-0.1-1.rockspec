package = "typst-lua"
rockspec_format = "3.0"
version = "0.1-1"
source = {
    url = "https://github.com/rousbound/typst-lua/releases/download/v1/typst.tar.gz",
    dir = "lib"
}
description = {
   summary = "Typst binding for Lua",
   detailed = [[
Typst-lua is a simple interface from Lua to Typst. It enables a Lua program to generate complex and dynamic pdfs, by passing variables directly to typst. 
   ]],
   license = "APACHE",
}
dependencies = {
   "lua >= 5.3",
}
--build = {
   --type = "none",
   --copy_directories = { "lib" },
   --modules = {
      --["typst-lua"] = "lib/typst.so",
   --},
--}

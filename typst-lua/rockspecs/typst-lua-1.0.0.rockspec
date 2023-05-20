package = "typst-lua"
version = "1.0-1"
source = {
   url = "https://github.com/rousbound/typst-lua/tree/main/tyspt-lua",
}
description = {
   summary = "Typst binding for Lua",
   detailed = [[
Typst-lua is a simple interface from Lua to Typst. It enables a Lua program to generate complex and dynamic pdfs. 
   ]],
   license = "APACHE",
}
dependencies = {
   "lua >= 5.1",
}
build = {
   type = "builtin",
   copy_directories = { "lib" },  -- copy the "lib" directory as it is
   modules = {
      example = "lib/typst.so",  -- location of the binary library
   },
}

package = "typst-lua53"
rockspec_format = "3.0"
version = "0.8-1"
source = {
    url = "https://github.com/rousbound/typst-lua",
    dir = "lib"
}
description = {
   summary = "Typst binding for Lua 5.3",
   detailed = [[
Typst-lua is a simple interface from Lua to Typst for Lua 5.3. It enables a Lua program to generate complex and dynamic pdfs, by passing variables directly to typst.
   ]],
   license = "APACHE",
}
dependencies = {
   "lua == 5.3",
}
build = {
   type = "none",
   install = {
      lib = {
         ["typst.so"] = "lib/libtypst53.so"
      }
   }
}

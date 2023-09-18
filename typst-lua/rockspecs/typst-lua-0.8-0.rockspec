package = "typst-lua"
rockspec_format = "3.0"
version = "0.8-0"
source = {
    url = "https://github.com/rousbound/typst-lua",
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
build = {
    type = "none",  -- specify 'none' build type as there's no need to build
    install = {
        lib = {"typst.so"}  -- 'typst' is the name of the binary in the tarball
    }
}

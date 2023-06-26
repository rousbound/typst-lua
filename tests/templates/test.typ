#set text(font: "New Computer Modern")

//-- Lua Table test --
Hello #_LUADATA.world\
Hello #_LUADATA.true\
Hello #_LUADATA.at("1")\
        LUA
//-- JSOAN test
Hello #_LUADATA.message\
Hello #_LUADATA.true\
Hello #_LUADATA.at("1")\
        LUA
//-- Additionas --
Hello #_LUADATA.at("nested_table").at("key")\
        LUA
//-- Lua Tabley
Hello #_LUADATA.at("array").at(0)\
Hello #_LUADATA.at("array").at(1)\
        LUA
//-- Lua Tabled Types
Hello #_LUADATA.at("mixed_types").at("key1")\
Hello #_LUADATA.at("mixed_types").at("key2")\
Hello #_LUADATA.at("mixed_types").at("key3")\


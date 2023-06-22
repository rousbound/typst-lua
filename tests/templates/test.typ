#set text(font: "New Computer Modern")

//-- Lua Table test --
Hello #_DATA.world\
Hello #_DATA.true\
Hello #_DATA.at("1")\

//-- JSON test
Hello #_DATA.message\
Hello #_DATA.true\
Hello #_DATA.at("1")\

//-- Additionas --
Hello #_DATA.at("nested_table").at("key")\

//-- Lua Tabley
Hello #_DATA.at("array").at(0)\
Hello #_DATA.at("array").at(1)\

//-- Lua Tabled Types
Hello #_DATA.at("mixed_types").at("key1")\
Hello #_DATA.at("mixed_types").at("key2")\
Hello #_DATA.at("mixed_types").at("key3")\


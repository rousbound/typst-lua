#set text(font: "New Computer Modern")
#let data = sys.inputs

-- Lua Table test --
 Hello #data.world\
 Hello #data.true\
 Hello #data.at("1")\
         LUA
 -- JSOAN test
 Hello #data.message\
 Hello #data.true\
 Hello #data.at("1")\
         LUA
 -- Additionas --
 Hello #data.at("nested_table").at("key")\
         LUA
 -- Lua Tabley
 Hello #data.at("array").at(0)\
 Hello #data.at("array").at(1)\
         LUA
 -- Lua Tabled Types
 Hello #data.at("mixed_types").at("key1")\
 Hello #data.at("mixed_types").at("key2")\
 Hello #data.at("mixed_types").at("key3")\


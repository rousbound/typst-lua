#set text(font: "New Computer Modern")

-- Lua Table test --
Hello #_DICT.world\
Hello #_DICT.true\
Hello #_DICT.at("1")\

-- JSON test --
Hello #_JSON.message\
Hello #_JSON.true\
Hello #_JSON.at("1")\

-- Additional Tests --
-- Lua Table: Nested Table
Hello #_DICT.at("nested_table").at("key")\

-- Lua Table: Array
Hello #_DICT.at("array").at(0)\
Hello #_DICT.at("array").at(1)\

-- Lua Table: Mixed Types
Hello #_DICT.at("mixed_types")at("key1")\
Hello #_DICT.at("mixed_types")at("key2")\
Hello #_DICT.at("mixed_types")at("key3")\

-- JSON: Nested Object
Hello #_JSON.at("nested_object").at("key")\

-- JSON: Array
Hello #_JSON.at("array").at(0)\
Hello #_JSON.at("array").at(1)\

-- JSON: Mixed Types
Hello #_JSON.at("mixed_types").at("key1")\
Hello #_JSON.at("mixed_types").at("key2")\
Hello #_JSON.at("mixed_types").at("key3")\

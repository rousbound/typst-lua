
#set text(font: "New Computer Modern")


== Lua Table tests
Hello #_LUADATA.world\
Hello #_LUADATA.true\
Hello #_LUADATA.at("1")\

== JSON-style tests
Hello #_LUADATA.message\
Hello #_LUADATA.true\
Hello #_LUADATA.at("1")\

== Nested table tests
Hello #_LUADATA.at("nested_table").at("key")\

== Array tests
#_LUADATA.at("array").at(0)\
#_LUADATA.at("array").at(1)\

== Mixed types
#_LUADATA.at("mixed_types").at("key1")\
#_LUADATA.at("mixed_types").at("key2")\
#_LUADATA.at("mixed_types").at("key3")\

== Extended Tests
-- Boolean access
#_LUADATA.at("mixed_types").at("key2")\

-- Number as string vs number key
Lua number key as int: #_LUADATA.at("1")\
Lua number key via string: #_LUADATA.at("1")\

//-- Missing field (should be none)
//missing: "#_LUADATA.at("nope")"



#set text(font: "New Computer Modern")
#let data = sys.inputs


== Lua Table tests
Hello #data.world\
Hello #data.true\
Hello #data.at("1")\

== JSON-style tests
Hello #data.message\
Hello #data.true\
Hello #data.at("1")\

== Nested table tests
Hello #data.at("nested_table").at("key")\

== Array tests
#data.at("array").at(0)\
#data.at("array").at(1)\

== Mixed types
#data.at("mixed_types").at("key1")\
#data.at("mixed_types").at("key2")\
#data.at("mixed_types").at("key3")\

== Extended Tests
-- Boolean access
#data.at("mixed_types").at("key2")\

-- Number as string vs number key
Lua number key as int: #data.at("1")\
Lua number key via string: #data.at("1")\

//-- Missing field (should be none)
//missing: "#data.at("nope")"


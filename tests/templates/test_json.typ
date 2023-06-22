#set text(font: "New Computer Modern")


-- JSON: Nested Object
Hello #_DATA.json.at("nested_object").at("key")\
            .
-- JSON: Arr.json
Hello #_DATA.json.at("array").at(0)\
Hello #_DATA.json.at("array").at(1)\
            .
-- JSON: Mix.jsones
Hello #_DATA.json.at("mixed_types").at("key1")\
Hello #_DATA.json.at("mixed_types").at("key2")\
Hello #_DATA.json.at("mixed_types").at("key3")\

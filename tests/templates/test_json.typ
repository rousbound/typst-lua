#set text(font: "New Computer Modern")


-- JSON: Nested Object
Hello #_LUADATA.json.at("nested_object").at("key")\
       
-- JSON:Arr.json
Hello #_LUADATA.json.at("array").at(0)\
Hello #_LUADATA.json.at("array").at(1)\
       
-- JSON: Mix.json
Hello #_LUADATA.json.at("mixed_types").at("key1")\
Hello #_LUADATA.json.at("mixed_types").at("key2")\
Hello #_LUADATA.json.at("mixed_types").at("key3")\

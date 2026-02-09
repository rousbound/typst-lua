#set text(font: "New Computer Modern")
#let data = sys.inputs


-- JSON: Nested Object
Hello #data.json.at("nested_object").at("key")\
       
-- JSON:Arr.json
Hello #data.json.at("array").at(0)\
Hello #data.json.at("array").at(1)\
       
-- JSON: Mix.json
Hello #data.json.at("mixed_types").at("key1")\
Hello #data.json.at("mixed_types").at("key2")\
Hello #data.json.at("mixed_types").at("key3")\

return {
    _DICT = typst.from_table(
        {
            world = "World!",
            [true] = true,
            [1] = "one",
            nested_table = {
                key = "Nested Value"
            },
            array = {"value1", "value2"},
            mixed_types = {
                key1 = "string value",
                key2 = true,
                key3 = 123
            }
        }
    ),
    _JSON = typst.from_json([[
        {
            "message": "Hello, world!",
            "true" : true,
            "1" : "one",
            "nested_object": {
                "key": "Nested Value"
            },
            "array": ["value1", "value2"],
            "mixed_types": {
                "key1": "string value",
                "key2": true,
                "key3": 123
            }
        }
    ]])
}

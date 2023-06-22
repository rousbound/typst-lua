return {
    json = typst.from_json([[
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

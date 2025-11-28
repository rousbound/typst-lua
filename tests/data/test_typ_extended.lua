return {
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
        key3 = 123,
        key4 = nil,      -- new
    },

    message = "asdsad",

    -- NEW TESTS

    json_like = {
        foo = "bar",
        number = 999,
        boolean = false,
        inner = { a = 1, b = 2 },
    },

    tricky_keys = {
        ["1"] = "string 1",
        [1] = "numeric 1",
        ["true"] = "string true",
        [false] = "boolean false",
        -- [3.14] = "float key",
    },

    deep_nesting = {
        level1 = {
            level2 = {
                level3 = {
                    value = "deep!"
                }
            }
        }
    },

    array_mixed = {
        "str",
        123,
        true,
        { nested = "yes" }
    },

    empty_table = {},

    null_test = nil,
}


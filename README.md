# typst-lua


Lua binding to [typst](https://github.com/typst/typst),
a new markup-based typesetting system that is powerful and easy to learn. Also has functions that enables lua to pass variables directly to typst.

## Installation

```bash
luarocks install typst-lua
```

## Usage/Docs

```lua
local typst = require"typst"

-----------------------------------------------------
-- Change root of module
-- "compile()" will read templates relative to this path
-- @param String desired root path
-- @return TypstCompiler rust object as UserData in Lua
local compiler = typst.compiler("templates")

-----------------------------------------------------
-- Compiles pdf with given template
-- @param String template name
-- @param Option<Table<String, TypstValue>> data
-- @return Option<Array> pdf bytes
-- @return Option<String> error message
local pdf_bytes, err = compiler:compile(
    "helloworld.typ",
    {
        _DICT = typst.lua_table(
            {
                world = "World!"
            }
        ),
        _TEXT = typst.text"World!",
        _JSON = typst.json(
            [[
                {
                    "world" : "World!"
                }
            ]]
        ),
    }
)
```

## Example

Example with the lua code above in the following "helloworld.typ" file:
```typst
Hello #_DICT.world
Hello #_JSON.world
Hello #_TEXT

```

Output in pdf will be:

```
Hello World!
Hello World!
Hello World!
```





## License

This work is released under the Apache-2.0 license. A copy of the license is provided in the [LICENSE](./LICENSE) file.


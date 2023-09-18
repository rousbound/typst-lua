# typst-lua


Lua binding to [typst](https://github.com/typst/typst),
a new markup-based typesetting system that is powerful and easy to learn. Also has functions that enables lua to pass values directly to typst.

## Installation

```bash
luarocks install typst-lua
```

## Usage/Docs

```lua
local typst = require"typst"

-----------------------------------------------------
-- Compiles pdf with given template
-- @param String template name
-- @param Option<Table<String, TypstValue>> 
-- @return Option<String> pdf bytes
-- @return Option<String> error message
local pdf_bytes, err = typst.compile(
    "helloworld.typ",
    {
       world = "World!",
       json = typst.from_json[[ { "world" : "World!" } ]],
    }
)
```

## Example

Example with the lua code above in the following "helloworld.typ" file:
```typst
Hello #_LUADATA.world
Hello #_LUADATA.json.world

```

Output in pdf will be:

```
Hello World!
Hello World!
```



## License

This work is released under the Apache-2.0 license. A copy of the license is provided in the [LICENSE](./LICENSE) file.


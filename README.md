# typst-lua


Lua binding to [typst](https://github.com/typst/typst),
a new markup-based typesetting system that is powerful and easy to learn. Also has enables lua to pass certain values directly to typst.

## Installation

```bash
luarocks install typst-lua
```

## Usage/Docs

```lua
local typst = require"typst"

-----------------------------------------------------
--- Compiles pdf with given template and optional data
--- Only works with string, number, bool and table
--- @param string template name
--- @param table|nil data 
--- @return string|nil pdf bytes
--- @return string|nil error message
local pdf_bytes, err = typst.compile(
    "helloworld.typ",
    {
       world = "World!",
    }
)

```

## Example

Example with the lua code above in the following "helloworld.typ" file:
```typst
Hello #_LUADATA.world

```

Output in pdf will be:

```
Hello World!
```

## License

This work is released under the Apache-2.0 license. A copy of the license is provided in the [LICENSE](./LICENSE) file.


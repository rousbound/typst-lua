# typst-lua


Lua binding to [typst](https://github.com/typst/typst),
a new markup-based typesetting system that is powerful and easy to learn.

## Installation

```bash
luarocks install typst-lua
```

## Usage

```lua
local typst = require"typst"
local json = require"dkjson"


-----------------------------------------------------
-- Change root of module
-- @param Option<String> desired root path
compiler = typst.compiler("templates")

-----------------------------------------------------
-- Compiles pdf with given template
-- @param String template name
-- @param Option<String> json string
-- @return Option<Array> pdf bytes
-- @return Option<String> error message
local pdf_bytes, err = compiler:compile("hello.typ", json.encode{data = "test"} )

```

## License

This work is released under the Apache-2.0 license. A copy of the license is provided in the [LICENSE](./LICENSE) file.


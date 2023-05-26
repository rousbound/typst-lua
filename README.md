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


-----------------------------------------------------
-- Change root of module
-- @param Option<String> string do path desejado
compiler = typst.compiler("templates")

-----------------------------------------------------
-- Compiles pdf with given template
-- @param String template name
-- @param Option<String> string de dados json 
-- @return Option<Array> de bytes do pdf
local pdf_bytes = compiler:compile("hello.typ", { table = {data = "test"} } )
local pdf_bytes = compiler:compile("hello.typ", { json = json.encode{data = "test"} } )

```

## License

This work is released under the Apache-2.0 license. A copy of the license is provided in the [LICENSE](./LICENSE) file.


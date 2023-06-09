# typst-lua


Lua binding to [typst](https://github.com/typst/typst),
a new markup-based typesetting system that is powerful and easy to learn. Also has a feature to pass lua tables directly to typst dicts.

## Installation

```bash
luarocks install typst-lua
```

## Usage

```lua
local typst = require"typst"


-----------------------------------------------------
-- Change root of module
-- @param Option<String> desired root path
local compiler = typst.compiler("templates")

-----------------------------------------------------
-- Compiles pdf with given template
-- Can accept json data to be stored in the _DICT variable
-- @param String template name
-- @param Option<Table> data
-- @return Option<Array> pdf bytes
-- @return Option<String> error message
local pdf_bytes, err = compiler:compile(
	"helloworld.typ",
	{world = "World!"}
)

```

## Example

Example with the lua code above in the following "helloworld.typ" file:
```typst
Hello #_DICT.world
```

Output in pdf will be:

```
Hello World!
```





## License

This work is released under the Apache-2.0 license. A copy of the license is provided in the [LICENSE](./LICENSE) file.


# typst-lua


Lua binding to [typst](https://github.com/typst/typst),
a new markup-based typesetting system that is powerful and easy to learn. Also has arguments that enable lua to pass certain values directly to typst.

## Installation

```bash
luarocks install typst-lua
```

## Usage/Docs

```lua
local typst = require"typst"

local pdf_bytes, err = typst.compile( "helloworld.typ", { who = "World!"} )

```

## Example

Example with the lua code above in the following "helloworld.typ" file:
```typst
Hello #sys.inputs.who

```

Output in pdf will be:

```
Hello World!
```

# Caveats and Gotchas

## sys.inputs

The LuaTable passed as input will be forcifully coerced into a TypstDict. This is done to follow Typst CLI API semantics, which is:

- Accept key-value pairs through `--input k=v` and save into sys.inputs. Any other more complex structure should be passed as string and serialized with json decode. Therefore, from within Typst document, sys.inputs is a Dict.

Any other LuaTable inside the sys.inputs will be converted following the next rule.

## Lua Tables → Typst Types

Typst distinguishes between **Arrays** and **Dictionaries**, unlike Lua's unified table type. The conversion follows these rules:

A LuaTable becomes a TypstArray **only** if it's a **dense, 1-indexed sequence**:

```lua
local data = {1, 2, 3, 4}
-- Typst: (1, 2, 3, 4)

local data = {[1] = "a", [2] = "b", [3] = "c"}
-- Typst: ("a", "b", "c")
```

Otherwise, it will be turned into a TypstDict:

```lua
local data = {[1] = 1, [2] = 2, [4] = 4}
-- Typst: ("1": 1, "2": 2, "4": 4)

local data = {name = "Alice", age = 30}
-- Typst: (name: "Alice", age: 30)

local data = {[1] = "first", name = "Alice"}
-- Typst: ("1": "first", name: "Alice")
```

## License

This work is released under the Apache-2.0 license. A copy of the license is provided in the [LICENSE](./LICENSE) file.


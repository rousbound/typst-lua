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
Hello #_LUADATA.who

```

Output in pdf will be:

```
Hello World!
```

## License

This work is released under the Apache-2.0 license. A copy of the license is provided in the [LICENSE](./LICENSE) file.


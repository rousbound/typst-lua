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

## Caveats and Gotcha's

Typst has different types for Array and Dict, differently from Lua tables. This kind of type coersion can be done in different ways, this project adopts the following stance:

- A Lua table will be coerced into an array in Typst *if and only if* it is a dense array
Ex: {[1] = 1, [2] = 2, [4] = 4} is sparse and will be coerced into a TypstDict like:
    --> ("1" : 1, "2" : 2, "4" : 4)

## License

This work is released under the Apache-2.0 license. A copy of the license is provided in the [LICENSE](./LICENSE) file.


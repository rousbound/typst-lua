package.cpath = package.cpath .. ';?.so'
local json = require"dkjson"
local typst = require"typst"

local dados = {
	name = "Teste"
}
local compiler = typst.compiler(".")
local pdf_bytes = compiler.compile("main.typ", json.encode(dados))


local fh, err = io.open("main.pdf", "wb")

fh:write(pdf_bytes)

--local result = typst.genpdf("main.typ", ".", json.encode(dados))

--local fh, err = io.open("main.pdf", "wb")

--fh:write(result)


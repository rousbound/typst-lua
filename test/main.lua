package.cpath = package.cpath .. ';?.so'
local typst = require"libtypst_lua"
local json = require"dkjson"

local dados = {
	name = "Teste"
}

local result = typst.genpdf("main.typ", ".", json.encode(dados))

local fh, err = io.open("main.pdf", "wb")

fh:write(result)


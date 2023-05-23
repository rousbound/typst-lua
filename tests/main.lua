package.cpath = package.cpath .. ';?.so'
print("package.cpath:", package.cpath)
local json = require"dkjson"
local typst = require"typst"

local dados = {
	name = "Testeadjasldh"
}
local compiler = typst.compiler(".") -- Seta o root do World
local pdf_bytes = compiler:compile("tests/test1.typ", json.encode(dados))
local fh, err = io.open("tests/test1_result.pdf", "wb")

fh:write(pdf_bytes)

local pdf_bytes = compiler:compile("tests/test2.typ", json.encode(dados))

local fh, err = io.open("tests/test2_result.pdf", "wb")

fh:write(pdf_bytes)

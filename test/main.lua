package.cpath = package.cpath .. ';?.so'
local success, luatyp = pcall(require, "libluatyp")
if not success then
    print("Error loading libluatyp:", luatyp)
else
	local json = require"dkjson"

	local dados = {
		name = "Teste"
	}

	local result = luatyp.genpdf("main.typ", ".", json.encode(dados))

	local fh, err = io.open("main.pdf", "wb")

	fh:write(result)
end


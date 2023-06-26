local typst = require"typst"
-- Generate color functions in global _ENV scope
for c, cc in pairs(
{
	green = {"\27[32m", "\27[0m"},
	yellow = {"\x1b[33m", "\x1b[0m"},
	blue = {"\027[34m", "\027[0m"},
}) do
	_ENV[c] = function(s) return cc[1]..s..cc[2] end
end

local join = function (t) return table.concat(t, "/") end


local tests = {
	compile = {
        ["test.typ"] = {"test.lua"},
        ["test_json.typ"] = {"test_json.lua"},
        ["test_blank.typ"] = {}
	},
}

local output_dir = "output"
local data_dir = "data"

local function write_pdf(bytes, output)
	local fh, err = io.open(output, "wb")
	fh:write(bytes)
	fh:close()
end

local function genpdf(template, file) 
	io.write(
		"Template '"..blue(template).."' "..(file and " with '"..yellow(file).."': " or "")
	)
	local test_data
	if file then 
		local path
		path = join{data_dir, file}
		test_data = loadfile(path, "t", {typst = typst})()
		assert(test_data, "Test data not found on path '"..path.."'")
	end

	local pdf_bytes, err = typst.compile(join{"templates", template}, test_data)

	assert(pdf_bytes,
		"Error generating pdf file of template '"..template.."': \n"
		.."Typst error: "..tostring(err)
	)

	assert(
		string.sub(pdf_bytes, 1, 5) == "%PDF-",
		"File generating isn't a pdf '"..template.."'"
	)

    --write_pdf(
        --pdf_bytes,
        --join{output_dir, template..".pdf"}
    --)
	print(green("OK"))
end


local function test(template, files, method)
	assert(files, "Test not defined")
	if #files > 0 then 
		for _, file in pairs(files) do
			genpdf(template, file)
		end
	else
		genpdf(template)
	end

end

for method, tests in pairs(tests) do 
	local keys = {}
	for key in pairs(tests) do
		table.insert(keys, key)
	end
	table.sort(keys)
	for _, key in ipairs(keys) do
		test(key, tests[key], method)
	end
end
print()
print(green("All tests were successfull"))

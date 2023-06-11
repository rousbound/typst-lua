package.cpath = "../target/release/?.so;"..package.cpath
local typst = require"typst"
local table = require"sintra.table"
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
		["test_simple_compile.typ"] = {"test_simple_compile.lua"},

	},
	compile_with = {
		["test_simple_compile_with.typ"] = {"test_simple_compile_with.lua"}
	}
}



local output_dir = "output"
local data_dir = "data"

local function write_pdf(bytes, output)
	local fh, err = io.open(output, "wb")
	fh:write(bytes)
	fh:close()
end

local compiler = typst.compiler("templates")

local function test(template, files, method)
	assert(files, "Test not defined")
	if #files > 0 then 
		for _, file in pairs(files) do
			
			io.write(
				"Template '"..blue(template).."' with '"..yellow(file).."': "
			)
			local path = join{data_dir, file}

			local pdf_bytes, err = compiler[method](compiler, template, loadfile(path, "t", {typst = typst})())

			assert(pdf_bytes,
				"Error generating pdf file of template '"..template.."': \n"
				.."Typst error: "..tostring(err)
			)

			assert(
				string.sub(pdf_bytes, 1, 5) == "%PDF-",
				"File generating isn't a pdf '"..template.."'"
			)

			local variant = ( #files > 1 and ("-"..file) or "" )
			write_pdf(
				pdf_bytes,
				join{output_dir, template..variant..".pdf"}
			)
			print(green("OK").."\n")
		end
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
	print(green("All tests were successfull"))
end

local socket = require "socket"
package.cpath = "./?.so;" .. package.cpath
local typst = require "typst"

local output_dir = "output"
local data_dir   = "data"

local function join(...)
    local t = {...}
    return table.concat(t, "/")
end

local function write_pdf(bytes, outpath)
    local fh = assert(io.open(outpath, "wb"))
    fh:write(bytes)
    fh:close()
end

local function load_data(data_file)
    if not data_file then return nil end
    local path = join(data_dir, data_file)
    return assert(loadfile(path, "t", _ENV)())
end

local function test_compile(template, data_file, should_error)
    local name = template
    if data_file then
        name = name .. " with " .. data_file
    end
    
    local data = load_data(data_file)
    local t0 = socket.gettime()
    local pdf_bytes, err = typst.compile(join("templates", template), data)
    local ms = (socket.gettime() - t0) * 1000
    
    if should_error then
        assert(err, "Expected compilation error but got none")
        assert(not pdf_bytes, "Expected no PDF output")
        print(string.format("OK: %s errored as expected (%.2f ms)", name, ms))
        print("Error:" .. err)
    else
        assert(not err, "Compilation error: " .. tostring(err))
        assert(pdf_bytes:sub(1,5) == "%PDF-", "Invalid PDF output")
        write_pdf(pdf_bytes, join(output_dir, template .. ".pdf"))
        print(string.format("OK: %s (%.2f ms)", name, ms))
    end
end

-- Tests
test_compile("test_error.typ", "test_typ_extended.lua", true)
test_compile("test_blank.typ")
test_compile("test.typ", "test_typ_extended.lua")
test_compile("test_decode.typ", "decoded_image.lua")
test_compile("test_extended.typ", "test_typ_extended.lua")
test_compile("test_download.typ")
test_compile("test_pdfinclusion.typ", "test_pdf_inclusion.lua")

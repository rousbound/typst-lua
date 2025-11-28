local socket = require "socket"     -- <-- gives socket.gettime()

package.cpath = "./?.so;" .. package.cpath
local typst = require "typst"

-- Colors
local function color(code)
    return function(s)
        return ("\27[%sm%s\27[0m"):format(code, s)
    end
end
local green  = color "32"
local yellow = color "33"
local blue   = color "34"

local join = function(t) return table.concat(t, "/") end

local tests = {
    compile = {
        ["test.typ"]       = {"test.lua"},
        ["test_blank.typ"] = {},
        ["test_extended.typ"] = {"test_typ_extended.lua"},
    },
}

local output_dir = "output"
local data_dir   = "data"

---------------------------------------------------------
-- Utility
---------------------------------------------------------

local function keys(t)
    local r = {}
    for k in pairs(t) do r[#r+1] = k end
    return r
end

local function write_pdf(bytes, outpath)
    local fh = assert(io.open(outpath, "wb"))
    fh:write(bytes)
    fh:close()
end

---------------------------------------------------------
-- PDF generation test (with millisecond timing)
---------------------------------------------------------

local function genpdf(template, data_file)
    io.write("Template " .. blue(template))

    if data_file then
        io.write(" with " .. yellow(data_file))
    end
    io.write(": ")

    -- Load data
    local data = nil
    if data_file then
        local path = join{data_dir, data_file}
        data = assert(loadfile(path, "t", { typst = typst })(),
            "Test data not found at " .. path)
    end

    -----------------------------------------------------
    -- HIGH-RES wall clock
    -----------------------------------------------------
    local t0 = socket.gettime()    -- seconds with microsecond precision

    print("Passing control to typst-compiler")
    local pdf_bytes, err = typst.compile(join{"templates", template}, data)

    assert(not err, "Typst error: " .. tostring(err))
    assert(pdf_bytes and pdf_bytes:sub(1,5) == "%PDF-", "Invalid PDF output")

    write_pdf(pdf_bytes, join{output_dir, template .. ".pdf"})

    local ms = (socket.gettime() - t0) * 1000

    print(green("OK") .. ("  [time: %.2f ms]"):format(ms))
end

---------------------------------------------------------
-- Test dispatcher
---------------------------------------------------------

local function run_test(template, files)
    if #files > 0 then
        for _, f in ipairs(files) do genpdf(template, f) end
    else
        genpdf(template)
    end
end

---------------------------------------------------------
-- Main loop
---------------------------------------------------------

local function main()
    for method, group in pairs(tests) do
        local ts = keys(group)
        table.sort(ts)
        for _, template in ipairs(ts) do
            run_test(template, group[template])
        end
    end

    print()
    print(green("All tests were successful"))
end

main()


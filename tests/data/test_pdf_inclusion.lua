local fh, err = io.open("templates/images/lua.pdf", "r")
assert(fh, err)
return {
        
    pdf = fh:read"a"
}


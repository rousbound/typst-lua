local fh, err = io.open("templates/images/lua.png", "r")
assert(fh, err)
return {
        
    image = fh:read"a"
}

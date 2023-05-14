#include <lua.h>
#include <lauxlib.h>
#include <stdlib.h>

int genpdf(lua_State *L) {
    const char *input = luaL_checkstring(L, 1);
    const char *output = luaL_checkstring(L, 2);
    const char *root = luaL_checkstring(L, 3);
    const char *json = luaL_optstring(L, 4, NULL);

    int result = rust_genpdf(input, output, root, json);

    lua_pushinteger(L, result);
    return 1;
}

int luaopen_genpdf(lua_State *L) {
    lua_register(L, "genpdf", genpdf);
    return 0;
}

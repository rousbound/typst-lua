use std::ffi::{CStr, CString};
use lua_sys::*;
use std::os::raw::c_int;
use typst_genpdf::Compiler;
use std::path::PathBuf;
use libc::{size_t, c_void};

unsafe extern "C" fn compiler_new(L: *mut lua_State) -> c_int {
    // Gets 'root' string from lua_State
    let root = {
        let raw_str = lua_tostring(L, 1);
        CStr::from_ptr(raw_str).to_string_lossy().into_owned()
    };

    // Creates new Compiler instance, and allocates on the heap
    let compiler = Box::new(Compiler::new(PathBuf::from(root)));
    // Gets pointer to Compiler instance allocated on the heap
    let compiler_ptr = Box::into_raw(compiler);

    // Passes Compiler instance pointer to lua, in a 'piece' of userdata
    let userdata = lua_newuserdata(L, std::mem::size_of::<*mut Compiler>() as size_t);
    std::ptr::write(userdata as *mut *mut Compiler, compiler_ptr);

    luaL_getmetatable(L, CString::new("typst.Compiler").unwrap().as_ptr());
    lua_setmetatable(L, -2);

    1
}

unsafe extern "C" fn compiler_compile(L: *mut lua_State) -> c_int {
    let (input, json) = {
        let raw_str_input = lua_tostring(L, 2);
        let input = CStr::from_ptr(raw_str_input).to_string_lossy().into_owned();

        let raw_str_json = lua_tostring(L, 3);
        let json = CStr::from_ptr(raw_str_json).to_string_lossy().into_owned();

        (input, json)
    };

    let compiler_ptr_ptr = luaL_checkudata(L, 1, CString::new("typst.Compiler").unwrap().as_ptr()) as *mut *mut Compiler;
    let compiler = &mut **compiler_ptr_ptr;

    let result = compiler.compile(PathBuf::from(input), Some(serde_json::from_str(&json).unwrap()));

    match result {
        Ok(bytes) => {
            lua_pushlstring(L, bytes.as_ptr() as *const i8, bytes.len() as size_t);
            1
        }
        Err(_) => {
            lua_pushnil(L);
            1
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn luaopen_typst(L: *mut lua_State) -> c_int {
    luaL_newmetatable(L, CString::new("typst.Compiler").unwrap().as_ptr());

    // Create a new table and set its fields
    lua_newtable(L);

    // Add compiler_compile as a function in this table
    lua_pushcfunction(L, Some(compiler_compile));
    lua_setfield(L, -2, CString::new("compile").unwrap().as_ptr());

    // Set this new table as the __index field of the metatable
    lua_setfield(L, -2, CString::new("__index").unwrap().as_ptr());

    // Remove the metatable from the stack
    lua_pop(L, 1);

    // Create a new table for the library and set its fields
    lua_newtable(L);

    // Add compiler_new as a function in this table
    lua_pushcfunction(L, Some(compiler_new));
    lua_setfield(L, -2, CString::new("compiler").unwrap().as_ptr());

    1
}

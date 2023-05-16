use std::ffi::{CStr, CString};
use lua_sys::*;
use std::os::raw::c_int;
use typst_genpdf::Compiler;
use std::path::PathBuf;
use libc::size_t;

unsafe extern "C" fn compiler_new(L: *mut lua_State) -> c_int {
    let root = {
        let raw_str = lua_tostring(L, 1);
        CStr::from_ptr(raw_str).to_string_lossy().into_owned()
    };
    
    let compiler = Box::new(Compiler::new(PathBuf::from(root)));
    let compiler_ptr = Box::into_raw(compiler);
    
    lua_pushlightuserdata(L, compiler_ptr as *mut std::ffi::c_void);
    lua_setglobal(L, CString::new("compiler").unwrap().as_ptr());

    0
}

unsafe extern "C" fn compiler_compile(L: *mut lua_State) -> c_int {
    let (input, json) = {
        let raw_str_input = lua_tostring(L, 1);
        let input = CStr::from_ptr(raw_str_input).to_string_lossy().into_owned();
        
        let raw_str_json = lua_tostring(L, 2);
        let json = CStr::from_ptr(raw_str_json).to_string_lossy().into_owned();
        
        (input, json)
    };
    
    lua_getglobal(L, CString::new("compiler").unwrap().as_ptr());
    let compiler_ptr = lua_touserdata(L, -1) as *mut Compiler;
    
    let result = (*compiler_ptr).compile(PathBuf::from(input), Some(serde_json::from_str(&json).unwrap()));
    
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
    lua_newtable(L);
    lua_pushcfunction(L, Some(compiler_new));
    lua_setfield(L, -2, CString::new("compiler").unwrap().as_ptr());
    
    lua_pushcfunction(L, Some(compiler_compile));
    lua_setfield(L, -2, CString::new("compile").unwrap().as_ptr());

    1
}

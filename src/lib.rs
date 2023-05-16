use libc::{
    c_char,
    size_t,
};
use std::ffi::{CStr, CString};
use std::path::PathBuf;
use serde_json::Value;
use std::str::FromStr;
use typst_genpdf::genpdf;
extern crate lua_sys;

use lua_sys::*;
use std::os::raw::c_int;

// This is the new wrapper function
unsafe extern "C" fn genpdf_c(L: *mut lua_State) -> c_int {
    let input = CStr::from_ptr(lua_tostring(L, 1)).to_string_lossy().into_owned();
    let root = CStr::from_ptr(lua_tostring(L, 2)).to_string_lossy().into_owned();
    let json = CStr::from_ptr(lua_tostring(L, 3)).to_string_lossy().into_owned();
    
    let input = PathBuf::from(input);
    let root = PathBuf::from(root);
    let json = if json.is_empty() {
        None
    } else {
        Some(serde_json::from_str(&json).unwrap())
    };

    let result = match genpdf(input, root, json) {
        Ok(data) => {
            // Convert the Vec<u8> to a pointer and get its length
            let len = data.len();
            let data_ptr = data.as_ptr();

            // Do not allow data to be deallocated
            std::mem::forget(data);

            // Push the data onto the Lua stack
            lua_pushlstring(L, data_ptr as *const i8, len as size_t);
        }
        Err(_) => {
            lua_pushnil(L);
        }
    };

    // The number of return values
    1
}

#[no_mangle]
pub unsafe extern "C" fn luaopen_libtypst_lua(L: *mut lua_State) -> c_int {
    // Create a new table
    lua_newtable(L);

    // Push the genpdf_c function onto the Lua stack
    lua_pushcfunction(L, Some(genpdf_c));

    // Create a C string for the field name
    let field_name = CString::new("genpdf").unwrap();

    // Set it as a field of the table
    lua_setfield(L, -2, field_name.as_ptr());

    // Return the table
    1
}

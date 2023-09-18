#![allow(non_snake_case)]
#![allow(temporary_cstring_as_ptr)]
use std::{
    ffi::{CStr, CString},
    os::raw::c_int,
    path::PathBuf,
};

#[cfg(feature = "lua54")]
extern crate rlua_lua54 as rlua;

#[cfg(feature = "lua53")]
extern crate rlua_lua53 as rlua;
use rlua::*;
use typst_compiler::{compile as typst_compile};
use libc::{size_t, printf};
use typst::eval::{Value, Dict, Array};


#[no_mangle]
#[allow(clippy::missing_safety_doc)]
// Define a C-compatible function to be called when the library is loaded
pub unsafe extern "C" fn luaopen_typst(L: *mut lua_State) -> c_int {
    // Create a new table to hold the library's functions
    lua_newtable(L);

    lua_add_method(L, -2, "compile", compile); // Add the __gc method directly to the metatable
    1
}
unsafe extern "C" fn compile(L: *mut lua_State) -> c_int {
    // Grab the second argument from the Lua stack as raw C string
    let input = lua_to_rust_string(L, 1);

    let mut data: Option<Value> = None;
    if !lua_isnil(L, 2) {
        let result = lua_table_to_typst_value(L, 2);

        data = match result {
            Ok(result) => Some(result),
            Err(e) => {
                let error_message = CString::new(e.to_string()).unwrap();
                lua_pushnil(L);
                lua_pushlstring(L, error_message.as_ptr(), error_message.
                                as_bytes_with_nul().len() as size_t);
                return 2
            }
        }
    }

    let result = typst_compile(&input, &data);

    match result {
        Ok(bytes) => {
            lua_pushlstring(L, bytes.as_ptr() as *const i8, bytes.len() as size_t);
            lua_pushnil(L);
            2
        }
        Err(e) => {
            let error_message = CString::new(e.to_string()).expect("Error");
            lua_pushnil(L);
            lua_pushlstring(L, error_message.as_ptr(), error_message.as_bytes_with_nul().len() as size_t);
            2
        }
    }
}

/// Helper function to add a Lua method to the table at the given index.
unsafe fn lua_add_method(
    L: *mut lua_State,
    index: c_int,
    name: &'static str,
    f: unsafe extern "C" fn(*mut lua_State) -> c_int)
{

    lua_pushcfunction(L, Some(f));
    lua_setfield(L, index, CString::new(name).expect("Error").as_ptr());
}


/// Helper function to retrieve a string from the Lua state and convert it to a Rust string.
unsafe fn lua_to_rust_string(L: *mut lua_State, index: c_int) -> String {
    let mut size: size_t = 0;
    let raw_str = lua_tolstring(L, index, &mut size);
    CStr::from_ptr(raw_str).to_str().expect("Error").to_owned()
}

unsafe fn lua_to_rust_string_no_pop(L: *mut lua_State, index: c_int) -> String {
    let raw_str = lua_tolstring(L, index, &mut 0);
    CStr::from_ptr(raw_str).to_str().unwrap().to_owned()
}

unsafe fn lua_table_to_typst_value(L: *mut lua_State, mut index: c_int) -> Result<Value, &'static str> {
    index = lua_absindex(L, index);

    let mut arr = Array::new();
    let mut map = Dict::new();
    let mut is_array = true;
    let mut expected_key = 1;

    lua_pushnil(L);
    while lua_next(L, index) != 0 {
        let value = match lua_type(L, -1) {
            LUA_TSTRING => {
                Ok(Value::Str(lua_to_rust_string_no_pop(L, -1).into()))
            },
            LUA_TTABLE => {
                Ok(lua_table_to_typst_value(L, -1).unwrap())
            },
            LUA_TNUMBER => {
                let number = lua_tonumber(L, -1);
                Ok(Value::Float(number))
            },
            LUA_TBOOLEAN => {
                let boolean = lua_toboolean(L, -1) != 0;
                Ok(Value::Bool(boolean))
            },
            LUA_TUSERDATA => {
                let raw_ptr = lua_touserdata(L, -1) as *mut Value;
                if raw_ptr.is_null() {
                    panic!("Received null pointer from Lua");
                    Err("Received null pointer from Lua")
                } else {
                    let value = unsafe {
                        // dereference the pointer to get the Value object
                        (*raw_ptr).clone() 
                    };
                    Ok(value)
                }
            },
            _ => Err("Type not expected, table should contain only string, number, table or boolean values")
        }?;
        // Check the key type
        match lua_type(L, -2) {
            LUA_TNUMBER => {
                let key_as_int = lua_tonumber(L, -2) as i32;
                if key_as_int != expected_key {
                    is_array = false;
                }
                expected_key += 1;
                if is_array {
                    arr.push(value.clone());
                }
                let key = key_as_int.to_string();
                map.insert(key.into(), value);
            },
            LUA_TSTRING => {
                is_array = false;
                let key = lua_to_rust_string_no_pop(L, -2);
                map.insert(key.into(), value);
            },
            LUA_TBOOLEAN => {
                is_array = false;
                let key = lua_toboolean(L, -1) != 0;
                map.insert(key.to_string().into(), value);
            },
            _ => {
                lua_pop(L, 1);
                return Err("Non-string or non-number key found");
            },
        }
        lua_pop(L, 1); // Pop the value
    }

    if is_array {
        Ok(Value::Array(arr))
    } else {
        Ok(Value::Dict(map))
    }
}





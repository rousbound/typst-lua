use std::{
    ffi::{CStr, CString},
    os::raw::c_int,
    path::PathBuf,
};

use lua_sys::*;
use typst_compiler::Compiler;
use libc::{size_t, c_void};
use typst::eval::{Value, Dict, Array};
use typst::eval::Str;
use ecow::EcoString;

// Helper function to retrieve a string from the Lua state and convert it to a Rust string.
unsafe fn lua_to_rust_string(L: *mut lua_State, index: c_int) -> String {
    let mut size: size_t = 0;
    let raw_str = lua_tolstring(L, index, &mut size);
    CStr::from_ptr(raw_str).to_str().unwrap().to_owned()
}

// Helper function to add a Lua method to the table at the given index.
unsafe fn lua_add_method(
    L: *mut lua_State,
    index: c_int,
    name: &'static str,
    f: unsafe extern "C" fn(*mut lua_State) -> c_int)
{

    lua_pushcfunction(L, Some(f));
    lua_setfield(L, index, CString::new(name).unwrap().as_ptr());
}

unsafe fn lua_table_is_array(L: *mut lua_State, mut index: c_int) -> bool {
    index = lua_absindex(L, index);
    let mut i = 1;
    loop {
        lua_pushinteger(L, i);
        lua_gettable(L, index);
        let is_nil = lua_type(L, -1) == LUA_TNIL;
        lua_pop(L, 1);
        if is_nil {
            break;
        }
        i += 1;
    }

    let mut total_keys = 0;
    lua_pushnil(L);
    while lua_next(L, index) != 0 {
        lua_pop(L, 1);
        total_keys += 1;
    }

    i - 1 == total_keys
}


//pub fn convert_json(value: serde_json::Value) -> Value {
    //match value {
        //serde_json::Value::Null => Value::None,
        //serde_json::Value::Bool(v) => Value::Bool(v),
        //serde_json::Value::Number(v) => match v.as_i64() {
            //Some(int) => Value::Int(int),
            //None => Value::Float(v.as_f64().unwrap_or(f64::NAN)),
        //},
        //serde_json::Value::String(v) => Value::Str(v.into()),
        //serde_json::Value::Array(v) => {
            //Value::Array(v.into_iter().map(convert_json).collect())
        //}
        //serde_json::Value::Object(v) => Value::Dict(
            //v.into_iter()
                //.map(|(key, value)| (key.into(), convert_json(value)))
                //.collect(),
        //),
    //}
//}

unsafe fn lua_table_to_typst_dict(L: *mut lua_State, mut index: c_int) -> Result<Value, &'static str> {
    index = lua_absindex(L, index);
    if lua_table_is_array(L, index) {
        let mut arr = Array::new();
        lua_pushnil(L);
        while lua_next(L, index) != 0 {
            let value = match lua_type(L, -1) {
                LUA_TSTRING => {
                    Ok(Value::Str(lua_to_rust_string(L, -1).into()))
                },
                LUA_TTABLE => {
                    Ok(lua_table_to_typst_dict(L, -1).unwrap())
                },
                LUA_TNUMBER => {
                    let number = lua_tonumber(L, -1);
                    Ok(Value::Float(number))

                },
                _ => Err("Type not expected")
            }?;
            arr.push(value);
            lua_pop(L, 1);
        }
        Ok(Value::Array(arr))
    } else {
        let mut map = Dict::new();
        lua_pushnil(L);
        while lua_next(L, index) != 0 {
            let key = lua_to_rust_string(L, -2);
            let value = match lua_type(L, -1) {
                LUA_TSTRING => {
                    Ok(Value::Str(lua_to_rust_string(L, -1).into()))
                },
                LUA_TTABLE => {
                    Ok(lua_table_to_typst_dict(L, -1).unwrap())
                },
                LUA_TNUMBER => {
                    let number = lua_tonumber(L, -1);
                    Ok(Value::Float(number))

                },

                _ => Err("Type not expected")
            }?;
            map.insert(key.into(), value);
            lua_pop(L, 1);
        }
        Ok(Value::Dict(map))
    }
}



// Define a C-compatible function to create a new Compiler instance
unsafe extern "C" fn compiler_new(L: *mut lua_State) -> c_int {
    // Create a new scope to keep variable lifetime under control
    let root = lua_to_rust_string(L, 1);

    // Create a new Compiler object with root path
    let compiler = Box::new(Compiler::new(PathBuf::from(root)));
    // Transform the boxed compiler into a raw pointer for FFI
    let compiler_ptr = Box::into_raw(compiler);

    // Allocate memory in the Lua VM for userdata (the raw Compiler pointer)
    let userdata = lua_newuserdata(L, std::mem::size_of::<*mut Compiler>() as size_t);
    // Write the raw compiler pointer to the newly allocated userdata
    std::ptr::write(userdata as *mut *mut Compiler, compiler_ptr);

    // Get the metatable for the Compiler type
    luaL_getmetatable(L, CString::new("typst_Compiler").unwrap().as_ptr());
    // Set the metatable for the userdata
    lua_setmetatable(L, -2);

    // Return 1 to Lua, indicating that we've left one return value on the stack
    1
}

unsafe extern "C" fn compiler_delete(L: *mut lua_State) -> c_int {
    // Get the raw Compiler pointer from the first argument (the userdata)
    let compiler_ptr_ptr = luaL_checkudata(L, 1, CString::new("typst_Compiler").unwrap().as_ptr()) as *mut *mut Compiler;
    // Dereference the pointer and drop the Box, deallocating the Compiler
    Box::from_raw(*compiler_ptr_ptr);
    // Return 0 to Lua, as we don't push anything onto the stack
    0
}


// Define a C-compatible function for the compile method
unsafe extern "C" fn compiler_compile(L: *mut lua_State) -> c_int {
    // Grab the second and third arguments from the Lua stack as raw C strings
    let input = lua_to_rust_string(L, 2);
    //let json = lua_to_rust_string(L, 3);

    let json: Value = match lua_table_to_typst_dict(L, 3) {

        Err(e) => {
            // If there was an error, push nil onto the Lua stack
            lua_pushnil(L);
            // Push error message onto the Lua stack
            //let error_message = CString::new(e.to_string()).unwrap();
            //lua_pushlstring(L, error_message.as_ptr(), error_message.to_bytes().len() as size_t);
            // Maybe I need to push the nul byte terminator, haven't tested it
            let error_message = CString::new(e.to_string()).unwrap();
            lua_pushlstring(L, error_message.as_ptr(), error_message.to_bytes_with_nul().len() as size_t);

            // Return 2 to Lua, indicating that we've left two return values on the stack
            return 2
        },
        Ok(json) => json
    };

    // Get the raw Compiler pointer from the first argument (the userdata)
    let compiler_ptr_ptr = luaL_checkudata(L, 1, CString::new("typst_Compiler").unwrap().as_ptr()) as *mut *mut Compiler;
    // Dereference the pointer to obtain a mutable reference to the Compiler
    let compiler = &mut **compiler_ptr_ptr;

    // Call the compile method on the Compiler and handle the result
    let result = compiler.compile(PathBuf::from(input), Some(json));

    // Match on the result to handle potential errors
    match result {
        Ok(bytes) => {
            // If successful, push the resulting bytes onto the Lua stack
            lua_pushlstring(L, bytes.as_ptr() as *const i8, bytes.len() as size_t);
            // Push nil into the stack for the error message
            lua_pushnil(L);
            // Return 2 to Lua, indicating that we've left two return values on the stack
            2
        }
        Err(e) => {
            // If there was an error, push nil onto the Lua stack
            lua_pushnil(L);
            // Push error message onto the Lua stack
            //let error_message = CString::new(e.to_string()).unwrap();
            //lua_pushlstring(L, error_message.as_ptr(), error_message.to_bytes().len() as size_t);
            // Maybe I need to push the nul byte terminator, haven't tested it
            let error_message = CString::new(e.to_string()).unwrap();
            lua_pushlstring(L, error_message.as_ptr(), error_message.to_bytes_with_nul().len() as size_t);

            // Return 2 to Lua, indicating that we've left two return values on the stack
            2
        }
}
}

// Define a C-compatible function to be called when the library is loaded
#[no_mangle]
pub unsafe extern "C" fn luaopen_typst(L: *mut lua_State) -> c_int {
    // Create a new metatable in the Lua state for Compiler objects
    luaL_newmetatable(L, CString::new("typst_Compiler").unwrap().as_ptr());

    // Create a new table to hold Compiler methods
    lua_newtable(L);

    // Push the compiler_compile function onto the stack
    lua_add_method(L, -2, "compile", compiler_compile); // Add the compile method to it

    // Set the table as the __index metamethod for the Compiler metatable
    lua_setfield(L, -2, CString::new("__index").unwrap().as_ptr());


    lua_add_method(L, -2, "__gc", compiler_delete); // Add the __gc method directly to the metatable

    // Remove the metatable from the stack
    lua_pop(L, 1);

    // Create a new table to hold the library's functions
    lua_newtable(L);

    lua_add_method(L, -2, "compiler", compiler_new); // Add the __gc method directly to the metatable

    // Return 1 to Lua, indicating that we've left one return value on the stack (the library table)
    1
}

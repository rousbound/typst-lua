use std::ffi::{CStr, CString};
use lua_sys::*;
use std::os::raw::c_int;
use typst_compiler::Compiler;
use std::path::PathBuf;
use libc::{size_t, c_void};

// Helper function to convert Lua strings to Rust String
unsafe fn lua_to_rust_string(L: *mut lua_State, index: c_int) -> String {
    let raw_str = lua_tostring(L, index);
    CStr::from_ptr(raw_str).to_string_lossy().into_owned()
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

// Define a C-compatible function for the compile method
unsafe extern "C" fn compiler_compile(L: *mut lua_State) -> c_int {
    // Grab the second and third arguments from the Lua stack as raw C strings
    let input = lua_to_rust_string(L, 2);
    let json = lua_to_rust_string(L, 3);

    // Get the raw Compiler pointer from the first argument (the userdata)
    let compiler_ptr_ptr = luaL_checkudata(L, 1, CString::new("typst_Compiler").unwrap().as_ptr()) as *mut *mut Compiler;
    // Dereference the pointer to obtain a mutable reference to the Compiler
    let compiler = &mut **compiler_ptr_ptr;

    // Call the compile method on the Compiler and handle the result
    let result = compiler.compile(PathBuf::from(input), Some(serde_json::from_str(&json).expect("Could not parse jsonfrom input")));

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
            let error_message = CString::new(e.to_string()).unwrap();
            lua_pushlstring(L, error_message.as_ptr(), error_message.to_bytes().len() as size_t);
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
    lua_pushcfunction(L, Some(compiler_compile));
    // Set the function as the value for the "compile" key in the table
    lua_setfield(L, -2, CString::new("compile").unwrap().as_ptr());

    // Set the table as the __index metamethod for the Compiler metatable
    lua_setfield(L, -2, CString::new("__index").unwrap().as_ptr());

    // Remove the metatable from the stack
    lua_pop(L, 1);

    // Create a new table to hold the library's functions
    lua_newtable(L);

    // Push the compiler_new function onto the stack
    lua_pushcfunction(L, Some(compiler_new));
    // Set the function as the value for the "compiler" key in the table
    lua_setfield(L, -2, CString::new("compiler").unwrap().as_ptr());

    // Return 1 to Lua, indicating that we've left one return value on the stack (the library table)
    1
}

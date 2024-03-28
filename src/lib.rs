#![allow(non_snake_case)]
extern crate alloc;
use bstr::BString;

#[cfg(feature = "lua51")]
extern crate mlua51_lua51 as mlua;
#[cfg(feature = "lua52")]
extern crate mlua52_lua52 as mlua;
#[cfg(feature = "lua53")]
extern crate mlua53_lua53 as mlua;
#[cfg(feature = "lua54")]
extern crate mlua54_lua54 as mlua;

use mlua::prelude::*;
// use typst::eval::{Array, Dict};
use typst::foundations::{Dict, Array};
use typst::foundations::Value as TypstValue;
use typst_compiler::compile as typst_compile;

#[mlua::lua_module]
pub fn typst(lua: &Lua) -> LuaResult<LuaTable> {
    let typst_table = lua.create_table()?;
    typst_table.set(
        "compile",
        lua.create_function(|lua, args: (String, LuaValue)| compile(lua, args))?,
    )?;
    Ok(typst_table)
}

fn compile(
    _lua: &Lua,
    (input, data): (String, LuaValue),
) -> LuaResult<(Option<BString>, Option<String>)> {
    let data = match data {
        LuaValue::Table(table) => lua_table_to_typst_value(table)?,
        _ => None,
    };

    let result = typst_compile(&input, &data);

    match result {
        Ok(bytes) => Ok((Some(BString::from(bytes)), None)),
        Err(e) => Ok((None, Some(e.to_string()))),
    }
}

fn lua_table_to_typst_value(table: mlua::Table) -> LuaResult<Option<TypstValue>> {
    let mut arr = Array::new();
    let mut map = Dict::new();
    let mut is_array = true;
    let mut expected_key = 1;

    for pair in table.pairs::<LuaValue, LuaValue>() {
        let (key, value) = pair?;
        let converted_value = lua_value_to_typst_value(value)?;
        update_data(
            &mut arr,
            &mut map,
            &mut is_array,
            &mut expected_key,
            key,
            converted_value,
        )?;
    }

    if is_array {
        Ok(Some(TypstValue::Array(arr)))
    } else {
        Ok(Some(TypstValue::Dict(map)))
    }
}

fn lua_value_to_typst_value(value: LuaValue) -> LuaResult<TypstValue> {
    match value {
        LuaValue::String(s) => Ok(TypstValue::Str(s.to_str()?.to_owned().into())),
        LuaValue::Table(t) => {
            let inner_value = lua_table_to_typst_value(t)?;
            inner_value.ok_or_else(|| LuaError::ToLuaConversionError {
                from: "Unsupported type",
                to: "Value",
                message: Some("Table type not supported".to_string()),
            })
        }
        LuaValue::Number(n) => Ok(TypstValue::Float(n)),
        LuaValue::Integer(n) => Ok(TypstValue::Int(n as i64)),
        LuaValue::Boolean(b) => Ok(TypstValue::Bool(b)),
        e => Err(LuaError::ToLuaConversionError {
            from: "Unsupported type",
            to: "Value",
            message: Some(format!("{} -- {}", e.to_string().unwrap(), e.type_name())),
        }),
    }
}

fn update_data(
    arr: &mut Array,
    map: &mut Dict,
    is_array: &mut bool,
    expected_key: &mut i32,
    key: LuaValue,
    value: TypstValue,
) -> LuaResult<()> {
    match key {
        LuaValue::Number(n) => {
            if n as i32 != *expected_key {
                *is_array = false;
            }
            *expected_key += 1;
            if *is_array {
                arr.push(value);
            } else {
                map.insert(n.to_string().into(), value);
            }
        }
        LuaValue::Integer(n) => {
            if n as i32 != *expected_key {
                *is_array = false;
            }
            *expected_key += 1;
            if *is_array {
                arr.push(value);
            } else {
                map.insert(n.to_string().into(), value);
            }
        }
        LuaValue::String(s) => {
            *is_array = false;
            map.insert(s.to_str()?.to_owned().into(), value);
        }
        LuaValue::Boolean(b) => {
            *is_array = false;
            map.insert(b.to_string().into(), value);
        }
        e => {
            return Err(LuaError::ToLuaConversionError {
                from: "Unsupported key type",
                to: "String or Number",
                message: Some(format!("{}", e.to_string().unwrap())),
            })
        }
    }
    Ok(())
}

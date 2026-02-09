use mlua::prelude::*;
use typst::foundations::{Array, Dict, Str, Value as TypstValue};
use typst_as_library;

// -------------------------------------
// TRAIT: FromLuaTypst
// -------------------------------------

trait FromLuaTypst {
    fn to_typst(self, lua: &Lua) -> LuaResult<TypstValue>;
}

// Implement for LuaValue
impl FromLuaTypst for LuaValue {
    fn to_typst(self, lua: &Lua) -> LuaResult<TypstValue> {
        match self {
            LuaValue::Nil => Ok(TypstValue::None),

            LuaValue::Boolean(b) => Ok(TypstValue::Bool(b)),
            LuaValue::Number(n) => Ok(TypstValue::Float(n)),
            LuaValue::Integer(n) => Ok(TypstValue::Int(n)),

            LuaValue::String(s) => match s.to_str() {
                Ok(text) => Ok(TypstValue::Str(Str::from(text.to_string()))),
                Err(_) => {
                    let bytes_vec: Vec<u8> = s.as_bytes().to_vec();
                    Ok(TypstValue::Bytes(typst::foundations::Bytes::new(bytes_vec)))
                }
            },

            LuaValue::Table(t) => t.to_typst(lua),

            LuaValue::UserData(ud) => {
                return Err(LuaError::RuntimeError(
                    "Lua userdata cannot be converted to Typst value".into(),
                ));
            }

            other => Err(LuaError::RuntimeError(format!(
                "Unsupported Lua value: {other:?}"
            ))),
        }
    }
}

// -------------------------------------
// Implement for Lua Table (no lifetime)
// -------------------------------------

impl FromLuaTypst for LuaTable {
    fn to_typst(self, lua: &Lua) -> LuaResult<TypstValue> {
        let mut arr = Array::new();
        let mut map = Dict::new();

        let mut is_array = true;
        let mut expected = 1;

        for pair in self.pairs::<LuaValue, LuaValue>() {
            let (key, value) = pair?;
            let v = value.to_typst(lua)?; // recursive

            match key {
                // numeric key
                LuaValue::Integer(idx) => {
                    if idx != expected {
                        is_array = false;
                    }
                    expected += 1;

                    if is_array {
                        arr.push(v.clone());
                    }

                    map.insert(Str::from(idx.to_string()), v);
                }

                // numeric key (float but integer-valued)
                LuaValue::Number(n) if n.fract() == 0.0 => {
                    let idx = n as i64;

                    if idx != expected {
                        is_array = false;
                    }
                    expected += 1;

                    if is_array {
                        arr.push(v.clone());
                    }

                    map.insert(Str::from(idx.to_string()), v);
                }

                // string key
                LuaValue::String(s) => {
                    is_array = false;
                    // let k: String = s.to_str()?.into();
                    let k = s.to_str()?.to_owned(); // String

                    map.insert(Str::from(k), v);
                }

                // boolean key
                LuaValue::Boolean(b) => {
                    is_array = false;
                    map.insert(Str::from(b.to_string()), v);
                }

                other => {
                    return Err(LuaError::RuntimeError(format!(
                        "Unsupported Lua table key: {other:?}"
                    )))
                }
            }
        }

        if is_array {
            Ok(TypstValue::Array(arr))
        } else {
            Ok(TypstValue::Dict(map))
        }
    }
}

// -------------------------------------
// Compile function exposed to Lua
// -------------------------------------

fn compile(
    lua: &Lua,
    (input, data): (LuaString, LuaValue),
) -> LuaResult<(Option<LuaString>, Option<LuaString>)> {
    let input_text = input.to_str()?.to_string();

    let typst_value_opt = match data {
        LuaValue::Table(_) => {
            // Only now do we attempt conversion
            match data.to_typst(lua) {
                Ok(val) => Some(val),
                Err(e) => {
                    let err_msg = lua.create_string(&format!(
                        "typst-lua: error converting lua table to typst value: {e}"
                    ))?;
                    return Ok((None, Some(err_msg)));
                }
            }
        }
        _ => None,
    };

    // Call typst compiler
    let pdf_bytes = match typst_as_library::compile(&input_text, &typst_value_opt) {
        Ok(bytes) => bytes,
        Err(e) => {
            let err_msg = lua.create_string(&format!("typst: {e}"))?;
            return Ok((None, Some(err_msg)));
        }
    };

    // Convert result to lua string
    let pdf = lua.create_string(&pdf_bytes)?;
    Ok((Some(pdf), None))
}

// -------------------------------------
// Module Export
// -------------------------------------

#[mlua::lua_module]
fn typst(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("compile", lua.create_function(compile)?)?;
    Ok(exports)
}

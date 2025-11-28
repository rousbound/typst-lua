use mlua::prelude::*;
use typst::foundations::{Array, Dict, Str, Value};
use typst_as_library;

// -------------------------------------
// TRAIT: FromLuaTypst
// -------------------------------------

trait FromLuaTypst {
    fn to_typst(self, lua: &Lua) -> LuaResult<Value>;
}

// Implement for LuaValue
impl FromLuaTypst for LuaValue {
    fn to_typst(self, lua: &Lua) -> LuaResult<Value> {
        match self {
            LuaValue::Nil => Ok(Value::None),

            LuaValue::Boolean(b) => Ok(Value::Bool(b)),
            LuaValue::Number(n)  => Ok(Value::Float(n)),
            LuaValue::Integer(n)  => Ok(Value::Int(n)),

            LuaValue::String(s) => {
                let s = s.to_str()?.to_string();     // MUST convert BorrowedStr → String
                Ok(Value::Str(Str::from(s)))
            }

            LuaValue::Table(t) => t.to_typst(lua),

            LuaValue::UserData(ud) => {
                let val = ud.borrow::<Value>()?;
                Ok(val.clone())
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
    fn to_typst(self, lua: &Lua) -> LuaResult<Value> {
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
                    let k = s.to_str()?.to_owned();     // String

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
            Ok(Value::Array(arr))
        } else {
            Ok(Value::Dict(map))
        }
    }
}

// -------------------------------------
// Compile function exposed to Lua
// -------------------------------------

fn compile(lua: &Lua, (input, data): (LuaString, LuaValue)) -> LuaResult<LuaString> {
    let input_text = input.to_str()?.to_string();

    let typst_value = data.to_typst(lua)?;

    // compile → Vec<u8>
    let pdf_bytes = typst_as_library::compile(&input_text, &Some(typst_value))
        .map_err(|e| LuaError::RuntimeError(e.to_string()))?;

    // Convert Vec<u8> → Lua binary string
    lua.create_string(&pdf_bytes)
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


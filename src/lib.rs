#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::ffi::{CStr, CString};
use mlua::ffi::lua54::{
  lua_State,
  luaL_newstate,
  luaL_dostring,
  luaL_openlibs,
  luaL_dofile,
  lua_close,
  lua_gc,
  lua_Number,
  lua_pushboolean,
  lua_pushnil,
  lua_pushnumber,
  lua_pushstring,
  lua_resume,
  lua_setglobal,
  lua_status,
  lua_yield,
  lua_Integer,
  lua_pushinteger,
  lua_getfield,
  lua_getglobal,
  lua_gettop,
  lua_pop,
  lua_replace,
  lua_setfield,
  lua_settop,
  lua_toboolean,
  lua_tonumber,
  lua_tostring,
  lua_type
};
use napi::Status;

#[napi(js_name = "STATUS")]
pub enum LuaStatus {
  LUA_OK = 0,
  LUA_YIELD = 1,
  LUA_ERRRUN = 2,
  LUA_ERRSYNTAX = 3,
  LUA_ERRMEM = 4,
  LUA_ERRERR = 5
}

#[napi(js_name = "GARBAGE_COLLECTION")]
pub enum LuaGC {
  LUA_GCSTOP = 0,
  LUA_GCRESTART = 1,
  LUA_GCCOLLECT = 2,
  LUA_GCCOUNT = 3,
  LUA_GCCOUNTB = 4,
  LUA_GCSTEP = 5,
  LUA_GCSETPAUSE = 6,
  LUA_GCSETSTEPMUL = 7,
  LUA_GCSETMAJORINC = 8,
  LUA_GCISRUNNING = 9,
  LUA_GCGEN = 10,
  LUA_GCINC = 11
}


#[napi(js_name = "Lua")]
pub struct JsLuaEngine {
  engine: *mut lua_State,
}

#[napi]
impl JsLuaEngine {
  #[napi(constructor)]
  pub fn new() -> napi::Result<Self> {
    unsafe {
      Ok(JsLuaEngine {
        engine: luaL_newstate()
      })
    }
  }

  #[napi(js_name = "openLibs")]
  pub fn open_libs(&self) -> napi::Result<()> {
    unsafe {
      luaL_openlibs(self.engine);
    }

    Ok(())
  }

  #[napi(js_name = "doString")]
  pub fn do_string(&self, script: String) -> napi::Result<i32> {
    let c_script = CString::new(script).unwrap();
    unsafe {
      Ok(luaL_dostring(self.engine, c_script.as_ptr()))
    }
  }

  #[napi(js_name = "doFile")]
  pub fn do_file(&self, path: String) -> napi::Result<i32> {
    let script_path = CString::new(path).unwrap();
    unsafe {
      Ok(luaL_dofile(self.engine, script_path.as_ptr()))
    }
  }

  #[napi(js_name = "pushString")]
  pub fn push_string(&self, value: String) -> napi::Result<String> {
    let value_c = CString::new(value).unwrap();
    unsafe {
      Ok(CStr::from_ptr(lua_pushstring(self.engine, value_c.as_ptr())).to_str().unwrap().to_string())
    }
  }

  #[napi(js_name = "pushNumber")]
  pub fn push_number(&self, value: i32) {
    unsafe {
      lua_pushnumber(self.engine, lua_Number::from(value));
    }
  }

  #[napi(js_name = "pushInteger")]
  pub fn push_integer(&self, value: i32) {
    unsafe {
      lua_pushinteger(self.engine, lua_Integer::from(value));
    }
  }

  #[napi(js_name = "pushBoolean")]
  pub fn push_boolean(&self, value: i32) {
    unsafe {
      lua_pushboolean(self.engine, value);
    }
  }

  #[napi(js_name = "pushNil")]
  pub fn push_nil(&self) {
    unsafe {
      lua_pushnil(self.engine);
    }
  }

  #[napi(js_name = "setGlobal")]
  pub fn set_global(&self, value: String) {
    let value_c = CString::new(value).unwrap();
    unsafe {
      lua_setglobal(self.engine, value_c.as_ptr());
    }
  }

  #[napi(js_name = "getGlobal")]
  pub fn get_global(&self, value: String) -> napi::Result<i32> {
    let value_c = CString::new(value).unwrap();
    unsafe {
      Ok(lua_getglobal(self.engine, value_c.as_ptr()))
    }
  }

  #[napi(js_name = "getType")]
  pub fn get_type(&self, value: i32) -> napi::Result<i32> {
    unsafe {
      Ok(lua_type(self.engine, value))
    }
  }

  #[napi(js_name = "toString")]
  pub fn to_string(&self, value: i32) -> napi::Result<String> {
    unsafe {
      Ok(CStr::from_ptr(lua_tostring(self.engine, value)).to_str().unwrap().to_string())
    }
  }

  #[napi(js_name = "toNumber")]
  pub fn to_number(&self, value: i32) -> napi::Result<i32> {
    unsafe {
      Ok(lua_tonumber(self.engine, value) as i32)
    }
  }

  #[napi(js_name = "toBoolean")]
  pub fn to_boolean(&self, value: i32) -> napi::Result<bool> {
    unsafe {
      Ok(lua_toboolean(self.engine, value) != 0)
    }
  }

  #[napi(js_name = "getTop")]
  pub fn get_top(&self) -> napi::Result<i32> {
    unsafe {
      Ok(lua_gettop(self.engine))
    }
  }

  #[napi(js_name = "setTop")]
  pub fn set_top(&self, value: i32) {
    unsafe {
      lua_settop(self.engine, value);
    }
  }

  #[napi(js_name = "replace")]
  pub fn replace(&self, value: i32) {
    unsafe {
      lua_replace(self.engine, value);
    }
  }

  #[napi(js_name = "pop")]
  pub fn pop(&self, value: i32) {
    unsafe {
      lua_pop(self.engine, value);
    }
  }

  #[napi(js_name = "setField")]
  pub fn set_filed(&self, value: i32, value2: String) {
    let value2_c = CString::new(value2).unwrap();
    unsafe {
      lua_setfield(self.engine, value, value2_c.as_ptr());
    }
  }

  #[napi(js_name = "getField")]
  pub fn get_field(&self, value: i32, value2: String) -> napi::Result<i32> {
    let value2_c = CString::new(value2).unwrap();
    unsafe {
      Ok(lua_getfield(self.engine, value, value2_c.as_ptr()))
    }
  }

  #[napi(js_name = "yield")]
  pub fn yield_fn(&self, value: i32) -> napi::Result<i32> {
    unsafe {
      Ok(lua_yield(self.engine, value))
    }
  }

  #[napi(js_name = "collectGarbage")]
  pub fn collect_garbage(&self, what: LuaGC, data: i32) -> napi::Result<i32> {
    unsafe {
      Ok(lua_gc(self.engine, what as i32, data))
    }
  }

  #[napi(js_name = "status")]
  pub fn status(&self) -> napi::Result<i32> {
    unsafe {
      Ok(lua_status(self.engine))
    }
  }

  #[napi(js_name = "Resume")]
  pub fn resume(&self, value: i32, value2: i32) -> napi::Result<i32> {
    unsafe {
      Ok(lua_resume(self.engine, self.engine, value, value2 as *mut i32))
    }
  }

  #[napi(js_name = "close")]
  pub fn close(&self) {
    unsafe {
      lua_close(self.engine);
    }
  }
}
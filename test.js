const w = require("./nodejs_lua_bindings_rs");

const l = new w.Lua();
l.openLibs();
console.log(l.doString("pridnt('hello')"));
l.pushInteger(5);
l.setGlobal("test");
l.doString("print('Test value: ' .. test .. '. Invoked from lua!')");
l.close();
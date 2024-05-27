const w = require("./index");

const l = new w.Lua();
l.openLibs();
console.log(l.doString("pridnt('hello')"));
l.pushInteger(5);
l.setGlobal("test");
l.doString("print('Test value: ' .. test .. '. Invoked from lua!')");
l.close();
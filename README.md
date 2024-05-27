# Node.js Lua Bindings in Rust

This project aims to provide bindings from Lua to Node.js using Rust. It leverages the power of the `mlua` and `napi-rs` modules to provide a seamless and efficient interface between the two languages.

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

- [Node.js](https://nodejs.org/)
- [Rust](https://www.rust-lang.org/)
- [Napi](https://napi.rs) (More about installing above)

## Installation

Clone the repository
```bash
git clone https://github.com/cymerous/nodejs_lua_bindings_rs.git
```

Navigate to the project directory
```bash
cd nodejs_lua_bindings_rs
```

To build the project, it’s necessary to install the Napi dependency. You can do this using one of the following commands:

If you’re using Yarn, run:
```bash
yarn global add @napi-rs/cli
```
If you’re using npm, use this command:
```bash
npm install -g @napi-rs/cli
```
Alternatively, if you’re using pnpm, you can use:
```bash
pnpm add -g @napi-rs/cli
```

Build the project
```bash
napi build --platform --release
```

## Usage

You can find an example in the [test.js](https://github.com/cymerous/nodejs_lua_bindings_rs/blob/main/test.js) file, or you can refer to the [Lua documentation](https://www.lua.org/source/5.4/lua.h.html). However, not all methods are utilized. You can view all available methods [here](https://github.com/cymerous/nodejs_lua_bindings_rs/blob/main/index.d.ts).

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](https://github.com/cymerous/nodejs_lua_bindings_rs/blob/main/license) file for details

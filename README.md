# Compiling Rust to wasm without wasm-pack

### **Reference table**
<br/>

1. [Before getting started](#Before-getting-started)
2. [Compiling](#Compiling)
3. [Exporting Functions](#Exporting-Functions)
4. [Importing functions](#Importing-functions)
5. [Exporting Tables](#Exporing-Tables)
6. [Custom sections](#Custom-sections)
7. [Limitations](#Limitations)
<br/>

## Before getting started

1. Install [rust and cargo](https://www.rust-lang.org/)

2. Install wasm targets
```sh
# with standard libary
rustup target add wasm32-wasi

# without file operations and other useful util from standard libary
rustup target add wasm32-unknown-unknown
```

3. Optional: Install wasm debug utils (wabt) 

**Debian**
```sh
sudo apt install wabt
```
**Fedora**
```sh
sudo dnf install wabt
```
**ARCH**
```
sudo yay -S wabt
```
**Source**
```sh
git clone --recursive https://github.com/WebAssembly/wabt
cd wabt
git submodule update --init

mkdir build
cd build
cmake ..
cmake --build .

```
## **Compiling**

**with wasm32-unknown-unknown architecture**
```sh
cargo build --target=wasm32-unknown-unknown
```
**with wasm32-wasi**
```sh
cargo build --target=wasm32-wasi
```


## **Exporting functions**
```rust
#[no_mangle]
pub fn foo(){
  // Do Somthing
}

```
## **Importing functions**
### Basic
```rust
extern "C" {
    fn import_foo() -> i32;
}

```
### With namespace
```rust
// Import function "import_fn" module/namespace "foo"
#[link(wasm_import_module = "namespace_foo")]
extern "C" {
    fn import_foo() -> i32;
}
```

**IMPORTANT:** Must use import function** otherwise compilier will optimise it away
**workaround/Solution**:
```rust
pub fn import(){
	drop(import_foo());
}

```

## **Exporting Tables**
**IMPORTANT**
  this will not export functions pointers unless built with RUSTFLAGS="-C link-arg=--export-table"

**EXAMPLE**

```sh
 # In terminal use
 RUSTFLAGS="-C link-arg=--export-table" cargo b --target=wasm32-unknown-unknown
```
**NOTE:** will export all function pointer

```rust
// In rust file

#[no_mangle]
pub extern "C" fn fnptr() -> i32{0}

const FNPTRS: [unsafe extern "C" fn() ->i32; 2] = [import_foo, fn_bar];

#[no_mangle]
pub unsafe extern "C" fn fn_bar() -> i32 {0}

#[no_mangle]
pub unsafe fn fnptrs(cond: i32) -> i32{
    FNPTRS[cond as usize ]()
} 
```
## **Custom sections**
```rust
#[used]
#[link_section = "Custom_Section_foo"]
pub static CUSTOMSECTIONFOO: () =();
```

**Possible use case for custom sections:**
- can be use "metadata programming"
- can be use to post process wasm file
- can be use metadata to create function with externref param
- can be use metadata to schedule a function exectuion
- can be use metadata to run undercertain condition

**NOTE:** There is currently an [issues](https://github.com/rust-lang/rust/issues/56639) relating to dependency
TODO: Research issue :(

## **Limitations**
- unable to use type externref :(
- unable to control table exports
- issues with [custom sections](https://github.com/rust-lang/rust/issues/56639)

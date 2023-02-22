# Compiling Rust to wasm without wasm-pack

<br>

### **Reference table**

1. [Before getting started](#Before-getting-started)
2. [Compiling](#Compiling)
3. [Exporting Functions](#Exporting-Functions)
4. [Importing functions](#Importing-functions)
5. [Exporting Tables](#Exporing-Tables)
6. [Custom sections](#Custom-sections)
7. [component model (in progress)](#component-model)
7. [Limitations](#Limitations)
8. [Debuging tips/notes](#Debuging-tips-and-notes)
9. [Other Resources](#Other-Resources)

<br>

## Before getting started

Install [rust and cargo](https://www.rust-lang.org/)

1\. Install wasm targets

<details>
<summary> 2. Install wasm targets</summary>

```sh
# with standard libary
rustup target add wasm32-wasi

# without file operations and other useful operations/utils from standard libary
rustup target add wasm32-unknown-unknown
```
</details>

<details><summary>3. Optional: Install wasm debug utils (wabt) and llvm</summary>

**Debian**
```sh
sudo apt install wabt llvm-15-dev
```
**Fedora**
```sh
sudo dnf install wabt llvm-15-dev
```
**ARCH**
```sh
sudo yay -S wabt llvm-15-dev
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
</details>

<br>

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

**Basic**
```rust
extern "C" {
    fn import_foo() -> i32;
}

```
**With namespace**
```rust
// Import function "import_fn" from module/namespace "namespace_foo"
#[link(wasm_import_module = "namespace_foo")]
extern "C" {
    fn import_foo() -> i32;
}
```

<br>

**IMPORTANT:** Must use import function** otherwise compilier will optimise it away
<br/>
**workaround/Solution**:
```rust
pub fn import(){
	drop(import_foo());
}

```

## **Exporting Tables**
```rust
// # Function pointer/wasm table
#[no_mangle]
pub fn fn_bar() -> i32 {
    0
}

#[used]
#[no_mangle]
pub static FNPTRS: [fn() -> i32; 1] = [fn_bar];
```

## **Custom sections**

```rust
#[used]
#[link_section = "Custom_Section_foo"]
pub static CUSTOMSECTIONFOO: [u8; 11] = *b"Hello World";
```
Extracting custom section data in terminal

````sh
  llvm-objdump -s -j Custom_Section_foo $PATH_TO_WASM FILE
````

#### **Possible use case for custom sections:**
- can be use to do something similiar to [metadata programming](https://stackoverflow.com/questions/514644/what-exactly-is-metaprogramming)
- can use metadata to post process wasm file
- can use metadata to post process wasm files to generate function with externref
- can use metadata to schedule a function exectuion
- can use metadata to run undercertain condition/events


**NOTE:** There is currently an [issues](https://github.com/rust-lang/rust/issues/56639) relating to dependency

TODO: Research issue :(

# **Component Model**
 Currently the Component Model Proposal is **in progress** but solves these main issues/introduce concept:
  - way to define types
  - An Abi
  - Module and component linking: dynamically composing modules into components. 
 [more info](https://www.fermyon.com/blog/webassembly-component-model)


<br>

## **Limitations**
- Code generation is not guarantee
- unable to use type externref :(
- issues with [custom sections](https://github.com/rust-lang/rust/issues/56639)

## **Debuging tips and notes**
- When in doubt make everything public (**pub**) and no mangling (**#[no_mangle]**) your code.
- Use i32 value for less code generation


<br/>

### **Other Resources** 
- [Understanding WebAssembly text format](https://developer.mozilla.org/en-US/docs/WebAssembly/Understanding_the_text_format)
- [WebAssembly Core Specification version 1](https://www.w3.org/TR/wasm-core-1/)
- [WebAssembly Core Specification version 2](https://www.w3.org/TR/wasm-core-2/)
- [Rust and WebAssembly](https://rustwasm.github.io/book/)
- [Rust + Wasm issues](https://github.com/rust-lang/rust/labels/O-wasm)

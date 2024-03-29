# Compiling Rust to wasm the hard way.

<br>

### **Reference table**

 1. [Before getting started](#Before-getting-started)
 2. [Compiling](#Compiling)
 3. [Exporting Functions](#Exporting-Functions)
 4. [Importing functions](#Importing-functions)
 5. [Exporting Tables](#Exporing-Tables)
 6. [Custom sections](#Custom-Sections)
 7. [Simd Support](#Simd-Support)
 8. [component model (in progress)](#component-model)
 9. [Rustflags](#Rustflags) 
10. [Small Binary](#Small-Binary)
11. [Limitations](#Limitations)
12. [Authors tips/notes](#Authors-tips-and-notes)
13. [Credits](#Credits)
14. [Other Resources](#Other-Resources)
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
extern {
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
Exporting table Quick and dirty way.
```sh
RUSTFLAGS="-C link-arg=--export-table" cargo b
````

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
<br>

cargo require the flags to export tables 
```sh
RUSTFLAGS="-C link-arg=--export-table" 
```
optionaly you can add this to $PROJECT_FOLDER/.cargo/config.toml

```toml
# $PROJECTFOLDER/.cargo/config.toml
[target.wasm32-unknown-unknown]
rustflags = ["-C","link-arg=--export-table"]
```
## **Custom Sections**

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

# **Simd Support**
 All credit goes to [nickbabcock](https://github.com/nickbabcock)
 Examples is from https://nickb.dev/blog/authoring-a-simd-enhanced-wasm-library-with-rust/
 Quick and dirty way
 ```
 RUSTFLAGS = "-C target-feature=+simd128" cargo build --target=wasm32-wasi
 ```

 Example:
 ```rust
 use core::arch::wasm32;
 #[no_mangle]
 #[target_feature(enablex = "simd128")]
 pub fn _mm_mul_epu32(a: wasm32::v128, b: wasm32::v128) -> wasm32::v128 {
     let mask = wasm32::u32x4(0xFFFF_FFFF, 0, 0xFFFF_FFFF, 0);
     let lo_a_0 = wasm32::v128_and(a, mask);
     let lo_b_0 = wasm32::v128_and(b, mask);
     wasm32::u64x2_mul(lo_a_0, lo_b_0)
 }
 
 ```

# **Component Model**
 Currently the Component Model Proposal is **in progress** but solves these main issues/introduce concept:
  - way to define types
  - An Abi
  - Module and component linking: dynamically composing modules into components. 
 [more info](https://www.fermyon.com/blog/webassembly-component-model)

# **Rustflags**
There are specific rustflags that enable specific wasm feature.

## **Multi threading**
**NOTE:** Multi-threading require to recomile the standard libary. [more information](https://rustwasm.github.io/docs/wasm-bindgen/examples/raytrace.html)
```sh
RUSTFLAGS='-C target-feature=+atomics,+bulk-memory' cargo b --target wasm32-unknown-unknown -Z build-std=panic_abort,std
```
## **EXPORTING TABLES**

```sh
RUSTFLAGS="-C link-arg=--export-table" cargo b
```

# **Small Binary**
Quick and dirty:
have look at [Cargo.toml](https://github.com/CloneSnows/Wasm-Playground/blob/main/Cargo.toml)
and run

```sh
cargo b --target=wasm32-wasi --profile small --feature small --quiet 
ls -lh ./target/wasm32-wasi/**/*.wasm
```

1. add this to cargo.toml 
```toml
[profile.release]
opt-level = 'z' # or you can use 's' option
panic = "abort"
strip = true
lto = true
```
2. use wasm-strip to reduce binary size

<br>

## **Limitations**
- Code generation is not guarantee
- unable to use type externref :(
- issues with [custom sections](https://github.com/rust-lang/rust/issues/56639)

## **Authors tips and notes**
- When in doubt make everything public (**pub**) and no mangling (**#[no_mangle]**) your code.
- Use i32 value for less code generation
- Using this guide may or may not correctly compile to wasm.

<br/>

## Credits
simd supports: [nickbabcock](https://github.com/nickbabcock)

### **Other Resources** 
- [Rust reference](https://doc.rust-lang.org/stable/reference/introduction.html)
- [Understanding WebAssembly text format](https://developer.mozilla.org/en-US/docs/WebAssembly/Understanding_the_text_format)
- [WebAssembly Core Specification version 1](https://www.w3.org/TR/wasm-core-1/)
- [WebAssembly Core Specification version 2](https://www.w3.org/TR/wasm-core-2/)
- [Rust and WebAssembly](https://rustwasm.github.io/book/)
- [Rust + Wasm issues](https://github.com/rust-lang/rust/labels/O-wasm)
- [Authoring a SIMD enhanced Wasm library with Rust](https://nickb.dev/blog/authoring-a-simd-enhanced-wasm-library-with-rust/)

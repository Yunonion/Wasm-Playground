#![no_main]
#![cfg_attr(small, feature(no_std))]
//!
//! Compiling wasm from rust
//!
#[cfg(not(target_family = "wasm"))]
compile_error!("Not supported Architecture use cargo build --target=wasm32-wasi");

// # Exporting/importing functions
#[no_mangle]
pub fn foo() {}

// import function "import_fn" module/namespace "foo"
#[link(wasm_import_module = "namespace_foo")]
extern "C" {
    // IMPORTANT: #[no_mangle] to disable standard symbol
    #[allow(unused_attributes)]
    #[no_mangle]
    pub fn import_foo() -> i32;

}

// IMPORTANT:
// Must use **import function** otherwise compilier
// will optimise it away
#[no_mangle]
pub unsafe fn import() {
    import_foo();
}

// # Function pointer/wasm table
// NOTES: Requires "-C link-arg=--export-table" rust flag to export table
// quick and dirty way
// ```sh
// RUSTFLAGS="-C link-arg=--export-table" cargo b
// ````
#[no_mangle]
pub fn fn_bar() -> i32 {
    0
}

#[used]
#[no_mangle]
pub static FNPTRS: [fn() -> i32; 1] = [fn_bar];

// Custom sections
// Viewing custom section's value
// llvm-objdump-15 -s -j Custom_Section_foo  ./target/wasm32-unknown-unknown/debug/wasm_playground.wasm
#[used]
#[link_section = "Custom_Section_foo"]
pub static CUSTOMSECTIONFOO: [u8; 11] = *b"Hello World";

// Possible use case for custom sections:
//  - can be use "metadata programming"
//  - can be use to post process wasm file
//  - can be use metadata to create function with param externref type
//  - can be use metadata to schedule a function exectuion
//  - can be use metadata to run undercertain condition

#[cfg(all(not(test), small))]
use core::panic::PanicInfo;

#[cfg(all(not(test), small))]
#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

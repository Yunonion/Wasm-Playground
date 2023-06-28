#![no_main]
//!
//! Compiling wasm from rust
//!
#[cfg(not(target_family = "wasm"))]
compile_error!("Not supported Architecture use cargo build --target=wasm32-wasi");

// Exporting functions from other modules or crates
// NOTE: This will Export everything from the crate or module
// Workaround: use wrapper function and refrain from using
// certain builtin macros i.e. #[no_mangle]
mod importwasm;

#[no_mangle]
pub fn foo() {}

// import function "import_fn" module/namespace "foo"
#[link(wasm_import_module = "namespace_foo")]
extern {
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
// llvm-objdump-15 -s -j .custom_section_foo  ./target/wasm32-unknown-unknown/debug/wasm_playground.wasm
#[used]
#[link_section = ".custom_section_foo"]
pub static CUSTOMSECTIONFOO: [u8; 11] = *b"Hello World";

// Possible use case for custom sections:
//  - can be use "metadata programming"
//  - can be use to post process wasm file
//  - can be use metadata to create function with param externref type
//  - can be use metadata to schedule a function exectuion
//  - can be use metadata to run undercertain condition

#[derive(Default)]
#[repr(C)]
pub struct Args {
    x: i32,
    y: i32,
    // z: 132,
    a: [i32; 10],
}

#[no_mangle]
pub fn args(args: &mut Args, v: i32) {
    args.x = v;
    args.y = v;
    // args.z = v;
    for i in &mut args.a {
        *i = v;
    }
}

// SIMD support
// All credit goes to [nickbabcock](https://github.com/nickbabcock)
// Examples copied from https://nickb.dev/blog/authoring-a-simd-enhanced-wasm-library-with-rust/
// Quick and dirty way
// ```
// RUSTFLAGS = "-C target-feature=+simd128" cargo build --target=wasm32-wasi
// ```
#[cfg(target_feature = "simd128")]
pub mod simdsupport {
    use core::arch::wasm32;
    #[no_mangle]
    pub fn _mm_mul_epu32(a: wasm32::v128, b: wasm32::v128) -> wasm32::v128 {
        let mask = wasm32::u32x4(0xFFFF_FFFF, 0, 0xFFFF_FFFF, 0);
        let lo_a_0 = wasm32::v128_and(a, mask);
        let lo_b_0 = wasm32::v128_and(b, mask);
        wasm32::u64x2_mul(lo_a_0, lo_b_0)
    }

    /// Shift left 8 bytes (aka _mm_bslli_si128 8)
    #[no_mangle]
    pub fn _mm_slli_si128_8(a: wasm32::v128) -> wasm32::v128 {
        let zero = wasm32::u64x2(0, 0);
        wasm32::u64x2_shuffle::<1, 2>(a, zero)
    }
    /// Rotate 4 bytes to the left
    #[no_mangle]
    fn rotate_4(a: wasm32::v128) -> wasm32::v128 {
        let ignored = wasm32::u64x2(0, 0);
        wasm32::u32x4_shuffle::<1, 0, 3, 2>(a, ignored)
    }
}

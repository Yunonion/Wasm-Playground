//! 
//! Compiling wasm from rust 
//! 
#[cfg(not(target_family="wasm"))]
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
	fn import_foo() -> i32;

}

// IMPORTANT: 
// Must use **import function** otherwise compilier 
// will optimise it away
#[no_mangle]
pub unsafe fn import(){
	drop(import_foo())
} 

//


// # Function pointer/wasm table
// IMPORTANT: this will not export functions pointers
// UNLESS: built with RUSTFLAGS="-C link-arg=--export-table"
// EXAMPLE:
//  in terminal use
// ```
// RUSTFLAGS="-C link-arg=--export-table" cargo b --target=wasm32-unknown-unknown
// ```
const FNPTRS: [unsafe extern "C" fn() ->i32; 2] = [import_foo, fn_bar];

#[no_mangle]
pub unsafe extern "C" fn fn_bar() -> i32 {0} //

#[no_mangle]
pub unsafe fn fnptrs(cond: i32) -> i32{
	FNPTRS[cond as usize ]()
}

//

// Custom sections**with Exporting tables enable**
#[used]
#[link_section = "Custom_Section_foo"]
pub static CUSTOMSECTIONFOO: () = ();

// Possible use case for custom sections: 
//  - can be use "metadata programming"
//  - can be use to post process wasm file
//  - can be use metadata to create function with param externref type
//  - can be use metadata to schedule a function exectuion
//  - can be use metadata to run undercertain condition

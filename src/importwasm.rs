#[used]
#[link_section = ".import_custom_section"]
pub static IMPORTCUSTOMSECTIONFOO: [u8; 11] = *b"Hello World";

#[no_mangle]
pub fn import_wasm() -> i32 {
    return 0;
}

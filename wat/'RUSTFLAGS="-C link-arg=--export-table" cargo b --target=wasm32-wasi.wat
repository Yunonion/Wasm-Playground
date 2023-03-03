(module
  (type (;0;) (func (result i32)))
  (type (;1;) (func))
  (import "namespace_foo" "import_foo" (func $import_foo (type 0)))
  (func $fn_bar (type 0) (result i32)
    i32.const 0)
  (func $foo (type 1))
  (func $import (type 1)
    call $import_foo
    drop)
  (func $dummy (type 1))
  (func $__wasm_call_dtors (type 1)
    call $dummy
    call $dummy)
  (func $fn_bar.command_export (type 0) (result i32)
    call $fn_bar
    call $__wasm_call_dtors)
  (func $foo.command_export (type 1)
    call $foo
    call $__wasm_call_dtors)
  (func $import.command_export (type 1)
    call $import
    call $__wasm_call_dtors)
  (table (;0;) 2 2 funcref)
  (memory (;0;) 17)
  (global $__stack_pointer (mut i32) (i32.const 1048576))
  (global (;1;) i32 (i32.const 1048576))
  (export "memory" (memory 0))
  (export "FNPTRS" (global 1))
  (export "fn_bar" (func $fn_bar.command_export))
  (export "foo" (func $foo.command_export))
  (export "import" (func $import.command_export))
  (elem (;0;) (i32.const 1) func $fn_bar)
  (data $.rodata (i32.const 1048576) "\01\00\00\00"))

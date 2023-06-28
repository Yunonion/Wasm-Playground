(module
  (type (;0;) (func (result i32)))
  (type (;1;) (func))
  (type (;2;) (func (param i32 i32)))
  (import "namespace_foo" "import_foo" (func $import_foo (type 0)))
  (func $import_wasm (type 0) (result i32)
    i32.const 0)
  (func $foo (type 1))
  (func $import (type 1)
    call $import_foo
    drop)
  (func $fn_bar (type 0) (result i32)
    i32.const 0)
  (func $args (type 2) (param i32 i32)
    local.get 0
    local.get 1
    i32.store offset=8
    local.get 0
    local.get 1
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store
    local.get 0
    i32.const 44
    i32.add
    local.get 1
    i32.store
    local.get 0
    i32.const 40
    i32.add
    local.get 1
    i32.store
    local.get 0
    i32.const 36
    i32.add
    local.get 1
    i32.store
    local.get 0
    i32.const 32
    i32.add
    local.get 1
    i32.store
    local.get 0
    i32.const 28
    i32.add
    local.get 1
    i32.store
    local.get 0
    i32.const 24
    i32.add
    local.get 1
    i32.store
    local.get 0
    i32.const 20
    i32.add
    local.get 1
    i32.store
    local.get 0
    i32.const 16
    i32.add
    local.get 1
    i32.store
    local.get 0
    i32.const 12
    i32.add
    local.get 1
    i32.store)
  (table (;0;) 2 2 funcref)
  (memory (;0;) 17)
  (global $__stack_pointer (mut i32) (i32.const 1048576))
  (global (;1;) i32 (i32.const 1048576))
  (global (;2;) i32 (i32.const 1048580))
  (global (;3;) i32 (i32.const 1048592))
  (export "memory" (memory 0))
  (export "import_wasm" (func $import_wasm))
  (export "foo" (func $foo))
  (export "import" (func $import))
  (export "fn_bar" (func $fn_bar))
  (export "args" (func $args))
  (export "FNPTRS" (global 1))
  (export "__indirect_function_table" (table 0))
  (export "__data_end" (global 2))
  (export "__heap_base" (global 3))
  (elem (;0;) (i32.const 1) func $fn_bar)
  (data $.rodata (i32.const 1048576) "\01\00\00\00"))

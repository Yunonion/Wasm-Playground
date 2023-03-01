(module
  (type (;0;) (func (result i32)))
  (type (;1;) (func))
  (import "namespace_foo" "import_foo" (func (;0;) (type 0)))
  (func (;1;) (type 1))
  (func (;2;) (type 1)
    call 0
    drop)
  (func (;3;) (type 0) (result i32)
    i32.const 0)
  (func (;4;) (type 1))
  (func (;5;) (type 1)
    call 4
    call 4)
  (func (;6;) (type 1)
    call 1
    call 5)
  (func (;7;) (type 1)
    call 2
    call 5)
  (func (;8;) (type 0) (result i32)
    call 3
    call 5)
  (table (;0;) 2 2 funcref)
  (memory (;0;) 17)
  (global (;0;) (mut i32) (i32.const 1048576))
  (global (;1;) i32 (i32.const 1048576))
  (export "memory" (memory 0))
  (export "FNPTRS" (global 1))
  (export "foo" (func 6))
  (export "import" (func 7))
  (export "fn_bar" (func 8))
  (elem (;0;) (i32.const 1) func 3)
  (data (;0;) (i32.const 1048576) "\01\00\00\00"))

(module
  (type (;0;) (func (result i32)))
  (type (;1;) (func))
  (type (;2;) (func (param i32 i32)))
  (import "namespace_foo" "import_foo" (func (;0;) (type 0)))
  (func (;1;) (type 1))
  (func (;2;) (type 1)
    call 0
    drop)
  (func (;3;) (type 0) (result i32)
    i32.const 0)
  (func (;4;) (type 2) (param i32 i32)
    (local i32)
    local.get 0
    local.get 1
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store
    i32.const 8
    local.set 2
    loop  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.const 48
        i32.ne
        br_if 0 (;@2;)
        return
      end
      local.get 0
      local.get 2
      i32.add
      local.get 1
      i32.store
      local.get 2
      i32.const 4
      i32.add
      local.set 2
      br 0 (;@1;)
    end)
  (table (;0;) 2 2 funcref)
  (memory (;0;) 17)
  (global (;0;) (mut i32) (i32.const 1048576))
  (global (;1;) i32 (i32.const 1048576))
  (global (;2;) i32 (i32.const 1048580))
  (global (;3;) i32 (i32.const 1048592))
  (export "memory" (memory 0))
  (export "foo" (func 1))
  (export "import" (func 2))
  (export "fn_bar" (func 3))
  (export "args" (func 4))
  (export "FNPTRS" (global 1))
  (export "import_wasm" (func 3))
  (export "__data_end" (global 2))
  (export "__heap_base" (global 3))
  (elem (;0;) (i32.const 1) func 3)
  (data (;0;) (i32.const 1048576) "\01\00\00\00"))

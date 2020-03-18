#![feature(todo_macro)]

use wasmer_runtime::{error, imports,func, Ctx, Array, instantiate, Func, WasmPtr};

fn main() -> error::Result<()> {
    // Let's get the .wasm file as bytes
    let wasm_bytes = include_bytes!("../asm/build/optimized.wasm");

    // Our import object, that allows exposing functions to our Wasm module.
    // We're not importing anything, so make an empty import object.
    let import_object = imports! {
        "env" => {
            "abort" => func!(abort),
        },
    };

    // Let's create an instance of Wasm module running in the wasmer-runtime
    let instance = instantiate(wasm_bytes, &import_object)?;

    // Let's get `add_one` as a function which takes one `u32` and returns one `u32`
    let add: Func<(i32, i32), i32> = instance.func("add")?;
    let result = add.call(1, 1)?;

    // Log the new value
    println!("Result: {}", result);

    // Asserting that the returned value from the function is our expected value.
    assert_eq!(result, 2);  // 1 + 1

    // Return OK since everything executed successfully!
    Ok(())
}

fn abort(ctx: &mut Ctx, message_ptr: WasmPtr<i32, Array>, filename_ptr: i32, lineNumber: i32, columnNumber: i32) {
    println!("message_ptr {:?} filename_ptr{:?} {:?} {:?}", message_ptr, filename_ptr, lineNumber, columnNumber);

    let memory = ctx.memory(0);

    let offset = message_ptr.offset();
    let size = memory.view::<u32>(offset / 4 -1);
    message_ptr.deref(memory, offset, size);
}

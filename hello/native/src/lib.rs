use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("Hello from Rust/Neon"))
}

// Call this function to put a cause a panic with a non-trivial backtrace.
fn crash(mut cx: FunctionContext) -> JsResult<JsString> {
    foo(42);
    Ok(cx.string("You won't get this"))
}

fn foo(x: usize) {
    bar(x);
}

fn bar(x: usize) {
    eprintln!("in function bar()");
    assert_eq!(x, 0);
}

register_module!(mut cx, {
    cx.export_function("hello", hello)?;
    cx.export_function("crash", crash)?;
    Ok(())
});

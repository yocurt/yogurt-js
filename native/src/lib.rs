#[macro_use]
extern crate neon;
extern crate libcurt;

use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

register_module!(mut cx, { cx.export_function("hello", hello) });

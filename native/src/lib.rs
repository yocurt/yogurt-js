#[macro_use]
extern crate neon;
extern crate libcurt;

use libcurt::*;
use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

fn get_example_result(mut cx: FunctionContext) -> JsResult<JsObject> {
    let result = libcurt::Result::new("test".to_string(), 2, 6);
    let js_result = JsObject::new(&mut cx);
    let js_text = cx.string(&result.get_text());
    let js_start = cx.number(result.get_start() as f64);
    let js_end = cx.number(result.get_end() as f64);
    js_result.set(&mut cx, "text", js_text).unwrap();
    js_result.set(&mut cx, "start", js_start).unwrap();
    js_result.set(&mut cx, "end", js_end).unwrap();
    Ok(js_result)
}

register_module!(mut m, {
    m.export_function("get_example_result", get_example_result)?;
    m.export_function("hello", hello)?;
    Ok(())
});

use neon::prelude::*;
use neon::register_module;
mod lotus;
fn start_as_api_fn(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    lotus::start_as_api();
    Ok(cx.undefined())
}
fn main_headless_fn(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    lotus::main_headless();
    Ok(cx.undefined())
}
register_module!(mut cx, {
    cx.export_function("start_as_api_fn", start_as_api_fn);
});

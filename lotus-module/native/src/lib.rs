#[macro_use(lazy_static)]
extern crate lazy_static;
use neon::prelude::*;
use neon::register_module;
use std::thread;

mod lotus;
fn main_headless_fn(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    lotus::main_headless();
    Ok(cx.undefined())
}
fn load_vst_fn(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    println!("coming soon yo");
    //lotus::load_vst();
    Ok(cx.undefined())
}
fn load_wav_fn(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let arg = cx.argument::<JsString>(0)?.value();
    lotus::load_wav(arg);
    Ok(cx.undefined())
}
fn set_playback_fn(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let arg = cx.argument::<JsNumber>(0)?.value();
    lotus::set_playback(arg as i64);
    Ok(cx.undefined())
}
fn shout_fn(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    println!("coming soon yo, this one is gonna be used for socket comms");
    //lotus::shout();
    Ok(cx.undefined())
}

fn pause_button_fn(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    lotus::pause_button();
    Ok(cx.undefined())
}

fn main_api_fn(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    thread::spawn(move || {
        lotus::main_api();
    });
    Ok(cx.undefined())
}

register_module!(mut cx, {
    cx.export_function("main_headless_fn", main_headless_fn)?;
    cx.export_function("load_vst_fn", load_vst_fn)?;
    cx.export_function("load_wav_fn", load_wav_fn)?;
    cx.export_function("set_playback_fn", set_playback_fn)?;
    cx.export_function("shout_fn", shout_fn)?;
    cx.export_function("pause_button_fn", pause_button_fn)?;
    cx.export_function("main_api_fn", main_api_fn)?;
    Ok(())
});

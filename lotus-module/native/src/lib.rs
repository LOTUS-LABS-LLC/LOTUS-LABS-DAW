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

fn add_clip_fn(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let arg1 = cx.argument::<JsNumber>(0)?.value();
    let arg2 = cx.argument::<JsNumber>(1)?.value();
    lotus::add_clip(arg1 as u16, arg2 as i64);
    Ok(cx.undefined())
}

fn get_playback_fn(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let tmp: i64 = lotus::get_playback();
    Ok(cx.number(tmp as f64))
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

fn set_mixer_asst_fn(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let arg1 = cx.argument::<JsNumber>(0)?.value();
    let arg2 = cx.argument::<JsNumber>(1)?.value();
    lotus::set_mixer_asst(arg1 as usize, arg2 as u16);
    Ok(cx.undefined())
}
fn play_tmp_fn(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let arg1 = cx.argument::<JsString>(0)?.value();
    lotus::play_tmp(arg1);
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
    cx.export_function("get_playback_fn", get_playback_fn)?;
    cx.export_function("set_mixer_asst_fn", set_mixer_asst_fn)?;
    cx.export_function("play_tmp_fn", play_tmp_fn)?;
    cx.export_function("add_clip_fn", add_clip_fn)?;
    Ok(())
});

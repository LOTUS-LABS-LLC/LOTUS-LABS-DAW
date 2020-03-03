console.log("loading engine...");
var mod = require('../native');
console.log("Lotus Engine API loaded...");
class OutFn {
constructor(){
}
main_headless = mod.main_headless_fn;
load_vst = mod.load_vst_fn;
load_wav = mod.load_wav_fn;
set_playback = mod.set_playback_fn;
get_playback = mod.get_playback_fn;
shout = mod.shout_fn;
read = mod.read_fn;
main_api = mod.main_api_fn;
pause_button = mod.pause_button_fn;
set_mixer_asst = mod.set_mixer_asst_fn;
play_tmp = mod.play_tmp_fn;
add_clip = mod.add_clip_fn;
}
module.exports = new OutFn();
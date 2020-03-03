pub use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
pub use hound;
pub use std::fs::File;
pub use std::io::BufReader;
pub use std::io::Cursor;
pub use std::io::Read;
pub use std::path::Path;
pub use std::sync::{Arc, Mutex, RwLock};
pub use std::thread;
pub use text_io::read;
pub use uds_windows::{UnixListener, UnixStream};
pub use vst::host::{Host, PluginLoader};
pub use vst::plugin::Plugin;

struct SampleHost;

static mut API_TMP: i64 = 0;
static mut API_FLAG: bool = false;

static mut MPB_TMP: i64 = 0;
static mut MPB_FLAG: bool = false;

static mut PAUSE_IO: bool = true;
static mut PLAY_TMP: bool = false;
static mut LOADING_PLSWAIT: bool = true;

struct Session {
    wavstor: Vec<Vec<i16>>,
    mixer_asst: Vec<u16>,
    wavsize: Vec<usize>,
    clipdata: Vec<Clip>,
    mixer_incom: Vec<u16>,
    mixer_active: Vec<u16>,
}

impl Session {
    fn playlist_mixer_get(&self, pb_idx: i64, mixer_n: usize, _sesh: &Session) -> i16 {
        let mut result: i16 = 0;
        for clip in &self.clipdata {
            //println!("CLIP: {:?} ", clip);
            let mut c = clip.clone();
            //if the mixer_n is equal to the mixer assignment of this clip's source index...
            if mixer_n == self.mixer_asst[c.smpidx] as usize {
                //todo add mixer effects run-through
                // divide by mixer incoming track and add to sample output
                result += c.get_pl(pb_idx, _sesh) / self.mixer_incom[mixer_n] as i16;
                //println!("{:?}", result);
            }
        }
        result
    }
}

lazy_static! {
    static ref TMP_PATH: Mutex<Stringctrl> = Mutex::new(Stringctrl {
        val: "".to_string()
    });
    static ref _MASTER_PLAYBACK: Mutex<Numctrl> = Mutex::new(Numctrl {
        val: 0i64
    });
    static ref sesh: Mutex<Session> = Mutex::new(Session {
        wavstor: [].to_vec(),                      //wav file storage for source index (all source indicies reference this vector's contents)
        mixer_asst: [].to_vec(),                   //mixer assignment by source index
        wavsize: [].to_vec(),                      //file size by source index
        clipdata: [].to_vec(),                     //playlist clips
        mixer_incom: Vec::with_capacity(256),      //number of incoming connections for each available mixer
        mixer_active: Vec::with_capacity(0)        //list of active mixer indicies
    });
}

impl Host for SampleHost {
    fn automate(&self, index: i32, value: f32) {
        println!("Parameter {} had its value changed to {}", index, value);
    }
}

#[derive(Debug, Copy, Clone)]
struct Clip {
    smpidx: usize,
    smp_startpt: i64,
    smp_endpt: i64,
    pl_startpt: i64,
}

impl Clip {
    fn set_smpstart(&mut self, new: i64) {
        self.smp_startpt = new;
    }
    fn set_smpend(&mut self, new: i64) {
        self.smp_endpt = new;
    }
    fn set_source(&mut self, new: usize) {
        self.smpidx = new;
    }
    fn set_plstart(&mut self, new: i64) {
        self.pl_startpt = new;
    }
    fn get_pl(&mut self, playback: i64, _sesh: &Session) -> i16 {
        //get sample from index in playlist space
        let tmp = get_vv(playback - self.pl_startpt, &_sesh.wavstor[self.smpidx]);
        tmp
    }
}

struct Stringctrl {
    val: String,
}

impl Stringctrl {
    fn change(&mut self, new: String) {
        self.val = new;
    }
}

struct Numctrl {
    val: i64,
}

impl Numctrl {
    fn change(&mut self, new: i64) {
        self.val = new;
    }
    fn incr(&mut self) {
        self.val += 1;
    }
    fn get(&mut self) -> i64 {
        self.val
    }
}

fn get_vv(idx: i64, vec: &Vec<i16>) -> i16 {
    //this is a wrapper for samples vectors, so that if the index is out of range it simply returns 0
    //with this playback can happen from any index on the sample even if a sample doesnt exist there
    //println!("{:?}", vec.len());
    if idx >= (vec.len() as i64) || idx < (0i64) {
        0 as i16
    } else {
        vec[idx as usize]
    }
}

pub fn play_tmp(path: String) {
    //plays a wav file from temporary memory, for things like playing a sample from the file browser
    unsafe {
        PLAY_TMP = false;
        TMP_PATH.lock().unwrap().change(path);
    }
}

pub fn pause_button() {
    //toggles master pause and play
    unsafe {
        PAUSE_IO = !PAUSE_IO;
        if PAUSE_IO {
            println!("PAUSED");
        } else {
            println!("PLAYING");
        }
    }
}

pub fn add_clip(srcidx: u16, plstart: i64) {
    let mut _sesh = &mut sesh.lock().unwrap();
    let leng = _sesh.wavstor[srcidx as usize].len() as i64;
    _sesh.clipdata.push(Clip {
        smpidx: srcidx as usize,
        pl_startpt: plstart,
        smp_startpt: 0,
        smp_endpt: (leng),
    });
    println!("clip added!");
}

pub fn toggle_mixer(mixer_n: u16) {
    //toggles mixer mute
    let mut _actives = &mut sesh.lock().unwrap().mixer_active;
    let index: i16 = match _actives.iter().position(|mn| *mn == mixer_n) {
        Some(s) => s as i16,
        None => -1i16,
    };
    if index == -1 {
        _actives.push(mixer_n);
    } else {
        _actives.remove(index as usize);
    }
}

pub fn main_headless() -> Result<(), anyhow::Error> {
    println!("[LOTUS ENGINE]");

    let host = cpal::default_host();
    println!("host configured...");
    let device = host
        .default_output_device()
        .expect("failed to find a default output device");
    println!("default host selected...");
    let config = device.default_output_config()?;
    println!("config set...");

    let mut burden: Vec<String> = [].to_vec();
    println!("enter file:");
    let user_inp: String = read!();

    burden.push(user_inp.to_string());

    // while user_inp != "" {
    //     println!("enter file (or leave blank to continue):");
    //     let user_inp: String = read!();
    //     burden.push(user_inp.to_string());
    // }

    println!("initializing master track...");
    sesh.lock().unwrap().mixer_active.push(0);
    sesh.lock().unwrap().mixer_incom.push(0);

    for n in burden {
        if n != "" {
            load_wav(n);
        }
    }

    add_clip(0, 0);

    pause_button();

    match config.sample_format() {
        cpal::SampleFormat::F32 => run::<f32>(&device, &config.into(), "f32".to_string())?,
        cpal::SampleFormat::I16 => run::<i16>(&device, &config.into(), "i16".to_string())?,
        cpal::SampleFormat::U16 => run::<u16>(&device, &config.into(), "u16".to_string())?,
    }

    Ok(())
}

pub fn main_api() -> Result<(), anyhow::Error> {
    println!("[LOTUS ENGINE]");

    let host = cpal::default_host();
    println!("host configured...");
    let device = host
        .default_output_device()
        .expect("failed to find a default output device");
    println!("default host selected...");
    let config = device.default_output_config()?;
    println!("default config set...");
    println!("initializing master track...");
    sesh.lock().unwrap().mixer_active.push(0);
    sesh.lock().unwrap().mixer_incom.push(0);
    println!("starting audio stream...");
    match config.sample_format() {
        cpal::SampleFormat::F32 => run::<f32>(&device, &config.into(), "f32".to_string())?,
        cpal::SampleFormat::I16 => run::<i16>(&device, &config.into(), "i16".to_string())?,
        cpal::SampleFormat::U16 => run::<u16>(&device, &config.into(), "u16".to_string())?,
    }

    Ok(())
}

pub fn load_vst(_path: String) {
    let host = Arc::new(Mutex::new(SampleHost));
    let path = Path::new(&_path);

    let mut loader = PluginLoader::load(path, host.clone()).unwrap();
    let mut instance = loader.instance().unwrap();
}

fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> T)
where
    T: cpal::Sample,
{
    //println!("writing...");
    for frame in output.chunks_mut(channels) {
        let value: T = cpal::Sample::from::<T>(&next_sample());
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}

pub fn set_mixer_asst(idx: usize, insertn: u16) {
    let mut tmp = sesh.lock().unwrap();
    //subtract the current mixer's input count
    tmp.mixer_incom[tmp.mixer_asst[idx] as usize];
    tmp.mixer_asst[idx] = insertn;
    //add to the new mixer's input count
    tmp.mixer_incom[insertn as usize];
}

pub fn load_wav(path: String) {
    unsafe {
        LOADING_PLSWAIT = true;
    }
    println!("loading wav: {}", path);
    let mut reader = hound::WavReader::open(path).unwrap();
    let samps = reader.samples::<i16>();
    let size = samps.len();
    println!("wavsize: {}", size);
    let mut tmp: Vec<i16> = Vec::with_capacity(size);
    let mut _sesh = sesh.lock().unwrap();

    //let mut index = 0;

    samps.for_each(|s| {
        let val = s.unwrap();
        //println!("{}", val);
        tmp.push(val);
    });
    _sesh.wavstor.push(tmp);

    println!("Setting to master track...");
    _sesh.mixer_asst.push(0);
    //push the size as well
    _sesh.wavsize.push(size);
    _sesh.mixer_incom[0] += 1;
    println!(
        "Master track now has {} incoming connections",
        _sesh.mixer_incom[0]
    );

    unsafe {
        LOADING_PLSWAIT = false;
    }
    println!("LOADED");
}

pub fn set_playback(index: i64) {
    let master_playback = &mut _MASTER_PLAYBACK.lock().unwrap();
    master_playback.change(index);
}

pub fn get_playback() -> i64 {
    let master_playback = &mut _MASTER_PLAYBACK.lock().unwrap();
    master_playback.get()
}

//todo get wavform image

// variants

fn run<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    format_type: String,
) -> std::result::Result<(), anyhow::Error>
where
    T: cpal::Sample,
{
    // accept connections and process them, spawning a new thread for each one
    println!("starting {} format stream...", format_type);

    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;
    let mut MASTER_PAUSE = false;
    //let mut MASTER_PLAYBACK = 0;
    println!("CONFIG: {:?}", config);

    let mut next_value = move || {
        unsafe {
            let _sesh = &sesh.lock().unwrap();
            let mut MASTER_PLAYBACK = &mut _MASTER_PLAYBACK.lock().unwrap();
            if !PAUSE_IO {
                MASTER_PLAYBACK.incr();
            }

            if API_FLAG {
                MASTER_PLAYBACK.change(API_TMP);
                API_FLAG = false;
            }

            //mix and output
            let mut s: i16 = 0;
            if !LOADING_PLSWAIT {
                //s = _sesh.wavstor[0][MASTER_PLAYBACK as usize];
                for mn in &_sesh.mixer_active {
                    s += _sesh.playlist_mixer_get(MASTER_PLAYBACK.get(), *mn as usize, _sesh)
                        / _sesh.mixer_active.len() as i16;
                }
                //println!("{:?}", s);
            }

            if format_type == "f32" {
                T::from(&((s as f32) / (32767f32)))
            } else {
                T::from(&s)
            }
        }
    };

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);
    println!("building stream...");
    let stream = device.build_output_stream(
        config,
        move |data: &mut [T]| write_data(data, channels, &mut next_value),
        err_fn,
    )?;
    println!("playing stream...");
    stream.play()?;
    //std::thread::sleep(std::time::Duration::from_millis(1000));
    println!("done i gueees");
    let dum: String = read!();

    Ok(())
}

//networking stuff

fn handle(stream: UnixStream) {}

pub fn shout(out: String) {
    let st = match UnixStream::connect("/tmp/siberianbreaks") {
        Ok(s) => s,
        Err(e) => {
            println!("[GlobalMGMT] socket likely destroyed{:?}", e);
            //TODO attempt to recreate and fix socket
            return;
        }
    };
}

pub fn read(index: u16) -> [u8; 4] {
    let mut st = UnixStream::connect("/tmp/siberianbreaks").unwrap();
    let mut res: [u8; 4] = [0, 0, 0, 0];
    st.read(&mut res);

    res
}

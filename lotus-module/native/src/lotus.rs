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
static mut PAUSE_IO: bool = true;
lazy_static! {
    static ref sesh: Mutex<Session> = Mutex::new(Session {
        wavstor: [].to_vec(),
    });
}

impl Host for SampleHost {
    fn automate(&self, index: i32, value: f32) {
        println!("Parameter {} had its value changed to {}", index, value);
    }
}

struct Session {
    wavstor: Vec<Vec<i16>>,
    // wavstor: Vec<Vec<f32>>,
}

pub fn pause_button() {
    unsafe {
        PAUSE_IO = !PAUSE_IO;
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

    while user_inp != "" {
        println!("enter file (or leave blank to continue):");
        let user_inp: String = read!();
        burden.push(user_inp.to_string());
    }

    println!("loading specified wav files...");

    for n in burden {
        if n != "" {
            load_wav(n);
        }
    }

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
    println!("config set... starting audio stream...");
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
    for frame in output.chunks_mut(channels) {
        let value: T = cpal::Sample::from::<T>(&next_sample());
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}

pub fn load_wav(path: String) {
    println!("loading wav: {}", path);
    let mut reader = hound::WavReader::open(path).unwrap();
    let samps = reader.samples::<i16>();
    let size = samps.len();
    println!("wavsize: {}", size);
    let mut tmp: Vec<i16> = Vec::with_capacity(size);

    //let mut index = 0;

    samps.for_each(|s| {
        let val = s.unwrap();
        //println!("{}", val);
        tmp.push(val);
    });
    sesh.lock().unwrap().wavstor.push(tmp);
    println!("LOADED");
}

pub fn set_playback(index: i64) {
    unsafe {
        API_TMP = index;
        API_FLAG = true;
    }
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
    println!("creating new session ({})...", format_type);

    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;
    let mut MASTER_PAUSE = false;
    let mut MASTER_PLAYBACK = 0;
    println!("CONFIG: {:?}", config);

    let mut next_value = move || {
        unsafe {
            if !PAUSE_IO {
                MASTER_PLAYBACK += 1;
            }

            if API_FLAG {
                MASTER_PLAYBACK = API_TMP;
                API_FLAG = false;
            }
        }

        //mix and output

        let s = sesh.lock().unwrap().wavstor[0][MASTER_PLAYBACK as usize];
        if format_type == "f32" {
            T::from(&((s as f32) / (32767f32)))
        } else {
            T::from(&s)
        }
    };

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T]| write_data(data, channels, &mut next_value),
        err_fn,
    )?;
    stream.play()?;

    //std::thread::sleep(std::time::Duration::from_millis(1000));
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

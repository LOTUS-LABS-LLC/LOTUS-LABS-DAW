extern crate anyhow;
extern crate cpal;

pub use hound;
pub use std::fs::File;
pub use std::io::BufReader;
pub use text_io::read;

pub use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};
pub use cpal::{StreamData, UnknownTypeOutputBuffer};

// todo:
// frame mixing formula: (decodeddata1 * 0.5 + decodeddata2* 0.5) ---> i16
// create a reinitialization for changing sample rate and such settings

struct Session {
    audios: Vec<hound::WavReader<BufReader<File>>>,
}

pub fn start_as_api() {
    println!("API is on the way!");
}

pub fn main_headless() {
    println!("[Lotus DAW Engine by sweatercuff]");
    //init stuff

    print!("Enter audio file: ");

    let _initdir: String = read!();

    println!("Createing new session...");

    let mut _activesesh = Session {
        audios: Vec::with_capacity(10),
    };

    add_audiof("bee.wav", &mut _activesesh);

    //CPAL INITIALIZATION
    println!("Starting CPAL...");

    //Select default audio host (asio, wasapi, coreaudio, etc...)
    let host = cpal::default_host();

    //Create an event loop
    let event_loop = host.event_loop();

    //Select the actual output device using the provided host
    let device = host
        .default_output_device()
        .expect("[CPAL] no output device available");

    //Process possible formats (which im guessing is sample rate and some other stuff???)
    let mut supported_formats_range = device
        .supported_output_formats()
        .expect("[CPAL] error while querying formats");

    //Select the first output format available
    let format = supported_formats_range
        .next()
        .expect("[CPAL] no supported format?!")
        .with_max_sample_rate();

    //Create an output stream using the event loop and get the ID for that stream process
    let stream_id = event_loop.build_output_stream(&device, &format).unwrap();
    event_loop
        .play_stream(stream_id)
        .expect("[CPAL] failed to play_stream");

    event_loop.run(move |stream_id, stream_result| {
        let stream_data = match stream_result {
            Ok(data) => data,
            Err(err) => {
                eprintln!("an error occurred on stream {:?}: {}", stream_id, err);
                return;
            }
            _ => return,
        };

        match stream_data {
            StreamData::Output {
                buffer: UnknownTypeOutputBuffer::U16(mut buffer),
            } => {
                for elem in buffer.iter_mut() {
                    *elem = u16::max_value() / 2;
                }
            }
            StreamData::Output {
                buffer: UnknownTypeOutputBuffer::I16(mut buffer),
            } => {
                for elem in buffer.iter_mut() {
                    *elem = 0;
                }
            }
            StreamData::Output {
                buffer: UnknownTypeOutputBuffer::F32(mut buffer),
            } => {
                for elem in buffer.iter_mut() {
                    *elem = 0.0;
                }
            }
            _ => (),
        }
    });
}

fn add_audiof(_dir: &str, sesh: &mut Session) {
    println!("[ADD_AUDIOF] {}", _dir);
    sesh.audios.push(hound::WavReader::open(_dir).unwrap());
}

// fn mixframes(v_frames: Vec<i16>) {
//     let mut mix: i16 = 0;
//     if v_frames.len() >= 1 {
//         for f in v_frames {
//             mix += f * 0.5 as i16;
//         }
//     }
// }

// fn write_audiof(_dir: &str, chans: u16, smp_rte: u32, bps: u16) {
//     let mut _tmpspc = hound::WavSpec {
//         channels: chans,
//         sample_rate: smp_rte,
//         bits_per_sample: bps,
//         sample_format: hound::SampleFormat::Int,
//     };
// }

use std::sync::Arc;

use ambient_ecs::{EntityId, SystemGroup, World};
use ambient_wasm::shared::{get_module_name, MessageType};

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    FromSample, SizedSample,
};

use std::{thread}; // use std::time::{Instant};
use std::sync::atomic::{AtomicBool, Ordering}; // AtomicUsize AtomicPtr
use glicol::Engine;
use std::sync::Mutex;

const BLOCK_SIZE: usize = 128;

pub fn systems() -> SystemGroup {
    ambient_wasm::client::systems()
}

pub fn initialize(world: &mut World) -> anyhow::Result<()> {

    let code = Arc::new(Mutex::new(String::from("")));
    let code_clone = Arc::clone(&code);
    world.set_code(code);

    let messenger = Arc::new(|world: &World, id: EntityId, type_: MessageType, message: &str| {
        let name = get_module_name(world, id);
        let (prefix, level) = match type_ {
            MessageType::Info => ("info", log::Level::Info),
            MessageType::Warn => ("warn", log::Level::Warn),
            MessageType::Error => ("error", log::Level::Error),
            MessageType::Stdout => ("stdout", log::Level::Info),
            MessageType::Stderr => ("stderr", log::Level::Info),
        };

        log::log!(level, "[{name}] {prefix}: {}", message.strip_suffix('\n').unwrap_or(message));
    });

    // url http://10.0.0.2:8999/content/assets/ping.ogg"
    // println!("url {}", &url);
    // let track = Track::from_vorbis(
    //     std::fs::read(&url)
    //         .unwrap()
    //         .to_vec(),
    // ).unwrap();

    // let stream = AudioStream::new().unwrap();
    // let source = track.decode();
    // eprintln!("Duration: {:?}", source.duration());
    // let sound = stream.mixer().play(source);
    // let now = Instant::now();
    // sound.wait_blocking();
    // eprintln!("Elapsed: {:?}", now.elapsed());

    // let host = cpal::default_host();
    // let device = host.default_output_device().expect("failed to find output device");
    // let config = device.default_output_config().unwrap();
    // println!("Default output config: {:?}", config);
    // let _audio_thread = thread::spawn(move || {
    //     let options = (code_clone, ());
    //     match config.sample_format() {
    //         cpal::SampleFormat::I8 => run_audio::<i8>(&device, &config.into(), options),
    //         cpal::SampleFormat::I16 => run_audio::<i16>(&device, &config.into(), options),
    //         cpal::SampleFormat::I32 => run_audio::<i32>(&device, &config.into(), options),
    //         cpal::SampleFormat::I64 => run_audio::<i64>(&device, &config.into(), options),
    //         cpal::SampleFormat::U8 => run_audio::<u8>(&device, &config.into(), options),
    //         cpal::SampleFormat::U16 => run_audio::<u16>(&device, &config.into(), options),
    //         cpal::SampleFormat::U32 => run_audio::<u32>(&device, &config.into(), options),
    //         cpal::SampleFormat::U64 => run_audio::<u64>(&device, &config.into(), options),
    //         cpal::SampleFormat::F32 => run_audio::<f32>(&device, &config.into(), options),
    //         cpal::SampleFormat::F64 => run_audio::<f64>(&device, &config.into(), options),
    //         sample_format => panic!("Unsupported sample format '{sample_format}'"),
    //     }
    // });

    ambient_wasm::client::initialize(world, messenger)?;

    Ok(())
}


pub fn run_audio<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    options: (
        Arc<Mutex<String>>,
        (),
    ),
) -> Result<(), anyhow::Error>
where
    T: SizedSample + FromSample<f32>,
{
    let (code_clone, _) = options;
    let sr = config.sample_rate.0 as usize;
    let mut engine = Engine::<BLOCK_SIZE>::new();
    engine.livecoding = false;
    engine.set_sr(sr);
    let channels = 2 as usize; //config.channels as usize;
    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);
    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            let mut code_guard = code_clone.lock().unwrap();
            if code_guard.contains("[update]") {
                let code = code_guard.replace("[update]", "");
                engine.update_with_code(&code);
                *code_guard = code;
            }
            let block_step = data.len() / channels;
            let blocks_needed = block_step / BLOCK_SIZE;
            let block_step = channels * BLOCK_SIZE;
            for current_block in 0..blocks_needed {
                let (block, _err_msg) = engine.next_block(vec![]);
                for i in 0..BLOCK_SIZE {
                    for chan in 0..channels {
                        let value: T = T::from_sample(block[chan][i]);
                        data[(i*channels+chan)+(current_block)*block_step] = value;
                    }
                }
            }
        },
        err_fn,
        None,
    )?;
    stream.play()?;
    loop {}
}

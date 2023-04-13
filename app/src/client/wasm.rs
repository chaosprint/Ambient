use std::sync::Arc;

use ambient_core::{asset_cache, async_ecs::async_run, hierarchy::children, runtime};
use ambient_ecs::{EntityId, SystemGroup, World};
use ambient_wasm::shared::{get_module_name, MessageType};
use parking_lot::Mutex;
use ambient_world_audio::systems::setup_hrtf;
use ambient_audio::{Source, track::TrackDecodeStream};
use ambient_world_audio::{AudioMessage, audio_sender, audio_emitter, audio_listener, hrtf_lib};

use std::sync::mpsc::{self, Sender, Receiver};

pub fn systems() -> SystemGroup {
    ambient_wasm::client::systems()
}

pub fn initialize(world: &mut World) -> anyhow::Result<()> {
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

    let (tx, rx): (Sender<AudioMessage>, Receiver<AudioMessage>) = mpsc::channel();

    std::thread::spawn(move || {
        let stream = ambient_audio::AudioStream::new().unwrap();
        while let Ok(message) = rx.recv() {
            match message {
                AudioMessage::Track(t) => {
                    // println!("got message track {:?}", t.decode().duration());
                    let sound = stream.mixer().play(t.decode());
                    sound.wait();
                },
                _ => unimplemented!()
            }
        }
    });
    world.add_resource(audio_sender(), Arc::new(Mutex::new(tx)));
    ambient_wasm::client::initialize(world, messenger).unwrap();
    Ok(())
}


    // let
    // let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("am.wav");
    // let track = ambient_audio::track::Track::from_wav(
    //     std::fs::read(path)
    //         .unwrap()
    //         .to_vec(),
    // )
    // .unwrap();
    // let stream = ambient_audio::AudioStream::new().unwrap();
    // let source = track.decode();
    // eprintln!("Duration: {:?}", source.duration());
    // let sound = stream.mixer().play(source);
    // sound.wait();



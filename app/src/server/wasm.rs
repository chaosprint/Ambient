use std::{collections::HashMap, path::PathBuf, sync::Arc};

use ambient_core::asset_cache;
use ambient_ecs::{EntityId, SystemGroup, World};
use ambient_project::Identifier;
use ambient_std::{
    asset_cache::SyncAssetKeyExt,
    asset_url::{AssetUrl, ServerBaseUrlKey},
};
pub use ambient_wasm::server::{on_forking_systems, on_shutdown_systems};
use ambient_wasm::shared::{
    client_bytecode_from_url, get_module_name, module_bytecode, remote_paired_id, spawn_module, MessageType, ModuleBytecode,
};
use anyhow::Context;

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
    ambient_wasm::server::systems()
}

pub fn initialize(world: &mut World, project_path: PathBuf, manifest: &ambient_project::Manifest) -> anyhow::Result<()> {

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

    let host = cpal::default_host();
    let device = host.default_output_device().expect("failed to find output device");
    let config = device.default_output_config().unwrap();
    println!("Default output config: {:?}", config);
    let _audio_thread = thread::spawn(move || {
        let options = (code_clone, ());
        match config.sample_format() {
            cpal::SampleFormat::I8 => run_audio::<i8>(&device, &config.into(), options),
            cpal::SampleFormat::I16 => run_audio::<i16>(&device, &config.into(), options),
            cpal::SampleFormat::I32 => run_audio::<i32>(&device, &config.into(), options),
            cpal::SampleFormat::I64 => run_audio::<i64>(&device, &config.into(), options),
            cpal::SampleFormat::U8 => run_audio::<u8>(&device, &config.into(), options),
            cpal::SampleFormat::U16 => run_audio::<u16>(&device, &config.into(), options),
            cpal::SampleFormat::U32 => run_audio::<u32>(&device, &config.into(), options),
            cpal::SampleFormat::U64 => run_audio::<u64>(&device, &config.into(), options),
            cpal::SampleFormat::F32 => run_audio::<f32>(&device, &config.into(), options),
            cpal::SampleFormat::F64 => run_audio::<f64>(&device, &config.into(), options),
            sample_format => panic!("Unsupported sample format '{sample_format}'"),
        }
    });

    ambient_wasm::server::initialize(world, messenger)?;

    let build_dir = project_path.join("build");

    let mut modules_to_entity_ids = HashMap::new();
    for target in ["client", "server"] {
        let wasm_component_paths: Vec<PathBuf> = std::fs::read_dir(build_dir.join(target))
            .ok()
            .map(|rd| rd.filter_map(Result::ok).map(|p| p.path()).filter(|p| p.extension().unwrap_or_default() == "wasm").collect())
            .unwrap_or_default();

        let is_sole_module = wasm_component_paths.len() == 1;
        for path in wasm_component_paths {
            let name =
                Identifier::new(&*path.file_stem().context("no file stem for {path:?}")?.to_string_lossy()).map_err(anyhow::Error::msg)?;

            let description = manifest.project.description.clone().unwrap_or_default();
            let description = if is_sole_module { description } else { format!("{description} ({name})") };

            let id = spawn_module(world, &name, description, true);
            modules_to_entity_ids.insert((target, name.as_ref().strip_prefix(target).unwrap_or(name.as_ref()).to_string()), id);

            if target == "client" {
                let relative_path = path.strip_prefix(&build_dir)?;

                let base_url = ServerBaseUrlKey.get(world.resource(asset_cache()));
                let bytecode_url = AssetUrl::parse(&relative_path.to_string_lossy())?.resolve(&base_url)?.to_string();

                world.add_component(id, client_bytecode_from_url(), bytecode_url)?;
            } else {
                let bytecode = std::fs::read(path)?;
                world.add_component(id, module_bytecode(), ModuleBytecode(bytecode))?;
            }
        }
    }

    for ((target, name), id) in modules_to_entity_ids.iter() {
        let corresponding = match *target {
            "client" => "server",
            "server" => "client",
            _ => unreachable!(),
        };
        if let Some(other_id) = modules_to_entity_ids.get(&(corresponding, name.clone())) {
            world.add_component(*id, remote_paired_id(), *other_id)?;
        }
    }

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
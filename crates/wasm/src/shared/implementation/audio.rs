use ambient_std::asset_url::{AbsAssetUrl, AssetUrl};
use ambient_world_audio::{play_sound_on_entity, audio_tracks as tracks, audio_emitter, audio_listener, hrtf_lib};
use ambient_audio::{AudioFromUrl, track::TrackDecodeStream, Attenuation, AudioEmitter, Source};
use ambient_network::{server::content_base_url, ServerWorldExt};
use ambient_core::{asset_cache, async_ecs::async_run, hierarchy::children, runtime};
// use std::collections::HashSet;
// use ambient_animation::animation_controller;
use anyhow::{Context, Ok};
use ambient_ecs::{query as ecs_query, with_component_registry, EntityId, World};
use ambient_std::{
    asset_cache::{AssetCache, AsyncAssetKey, AsyncAssetKeyExt},
    download_asset::{AssetError, BytesFromUrl},
    unwrap_log_err,
};

use std::sync::Arc;
use parking_lot::Mutex;

use super::{
    super::{
        conversion::{FromBindgen, IntoBindgen},
        wit,
    },
    // component::convert_components_to_entity_data,
};

pub(crate) fn load(world: &mut World, soundurl: String) -> anyhow::Result<()> {
    println!("Loading sound: {:?}", soundurl);
    let assets = world.resource(asset_cache()).clone();
    let content_url = world.synced_resource(content_base_url()).unwrap();
    let base_url = &AbsAssetUrl::parse(content_url)?;
    let asset_url = AssetUrl::parse(soundurl)?.resolve(base_url)?;
    let runtime = world.resource(runtime()).clone();
    let async_run = world.resource(async_run()).clone();

    runtime.spawn(async move {
        let audio_url = AudioFromUrl { url: asset_url };
        let track = audio_url.get(&assets).await.expect("Failed to load audio");
        // let source = std::sync::Arc::new(track);
        // println!("Loaded sound duration: {:?}", source.duration());

        async_run.run(move |world| {
            world.add_resource(tracks(), vec![track]);
        });
    });
    Ok(())
}

pub(crate) fn play(world: &mut World, id: wit::types::EntityId, index: u32) -> anyhow::Result<()> {
    let source_vec = world.resource(tracks());
    let source = source_vec[index as usize].clone().decode();

    let stream = ambient_audio::AudioStream::new().unwrap();
    let sound = stream.mixer().play(source);
    Ok(())
}

pub(crate) fn set_listener(world: &mut World, entity: wit::types::EntityId, listener: wit::audio::AudioListener) -> anyhow::Result<()> {
    // let mut audio_listeners = world.resource_mut(audio_listeners());
    world.add_component(
        entity.from_bindgen(),
        audio_listener(),
        Arc::new(Mutex::new(listener.from_bindgen()))
    ).unwrap();
    Ok(())
}

pub(crate) fn set_emitter(world: &mut World, entity: wit::types::EntityId) -> anyhow::Result<()> {
    let pos = glam::vec3(0.0_f32.cos() * 16.0, 0.0_f32.sin() * 16.0, 2.0);
    let emitter = Arc::new(Mutex::new(AudioEmitter {
        amplitude: 5.0,
        attenuation: Attenuation::InversePoly { quad: 0.1, lin: 0.0, constant: 1.0 },
        pos,
    }));
    world.add_component(entity.from_bindgen(), audio_emitter(), emitter).unwrap();
    Ok(())
}
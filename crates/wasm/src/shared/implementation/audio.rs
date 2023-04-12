use ambient_std::asset_url::{AbsAssetUrl, AssetUrl};
use ambient_world_audio::{play_sound_on_entity, audio_listener};
use ambient_audio::{AudioFromUrl, track::TrackDecodeStream, Source};
use ambient_world_audio::{audio_tracks as tracks};
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
        let source = std::sync::Arc::new(track.decode());
        println!("Loaded sound duration: {:?}", source.duration());

        async_run.run(move |world| {
            world.add_resource(tracks(), vec![source]);
        });
    });
    Ok(())
}

pub(crate) fn play(world: &mut World, index: u32) -> anyhow::Result<()> {
    let source_vec = world.resource(tracks());
    let source = &source_vec[index as usize];
    println!("loaded sound: {:?}", source.duration());
    Ok(())
}

pub(crate) fn set_listener(world: &mut World, entity: wit::types::EntityId, listener: wit::audio::AudioListener) -> anyhow::Result<()> {
    // let mut audio_listeners = world.resource_mut(audio_listeners());
    world.set(entity.from_bindgen(), audio_listener(), Arc::new(Mutex::new(listener.from_bindgen())));
    Ok(())
}

// pub(crate) fn load(world: &World, audio_tracks: wit::audio::AudioTracks) -> anyhow::Result<()> {
    // let assets = world.resource(asset_cache()).clone();

    // let url = world.synced_resource(content_base_url()).unwrap();
    // let base_url = &AbsAssetUrl::parse(url)?;
    // let asset_url = AssetUrl::parse(sound.url)?.resolve(base_url)?;
    // println!("Loading sound: {:?}", audio_tracks);
    // runtime.spawn(async move {

        // let audio_url = AudioFromUrl { url: asset_url };
        // let track = unwrap_log_err!(audio_url.load(assets).await);
        // let mut source = track.decode();
        // println!("Loaded sound duration: {:?}", source.duration());
        // let track = unwrap_log_err!(audio_url.load(assets).await).decode();
        // let entity_id = world.resource_entity();

        // let mut source = track.decode();
        // let base_ent_id = obj.resource(children())[0];
        // println!("Loaded sound base_ent_id: {:?}", obj);
        // let obj = unwrap_log_err!(url.get(&assets).await);
        // let base_ent_id = obj.resource(children())[0];

        // TODO: This only handles prefabs with a single entity

        // let entity = obj.clone_entity(base_ent_id).unwrap();
        // async_run.run(move |world| {
        //     world.add_component(
        //         entity_id,
        //         tracks(),
        //         audio_tracks.from_bindgen(),
                // ambient_audio::AudioTracks { tracks: tracks.into_iter().map(|x|x.from_bindgen()).collect() },
    //         );
    //     });
    // });
    // Ok(())

// }
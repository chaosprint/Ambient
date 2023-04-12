use ambient_world_audio::{play_sound_on_entity as play, track as track_component};
use ambient_audio::{AudioFromUrl, track::TrackDecodeStream, Source};
use ambient_std::{
    asset_url::{AbsAssetUrl, AssetUrl}
};
use ambient_std::{
    asset_cache::{AsyncAssetKey}
};
use ambient_network::{ServerWorldExt, server::content_base_url};
use ambient_ecs::{World, Component};
// use ambient_core::audio::{audio_emitter};
use anyhow::Ok;
use ambient_core::{asset_cache, async_ecs::async_run, runtime};
use std::collections::HashMap;
use super::{
    super::{
        conversion::{FromBindgen, IntoBindgen},
        wit,
    },
    // component::convert_components_to_entity_data,
};
use crate::shared::implementation::component::get_component_type;

pub(crate) fn add_sound(
    world: &mut World,
    index: u32,
    path: String,
) -> anyhow::Result<()> {
    let asset = world.resource(asset_cache()).clone();
    let url = world.synced_resource(content_base_url()).unwrap();
    let base_url = &AbsAssetUrl::parse(url)?;
    let asset_url = AssetUrl::parse(path)?.resolve(&base_url)?;

    // let runtime = world.resource(runtime());
    // runtime.spawn(async move {
    //     let audio_url = AudioFromUrl { url: asset_url };
    //     let track = audio_url.load(asset).await.unwrap();
    //     let mut source = track.decode();
    //     // let sample_count = source.sample_count().unwrap();
    //     // let sample_rate =  source.sample_rate();
    //     let v = source.to_vec();
    //     let component_type = get_component_type(index).unwrap();
    //     world.add_resource(component_type, v.into_bindgen());
    // });

    // let component_type = get_component_type(index).unwrap();
    // world.add_resource(component_type, vec![0.1_f32]);
    Ok(())
}

pub(crate) fn play_sound_on_entity(
    world: &World,
    entity: wit::types::EntityId,
    name: String,
) -> anyhow::Result<()> {
    // let mut soundlib = world.resource(tracks());

    println!("play sound {:?} on entity: {:?} ", name , entity);
    Ok(())
}

pub(crate) fn listener(
    world: &World,
) -> anyhow::Result<()> {
    // let url = world.synced_resource(content_base_url()).unwrap();
    // let base_url = &AbsAssetUrl::parse(url)?;
    // let asset_url = AssetUrl::parse(path)?.resolve(&base_url)?;
    // Ok(Some(asset_url.to_string()))
    Ok(())
}

pub(crate) fn emitter(
    world: &World,
) -> anyhow::Result<()> {
    // let url = world.synced_resource(content_base_url()).unwrap();
    // let base_url = &AbsAssetUrl::parse(url)?;
    // let asset_url = AssetUrl::parse(path)?.resolve(&base_url)?;
    // Ok(Some(asset_url.to_string()))
    Ok(())
}
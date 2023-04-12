use ambient_std::asset_url::{AbsAssetUrl, AssetUrl};

use ambient_network::{server::content_base_url, ServerWorldExt};

// use std::collections::HashSet;
// use ambient_animation::animation_controller;
// use ambient_core::transform::translation;
use ambient_ecs::{World}; // query as ecs_query, with_component_registry, EntityId,
// use anyhow::Context;

use super::{
    super::{
        // conversion::{FromBindgen, IntoBindgen},
        wit,
    },
    // component::convert_components_to_entity_data,
};

pub(crate) fn load(world: &World, sound: wit::audio::Sound) -> anyhow::Result<()> {
    let url = world.synced_resource(content_base_url()).unwrap();
    let base_url = &AbsAssetUrl::parse(url)?;
    let asset_url = AssetUrl::parse(sound.url)?.resolve(base_url)?;
    println!("Loading sound: {:?} looping {:?}", asset_url, sound.looping);
    Ok(())
}
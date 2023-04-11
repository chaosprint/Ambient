use ambient_world_audio::*;
use ambient_network::{ServerWorldExt, server::content_base_url};
use ambient_ecs::World;
use anyhow::Ok;

pub(crate) fn spawn_on_world(
    world: &World,
) -> anyhow::Result<()> {
    // let url = world.synced_resource(content_base_url()).unwrap();
    // let base_url = &AbsAssetUrl::parse(url)?;
    // let asset_url = AssetUrl::parse(path)?.resolve(&base_url)?;
    // Ok(Some(asset_url.to_string()))
    Ok(())
}
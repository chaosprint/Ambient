use ambient_api::prelude::*;
use ambient_api::{
    components::core::{
        app::main_scene,
        camera::aspect_ratio_from_window,
        prefab::{prefab_from_url, spawned},
        transform::{lookat_center, rotation, translation},
    },
    concepts::{make_perspective_infinite_reverse_camera, make_transformable},
};

#[main]
async fn main() {
    // Load the asset
    println!("asset url can be accessed from client: {}", asset::url("assets/Cube.glb").unwrap());
    audiosys::add_sound("test".to_string(), asset::url("assets/ping.ogg").unwrap());
    let cube_id = Entity::new()
        .with_merge(make_transformable())
        .with(prefab_from_url(), asset::url("assets/Cube.glb").unwrap())
        .with(components::is_the_best(), true)
        .with(translation(), vec3(0.0, 0.3, 0.4))
        .spawn();
    entity::wait_for_component(cube_id, spawned()).await;

}
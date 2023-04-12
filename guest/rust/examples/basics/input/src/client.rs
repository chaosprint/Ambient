use ambient_api::{
    components::core::{
        app::main_scene,
        camera::aspect_ratio_from_window,
        physics::{
            angular_velocity, box_collider, dynamic, linear_velocity, physics_controlled,
            visualizing,
        },
        prefab::prefab_from_url,
        primitives::cube,
        rendering::{cast_shadows, color},
        transform::{lookat_center, rotation, scale, translation},
    },
    audio::{AudioListener},
    concepts::{make_perspective_infinite_reverse_camera, make_transformable},
    prelude::*,
};

#[main]
pub fn main() {
    let cam = Entity::new()
    .with_merge(make_perspective_infinite_reverse_camera())
    .with(aspect_ratio_from_window(), EntityId::resources())
    .with_default(main_scene())
    .with(translation(), vec3(5., 5., 4.))
    .with(lookat_center(), vec3(0., 0., 0.))
    .spawn();

    let plane = Entity::new()
    .with_merge(make_transformable())
    .with(prefab_from_url(), asset::url("assets/Shape.glb").unwrap())
    .spawn();

    audio::load(asset::url("assets/bonk.ogg").unwrap());

    audio::set_listener(
        cam,
        Mat4::IDENTITY,
        Vec3::X * 0.3
    );

    audio::set_emitter(plane);

    ambient_api::messages::Frame::subscribe(move |_| {
        let (delta, _) = player::get_raw_input_delta();

        if !delta.keys.is_empty() {
            audio::play(plane, 0);
            println!("Pressed the keys {:?}", delta.keys);
        }
        if !delta.keys_released.is_empty() {
            println!("Released the keys {:?}", delta.keys_released);
        }
        if !delta.mouse_buttons.is_empty() {
            println!("Pressed the mouse buttons {:?}", delta.mouse_buttons);
        }
        if !delta.mouse_buttons_released.is_empty() {
            println!(
                "Released the mouse buttons {:?}",
                delta.mouse_buttons_released
            );
        }
        if delta.mouse_wheel != 0.0 {
            println!("Scrolled {}", delta.mouse_wheel);
        }
        // if delta.mouse_position.length_squared() != 0.0 {
        //     println!("Moved their mouse by {}", delta.mouse_position);
        // }
    });
}

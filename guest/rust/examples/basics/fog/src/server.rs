use ambient_api::{
    components::core::{
        app::main_scene,
        camera::{aspect_ratio_from_window, fog},
        player::player,
        primitives::{cube, quad},
        rendering::{
            cast_shadows, color, fog_color, fog_density, fog_height_falloff, light_diffuse, sky,
            sun,
        },
        transform::{lookat_center, rotation, scale, translation},
    },
    concepts::{make_perspective_infinite_reverse_camera, make_transformable},
    prelude::*,
};

#[main]
pub fn main() {
    Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with(aspect_ratio_from_window(), EntityId::resources())
        .with_default(main_scene())
        .with_default(fog())
        .with(translation(), vec3(0., -5., 3.))
        .with(lookat_center(), vec3(0., 0., 2.))
        .spawn();

    let sun = Entity::new()
        .with_merge(make_transformable())
        .with_default(sun())
        .with(rotation(), Quat::from_rotation_y(-1.))
        .with_default(main_scene())
        .with(light_diffuse(), Vec3::ONE)
        .with(fog_color(), vec3(1., 1., 1.))
        .with(fog_density(), 0.001)
        .with(fog_height_falloff(), 0.01)
        .spawn();

    Entity::new()
        .with_merge(make_transformable())
        .with_default(sky())
        .spawn();

    Entity::new()
        .with_merge(make_transformable())
        .with_default(quad())
        .with(scale(), Vec3::ONE * 1000.)
        .with(color(), vec4(1., 0., 0., 1.))
        .spawn();

    for i in 0..10 {
        Entity::new()
            .with_merge(make_transformable())
            .with_default(cube())
            .with(translation(), vec3(0., 1. * (2f32).powi(i), 1.))
            .with(scale(), Vec3::ONE * 2.)
            .with(color(), vec4(0., 1., 0., 1.))
            .with_default(cast_shadows())
            .spawn();
    }
    query(player()).build().each_frame(move |ids| {
        for (id, _) in ids {
            let Some((delta, _)) = player::get_raw_input_delta(id) else { continue; };

            let set_fog_density = |density| {
                println!("Fog density: {density}");
                entity::set_component(sun, fog_density(), density);
            };

            let set_fog_height_falloff = |height_falloff| {
                println!("Fog height_falloff: {height_falloff}");
                entity::set_component(sun, fog_height_falloff(), height_falloff);
            };

            if delta.keys.contains(&player::KeyCode::Key1) {
                set_fog_density(1.);
            }
            if delta.keys.contains(&player::KeyCode::Key2) {
                set_fog_density(0.1);
            }
            if delta.keys.contains(&player::KeyCode::Key3) {
                set_fog_density(0.01);
            }
            if delta.keys.contains(&player::KeyCode::Key4) {
                set_fog_density(0.0);
            }

            if delta.keys.contains(&player::KeyCode::Q) {
                set_fog_height_falloff(1.);
            }
            if delta.keys.contains(&player::KeyCode::W) {
                set_fog_height_falloff(0.1);
            }
            if delta.keys.contains(&player::KeyCode::E) {
                set_fog_height_falloff(0.01);
            }
            if delta.keys.contains(&player::KeyCode::R) {
                set_fog_height_falloff(0.0);
            }
        }
    });
}
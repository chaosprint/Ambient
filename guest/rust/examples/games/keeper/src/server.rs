use crate::{
    components::{ball},
};
use ambient_api::{
    components::core::{
        app::main_scene,
        camera::aspect_ratio_from_window,
        ecs::ids,
        physics::{
            character_controller_height, character_controller_radius, physics_controlled,
            plane_collider, sphere_collider, visualizing,
        },
        physics::{unit_velocity,
            angular_velocity, box_collider, dynamic, linear_velocity},
        prefab::{prefab_from_url, spawned},
        primitives::{cube, quad, sphere},
        rendering::{cast_shadows, color, outline, pbr_material_from_url},
        transform::{lookat_center, rotation, scale, translation},
        model::model_from_url,
    },
    // player::{user_id, get_raw_input_delta},
    concepts::{make_perspective_infinite_reverse_camera, make_sphere, make_transformable},
    entity::{AnimationAction, AnimationController},
    player::{KeyCode},
    prelude::*,
    rand,
};

use std::f32::consts::PI;

#[main]
pub async fn main() {

    let cam = Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with(aspect_ratio_from_window(), EntityId::resources())
        .with_default(main_scene())
        .with(translation(), vec3(-10.1, 0.0, 5.0))
        .with(lookat_center(), vec3(0., 0., 0.0))
        .spawn();

    let floor = Entity::new()
        .with_merge(make_transformable())
        .with_default(quad())
        .with_default(plane_collider())
        .with(scale(), Vec3::ONE * 20.)
        .with(color(), vec4(0.1, 0.3, 0.1, 0.5))
        .spawn();

    let ball = Entity::new()
        .with_default(main_scene())
        .with_default(physics_controlled())
        .with_default(ball())
        .with(dynamic(), true)
        .with(scale(), Vec3::ONE*0.3)
        .with(translation(), vec3(8., 0., 0.3))
        .with(linear_velocity(), Vec3::ONE)
        .with(sphere_collider(), 0.3)
        .with(model_from_url(), asset::url("assets/ball.glb").unwrap())
        .spawn();

    let idle = Entity::new()
    .with_merge(make_transformable())
    .with(
        model_from_url(),
        asset::url("assets/keeper_idle.fbx").unwrap(),
    )
    .with_default(physics_controlled())
    .with_default(visualizing())
    .with(dynamic(), true)
    .with(box_collider(), vec3(0.5, 0.1, 1.0))
    .with(translation(), vec3(0.0, 0.0, 0.0))
    .with(rotation(), Quat::from_axis_angle(Vec3::Z, -PI/2.))
    .spawn();
    // .with(linear_velocity(), vec3(0.0, 0.0, 0.0))

    // query(player()).build().each_frame(move |players| {
        // for (playerid, _) in players {
            let (delta, _) = player::get_raw_input_delta();
            if delta.keys.contains(&KeyCode::Space) {
                let velocity_x = random::<f32>()*10.0-25.0;
                let velocity_y = random::<f32>()*6.0-3.0;
                let velocity_z = random::<f32>()*6.0;
                println!("v x y z {} {} {}", velocity_x, velocity_y, velocity_z);

                entity::set_components(
                    ball,
                    Entity::new()
                    .with(translation(), Vec3::new(10.0, 0.0, 0.3))
                    .with(linear_velocity(), Vec3::new(velocity_x, velocity_y, velocity_z))
                    // .with(rotation(), random::<Vec3>())
                );

                // entity::set_component(
                //     idle,
                //     translation(),
                //     vec3(0.0, 0.0, 0.0),
                // );
                // entity::set_component(
                //     idle,
                //     linear_velocity(),
                //     Vec3::ZERO,
                // );
                entity::set_animation_controller(
                    idle,
                    AnimationController {
                        actions: &[
                            AnimationAction {
                                clip_url: &asset::url(
                                    "assets/keeper_idle.fbx/animations/Armature.anim",
                                )
                                .unwrap(),
                                looping: true,
                                weight: 0.5,
                            },
                        ],
                        apply_base_pose: false,
                    },
                );
            }

            if delta.keys.contains(&KeyCode::Left) {
                // entity::set_component(left, translation(), vec3(8.0, 0.0, 0.0));
                entity::set_animation_controller(
                    idle,
                    AnimationController {
                        actions: &[
                            AnimationAction {
                                clip_url: &asset::url(
                                    "assets/keeper_save_left.fbx/animations/Armature.anim",
                                )
                                .unwrap(),
                                looping: false,
                                weight: 0.5,
                            },
                            AnimationAction {
                                clip_url: &asset::url(
                                    "assets/keeper_idle.fbx/animations/Armature.anim",
                                )
                                .unwrap(),
                                looping: true,
                                weight: 0.5,
                            },
                        ],
                        apply_base_pose: false,
                    },
                );
            }

            if delta.keys.contains(&KeyCode::Right) {
                entity::set_animation_controller(
                    idle,
                    AnimationController {
                        actions: &[
                            AnimationAction {
                                clip_url: &asset::url(
                                    "assets/keeper_save_right.fbx/animations/Armature.anim",
                                )
                                .unwrap(),
                                looping: false,
                                weight: 0.5,
                            },
                            AnimationAction {
                                clip_url: &asset::url(
                                    "assets/keeper_idle.fbx/animations/Armature.anim",
                                )
                                .unwrap(),
                                looping: true,
                                weight: 0.5,
                            },
                        ],
                        apply_base_pose: false,
                    },
                );
                // entity::set_component(
                //     idle,
                //     linear_velocity(),
                //     vec3(1.0, 6.0, 0.0),
                // );
            }
        }
    });

    /*loop {
        // let max_angular_velocity = 360.0f32.to_radians();
        // let new_angular_velocity = (random::<Vec3>() - 0.5) * 2. * max_angular_velocity;
        let velocity_x = random::<f32>()*10.0-25.0;
        let velocity_y = random::<f32>()*6.0-3.0;
        let velocity_z = random::<f32>()*6.0;
        println!("v x y z {} {} {}", velocity_x, velocity_y, velocity_z);

        entity::set_components(
            ball,
            Entity::new()
            .with(translation(), Vec3::new(10.0, 0.0, 0.3))
            .with(linear_velocity(), Vec3::new(velocity_x, velocity_y, velocity_z))
            // .with(rotation(), random::<Vec3>())
        );

        sleep(3.5).await;
        entity::set_component(
            idle,
            translation(),
            vec3(-5.0, 0.0, 0.0),
        );
        entity::set_component(
            idle,
            linear_velocity(),
            Vec3::ZERO,
        );
        entity::set_animation_controller(
            idle,
            AnimationController {
                actions: &[
                    AnimationAction {
                        clip_url: &asset::url(
                            "assets/keeper_idle.fbx/animations/Armature.anim",
                        )
                        .unwrap(),
                        looping: true,
                        weight: 0.5,
                    },
                ],
                apply_base_pose: false,
            },
        );
    }
} */
    // EventOk
}
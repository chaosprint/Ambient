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
    concepts::{make_perspective_infinite_reverse_camera, make_transformable},
    prelude::*,
};

#[main]
pub async fn main() {
    let cam = Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with(aspect_ratio_from_window(), EntityId::resources())
        .with_default(main_scene())
        .with(translation(), vec3(5., 5., 4.))
        .with(lookat_center(), vec3(0., 0., 0.))
        .spawn();

    let cube = Entity::new()
        .with_merge(make_transformable())
        .with_default(cube())
        .with_default(visualizing())
        .with_default(physics_controlled())
        .with_default(cast_shadows())
        .with_default(linear_velocity())
        .with_default(angular_velocity())
        .with(box_collider(), Vec3::ONE)
        .with(dynamic(), true)
        .with(translation(), vec3(0., 0., 5.))
        .with(rotation(), Quat::IDENTITY)
        .with(scale(), vec3(0.5, 0.5, 0.5))
        .with(color(), Vec4::ONE)
        .spawn();

    let bonk = audio::load(
        Sound {
            url: asset::url("assets/bonk.wav").unwrap(),
            looping: false,
            ..Default::default()
        }
    );

    // audio::set_emitter(
    //     cube,
    //     AudioEmitter {
    //         amplitude: 5.0,
    //         attenuation: Attenuation::InversePoly { quad: 0.1, lin: 0.0, constant: 1.0 },
    //         pos: entity::get_component(plate, translation()).unwrap(),
    //     };
    // );

    // audio::set_listener(
    //     cam,
    //     AudioListener::new(Mat4::IDENTITY, Vec3::X * 0.3)
    // );

    // when hit, play a sound
    // entity::play_sound(plate, bonk);

    Entity::new()
        .with_merge(make_transformable())
        .with(prefab_from_url(), asset::url("assets/Shape.glb").unwrap())
        .spawn();

    ambient_api::messages::Collision::subscribe(|msg| {
        // TODO: play a sound instead
        println!("Bonk! {:?} collided", msg.ids);
    });

    ambient_api::messages::Frame::subscribe(move |_| {
        for hit in physics::raycast(Vec3::Z * 20., -Vec3::Z) {
            if hit.entity == cube {
                println!("The raycast hit the cube: {hit:?}");
            }
        }
    });

    loop {
        let max_linear_velocity = 2.5;
        let max_angular_velocity = 360.0f32.to_radians();

        sleep(5.).await;

        let new_linear_velocity = (random::<Vec3>() - 0.5) * 2. * max_linear_velocity;
        let new_angular_velocity = (random::<Vec3>() - 0.5) * 2. * max_angular_velocity;
        println!("And again! Linear velocity: {new_linear_velocity:?} | Angular velocity: {new_angular_velocity:?}");
        entity::set_components(
            cube,
            Entity::new()
                .with(translation(), vec3(0., 0., 5.))
                .with(rotation(), Quat::IDENTITY)
                .with(linear_velocity(), new_linear_velocity)
                .with(angular_velocity(), new_angular_velocity)
                .with(color(), random::<Vec3>().extend(1.)),
        );
    }
}

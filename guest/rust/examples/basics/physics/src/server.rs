use ambient_api::{
    components::core::{
        app::main_scene,
        camera::aspect_ratio_from_window,
        ecs::ids,
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

fn symbol(s: &str) -> String {
    s.to_string()
}

fn url(s: &str) -> String {
    s.to_string()
}

#[main]
pub async fn main() {

    audiosys::add_sound(symbol("ping"), asset::url("assets/ping.ogg").unwrap());

    Entity::new()
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

    Entity::new()
        .with_merge(make_transformable())
        .with(prefab_from_url(), asset::url("assets/Shape.glb").unwrap())
        .spawn();

    on(event::FRAME, |_| {

        // audiosys.set_distance();
        // audiosys.set_angle();
    });

    on(event::COLLISION, |c| {
        audiosys::init(symbol("[play]ping"));
        // audiosys::get_sound("bonk").unwrap().volumn(0.2).play();
        // psedo code
        // if bonksound.lock().is_playing() {
            // bonksound.lock().fade_out(std::time::Duration::from_secs(0.1));
        // }
        // bonksound.lock() = audiosys.get_sound("bonk").unwrap().volumn(0.5).space().play();
        // audiosys.run_script("o: sin 440");
        // sound::clip("bonk").play(); // -> play in client.wasm -> msg to client.rs.runtime -> clap/audioworklet
        // sound::script("o: sin 440").play().stop_after(std::time::Duration::from_secs(4.0));
        // entity::get_component(cube, sound()).unwrap().distance(30.).volumn(0.1/*TODO: calculate this*/).play();
        // println!("Bonk! {:?} collided", c.get(ids()).unwrap());
    });

    on(event::FRAME, move |_| {
        for hit in physics::raycast(Vec3::Z * 20., -Vec3::Z) {
            if hit.entity == cube {
                // println!("The raycast hit the cube: {hit:?}");
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

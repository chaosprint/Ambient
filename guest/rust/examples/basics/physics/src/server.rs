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

use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

#[main]
pub async fn main() -> EventResult {

    let triggered = Arc::new(AtomicBool::new(false));
    let _triggered = triggered.clone();
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

    println!("[glicol_code]o: noise 42 >> lpf 1500 1 >> mul ~amp >> mul ~env >> plate 0.1;
    ~env: imp 0.01 >> envperc 0.001 0.1; ~amp: sig 0.0");

    on(event::COLLISION, move |c| {
        _triggered.store(true, Ordering::SeqCst);
        let v = entity::get_component(cube, linear_velocity()).unwrap().length();
        println!("velocity {:?}", entity::get_component(cube, linear_velocity()).unwrap().length());
        let scale = 1.0 - (-2.0 * (v/10.).clamp(0.,1.)).exp();
        println!("[glicol_msg]~amp, 0, 0, {:.4}; ~env, 0, 1, 0; o, 1, 0, {:.4}", scale, scale*2000.+500. );
        // println!("[glicol_msg]");
        // println!("Bonk! {:?} collided", c.get(ids()).unwrap());
        EventOk
    });

    // run_async(async move {
    //     if triggered.load(Ordering::SeqCst) {
    //         sleep(0.11).await;
    //         println!("[glicol_msg]~env, 0, 0, 1");
    //         return EventOk;
    //     }
    //     EventOk
    // });

    on(event::FRAME, move |_| {
        for hit in physics::raycast(Vec3::Z * 20., -Vec3::Z) {
            if hit.entity == cube {
                // println!("The raycast hit the cube: {hit:?}");
            }
        }
        EventOk
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

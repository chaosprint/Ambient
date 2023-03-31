// A synth hero game.

use ambient_api::{
    components::core::{
        app::main_scene,
        camera::aspect_ratio_from_window,
        primitives::{cube, quad},
        player::player,
        prefab::{prefab_from_url, spawned},
        rendering::{ pbr_material_from_url, color, cast_shadows, outline},
        transform::{lookat_center, scale, translation},
    },
    concepts::{make_perspective_infinite_reverse_camera, make_transformable},
    prelude::*,
    player::KeyCode,
};
use components::cell;
use palette::{FromColor, Hsl, Srgb};

use std::sync::{Arc, atomic::Ordering, atomic::AtomicBool};
use std::sync::Mutex;
use crate::components::{grid_side_length, grid_x, grid_y};

#[main]
pub async fn main() -> EventResult {
    let mut pressed = Arc::new(AtomicBool::new(false));
    let mut pressed_block = Arc::new(Mutex::new(vec![]));

    Entity::new()
        .with_merge(make_perspective_infinite_reverse_camera())
        .with(aspect_ratio_from_window(), EntityId::resources())
        .with_default(main_scene())
        .with(translation(), vec3(0., 5., 8.))
        .with(lookat_center(), vec3(0., 0., 0.))
        .spawn();

    let board_length = 20.0;
    let board_width = 3.0;
    let board_height = 0.2;
    let rolling_speed = 0.1;

    let board = Entity::new()
    .with_merge(make_transformable())
    .with_default(cube())
    .with(translation(), vec3(0., -board_length / 2.0, 0.))
    .with(scale(), vec3(board_width, board_length, board_height))
    .with(
        pbr_material_from_url(),
        asset::url("assets/pipeline.json/0/mat.json").unwrap(),
    )
    .with_default(cast_shadows())
    .spawn();

    let n_rows = 20; // Number of rows of blocks.
    let n_cols = 3; // Number of columns of blocks.

    let block_width = board_width / n_cols as f32;
    let block_height = 0.2;
    let block_length = board_length / n_rows as f32;
    let mut blocks = vec![];

    for row in 0..n_rows {
        for col in 0..n_cols {
            blocks.push(Entity::new()
                .with_merge(make_transformable())
                .with_default(cube())
                .with(
                    translation(),
                    vec3(
                        col as f32 * block_width - board_width / 2.0 + block_width / 2.0,
                        row as f32 * block_length - board_length + block_length / 2.0,
                        0.3,
                    ),
                )
                .with(scale(), vec3(block_width * 0.7, block_length * 0.7, block_height))
                .with(color(), vec4(0.0, 0.9, 0.9, 0.8))
                .spawn());
        }
    };

    let mut cells = Vec::new();
    for x in 0..3 {
        let id = Entity::new()
            .with_merge(make_transformable())
            // .with_default(cube())
            .with(prefab_from_url(), asset::url("assets/Cylinder.glb").unwrap())
            .with(translation(), vec3(x as f32 - 1., 3., 0.8))
            .with(components::is_the_best(), true)
            .with(scale(), vec3(block_width * 0.4, block_length * 0.4, 0.3))
            .with(color(), vec4(0.9, 0.1, 0.1, 0.7))
            .spawn();

        entity::wait_for_component(id, spawned()).await;
        cells.push(id);
    }

    spawn_query(player()).bind(|ids| {
        for (id, _) in ids {
            entity::add_component(id, cell(), 0);
        }
    });

    on(event::FRAME, move |_| {

        for cell in &cells {
            entity::remove_component(*cell, outline());
        }

        let players = entity::get_all(player());
        let n_players = players.len();

        for (i, player) in players.into_iter().enumerate() {

            let player_color = Srgb::from_color(Hsl::from_components((
                360. * i as f32 / n_players as f32,
                1.,
                0.5,
            )));
            let player_color = vec4(player_color.red, player_color.green, player_color.blue, 1.);

            let Some(cell) = entity::get_component(player, components::cell()) else { continue; };
            let Some((delta, _)) = player::get_raw_input_delta(player) else { continue; };
            let mut x = cell % 3;
            let keys = &delta.keys;
            let keys_released = &delta.keys_released;
            if keys.contains(&KeyCode::Left) {
                x = (x + 3 - 1) % 3;
                match x {
                    0 => println!("[glicol_msg]~freq, 0, 0, {:.4}", 440.0*2.0_f32.powf(( 40.-69.0)/12.0)),
                    1 => println!("[glicol_msg]~freq, 0, 0, {:.4}", 440.0*2.0_f32.powf(( 43.-69.0)/12.0)),
                    2 => println!("[glicol_msg]~freq, 0, 0, {:.4}", 440.0*2.0_f32.powf(( 45.-69.0)/12.0)),
                    _ => (),
                }
            }
            if keys.contains(&KeyCode::Right) {
                x = (x + 1) % 3;
                match x {
                    0 => println!("[glicol_msg]~freq, 0, 0, {:.4}", 440.0*2.0_f32.powf(( 40.-69.0)/12.0)),
                    1 => println!("[glicol_msg]~freq, 0, 0, {:.4}", 440.0*2.0_f32.powf(( 43.-69.0)/12.0)),
                    2 => println!("[glicol_msg]~freq, 0, 0, {:.4}", 440.0*2.0_f32.powf(( 45.-69.0)/12.0)),
                    _ => (),
                }
            }
            if keys.contains(&KeyCode::Up)  {
                x = (x + 3 - 1) % 3;
                match x {
                    0 => println!("[glicol_msg]~freq, 0, 0, {:.4}", 440.0*2.0_f32.powf(( 40.-69.0)/12.0)),
                    1 => println!("[glicol_msg]~freq, 0, 0, {:.4}", 440.0*2.0_f32.powf(( 43.-69.0)/12.0)),
                    2 => println!("[glicol_msg]~freq, 0, 0, {:.4}", 440.0*2.0_f32.powf(( 45.-69.0)/12.0)),
                    _ => (),
                }
            }
            if keys.contains(&KeyCode::Down)  {
                x = (x + 1) % 3;
                match x {
                    0 => println!("[glicol_msg]~freq, 0, 0, {:.4}", 440.0*2.0_f32.powf(( 40.-69.0)/12.0)),
                    1 => println!("[glicol_msg]~freq, 0, 0, {:.4}", 440.0*2.0_f32.powf(( 43.-69.0)/12.0)),
                    2 => println!("[glicol_msg]~freq, 0, 0, {:.4}", 440.0*2.0_f32.powf(( 45.-69.0)/12.0)),
                    _ => (),
                }
            }
            let cell = x;

            let cell = if keys.contains(&KeyCode::Key1) {
                println!("[glicol_msg]~freq, 0, 0, {:.4}", 440.0*2.0_f32.powf(( 40.-69.0)/12.0));
                0
            } else if keys.contains(&KeyCode::Key2) {
                println!("[glicol_msg]~freq, 0, 0, {:.4}", 440.0*2.0_f32.powf(( 43.-69.0)/12.0));
                1
            } else if keys.contains(&KeyCode::Key3) {
                println!("[glicol_msg]~freq, 0, 0, {:.4}", 440.0*2.0_f32.powf(( 45.-69.0)/12.0));
                2
            } else {
                cell
            };

            entity::add_component_if_required(cells[cell as usize], outline(), player_color);
            entity::set_component(player, components::cell(), cell);

            if keys.contains(&KeyCode::Space) {
                pressed.store(true, Ordering::Relaxed);
                entity::set_component(cells[cell as usize], scale(), vec3(block_width * 0.4, block_length * 0.4, 0.1));
                println!("[glicol_msg]~env, 0, 0, 1");
            }
            // need to figure out the logic for which blocks are pressed
            if keys_released.contains(&KeyCode::Space) {
                pressed.store(false, Ordering::Relaxed);
                entity::set_component(cells[cell as usize], scale(), vec3(block_width * 0.4, block_length * 0.4, 0.3));
                println!("[glicol_msg]~env, 0, 0, 0");

                // when released, no blocks are pressed, for sure
                let mut pressed_vec = pressed_block.lock().unwrap();
                for block in (*pressed_vec).iter() {
                    entity::set_component(*block, scale(), vec3(block_width * 0.7, block_length * 0.7, block_height));
                    entity::set_component(*block, color(), vec4(0.0, 0.9, 0.9, 0.8));
                }
                (*pressed_vec).clear();
            }

            pressed_block.lock().unwrap().clear();
            if pressed.load(Ordering::Relaxed) {
                let pressed_pos = entity::get_component::<Vec3>(cells[cell as usize], translation()).unwrap_or_default();
                for block in &blocks {
                    entity::set_component(*block, scale(), vec3(block_width * 0.7, block_length * 0.7, board_height));
                    entity::set_component(*block, color(), vec4(0.0, 0.9, 0.9, 0.8));
                    let mut blockpos = entity::get_component::<Vec3>(*block, translation()).unwrap_or_default();
                    let is_overlap = (blockpos.y < pressed_pos.y + block_length * 0.7 && blockpos.y > pressed_pos.y - block_length * 0.7)
                        && (blockpos.x < pressed_pos.x + block_width && blockpos.x > pressed_pos.x - block_width);
                    if is_overlap {
                        pressed_block.lock().unwrap().push(*block);
                    }
                }
            }

            for block in (*pressed_block.lock().unwrap()).iter() {
                entity::set_component(*block, scale(), vec3(block_width * 0.7, block_length * 0.7, board_height/2.0));
                entity::set_component(*block, color(), vec4(0.0, 0.2, 0.6, 0.5));
            }
        }

        let speed_scale = 0.2;
        let mut position = entity::get_component::<Vec3>(board, translation()).unwrap_or_default();
        position.y += rolling_speed * speed_scale;
        if position.y > board_length / 2.0 {
            position.y = -board_length / 2.0;
        }

        entity::set_component(board, translation(), position);
        for block in &blocks {
            let mut position = entity::get_component::<Vec3>(*block, translation()).unwrap_or_default();
            position.y += rolling_speed * speed_scale;

            if position.y > board_length / 2.0 {
                position.y = -board_length / 2.0;
            }
            entity::set_component(*block, translation(), position);
        }

        EventOk
    });

    println!(
"[glicol_code]~osc1: saw ~freq >> mul 0.1;
~osc2: saw ~freq2 >> mul 0.1;
~mod: sin 1 >> mul 300 >> add 500
o: mix ~osc1 ~osc2 >> lpf ~mod 3.0 >> mul ~env >> plate 0.1;
~freq: sig 100;
~freq2: ~freq >> add 1;
~env: sig 0 >> adsr 0.01 0.01 0.9 0.1"
);
    EventOk
}

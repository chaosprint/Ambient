// A synth hero game.

use ambient_api::{
    components::core::{
        app::main_scene,
        camera::aspect_ratio_from_window,
        primitives::{cube, quad},
        player::player,
        rendering::{ pbr_material_from_url, color, cast_shadows, outline},
        transform::{lookat_center, scale, translation},
    },
    concepts::{make_perspective_infinite_reverse_camera, make_transformable},
    prelude::*,
    player::KeyCode,
};
use components::cell;
use palette::{FromColor, Hsl, Srgb};

use crate::components::{grid_side_length, grid_x, grid_y};

#[main]
pub async fn main() -> EventResult {
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


    let n_rows = 30; // Number of rows of blocks.
    let n_cols = 3; // Number of columns of blocks.

    let block_width = board_width / n_cols as f32;
    let block_height = 0.2;
    let block_length = board_length / n_rows as f32;

    for row in 0..n_rows {
        for col in 0..n_cols {
            Entity::new()
                .with_merge(make_transformable())
                .with_default(cube())
                .with(
                    translation(),
                    vec3(
                        col as f32 * block_width - board_width / 2.0 + block_width / 2.0,
                        row as f32 * block_length - board_length / 2.0 + block_length / 2.0,
                        block_height / 2.0,
                    ),
                )
                .with(scale(), vec3(block_width * 0.9, block_length * 0.9, block_height))
                .with(color(), vec4(0.9, 0.9, 0.9, 0.1))
                .spawn();
        }
    };

    let mut cells = Vec::new();
    for x in 0..3 {
        let id = Entity::new()
            .with_merge(make_transformable())
            .with_default(cube())
            .with(translation(), vec3(x as f32 - 1., 3., 0.1))
            .with(scale(), vec3(0.9, 0.9, 0.3))
            .with(color(), vec4(0.2, 0.2, 0.2, 0.5))
            .spawn();
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
            }
            if keys.contains(&KeyCode::Right) {
                x = (x + 1) % 3;
            }
            if keys.contains(&KeyCode::Up)  {
                x = (x + 3 - 1) % 3;
            }
            if keys.contains(&KeyCode::Down)  {
                x = (x + 1) % 3;
            }
            let cell = x;

            let cell = if keys.contains(&KeyCode::Key1) {
                0
            } else if keys.contains(&KeyCode::Key2) {
                1
            } else if keys.contains(&KeyCode::Key3) {
                2
            } else {
                cell
            };

            if keys.contains(&KeyCode::Key3) {
                let cell = 2;
            }

            entity::add_component_if_required(cells[cell as usize], outline(), player_color);
            entity::set_component(player, components::cell(), cell);

            if keys.contains(&KeyCode::Space) {
                entity::set_component(cells[cell as usize], scale(), vec3(0.9, 0.9, 0.1));
                // entity::set_component(cells[cell as usize], color(), player_color);
                match cell {
                    0 => println!("[glicol_msg]~freq, 0, 0, 261.63; ~amp, 0, 0, 1"),
                    1 => println!("[glicol_msg]~freq, 0, 0, 329.63; ~amp, 0, 0, 1"),
                    2 => println!("[glicol_msg]~freq, 0, 0, 392.00; ~amp, 0, 0, 1"),
                    _ => (),
                }
            }
            if keys_released.contains(&KeyCode::Space) {
                entity::set_component(cells[cell as usize], scale(), vec3(0.9, 0.9, 0.3));
                // entity::set_component(cells[cell as usize], color(), vec4(0.2, 0.2, 0.2, 0.5));
                println!("[glicol_msg]~amp, 0, 0, 0");
                // match cell {
                //     0 => println!("~amp, 0, 0, 0"),
                //     1 => println!("~amp, 0, 0, 0"),
                //     2 => println!("~amp, 0, 0, 0"),
                //     _ => (),
                // }
            }
        }

        let mut position = entity::get_component::<Vec3>(board, translation()).unwrap_or_default();

        position.y += rolling_speed * 0.1;

        if position.y > board_length / 2.0 {
            position.y = -board_length / 2.0;
        }

        entity::set_component(board, translation(), position);

        EventOk
    });

    EventOk
}

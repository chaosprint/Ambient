use std::{collections::HashMap, net::SocketAddr, path::PathBuf, process::exit, sync::Arc, time::Duration};

use ambient_app::{fps_stats, window_title, AppBuilder};
use ambient_cameras::UICamera;
use ambient_core::{
    runtime,
    window::{cursor_position, window_ctl, window_logical_size, window_physical_size, window_scale_factor, WindowCtl},
};
use ambient_debugger::Debugger;
use ambient_ecs::{Entity, EntityId, SystemGroup};
use ambient_element::{element_component, Element, ElementComponentExt, Hooks};
use ambient_network::client::{
    GameClient, GameClientNetworkStats, GameClientRenderTarget, GameClientServerStats, GameClientView, GameClientWorld, UseOnce,
};
use ambient_std::{asset_cache::AssetCache, cb, friendly_id};
use ambient_ui::{Button, Dock, FlowColumn, FocusRoot, MeasureSize, ScrollArea, StylesExt, Text, UIExt, WindowSized, STREET};
use glam::{uvec2, vec4, Vec2};

use crate::{cli::RunCli, shared};
use ambient_ecs_editor::ECSEditor;
use ambient_layout::{docking, padding, width, Borders};

// use ambient_world_audio

use ambient_audio::{AudioStream};
use ambient_world_audio::{systems::setup_audio}; //audio_emitter, audio_listener, play_sound_on_entity,

pub mod player;
mod wasm;

/// Construct an app and enter the main client view
pub async fn run(assets: AssetCache, server_addr: SocketAddr, run: &RunCli, project_path: Option<PathBuf>) {
    let user_id = run.user_id.clone().unwrap_or_else(|| format!("user_{}", friendly_id()));
    let headless = if run.headless { Some(uvec2(600, 600)) } else { None };

    let is_debug = std::env::var("AMBIENT_DEBUGGER").is_ok() || run.debugger;

    let stream = AudioStream::new().unwrap();

    AppBuilder::new()
        .ui_renderer(true)
        .with_asset_cache(assets)
        .headless(headless)
        .update_title_with_fps_stats(false)
        .run(move |app, _runtime| {
            setup_audio(&mut app.world, stream.mixer().clone()).unwrap();
            app.systems.add(Box::new(ambient_world_audio::systems::spatial_audio_systems()));
            *app.world.resource_mut(window_title()) = "Ambient".to_string();
            MainApp { server_addr, user_id, show_debug: is_debug, golden_image_test: run.golden_image_test, project_path }
                .el()
                .spawn_interactive(&mut app.world);
        })
        .await;
}

#[element_component]
fn TitleUpdater(hooks: &mut Hooks) -> Element {
    let net = hooks.consume_context::<GameClientNetworkStats>().map(|stats| stats.0);
    let world = &hooks.world;
    let title = world.resource(window_title());
    let fps = world.get_cloned(hooks.world.resource_entity(), fps_stats()).ok().filter(|f| !f.fps().is_nan());

    let title = match (fps, net) {
        (None, None) => title.clone(),
        (Some(fps), None) => format!("{} [{}]", title, fps.dump_both()),
        (None, Some(net)) => format!("{} [{}]", title, net),
        (Some(fps), Some(net)) => format!("{} [{}, {}]", title, fps.dump_both(), net),
    };
    world.resource(window_ctl()).send(WindowCtl::SetTitle(title)).ok();

    Element::new()
}

#[element_component]
fn MainApp(
    hooks: &mut Hooks,
    server_addr: SocketAddr,
    project_path: Option<PathBuf>,
    user_id: String,
    show_debug: bool,
    golden_image_test: Option<f32>,
) -> Element {
    let update_network_stats = hooks.provide_context(GameClientNetworkStats::default);
    let update_server_stats = hooks.provide_context(GameClientServerStats::default);
    let (loaded, set_loaded) = hooks.use_state(false);

    FocusRoot::el([
        UICamera.el(),
        player::PlayerRawInputHandler.el(),
        TitleUpdater.el(),
        WindowSized::el([GameClientView {
            server_addr,
            user_id,
            on_disconnect: cb(move || {}),
            init_world: cb(UseOnce::new(Box::new(move |world, _render_target| {
                wasm::initialize(world).unwrap();

                UICamera.el().spawn_static(world);
            }))),
            on_loaded: cb(move |_game_state, _game_client| {
                set_loaded(true);
                Ok(Box::new(|| {}))
            }),
            error_view: cb(move |error| Dock(vec![Text::el("Error").header_style(), Text::el(error)]).el()),
            on_network_stats: cb(move |stats| update_network_stats(stats)),
            on_server_stats: cb(move |stats| update_server_stats(stats)),
            systems_and_resources: cb(|| {
                let mut resources = Entity::new();

                let bistream_handlers = HashMap::new();
                resources.set(ambient_network::client::bi_stream_handlers(), bistream_handlers);

                let unistream_handlers = HashMap::new();
                resources.set(ambient_network::client::uni_stream_handlers(), unistream_handlers);

                let dgram_handlers = HashMap::new();
                resources.set(ambient_network::client::datagram_handlers(), dgram_handlers);

                (systems(), resources)
            }),
            create_rpc_registry: cb(shared::create_server_rpc_registry),
            on_in_entities: None,
            inner: Dock::el(vec![
                if let Some(seconds) = golden_image_test.filter(|_| loaded) {
                    GoldenImageTest::el(project_path, seconds)
                } else {
                    Element::new()
                },
                GameView { show_debug }.el(),
            ]),
        }
        .el()]),
    ])
}

#[element_component]
fn GoldenImageTest(hooks: &mut Hooks, project_path: Option<PathBuf>, seconds: f32) -> Element {
    let (render_target, _) = hooks.consume_context::<GameClientRenderTarget>().unwrap();
    let render_target_ref = hooks.use_ref_with(|_| render_target.clone());
    *render_target_ref.lock() = render_target.clone();
    let screenshot_path = project_path.unwrap_or(PathBuf::new()).join("screenshot.png");
    let (old_screnshot, _) = hooks.use_state_with(|_| {
        tracing::info!("Loading screenshot from {:?}", screenshot_path);
        Some(Arc::new(image::open(&screenshot_path).ok()?))
    });

    let rt = hooks.world.resource(runtime()).clone();
    // Check every 1 second if the golden image test matches
    hooks.use_interval_deps(Duration::from_secs_f32(1.), false, render_target.0.color_buffer.id, {
        let render_target = render_target.clone();
        move |_| {
            if let Some(old) = old_screnshot.clone() {
                let render_target = render_target.clone();
                rt.spawn(async move {
                    log::info!("Comparing new and old screenshots");
                    let new = render_target.0.color_buffer.reader().read_image().await.unwrap().into_rgba8();

                    let hasher = image_hasher::HasherConfig::new().to_hasher();

                    let hash1 = hasher.hash_image(&new);
                    let hash2 = hasher.hash_image(&*old);
                    let dist = hash1.dist(&hash2);
                    if dist <= 2 {
                        tracing::info!("Screenshots are identical, exiting");
                        exit(0);
                    } else {
                        tracing::info!("Screenshot differ, distance={dist}");
                    }
                });
            }
        }
    });
    hooks.use_spawn(move |world| {
        world.resource(runtime()).spawn(async move {
            tokio::time::sleep(Duration::from_secs_f32(seconds)).await;
            let render_target = render_target_ref.lock().clone();
            tracing::info!("Saving screenshot to {:?}", screenshot_path);
            let new = render_target.0.color_buffer.reader().read_image().await.unwrap().into_rgba8();
            tracing::info!("Screenshot saved");
            new.save(screenshot_path).unwrap();
            exit(1);
        });

        Box::new(|_| {})
    });
    Element::new()
}

#[element_component]
fn GameView(hooks: &mut Hooks, show_debug: bool) -> Element {
    let (state, _) = hooks.consume_context::<GameClient>().unwrap();
    let (render_target, _) = hooks.consume_context::<GameClientRenderTarget>().unwrap();
    let (show_ecs, set_show_ecs) = hooks.use_state(false);
    let (ecs_size, set_ecs_size) = hooks.use_state(Vec2::ZERO);

    const ECS_WIDTH: f32 = 600.;

    hooks.use_frame({
        let state = state.clone();
        let render_target = render_target.clone();
        move |world| {
            let mut state = state.game_state.lock();
            let scale_factor = *world.resource(window_scale_factor());
            let mut mouse_pos = *world.resource(cursor_position());
            mouse_pos.x -= ecs_size.x;
            state.world.set_if_changed(EntityId::resources(), cursor_position(), mouse_pos).unwrap();
            let size = uvec2(render_target.0.color_buffer.size.width, render_target.0.color_buffer.size.height);
            state
                .world
                .set_if_changed(EntityId::resources(), window_logical_size(), (size.as_vec2() / scale_factor as f32).as_uvec2())
                .unwrap();
            state.world.set_if_changed(EntityId::resources(), window_physical_size(), size).unwrap();
            state.world.set_if_changed(EntityId::resources(), window_scale_factor(), scale_factor).unwrap();
        }
    });

    Dock::el([
        if show_debug {
            MeasureSize::el(
                FlowColumn::el([
                    Button::new(if show_ecs { "\u{f137}" } else { "\u{f138}" }, move |_| set_show_ecs(!show_ecs))
                        .style(ambient_ui::ButtonStyle::Flat)
                        .toggled(show_ecs)
                        .el(),
                    if show_ecs {
                        ScrollArea::el(
                            ECSEditor {
                                get_world: cb({
                                    let state = state.clone();
                                    move |res| {
                                        let state = state.game_state.lock();
                                        res(&state.world)
                                    }
                                }),
                                on_change: cb(|_, _| {}),
                            }
                            .el(),
                        )
                        .with(width(), ECS_WIDTH)
                    } else {
                        Element::new()
                    },
                ])
                .with(docking(), ambient_layout::Docking::Left)
                .with_background(vec4(0., 0., 0., 1.))
                .with(padding(), Borders::even(STREET)),
                set_ecs_size,
            )
        } else {
            Element::new()
        },
        if show_debug {
            Debugger {
                get_state: cb(move |cb| {
                    let mut game_state = state.game_state.lock();
                    let game_state = &mut *game_state;
                    cb(&mut game_state.renderer, &render_target.0, &mut game_state.world);
                }),
            }
            .el()
            .with(docking(), ambient_layout::Docking::Bottom)
            .with(padding(), Borders::even(STREET))
        } else {
            Element::new()
        },
        if show_debug {
            Dock::el([GameClientWorld.el()])
                .with_background(vec4(0.2, 0.2, 0.2, 1.))
                .with(padding(), Borders { left: 1., top: 0., right: 0., bottom: 1. })
        } else {
            GameClientWorld.el()
        },
    ])
}

fn systems() -> SystemGroup {
    SystemGroup::new(
        "client",
        vec![
            Box::new(ambient_decals::client_systems()),
            Box::new(ambient_primitives::systems()),
            Box::new(ambient_sky::systems()),
            Box::new(ambient_water::systems()),
            Box::new(ambient_physics::client_systems()),
            Box::new(wasm::systems()),
            Box::new(player::systems_final()),
        ],
    )
}

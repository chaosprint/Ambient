use crate::{
    global::{EntityId},
    internal::{
        // component::{Component, Entity, SupportedValue, UntypedComponent},
        conversion::{IntoBindgen}, // FromBindgen,
        wit,
    },
    prelude::*
};

// pub use ambient_audio::AudioListener;
pub use wit::audio::AudioListener;

/// Load sound into world
pub fn load(url: impl AsRef<str>) {
    wit::audio::load(url.as_ref())
}

/// Play sound
pub fn play(entity: EntityId, index: u32) {
    wit::audio::play(entity.into_bindgen(), index)
}

/// Set listener
pub fn set_listener(entity: EntityId, transform: Mat4, ear_distance: Vec3) {
    wit::audio::set_listener(entity.into_bindgen(), AudioListener {
        transform: transform.into_bindgen(),
        ear_distance: ear_distance.into_bindgen(),
    });
}

/// Set listener
pub fn set_emitter(entity: EntityId) {
    wit::audio::set_emitter(entity.into_bindgen());
}
use crate::{
    global::{EntityId, Vec3},
    internal::{
        component::{Component, Entity, SupportedValue, UntypedComponent},
        conversion::{FromBindgen, IntoBindgen},
        wit,
    },
    prelude::block_until,
};

/// Spawn audio emitters (sound playing based on the url) on the world you call this.
pub fn add_sound(component: Component<Vec<f32>>, path: impl AsRef<str>) {
    wit::audio::add_sound(component.index(), path.as_ref())
}

/// Spawn audio emitters (sound playing based on the url) on the world you call this.
pub fn play_sound_on_entity(entity: EntityId, name: impl AsRef<str>) {
    wit::audio::play_sound_on_entity(entity.into_bindgen(), name.as_ref())
}

/// Make an audio listener to an entity as component.
pub fn listener() {
    wit::audio::listener();
}

/// Make an audio emitter to an entity as component.
pub fn emitter() {
    wit::audio::emitter();
}
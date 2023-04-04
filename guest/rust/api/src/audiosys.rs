use crate::{
    global::{EntityId, Vec3},
    internal::{
        component::{Component, Entity, SupportedValue, UntypedComponent},
        conversion::{FromBindgen, IntoBindgen},
        wit,
    },
    prelude::block_until,
};

pub use wit::entity::{AnimationAction, AnimationController};

/// POC audio system
pub fn init(code: String) -> u32 {
    wit::audiosys::init(&code).from_bindgen()
}

/// POC sample adding
pub fn add_sound(name: String, url: String) {
    wit::audiosys::add_sound(&name, &url).from_bindgen();
}
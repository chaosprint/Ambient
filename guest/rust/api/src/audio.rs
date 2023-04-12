use crate::{
    global::{EntityId, Vec3},
    internal::{
        component::{Component, Entity, SupportedValue, UntypedComponent},
        conversion::{FromBindgen, IntoBindgen},
        wit,
    },
    prelude::block_until,
};

pub use wit::audio::AudioListener;
// pub use wit::entity::{AnimationAction, AnimationController};

// /// load a sound, to world resources
// pub fn load(tracks: Vec<AudioTrack>) {
//     let audio_tracks = AudioTracks {
//         tracks
//     }
//     wit::audio::load(audio_tracks);
// }

use crate::internal::wit;

/// Load sound into world
pub fn load(url: impl AsRef<str>) {
    wit::audio::load(url.as_ref())
}

/// Play sound
pub fn play(index: u32) {
    wit::audio::play(index)
}

/// Set listener
pub fn set_listener(entity: EntityId, listener: AudioListener) {
    wit::audio::set_listener(entity, listener)
}
mod assets;
mod error;
mod mixer;
// mod sink;
mod stream;

mod barycentric;
pub mod blt;
/// Fast fourier transform
pub mod hrtf;
pub mod signal;
pub mod source;
mod spatial;
pub mod track;
pub mod utils;
pub mod value;
pub mod vorbis;
pub mod wav;

pub use assets::*;
pub use error::*;
pub use mixer::*;
// pub use sink::*;
pub use source::*;
pub use spatial::*;
pub use stream::*;

pub const MAX_CHANNELS: usize = 8;

pub type ChannelCount = u16;
pub type SampleRate = u64;
pub type Frame = glam::Vec2;

pub struct SoundPlayer {
    source: track::TrackDecodeStream,
    volumn: f32,
    looping: bool,
    audio_stream_ref: std::sync::Arc<AudioStream>,
}

impl SoundPlayer {
    pub fn new(source: track::TrackDecodeStream, audio_stream_ref: std::sync::Arc<AudioStream>) -> Self {
        Self {
            source,
            volumn: 1.0,
            looping: false,
            audio_stream_ref,
        }
    }

    pub fn volumn(mut self, volumn: f32) -> Self {
        self.volumn = volumn;
        self
    }

    pub fn looping(mut self, looping: bool) -> Self {
        self.looping = looping;
        self
    }

    pub fn play(self) {
        // let audio_stream_ref = Arc::clone(&audio_stream.audio_stream);
        // let _sound = (*self.audio_stream_ref).mixer().play(self.source);
    }
}
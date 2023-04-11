use crate::internal::wit;

/// Spawn audio emitters (sound playing based on the url) on the world you call this.
pub fn spawn_emitters() -> () {
    wit::audio::spawn_emitters(path: impl AsRef<str>)
}
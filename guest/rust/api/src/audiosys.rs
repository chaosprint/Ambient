use crate::internal::wit;

/// audio
pub fn add_sound(name: String, path: String) {
    wit::client_audiosys::add_sound(&name, &path)
}
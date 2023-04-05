use crate::internal::wit;

/// Add sound to the world
pub fn add_sound(name: String, path: String) {
    wit::client_audiosys::add_sound(&name, &path)
}

/// Get sound from the world
pub fn get_sound(name: String) {
    wit::client_audiosys::get_sound(&name)
}
use crate::shared::{implementation::unsupported, wit};

use super::Bindings;

impl wit::client_audiosys::Host for Bindings {
    fn add_sound(
        &mut self,
        _name: String,
        _url: String,
    ) -> anyhow::Result<()> {
        unsupported()
    }
    fn play_sound(
        &mut self,
        _name: String
    ) -> anyhow::Result<()> {
        unsupported()
    }
}

impl wit::client_message::Host for Bindings {
    fn send(
        &mut self,
        _: wit::client_message::Target,
        _: String,
        _: Vec<u8>,
    ) -> anyhow::Result<()> {
        unsupported()
    }
}
impl wit::client_player::Host for Bindings {
    fn get_raw_input(&mut self) -> anyhow::Result<wit::client_player::RawInput> {
        unsupported()
    }

    fn get_prev_raw_input(&mut self) -> anyhow::Result<wit::client_player::RawInput> {
        unsupported()
    }
}

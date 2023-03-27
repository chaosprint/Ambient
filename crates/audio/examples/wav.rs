use ambient_sys::time::Instant;

use ambient_audio::{track::Track, AudioStream, Source};

fn main() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/amen_break.wav");
    let track = Track::from_wav(
        std::fs::read(path)
            .unwrap()
            .to_vec(),
    )
    .unwrap();

    let stream = AudioStream::new().unwrap();

    let source = track.decode();
    eprintln!("Duration: {:?}", source.duration());
    let sound = stream.mixer().play(source);
    let now = Instant::now();
    sound.wait_blocking();
    eprintln!("Elapsed: {:?}", now.elapsed());
}

use std::time::Duration;

use ambient_audio::{track::Track, AudioStream, Source};
use ambient_std::time::IntoDuration;
use rand::{seq::SliceRandom, thread_rng};

fn main() -> color_eyre::Result<()> {
    let stream = AudioStream::new()?;

    let path1 = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/amen_break.wav");
    let track1 = Track::from_wav(
        std::fs::read(path1)
            .unwrap()
            .to_vec(),
    )
    .unwrap();

    let path2 = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/dun_dun_dun.wav");
    let track2 = Track::from_wav(
        std::fs::read(path2)
            .unwrap()
            .to_vec(),
    )
    .unwrap();

    let sources = vec![track1, track2];

    let mut rng = thread_rng();
    for _ in 0..5 {
        let a = sources
            .choose(&mut rng)
            .unwrap()
            .decode()
            .take(Duration::from_secs(5));

        let b = sources
            .choose(&mut rng)
            .unwrap()
            .decode()
            .take(Duration::from_secs(5));

        let c = sources
            .choose(&mut rng)
            .unwrap()
            .decode()
            .take(Duration::from_secs(5));

        let source = a.crossfade(b, 200.ms()).crossfade(c, 200.ms());

        eprintln!("Source is {:?} in duration", source.duration());

        stream.mixer().play(source).wait_blocking();
    }

    Ok(())
}

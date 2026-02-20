use rodio::source::SineWave;
use rodio::{OutputStreamBuilder, Sink, Source};
use std::time::Duration;

pub struct Audio {
    sink: Sink,
    playing: bool,
    // keep the stream alive for the lifetime of Audio
    _stream: rodio::OutputStream,
}

impl Audio {
    pub fn new(frequency: f32, volume: f32) -> Self {
        let stream = OutputStreamBuilder::open_default_stream().expect("open default audio stream");

        let sink = Sink::connect_new(&stream.mixer());

        let source = SineWave::new(frequency)
            .take_duration(Duration::from_secs(3600))
            .amplify(volume);

        sink.append(source);
        sink.pause();

        Self {
            sink,
            playing: false,
            _stream: stream,
        }
    }

    pub fn update(&mut self, sound_timer: u8) {
        if sound_timer > 0 {
            if !self.playing {
                self.sink.play();
                self.playing = true;
            }
        } else if self.playing {
            self.sink.pause();
            self.playing = false;
        }
    }
}

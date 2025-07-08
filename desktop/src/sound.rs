use rodio::{OutputStream, OutputStreamHandle, Sink, source::SineWave};

pub struct Sound {
    _stream: OutputStream,
    handle: OutputStreamHandle,
    beep_sink: Option<Sink>,
}

impl Sound {
    pub fn new() -> Self {
        let (stream, handle) = OutputStream::try_default().unwrap();
        Sound {
            _stream: stream,
            handle,
            beep_sink: None,
        }
    }

    pub fn start_beep(&mut self) {
        if self.beep_sink.is_none() || self.beep_sink.as_ref().unwrap().empty() {
            let sink = Sink::try_new(&self.handle).unwrap();
            sink.append(SineWave::new(440.0));
            sink.play();
            self.beep_sink = Some(sink);
        }
    }

    pub fn stop_beep(&mut self) {
        if let Some(sink) = &self.beep_sink {
            sink.stop();
        }
        self.beep_sink = None;
    }
}

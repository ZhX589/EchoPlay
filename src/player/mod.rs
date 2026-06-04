pub mod playlist;
pub mod state;

use std::io::BufReader;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use rodio::{Decoder, OutputStream, Sink};

use crate::error::EchoError;
use crate::media::Media;
use crate::player::state::{PlayerInfo, PlayerState};

/// Audio player wrapping rodio.
///
/// Runs in its own thread (rodio requirement). Playback state is shared
/// via `Arc<Mutex<...>>`.
pub struct Player {
    state: Arc<Mutex<PlayerState>>,
    info: Arc<Mutex<PlayerInfo>>,
    sink: Arc<Mutex<Sink>>,
    // Keep the stream alive — dropping it stops audio output.
    _stream: OutputStream,
    _stream_handle: rodio::OutputStreamHandle,
}

impl Player {
    /// Create a new player with default audio output.
    pub fn new() -> Result<Self, EchoError> {
        let (stream, stream_handle) =
            OutputStream::try_default().map_err(|e| EchoError::ProviderIO {
                message: "failed to open audio output".into(),
                source: Some(anyhow::anyhow!(e)),
            })?;

        let sink = Sink::try_new(&stream_handle).map_err(|e| EchoError::ProviderIO {
            message: "failed to create audio sink".into(),
            source: Some(anyhow::anyhow!(e)),
        })?;

        Ok(Self {
            state: Arc::new(Mutex::new(PlayerState::Stopped)),
            info: Arc::new(Mutex::new(PlayerInfo::default())),
            sink: Arc::new(Mutex::new(sink)),
            _stream: stream,
            _stream_handle: stream_handle,
        })
    }

    /// Play a media source (local file path or URL).
    pub fn play(&self, media: &Media) -> Result<(), EchoError> {
        let sink = self.sink.lock().unwrap();
        sink.stop();

        let source = self.load_source(media)?;
        sink.append(source);
        sink.play();

        *self.state.lock().unwrap() = PlayerState::Playing;
        *self.info.lock().unwrap() = PlayerInfo::default();
        Ok(())
    }

    /// Pause playback.
    pub fn pause(&self) {
        let sink = self.sink.lock().unwrap();
        if !sink.is_paused() {
            sink.pause();
            *self.state.lock().unwrap() = PlayerState::Paused;
        }
    }

    /// Resume playback after pause.
    pub fn resume(&self) {
        let sink = self.sink.lock().unwrap();
        if sink.is_paused() {
            sink.play();
            *self.state.lock().unwrap() = PlayerState::Playing;
        }
    }

    /// Toggle between play and pause.
    pub fn toggle(&self) {
        let state = *self.state.lock().unwrap();
        match state {
            PlayerState::Playing => self.pause(),
            PlayerState::Paused => self.resume(),
            PlayerState::Stopped => {}
        }
    }

    /// Stop playback and clear the queue.
    pub fn stop(&self) {
        let sink = self.sink.lock().unwrap();
        sink.stop();
        *self.state.lock().unwrap() = PlayerState::Stopped;
        *self.info.lock().unwrap() = PlayerInfo::default();
    }

    /// Set volume (0.0 to 1.0).
    pub fn set_volume(&self, volume: f32) {
        let sink = self.sink.lock().unwrap();
        sink.set_volume(volume.clamp(0.0, 1.0));
    }

    /// Get current volume.
    pub fn volume(&self) -> f32 {
        let sink = self.sink.lock().unwrap();
        sink.volume()
    }

    /// Seek forward by the given duration.
    pub fn seek_forward(&self, duration: Duration) {
        let sink = self.sink.lock().unwrap();
        if let Err(e) = sink.try_seek(duration) {
            tracing::warn!("seek failed: {}", e);
        }
    }

    /// Check if playback has finished (queue empty and not paused).
    pub fn is_finished(&self) -> bool {
        let sink = self.sink.lock().unwrap();
        sink.empty()
    }

    /// Get current player state.
    pub fn state(&self) -> PlayerState {
        *self.state.lock().unwrap()
    }

    /// Get current player info.
    pub fn info(&self) -> PlayerInfo {
        *self.info.lock().unwrap()
    }

    /// Update state to Stopped if playback finished.
    pub fn tick(&self) {
        let state = *self.state.lock().unwrap();
        if state == PlayerState::Playing && self.is_finished() {
            *self.state.lock().unwrap() = PlayerState::Stopped;
        }
    }

    fn load_source(&self, media: &Media) -> Result<Decoder<BufReader<std::fs::File>>, EchoError> {
        let file = std::fs::File::open(&media.url).map_err(|e| EchoError::ProviderIO {
            message: format!("failed to open audio file: {}", media.url),
            source: Some(anyhow::anyhow!(e)),
        })?;
        let reader = BufReader::new(file);
        Decoder::new(reader).map_err(|e| EchoError::ProviderIO {
            message: format!("failed to decode audio: {}", media.url),
            source: Some(anyhow::anyhow!(e)),
        })
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::new().expect("failed to create default audio player")
    }
}

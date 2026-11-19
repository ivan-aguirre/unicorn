//! Sons do jogo: sintetizados uma vez na inicialização e tocados conforme os eventos.

pub mod synth;

use macroquad::audio::{PlaySoundParams, Sound, load_sound_from_bytes, play_sound};

use crate::game::{Events, Mode};

/// Efeitos sonoros carregados.
pub struct Sfx {
    jump: Sound,
    heart: Sound,
    celebrate: Sound,
    pause: Sound,
    resume: Sound,
}

impl Sfx {
    /// Sintetiza e carrega os sons. Sem dispositivo de áudio, o jogo segue mudo.
    pub async fn load() -> Option<Self> {
        Some(Self {
            jump: load(synth::JUMP).await?,
            heart: load(synth::HEART).await?,
            celebrate: load(synth::CELEBRATE).await?,
            pause: load(synth::PAUSE).await?,
            resume: load(synth::RESUME).await?,
        })
    }

    /// Toca os sons dos eventos do quadro (a festa tem prioridade sobre o coração).
    pub fn play(&self, events: Events) {
        if events.celebrated {
            once(&self.celebrate);
        } else if events.hearts > 0 {
            once(&self.heart);
        }
        if events.jumped {
            once(&self.jump);
        }
        match events.mode_changed {
            Some(Mode::Paused) => once(&self.pause),
            Some(Mode::Playing) => once(&self.resume),
            None => {}
        }
    }
}

async fn load(tones: &[synth::Tone]) -> Option<Sound> {
    load_sound_from_bytes(&synth::wav(&synth::render(tones)))
        .await
        .ok()
}

fn once(sound: &Sound) {
    play_sound(
        sound,
        PlaySoundParams {
            looped: false,
            volume: 1.0,
        },
    );
}

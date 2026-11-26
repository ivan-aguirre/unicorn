//! Sons do jogo: sintetizados uma vez na inicialização e tocados conforme os eventos (do
//! passeio e do modo Letras).

pub mod synth;

use macroquad::audio::{PlaySoundParams, Sound, load_sound_from_bytes, play_sound};

use crate::game::{Events, Mode};
use crate::letters::{LetterEvents, Press};

/// Efeitos sonoros carregados.
pub struct Sfx {
    jump: Sound,
    heart: Sound,
    celebrate: Sound,
    pause: Sound,
    resume: Sound,
    pop: Sound,
    miss: Sound,
}

impl Sfx {
    /// Sintetiza e carrega os sons. Sem dispositivo de áudio, o jogo segue mudo.
    pub async fn load() -> Option<Self> {
        let all = [
            synth::JUMP,
            synth::HEART,
            synth::CELEBRATE,
            synth::PAUSE,
            synth::RESUME,
            synth::POP,
            synth::MISS,
        ];
        let mut sounds = Vec::with_capacity(all.len());
        for tones in all {
            sounds.push(load(tones).await?);
        }
        let [jump, heart, celebrate, pause, resume, pop, miss] = sounds.try_into().ok()?;
        Some(Self {
            jump,
            heart,
            celebrate,
            pause,
            resume,
            pop,
            miss,
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

    /// Toca os sons do modo Letras: letra certa ou tecla errada e, por cima, a festa.
    pub fn play_letters(&self, events: LetterEvents) {
        match events.press {
            Some(Press::Hit) => once(&self.pop),
            Some(Press::Missed) => once(&self.miss),
            None => {}
        }
        if events.celebrated {
            once(&self.celebrate);
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

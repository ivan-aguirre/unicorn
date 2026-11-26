//! Modo "Letras": um desenho e a palavra dele aparecem, e a criança digita a mesma palavra.
//!
//! Um teclado desenhado na tela acende a tecla da próxima letra. Tecla errada não pune: a tecla
//! apertada só pisca em cinza. A palavra completa vale uma estrela e a festa do arco-íris; depois
//! da festa vem a próxima palavra. Puro: testável sem janela.

pub mod keyboard;
pub mod words;

use fastrand::Rng;

use crate::effects::Effects;
use crate::input::Input;
use crate::level::Level;
use crate::player::Player;
use crate::view::VIEW_W;
use words::WordTask;

/// Duração do brilho de uma tecla apertada (certa ou errada).
pub const FLASH_TIME: f32 = 0.35;
/// Onde o unicórnio fica (à esquerda do teclado).
const PLAYER_X: f32 = 66.0;

/// Como uma tecla deve aparecer no teclado desenhado.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyState {
    Normal,
    /// A tecla da próxima letra da palavra: acesa.
    Target,
    /// Acabou de ser apertada certa: afundada e acesa.
    Hit,
    /// Acabou de ser apertada errada: cinza.
    Miss,
}

/// Uma tecla que acabou de ser apertada.
#[derive(Debug, Clone, Copy, PartialEq)]
struct KeyFlash {
    key: char,
    hit: bool,
    timer: f32,
}

/// Resultado de uma tecla apertada com uma letra para digitar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Press {
    /// Tecla certa: a letra entrou na palavra.
    Hit,
    /// Tecla errada.
    Missed,
}

/// O que aconteceu num passo (para tocar sons).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct LetterEvents {
    pub press: Option<Press>,
    /// A palavra ficou completa: festa.
    pub celebrated: bool,
}

impl LetterEvents {
    /// Junta os eventos de vários passos do mesmo quadro (um acerto vale mais que um erro).
    #[must_use]
    pub const fn merge(self, other: Self) -> Self {
        let press = match (self.press, other.press) {
            (Some(Press::Hit), _) | (_, Some(Press::Hit)) => Some(Press::Hit),
            (Some(Press::Missed), _) | (_, Some(Press::Missed)) => Some(Press::Missed),
            (None, None) => None,
        };
        Self {
            press,
            celebrated: self.celebrated || other.celebrated,
        }
    }

    /// Acertou a tecla.
    #[must_use]
    pub const fn hit(self) -> bool {
        matches!(self.press, Some(Press::Hit))
    }
}

/// Estado do modo Letras.
#[derive(Debug)]
pub struct Letters {
    pub level: Level,
    pub player: Player,
    pub effects: Effects,
    /// A palavra da vez e quanto dela já foi digitado.
    pub words: WordTask,
    flashes: Vec<KeyFlash>,
    stars: u32,
    time: f32,
    rng: Rng,
}

impl Letters {
    /// Novo jogo de palavras, com o unicórnio parado na beira do lago.
    #[must_use]
    pub fn new(level: Level, mut rng: Rng) -> Self {
        let player = Player::spawn(&level, PLAYER_X);
        let effects = Effects::new(rng.fork());
        let words = WordTask::new(&mut rng);
        Self {
            level,
            player,
            effects,
            words,
            flashes: Vec::new(),
            stars: 0,
            time: 0.0,
            rng,
        }
    }

    /// Palavras completas desde o começo (uma estrela cada).
    #[must_use]
    pub const fn stars(&self) -> u32 {
        self.stars
    }

    /// Tempo de jogo (anima o cenário).
    #[must_use]
    pub const fn time(&self) -> f32 {
        self.time
    }

    /// A letra a digitar agora (nenhuma durante a festa).
    #[must_use]
    pub fn target(&self) -> Option<char> {
        self.words.next_letter()
    }

    /// Como a tecla `key` aparece agora.
    #[must_use]
    pub fn key_state(&self, key: char) -> KeyState {
        match self.flashes.iter().rev().find(|f| f.key == key) {
            Some(f) if f.hit => KeyState::Hit,
            Some(_) => KeyState::Miss,
            None if self.target() == Some(key) => KeyState::Target,
            None => KeyState::Normal,
        }
    }

    /// Um passo de `dt` segundos com as letras digitadas nele (já normalizadas, ver
    /// [`keyboard::normalize`]).
    pub fn step(&mut self, dt: f32, typed: &[char]) -> LetterEvents {
        self.time += dt;
        self.player.update(Input::default(), &self.level, dt);
        self.effects.update(dt);
        self.age_flashes(dt);
        self.words.update(dt, &mut self.rng);
        typed.iter().fold(LetterEvents::default(), |events, &c| {
            events.merge(self.press(c))
        })
    }

    fn age_flashes(&mut self, dt: f32) {
        for f in &mut self.flashes {
            f.timer -= dt;
        }
        self.flashes.retain(|f| f.timer > 0.0);
    }

    fn press(&mut self, key: char) -> LetterEvents {
        let target = self.target();
        if target == Some(key) {
            return self.type_letter(key);
        }
        self.flash(key, false);
        LetterEvents {
            press: target.map(|_| Press::Missed),
            ..LetterEvents::default()
        }
    }

    fn flash(&mut self, key: char, hit: bool) {
        self.flashes.retain(|f| f.key != key);
        self.flashes.push(KeyFlash {
            key,
            hit,
            timer: FLASH_TIME,
        });
    }

    /// Letra certa: ela entra no espaço dela; com a palavra completa, estrela e festa.
    fn type_letter(&mut self, key: char) -> LetterEvents {
        self.flash(key, true);
        let celebrated = self.words.advance_letter();
        if celebrated {
            self.stars += 1;
            self.player.cheer();
            self.effects.celebrate(0.0, VIEW_W);
        }
        LetterEvents {
            press: Some(Press::Hit),
            celebrated,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clock::STEP;
    use crate::level::SEED;

    fn letters(seed: u64) -> Letters {
        Letters::new(Level::generate(SEED), Rng::with_seed(seed))
    }

    /// Digita a palavra letra por letra: só a última dá festa.
    fn type_word(l: &mut Letters, word: &str) {
        let last = word.len() - 1;
        for (i, c) in word.chars().enumerate() {
            assert_eq!(l.target(), Some(c));
            let e = l.step(STEP, &[c]);
            assert_eq!(e.press, Some(Press::Hit));
            assert_eq!(e.celebrated, i == last, "festa só no fim");
        }
    }

    #[test]
    fn comeca_com_uma_palavra_e_a_tecla_da_primeira_letra_acesa() {
        let l = letters(1);
        let first = l.words.word().text.chars().next().unwrap();
        assert_eq!(l.target(), Some(first));
        assert_eq!(l.key_state(first), KeyState::Target);
        assert_eq!(l.words.typed(), 0);
        assert_eq!(l.stars(), 0);
    }

    #[test]
    fn digitar_a_palavra_inteira_da_festa_e_estrela() {
        let mut l = letters(2);
        let word = l.words.word().text;
        type_word(&mut l, word);
        assert_eq!(l.stars(), 1);
        assert!(l.effects.celebration() > 0.0);
        assert_eq!(l.target(), None, "nada a digitar durante a festa");
        assert_eq!(l.step(STEP, &['A']).press, None);
        for _ in 0..(120 * 5) {
            l.step(STEP, &[]);
        }
        assert_ne!(l.words.word().text, word, "veio outra palavra");
        assert_eq!(l.words.typed(), 0);
    }

    #[test]
    fn letra_errada_ou_fora_de_ordem_nao_conta() {
        let mut l = letters(3);
        let word = l.words.word().text;
        let second = word.chars().nth(1).unwrap();
        let wrong = if word.starts_with('Q') { 'W' } else { 'Q' };
        for c in [wrong, second]
            .into_iter()
            .filter(|c| !word.starts_with(*c))
        {
            let e = l.step(STEP, &[c]);
            assert_eq!(e.press, Some(Press::Missed));
            assert_eq!(l.key_state(c), KeyState::Miss);
        }
        assert_eq!(l.words.typed(), 0);
        assert_eq!(l.stars(), 0);
        for _ in 0..120 {
            l.step(STEP, &[]);
        }
        assert_eq!(l.key_state(wrong), KeyState::Normal, "o cinza passa");
    }

    #[test]
    fn letra_certa_entra_e_a_tecla_afunda() {
        let mut l = letters(4);
        let first = l.words.word().text.chars().next().unwrap();
        l.step(STEP, &[first]);
        assert_eq!(l.words.typed(), 1);
        assert_eq!(l.key_state(first), KeyState::Hit);
    }

    #[test]
    fn eventos_juntam_dando_preferencia_ao_acerto() {
        let a = LetterEvents {
            press: Some(Press::Hit),
            ..LetterEvents::default()
        };
        let b = LetterEvents {
            press: Some(Press::Missed),
            celebrated: true,
        };
        let m = a.merge(b);
        assert!(m.hit() && m.celebrated);
        assert_eq!(
            b.merge(a).press,
            Some(Press::Hit),
            "acerto vale mais que erro"
        );
        assert_eq!(b.merge(LetterEvents::default()).press, Some(Press::Missed));
    }
}

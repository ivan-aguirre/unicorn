//! Modo "Letras": bolhas com letras sobem do lago e a criança aperta a tecla para estourá-las.
//!
//! Um teclado desenhado na tela acende a tecla certa. Tecla errada não pune: a tecla apertada
//! só pisca em cinza. Puro: testável sem janela.

pub mod bubble;
pub mod keyboard;
pub mod progress;
pub mod words;

use fastrand::Rng;
use macroquad::math::Vec2;

use crate::effects::Effects;
use crate::input::Input;
use crate::level::Level;
use crate::level::scenery::range;
use crate::player::Player;
use crate::view::VIEW_W;
use bubble::Bubble;
use progress::{Progress, Stage};
use words::WordTask;

/// Bolhas estouradas que valem uma festa.
pub const STARS_PER_PARTY: u32 = 20;
/// Espera até a próxima bolha depois de um estouro.
const NEXT_DELAY: f32 = 0.9;
/// Espera até uma bolha que saiu pelo alto voltar.
const RETURN_DELAY: f32 = 1.5;
/// Duração do brilho de uma tecla apertada (certa ou errada).
pub const FLASH_TIME: f32 = 0.35;
/// Duração da animação de estouro.
pub const POP_TIME: f32 = 0.35;
/// Onde o unicórnio fica (à esquerda do teclado).
const PLAYER_X: f32 = 66.0;
/// Velocidades de subida das bolhas (multiplicam a normal), da mais lenta à mais rápida.
pub const SPEEDS: [f32; 5] = [0.5, 0.75, 1.0, 1.5, 2.0];
/// Velocidade inicial: a do meio (normal).
const DEFAULT_SPEED: usize = 2;
/// Quanto tempo o indicador de velocidade fica em destaque depois de uma mudança.
pub const SPEED_SHOWN: f32 = 1.5;
/// Faixa horizontal onde as bolhas surgem (acima do teclado, longe do unicórnio e do sol).
const BUBBLE_X: (f32, f32) = (200.0, 520.0);

/// Como uma tecla deve aparecer no teclado desenhado.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyState {
    Normal,
    /// A tecla da letra da bolha: acesa.
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

/// Bolha estourando: onde, qual letra e há quanto tempo.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pop {
    pub center: Vec2,
    pub letter: char,
    pub phase: f32,
    pub age: f32,
}

/// Resultado de uma tecla apertada com uma bolha no ar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Press {
    /// Tecla certa: a bolha estourou.
    Popped,
    /// Tecla errada.
    Missed,
}

/// O que aconteceu num passo (para tocar sons).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct LetterEvents {
    pub press: Option<Press>,
    pub word_done: bool,
    pub celebrated: bool,
}

impl LetterEvents {
    /// Junta os eventos de vários passos do mesmo quadro (um estouro vale mais que um erro).
    #[must_use]
    pub const fn merge(self, other: Self) -> Self {
        let press = match (self.press, other.press) {
            (Some(Press::Popped), _) | (_, Some(Press::Popped)) => Some(Press::Popped),
            (Some(Press::Missed), _) | (_, Some(Press::Missed)) => Some(Press::Missed),
            (None, None) => None,
        };
        Self {
            press,
            word_done: self.word_done || other.word_done,
            celebrated: self.celebrated || other.celebrated,
        }
    }

    /// A bolha estourou.
    #[must_use]
    pub const fn popped(self) -> bool {
        matches!(self.press, Some(Press::Popped))
    }
}

/// Estado do modo Letras.
#[derive(Debug)]
pub struct Letters {
    pub level: Level,
    pub player: Player,
    pub effects: Effects,
    pub progress: Progress,
    /// A palavra da vez, na última fase.
    pub words: Option<WordTask>,
    bubble: Option<Bubble>,
    wait: f32,
    /// Letra da bolha que saiu pelo alto (volta na próxima).
    returning: Option<char>,
    last: Option<char>,
    flashes: Vec<KeyFlash>,
    pop: Option<Pop>,
    stars: u32,
    /// Índice em [`SPEEDS`].
    speed: usize,
    /// Tempo restante de destaque do indicador de velocidade.
    speed_shown: f32,
    time: f32,
    rng: Rng,
}

impl Letters {
    /// Novo jogo de letras, com o unicórnio parado na beira do lago.
    #[must_use]
    pub fn new(level: Level, mut rng: Rng) -> Self {
        let player = Player::spawn(&level, PLAYER_X);
        let effects = Effects::new(rng.fork());
        Self {
            level,
            player,
            effects,
            progress: Progress::default(),
            words: None,
            bubble: None,
            wait: NEXT_DELAY,
            returning: None,
            last: None,
            flashes: Vec::new(),
            pop: None,
            stars: 0,
            speed: DEFAULT_SPEED,
            speed_shown: 0.0,
            time: 0.0,
            rng,
        }
    }

    /// A bolha subindo, se houver.
    #[must_use]
    pub const fn bubble(&self) -> Option<&Bubble> {
        self.bubble.as_ref()
    }

    /// A bolha estourando, se houver.
    #[must_use]
    pub const fn pop(&self) -> Option<&Pop> {
        self.pop.as_ref()
    }

    /// Nível de velocidade das bolhas: índice em [`SPEEDS`] (0 é o mais lento).
    #[must_use]
    pub const fn speed(&self) -> usize {
        self.speed
    }

    /// Tempo restante de destaque do indicador de velocidade (0 sem destaque).
    #[must_use]
    pub const fn speed_shown(&self) -> f32 {
        self.speed_shown
    }

    /// Bolhas mais rápidas (até o limite).
    pub const fn faster(&mut self) {
        if self.speed + 1 < SPEEDS.len() {
            self.speed += 1;
        }
        self.speed_shown = SPEED_SHOWN;
    }

    /// Bolhas mais lentas (até o limite).
    pub const fn slower(&mut self) {
        self.speed = self.speed.saturating_sub(1);
        self.speed_shown = SPEED_SHOWN;
    }

    /// Bolhas estouradas desde o começo.
    #[must_use]
    pub const fn stars(&self) -> u32 {
        self.stars
    }

    /// Tempo de jogo (anima o cenário).
    #[must_use]
    pub const fn time(&self) -> f32 {
        self.time
    }

    /// Como a tecla `key` aparece agora.
    #[must_use]
    pub fn key_state(&self, key: char) -> KeyState {
        match self.flashes.iter().rev().find(|f| f.key == key) {
            Some(f) if f.hit => KeyState::Hit,
            Some(_) => KeyState::Miss,
            None if self.bubble.is_some_and(|b| b.letter == key) => KeyState::Target,
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
        if let Some(w) = &mut self.words {
            w.update(dt, &mut self.rng);
        }
        let mut events = LetterEvents::default();
        for &c in typed {
            events = events.merge(self.press(c));
        }
        self.update_bubble(dt);
        events
    }

    fn age_flashes(&mut self, dt: f32) {
        self.speed_shown = (self.speed_shown - dt).max(0.0);
        for f in &mut self.flashes {
            f.timer -= dt;
        }
        self.flashes.retain(|f| f.timer > 0.0);
        if let Some(p) = &mut self.pop {
            p.age += dt;
        }
        self.pop = self.pop.filter(|p| p.age < POP_TIME);
    }

    fn press(&mut self, key: char) -> LetterEvents {
        let target = self.bubble.map(|b| b.letter);
        if target == Some(key) {
            return self.burst();
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

    /// Estoura a bolha: brilhos, estrela, progresso, palavra e, a cada tantas, festa.
    fn burst(&mut self) -> LetterEvents {
        let Some(b) = self.bubble.take() else {
            return LetterEvents::default();
        };
        let center = b.center();
        self.flash(b.letter, true);
        self.pop = Some(Pop {
            center,
            letter: b.letter,
            phase: b.phase,
            age: 0.0,
        });
        self.effects.sparkle(center);
        self.player.cheer();
        self.stars += 1;
        self.last = Some(b.letter);
        self.wait = NEXT_DELAY;
        self.progress.hit(b.letter);
        let word_done = self.advance_word();
        let celebrated = self.stars.is_multiple_of(STARS_PER_PARTY);
        if celebrated {
            self.effects.celebrate(0.0, VIEW_W);
        }
        LetterEvents {
            press: Some(Press::Popped),
            word_done,
            celebrated,
        }
    }

    /// Na última fase, conta a letra na palavra. Devolve `true` se completou a palavra.
    fn advance_word(&mut self) -> bool {
        if self.progress.stage() != Stage::Words {
            return false;
        }
        let Some(w) = &mut self.words else {
            self.words = Some(WordTask::new(&mut self.rng));
            return false;
        };
        let done = w.advance_letter();
        if done {
            self.effects.shower(0.0, VIEW_W);
        }
        done
    }

    fn update_bubble(&mut self, dt: f32) {
        if let Some(b) = &mut self.bubble {
            let speed = SPEEDS.get(self.speed).copied().unwrap_or(1.0);
            b.update(dt, speed);
            if b.gone() {
                self.returning = Some(b.letter);
                self.bubble = None;
                self.wait = RETURN_DELAY;
            }
            return;
        }
        self.wait -= dt;
        if self.wait <= 0.0 {
            self.spawn();
        }
    }

    fn spawn(&mut self) {
        let Some(letter) = self.returning.take().or_else(|| self.next_letter()) else {
            return;
        };
        let x = range(&mut self.rng, BUBBLE_X.0, BUBBLE_X.1).round();
        let phase = range(&mut self.rng, 0.0, std::f32::consts::TAU);
        self.bubble = Some(Bubble::new(letter, x, phase));
    }

    /// Próxima letra: a da palavra, na última fase; senão, sorteada entre as liberadas.
    fn next_letter(&mut self) -> Option<char> {
        if self.progress.stage() == Stage::Words {
            let rng = &mut self.rng;
            return self
                .words
                .get_or_insert_with(|| WordTask::new(rng))
                .next_letter();
        }
        Some(self.progress.pick(&mut self.rng, self.last))
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

    /// Espera a próxima bolha aparecer e devolve a letra dela.
    fn next_bubble(l: &mut Letters) -> char {
        for _ in 0..(120 * 5) {
            if let Some(b) = l.bubble() {
                return b.letter;
            }
            l.step(STEP, &[]);
        }
        panic!("nenhuma bolha apareceu");
    }

    #[test]
    fn primeira_bolha_e_uma_vogal_e_a_tecla_acende() {
        let mut l = letters(1);
        let c = next_bubble(&mut l);
        assert!("AEIOU".contains(c));
        assert_eq!(l.key_state(c), KeyState::Target);
        assert_eq!(l.key_state('Z'), KeyState::Normal);
    }

    #[test]
    fn tecla_certa_estoura_e_conta_estrela() {
        let mut l = letters(2);
        let c = next_bubble(&mut l);
        let e = l.step(STEP, &[c]);
        assert_eq!(e.press, Some(Press::Popped));
        assert_eq!(l.bubble(), None);
        assert_eq!(l.stars(), 1);
        assert_eq!(l.progress.hits(c), 1);
        assert_eq!(l.key_state(c), KeyState::Hit);
        assert!(l.pop().is_some());
        assert_ne!(l.effects.particles(), []);
    }

    #[test]
    fn tecla_errada_so_pisca_e_nao_muda_nada() {
        let mut l = letters(3);
        let c = next_bubble(&mut l);
        let wrong = if c == 'Q' { 'W' } else { 'Q' };
        let e = l.step(STEP, &[wrong]);
        assert_eq!(e.press, Some(Press::Missed));
        assert_eq!(l.bubble().map(|b| b.letter), Some(c));
        assert_eq!(l.stars(), 0);
        assert_eq!(l.key_state(wrong), KeyState::Miss);
        for _ in 0..120 {
            l.step(STEP, &[]);
        }
        assert_eq!(l.key_state(wrong), KeyState::Normal, "o cinza passa");
    }

    #[test]
    fn nunca_ha_mais_de_uma_bolha_e_a_que_sai_pelo_alto_volta() {
        let mut l = letters(4);
        let c = next_bubble(&mut l);
        let mut left = false;
        for _ in 0..(120 * 40) {
            l.step(STEP, &[]);
            left |= l.bubble().is_none();
            if left && l.bubble().is_some() {
                break;
            }
        }
        assert!(left, "a bolha deveria sair pelo alto");
        assert_eq!(l.bubble().map(|b| b.letter), Some(c), "a mesma letra volta");
    }

    #[test]
    fn velocidade_sobe_e_desce_dentro_dos_limites() {
        let mut l = letters(6);
        assert_eq!(l.speed(), DEFAULT_SPEED);
        assert!(
            (SPEEDS[DEFAULT_SPEED] - 1.0).abs() < f32::EPSILON,
            "começa na normal"
        );
        for _ in 0..10 {
            l.faster();
        }
        assert_eq!(l.speed(), SPEEDS.len() - 1);
        assert!(l.speed_shown() > 0.0);
        for _ in 0..10 {
            l.slower();
        }
        assert_eq!(l.speed(), 0);
        for _ in 0..(120 * 2) {
            l.step(STEP, &[]);
        }
        assert!(l.speed_shown().abs() < f32::EPSILON, "o destaque passa");
    }

    #[test]
    fn bolha_rapida_sobe_mais_que_a_lenta() {
        let rise = |faster: bool| {
            let mut l = letters(7);
            next_bubble(&mut l);
            let change: fn(&mut Letters) = if faster {
                Letters::faster
            } else {
                Letters::slower
            };
            for _ in 0..SPEEDS.len() {
                change(&mut l);
            }
            let start = l.bubble().unwrap().y;
            for _ in 0..120 {
                l.step(STEP, &[]);
            }
            start - l.bubble().unwrap().y
        };
        // A mais rápida (2x) contra a mais lenta (0,5x).
        assert!(rise(true) > rise(false) * 3.5);
    }

    #[test]
    fn festa_a_cada_vinte_estrelas() {
        let mut l = letters(5);
        let mut parties = 0;
        for _ in 0..(STARS_PER_PARTY * 2) {
            let c = next_bubble(&mut l);
            parties += u32::from(l.step(STEP, &[c]).celebrated);
        }
        assert_eq!(parties, 2);
        assert!(l.effects.celebration() > 0.0);
    }

    #[test]
    fn eventos_juntam_dando_preferencia_ao_estouro() {
        let a = LetterEvents {
            press: Some(Press::Popped),
            ..LetterEvents::default()
        };
        let b = LetterEvents {
            press: Some(Press::Missed),
            word_done: true,
            ..LetterEvents::default()
        };
        let m = a.merge(b);
        assert!(m.popped() && m.word_done && !m.celebrated);
        assert_eq!(
            b.merge(a).press,
            Some(Press::Popped),
            "estouro vale mais que erro"
        );
        assert_eq!(b.merge(LetterEvents::default()).press, Some(Press::Missed));
    }
}

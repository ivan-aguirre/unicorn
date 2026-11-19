//! Estado completo do jogo e um passo de simulação (sem desenho nem som: testável).

use fastrand::Rng;

use crate::camera::Camera;
use crate::effects::Effects;
use crate::hearts::{HeartCounter, Hearts};
use crate::input::Input;
use crate::level::Level;
use crate::player::Player;
use crate::view::VIEW_W;

/// Onde o unicórnio aparece.
const SPAWN_X: f32 = 96.0;

/// Jogando ou pausado.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Playing,
    Paused,
}

/// O que aconteceu num passo (para tocar sons, por exemplo).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Events {
    pub jumped: bool,
    pub hearts: u32,
    pub celebrated: bool,
    /// Novo modo, se Enter pausou ou retomou o jogo.
    pub mode_changed: Option<Mode>,
}

impl Events {
    /// Junta os eventos de vários passos do mesmo quadro.
    #[must_use]
    pub const fn merge(self, other: Self) -> Self {
        Self {
            jumped: self.jumped || other.jumped,
            hearts: self.hearts + other.hearts,
            celebrated: self.celebrated || other.celebrated,
            mode_changed: match other.mode_changed {
                Some(mode) => Some(mode),
                None => self.mode_changed,
            },
        }
    }
}

/// O jogo inteiro.
#[derive(Debug)]
pub struct Game {
    pub level: Level,
    pub player: Player,
    pub hearts: Hearts,
    pub counter: HeartCounter,
    pub effects: Effects,
    pub camera: Camera,
    pub mode: Mode,
    /// Tempo de jogo (não avança na pausa): anima o cenário.
    pub time: f32,
}

impl Game {
    /// Novo jogo no nível dado; `rng` sorteia os corações (diferentes a cada partida).
    #[must_use]
    pub fn new(level: Level, mut rng: Rng) -> Self {
        let player = Player::spawn(&level, SPAWN_X);
        let camera = Camera::new(player.pos.x, player.facing, level.length());
        let hearts = Hearts::new(&level, rng.fork());
        let effects = Effects::new(rng.fork());
        Self {
            level,
            player,
            hearts,
            counter: HeartCounter::default(),
            effects,
            camera,
            mode: Mode::Playing,
            time: 0.0,
        }
    }

    /// Um passo de simulação de `dt` segundos.
    pub fn step(&mut self, input: Input, dt: f32) -> Events {
        let mut events = Events::default();
        if input.pause.pressed {
            self.mode = match self.mode {
                Mode::Playing => Mode::Paused,
                Mode::Paused => Mode::Playing,
            };
            events.mode_changed = Some(self.mode);
        }
        if self.mode == Mode::Paused {
            return events;
        }
        self.time += dt;
        events.jumped = self.player.update(input, &self.level, dt);
        let length = self.level.length();
        self.camera
            .follow(self.player.pos.x, self.player.facing, length, dt);
        self.collect_hearts(&mut events);
        let ahead = self.player.facing.sign();
        self.hearts.update(dt, &self.level, self.camera.x, ahead);
        self.effects.update(dt);
        events
    }

    fn collect_hearts(&mut self, events: &mut Events) {
        for pos in self.hearts.collect(self.player.hitbox()) {
            events.hearts += 1;
            self.effects.sparkle(pos);
            if self.counter.add() {
                events.celebrated = true;
                self.effects.celebrate(self.camera.x, VIEW_W);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clock::STEP;
    use crate::input::Button;
    use crate::level::SEED;

    fn game() -> Game {
        Game::new(Level::generate(SEED), Rng::with_seed(2))
    }

    const PAUSE: Input = Input {
        right: false,
        pause: Button::TAPPED,
        ..Input::RIGHT
    };

    #[test]
    fn enter_pausa_e_retoma() {
        let mut g = game();
        assert_eq!(g.step(PAUSE, STEP).mode_changed, Some(Mode::Paused));
        assert_eq!(g.mode, Mode::Paused);
        assert_eq!(g.step(PAUSE, STEP).mode_changed, Some(Mode::Playing));
        assert_eq!(g.mode, Mode::Playing);
        assert_eq!(g.step(Input::default(), STEP).mode_changed, None);
    }

    #[test]
    fn pausado_nada_se_move() {
        let mut g = game();
        g.step(PAUSE, STEP);
        let (x, t) = (g.player.pos.x, g.time);
        let right = Input {
            jump: Button::TAPPED,
            ..Input::RIGHT
        };
        for _ in 0..120 {
            let e = g.step(right, STEP);
            assert!(!e.jumped);
        }
        assert!((g.player.pos.x - x).abs() < f32::EPSILON);
        assert!((g.time - t).abs() < f32::EPSILON);
    }

    #[test]
    fn juntar_eventos_soma_coracoes() {
        let a = Events {
            hearts: 2,
            jumped: true,
            ..Events::default()
        };
        let b = Events {
            hearts: 1,
            celebrated: true,
            ..Events::default()
        };
        let m = a.merge(b);
        assert_eq!(m.hearts, 3);
        assert!(m.jumped && m.celebrated);
        assert_eq!(m.mode_changed, None);
        let paused = Events {
            mode_changed: Some(Mode::Paused),
            ..Events::default()
        };
        assert_eq!(
            paused.merge(Events::default()).mode_changed,
            Some(Mode::Paused)
        );
    }

    #[test]
    fn camera_acompanha_o_unicornio() {
        let mut g = game();
        for _ in 0..(120 * 10) {
            g.step(Input::RIGHT, STEP);
        }
        let x = g.player.pos.x;
        assert!(x > 1000.0);
        assert!((g.camera.x..g.camera.x + VIEW_W).contains(&x));
    }
}

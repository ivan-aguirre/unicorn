//! O unicórnio: movimento, pulo, colisão com o chão e plataformas e pose de animação.
//!
//! A posição é a dos pés (centro na horizontal). Valores pensados para crianças pequenas:
//! velocidade moderada, pulo generoso, tolerância para pular logo após sair da beira
//! ("coyote time") e para apertar espaço um pouco antes de tocar o chão.

use macroquad::math::{Rect, Vec2};

use crate::animation::{Animator, Expression, Motion, Pose};
use crate::input::Input;
use crate::level::Level;

/// Meia largura da caixa de colisão.
pub const HALF_W: f32 = 16.0;
/// Altura da caixa de colisão.
pub const HEIGHT: f32 = 46.0;
/// Altura máxima do pulo (pés acima do ponto de partida).
pub const JUMP_HEIGHT: f32 = 106.0;
/// Gravidade, em px/s².
pub const GRAVITY: f32 = 1500.0;
/// Velocidade inicial do pulo: `sqrt(2 * GRAVITY * JUMP_HEIGHT)`.
const JUMP_SPEED: f32 = 563.915;
/// Velocidade máxima andando.
pub const MAX_SPEED: f32 = 185.0;
const GROUND_ACCEL: f32 = 1080.0;
const AIR_ACCEL: f32 = 800.0;
const FRICTION: f32 = 1500.0;
const MAX_FALL: f32 = 700.0;
/// Tempo, após sair de uma beira, em que ainda dá para pular.
const COYOTE_TIME: f32 = 0.1;
/// Tempo em que um toque em espaço fica guardado antes de tocar o chão.
const JUMP_BUFFER: f32 = 0.15;
/// Para onde o unicórnio olha.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    Left,
    Right,
}

impl Facing {
    /// -1 para a esquerda, 1 para a direita.
    #[must_use]
    pub const fn sign(self) -> f32 {
        match self {
            Self::Left => -1.0,
            Self::Right => 1.0,
        }
    }
}

/// Onde o unicórnio está apoiado.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Support {
    Air,
    Ground,
    Platform(usize),
}

/// Estado do unicórnio.
#[derive(Debug, Clone)]
pub struct Player {
    pub pos: Vec2,
    pub vel: Vec2,
    pub facing: Facing,
    support: Support,
    coyote: f32,
    jump_buffer: f32,
    anim: Animator,
}

impl Player {
    /// Unicórnio parado no chão em `x`.
    #[must_use]
    pub fn spawn(level: &Level, x: f32) -> Self {
        Self {
            pos: Vec2::new(x, level.terrain.height_at(x)),
            vel: Vec2::ZERO,
            facing: Facing::Right,
            support: Support::Ground,
            coyote: 0.0,
            jump_buffer: 0.0,
            anim: Animator::default(),
        }
    }

    /// Avança um passo de simulação. Devolve `true` se começou um pulo.
    pub fn update(&mut self, input: Input, level: &Level, dt: f32) -> bool {
        self.steer(input.axis(), dt);
        let jumped = self.try_jump(input, dt);
        self.move_horizontally(level, dt);
        if self.support == Support::Air {
            self.move_vertically(level, dt);
        }
        self.anim.tick(self.motion(), dt);
        jumped
    }

    /// Se está apoiado no chão ou numa plataforma.
    #[must_use]
    pub fn on_ground(&self) -> bool {
        self.support != Support::Air
    }

    /// Caixa de colisão (pés na base).
    #[must_use]
    pub fn hitbox(&self) -> Rect {
        Rect::new(
            self.pos.x - HALF_W,
            self.pos.y - HEIGHT,
            HALF_W * 2.0,
            HEIGHT,
        )
    }

    /// Pose do corpo para a animação.
    #[must_use]
    pub fn pose(&self) -> Pose {
        self.anim.pose(self.motion())
    }

    /// Expressão do rosto para a animação.
    #[must_use]
    pub fn expression(&self) -> Expression {
        self.anim.expression(self.motion())
    }

    /// Pegou um coração: carinha de alegria.
    pub const fn cheer(&mut self) {
        self.anim.cheer();
    }

    fn motion(&self) -> Motion {
        Motion {
            airborne: self.support == Support::Air,
            rising: self.vel.y < 0.0,
            speed: self.vel.x.abs(),
        }
    }

    /// Acelera na direção pedida ou freia por atrito; vira o rosto para onde anda.
    fn steer(&mut self, axis: f32, dt: f32) {
        if axis.abs() > 0.0 {
            self.facing = if axis < 0.0 {
                Facing::Left
            } else {
                Facing::Right
            };
            let accel = if self.on_ground() {
                GROUND_ACCEL
            } else {
                AIR_ACCEL
            };
            let target = axis * MAX_SPEED;
            self.vel.x = approach(self.vel.x, target, accel * dt);
        } else {
            self.vel.x = approach(self.vel.x, 0.0, FRICTION * dt);
        }
    }

    /// Pula se espaço foi apertado (há pouco) ou está segurado, e se ainda dá para pular.
    fn try_jump(&mut self, input: Input, dt: f32) -> bool {
        self.jump_buffer = if input.jump.pressed {
            JUMP_BUFFER
        } else {
            (self.jump_buffer - dt).max(0.0)
        };
        self.coyote = if self.on_ground() {
            COYOTE_TIME
        } else {
            (self.coyote - dt).max(0.0)
        };
        let wants = self.jump_buffer > 0.0 || (input.jump.held && self.on_ground());
        if !(wants && self.coyote > 0.0) {
            return false;
        }
        self.vel.y = -JUMP_SPEED;
        self.support = Support::Air;
        self.coyote = 0.0;
        self.jump_buffer = 0.0;
        true
    }

    /// Move na horizontal, segue o chão/plataforma e respeita as bordas do nível.
    fn move_horizontally(&mut self, level: &Level, dt: f32) {
        let x = self.vel.x.mul_add(dt, self.pos.x);
        let clamped = x.clamp(HALF_W, level.length() - HALF_W);
        if (clamped - x).abs() > 0.0 {
            self.vel.x = 0.0;
        }
        self.pos.x = clamped;
        match self.support {
            Support::Ground => self.pos.y = level.terrain.height_at(self.pos.x),
            Support::Platform(i) => self.follow_platform(level, i),
            Support::Air => {}
        }
    }

    /// Continua sobre a plataforma ou começa a cair ao sair da beira.
    fn follow_platform(&mut self, level: &Level, index: usize) {
        match level.platforms.get(index) {
            Some(p) if p.covers(self.pos.x) => self.pos.y = p.y,
            _ => {
                self.support = Support::Air;
                self.vel.y = 0.0;
            }
        }
    }

    /// Gravidade e aterrissagem (plataformas só de cima para baixo).
    fn move_vertically(&mut self, level: &Level, dt: f32) {
        self.vel.y = GRAVITY.mul_add(dt, self.vel.y).min(MAX_FALL);
        let from = self.pos.y;
        let to = self.vel.y.mul_add(dt, from);
        self.pos.y = to;
        if self.vel.y < 0.0 {
            return;
        }
        if let Some(i) = level.landing_platform(self.pos.x, from, to) {
            self.land(
                Support::Platform(i),
                level.platforms.get(i).map_or(to, |p| p.y),
            );
            return;
        }
        let ground = level.terrain.height_at(self.pos.x);
        if to >= ground {
            self.land(Support::Ground, ground);
        }
    }

    const fn land(&mut self, support: Support, y: f32) {
        self.anim.landed(self.vel.y);
        self.support = support;
        self.pos.y = y;
        self.vel.y = 0.0;
    }
}

/// Aproxima `value` de `target` em no máximo `delta`.
fn approach(value: f32, target: f32, delta: f32) -> f32 {
    if value < target {
        (value + delta).min(target)
    } else {
        (value - delta).max(target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clock::STEP;
    use crate::input::Button;
    use crate::level::{Platform, Scenery, Terrain};
    use fastrand::Rng;

    fn level_with(points: &[(f32, f32)], platforms: Vec<Platform>) -> Level {
        Level {
            terrain: Terrain::from_points(points.iter().map(|&(x, y)| Vec2::new(x, y)).collect()),
            platforms,
            scenery: Scenery::generate(&mut Rng::with_seed(1), 2000),
        }
    }

    fn flat() -> Level {
        level_with(&[(0.0, 160.0), (2000.0, 160.0)], Vec::new())
    }

    fn run(player: &mut Player, level: &Level, input: Input, seconds: f32) -> bool {
        let mut jumped = false;
        let mut first = input;
        for _ in 0..steps(seconds) {
            jumped |= player.update(first, level, STEP);
            first = first.without_edges();
        }
        jumped
    }

    fn steps(seconds: f32) -> usize {
        std::iter::successors(Some(0.0_f32), |t| Some(t + STEP))
            .take_while(|t| *t < seconds)
            .count()
    }

    const RIGHT: Input = Input::RIGHT;
    const JUMP: Input = Input {
        right: false,
        jump: Button {
            pressed: true,
            held: false,
        },
        ..Input::RIGHT
    };

    #[test]
    fn anda_para_a_direita_ate_a_velocidade_maxima() {
        let level = flat();
        let mut p = Player::spawn(&level, 100.0);
        run(&mut p, &level, RIGHT, 1.0);
        assert!((p.vel.x - MAX_SPEED).abs() < 1e-3);
        assert!(p.pos.x > 180.0);
        assert_eq!(p.facing, Facing::Right);
        assert!(matches!(p.pose(), Pose::Walk(_)));
    }

    #[test]
    fn para_ao_soltar_e_vira_para_a_esquerda() {
        let level = flat();
        let mut p = Player::spawn(&level, 500.0);
        run(&mut p, &level, Input::LEFT, 0.5);
        assert_eq!(p.facing, Facing::Left);
        assert!(p.vel.x < 0.0);
        run(&mut p, &level, Input::default(), 0.5);
        assert!(p.vel.x.abs() < f32::EPSILON);
        assert!(matches!(p.pose(), Pose::Idle(_)));
    }

    #[test]
    fn pulo_alcanca_a_altura_prometida_e_volta_ao_chao() {
        let level = flat();
        let mut p = Player::spawn(&level, 300.0);
        let mut apex = p.pos.y;
        assert!(p.update(JUMP, &level, STEP));
        assert_eq!(p.pose(), Pose::Jump);
        for _ in 0..steps(1.5) {
            p.update(Input::default(), &level, STEP);
            apex = apex.min(p.pos.y);
        }
        let height = 160.0 - apex;
        assert!(
            (height - JUMP_HEIGHT).abs() < JUMP_HEIGHT * 0.03,
            "altura do pulo = {height}"
        );
        assert!(p.on_ground());
        assert!((p.pos.y - 160.0).abs() < f32::EPSILON);
    }

    #[test]
    fn nao_pula_no_ar_sem_coyote_time() {
        let level = flat();
        let mut p = Player::spawn(&level, 300.0);
        run(&mut p, &level, JUMP, 0.3);
        let vy = p.vel.y;
        assert!(!p.update(JUMP, &level, STEP));
        assert!(p.vel.y > vy, "pulo duplo não deve acontecer");
    }

    #[test]
    fn segurar_espaco_continua_pulando() {
        let level = flat();
        let mut p = Player::spawn(&level, 300.0);
        let held = Input {
            jump: Button {
                pressed: false,
                held: true,
            },
            ..Input::default()
        };
        let mut jumps = 0;
        for _ in 0..steps(3.0) {
            jumps += u32::from(p.update(held, &level, STEP));
        }
        assert!(jumps >= 3, "pulos = {jumps}");
    }

    #[test]
    fn toque_pouco_antes_de_aterrissar_e_lembrado() {
        let level = flat();
        let mut p = Player::spawn(&level, 300.0);
        p.update(JUMP, &level, STEP);
        // Espera até estar quase no chão, descendo.
        while !(p.vel.y > 0.0 && 160.0 - p.pos.y < 6.0) {
            p.update(Input::default(), &level, STEP);
        }
        p.update(JUMP, &level, STEP);
        let mut jumped = false;
        for _ in 0..steps(0.1) {
            jumped |= p.update(Input::default(), &level, STEP);
        }
        assert!(jumped);
    }

    #[test]
    fn segue_aclives_e_declives() {
        let level = level_with(
            &[(0.0, 170.0), (100.0, 170.0), (200.0, 120.0), (400.0, 170.0)],
            Vec::new(),
        );
        let mut p = Player::spawn(&level, 50.0);
        for _ in 0..steps(3.0) {
            p.update(RIGHT, &level, STEP);
            assert!(p.on_ground(), "desgrudou do chão em x={}", p.pos.x);
            assert!((p.pos.y - level.terrain.height_at(p.pos.x)).abs() < 1e-3);
        }
        assert!(p.pos.x > 300.0);
    }

    #[test]
    fn aterrissa_na_plataforma_e_cai_ao_sair_da_beira() {
        let platform = Platform {
            x: 280.0,
            y: 116.0,
            w: 48.0,
        };
        let level = level_with(&[(0.0, 160.0), (2000.0, 160.0)], vec![platform]);
        let mut p = Player::spawn(&level, 300.0);
        p.update(JUMP, &level, STEP);
        run(&mut p, &level, Input::default(), 1.0);
        assert!(p.on_ground());
        assert!((p.pos.y - 116.0).abs() < f32::EPSILON, "y = {}", p.pos.y);
        run(&mut p, &level, RIGHT, 1.5);
        assert!(
            (p.pos.y - 160.0).abs() < f32::EPSILON,
            "deveria ter caído no chão"
        );
    }

    #[test]
    fn passa_pela_plataforma_vindo_de_baixo() {
        let platform = Platform {
            x: 280.0,
            y: 130.0,
            w: 48.0,
        };
        let level = level_with(&[(0.0, 160.0), (2000.0, 160.0)], vec![platform]);
        let mut p = Player::spawn(&level, 300.0);
        p.update(JUMP, &level, STEP);
        let mut apex = p.pos.y;
        for _ in 0..steps(0.4) {
            p.update(Input::default(), &level, STEP);
            apex = apex.min(p.pos.y);
        }
        assert!(apex < 130.0, "deveria atravessar a plataforma subindo");
    }

    #[test]
    fn nao_sai_pelas_bordas_do_nivel() {
        let level = flat();
        let mut p = Player::spawn(&level, 20.0);
        run(&mut p, &level, Input::LEFT, 2.0);
        assert!((p.pos.x - HALF_W).abs() < f32::EPSILON);
        run(&mut p, &level, RIGHT, 30.0);
        assert!((p.pos.x - (2000.0 - HALF_W)).abs() < f32::EPSILON);
    }

    #[test]
    fn velocidade_do_pulo_corresponde_a_altura() {
        let expected = (2.0 * GRAVITY * JUMP_HEIGHT).sqrt();
        assert!((JUMP_SPEED - expected).abs() < 0.01);
    }
}

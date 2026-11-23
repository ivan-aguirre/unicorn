//! Efeitos visuais leves: brilhos ao pegar coração ou estourar bolha, a chuva de estrelas da
//! palavra completa e a festa ao completar a meta.

use fastrand::Rng;
use macroquad::math::Vec2;

use crate::level::scenery::range;

/// Limite de partículas vivas (custo constante, sem alocações durante o jogo).
pub const MAX_PARTICLES: usize = 256;
/// Duração da festa da meta de corações.
pub const CELEBRATION_TIME: f32 = 3.5;
const SPARKLES_PER_HEART: u8 = 8;
const CELEBRATION_PARTICLES: u8 = 72;
/// Estrelas e brilhos ao completar uma palavra (uma festa menor, sem arco-íris).
const SHOWER_PARTICLES: u8 = 28;
const PARTICLE_GRAVITY: f32 = 270.0;

/// Aparência de uma partícula.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParticleKind {
    Sparkle,
    Heart,
    Star,
}

/// Partícula simples com velocidade e tempo de vida.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub life: f32,
    pub kind: ParticleKind,
}

/// Sistema de partículas e o relógio da festa.
#[derive(Debug)]
pub struct Effects {
    particles: Vec<Particle>,
    celebration: f32,
    rng: Rng,
}

impl Effects {
    /// Sistema vazio, com capacidade reservada de uma vez.
    #[must_use]
    pub fn new(rng: Rng) -> Self {
        Self {
            particles: Vec::with_capacity(MAX_PARTICLES),
            celebration: 0.0,
            rng,
        }
    }

    /// Anel de brilhos onde um coração foi pego.
    pub fn sparkle(&mut self, at: Vec2) {
        for i in 0..SPARKLES_PER_HEART {
            let angle = std::f32::consts::TAU * f32::from(i) / f32::from(SPARKLES_PER_HEART);
            let speed = range(&mut self.rng, 85.0, 135.0);
            let vel = Vec2::from_angle(angle) * speed;
            self.push(Particle {
                pos: at,
                vel,
                life: 0.45,
                kind: ParticleKind::Sparkle,
            });
        }
    }

    /// Chuva de corações e estrelas pela tela toda e o arco-íris da festa.
    pub fn celebrate(&mut self, view_x: f32, view_w: f32) {
        self.celebration = CELEBRATION_TIME;
        self.rain((view_x, view_w), CELEBRATION_PARTICLES, |i| {
            if i % 3 == 0 {
                ParticleKind::Star
            } else {
                ParticleKind::Heart
            }
        });
    }

    /// Chuva curta de estrelas e brilhos (palavra completa), sem o arco-íris da festa.
    pub fn shower(&mut self, view_x: f32, view_w: f32) {
        self.rain((view_x, view_w), SHOWER_PARTICLES, |i| {
            if i % 2 == 0 {
                ParticleKind::Star
            } else {
                ParticleKind::Sparkle
            }
        });
    }

    /// `count` partículas caindo do alto da tela `(x, largura)`, do tipo dado pelo índice.
    fn rain(&mut self, (view_x, view_w): (f32, f32), count: u8, kind: impl Fn(u8) -> ParticleKind) {
        for i in 0..count {
            let pos = Vec2::new(
                range(&mut self.rng, view_x, view_x + view_w),
                range(&mut self.rng, -100.0, -14.0),
            );
            let vel = Vec2::new(
                range(&mut self.rng, -50.0, 50.0),
                range(&mut self.rng, 16.0, 100.0),
            );
            let life = range(&mut self.rng, 2.0, CELEBRATION_TIME);
            self.push(Particle {
                pos,
                vel,
                life,
                kind: kind(i),
            });
        }
    }

    /// Move as partículas e remove as que acabaram.
    pub fn update(&mut self, dt: f32) {
        self.celebration = (self.celebration - dt).max(0.0);
        for p in &mut self.particles {
            p.vel.y = PARTICLE_GRAVITY.mul_add(dt * gravity_scale(p.kind), p.vel.y);
            p.pos += p.vel * dt;
            p.life -= dt;
        }
        self.particles.retain(|p| p.life > 0.0);
    }

    /// Partículas vivas.
    #[must_use]
    pub fn particles(&self) -> &[Particle] {
        &self.particles
    }

    /// Tempo restante da festa (0 quando não há festa).
    #[must_use]
    pub const fn celebration(&self) -> f32 {
        self.celebration
    }

    fn push(&mut self, particle: Particle) {
        if self.particles.len() < MAX_PARTICLES {
            self.particles.push(particle);
        }
    }
}

/// Brilhos quase não caem; corações e estrelas da festa caem devagar, flutuando.
const fn gravity_scale(kind: ParticleKind) -> f32 {
    match kind {
        ParticleKind::Sparkle => 0.2,
        ParticleKind::Heart | ParticleKind::Star => 0.15,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn effects() -> Effects {
        Effects::new(Rng::with_seed(4))
    }

    #[test]
    fn brilho_cria_anel_que_some_sozinho() {
        let mut fx = effects();
        fx.sparkle(Vec2::new(10.0, 10.0));
        assert_eq!(fx.particles().len(), usize::from(SPARKLES_PER_HEART));
        for _ in 0..60 {
            fx.update(1.0 / 60.0);
        }
        assert_eq!(fx.particles(), []);
    }

    #[test]
    fn festa_dura_um_tempo_e_termina() {
        let mut fx = effects();
        fx.celebrate(0.0, 384.0);
        assert!(fx.celebration() > 0.0);
        assert_ne!(fx.particles(), []);
        for _ in 0..(60 * 4) {
            fx.update(1.0 / 60.0);
        }
        assert!(fx.celebration().abs() < f32::EPSILON);
        assert_eq!(fx.particles(), []);
    }

    #[test]
    fn chuva_de_estrelas_nao_comeca_a_festa() {
        let mut fx = effects();
        fx.shower(100.0, 384.0);
        assert_eq!(fx.particles().len(), usize::from(SHOWER_PARTICLES));
        assert!(fx.celebration().abs() < f32::EPSILON);
        assert!(
            fx.particles()
                .iter()
                .all(|p| (100.0..=484.0).contains(&p.pos.x))
        );
        assert!(fx.particles().iter().any(|p| p.kind == ParticleKind::Star));
    }

    #[test]
    fn numero_de_particulas_e_limitado() {
        let mut fx = effects();
        for _ in 0..10 {
            fx.celebrate(0.0, 384.0);
            fx.sparkle(Vec2::ZERO);
        }
        assert_eq!(fx.particles().len(), MAX_PARTICLES);
    }
}

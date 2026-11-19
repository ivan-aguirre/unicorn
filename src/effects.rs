//! Efeitos visuais leves: brilhos ao pegar coração e a festa ao completar 100.

use fastrand::Rng;
use macroquad::math::Vec2;

use crate::level::scenery::range;

/// Limite de partículas vivas (custo constante, sem alocações durante o jogo).
pub const MAX_PARTICLES: usize = 256;
/// Duração da festa dos 100 corações.
pub const CELEBRATION_TIME: f32 = 3.5;
const SPARKLES_PER_HEART: u8 = 8;
const CELEBRATION_PARTICLES: u8 = 72;
const PARTICLE_GRAVITY: f32 = 160.0;

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
            let speed = range(&mut self.rng, 50.0, 80.0);
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
        for i in 0..CELEBRATION_PARTICLES {
            let kind = if i % 3 == 0 {
                ParticleKind::Star
            } else {
                ParticleKind::Heart
            };
            let pos = Vec2::new(
                range(&mut self.rng, view_x, view_x + view_w),
                range(&mut self.rng, -60.0, -8.0),
            );
            let vel = Vec2::new(
                range(&mut self.rng, -30.0, 30.0),
                range(&mut self.rng, 10.0, 60.0),
            );
            let life = range(&mut self.rng, 2.0, CELEBRATION_TIME);
            self.push(Particle {
                pos,
                vel,
                life,
                kind,
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
    fn numero_de_particulas_e_limitado() {
        let mut fx = effects();
        for _ in 0..10 {
            fx.celebrate(0.0, 384.0);
            fx.sparkle(Vec2::ZERO);
        }
        assert_eq!(fx.particles().len(), MAX_PARTICLES);
    }
}

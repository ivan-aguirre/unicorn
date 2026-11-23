//! A bolha com a letra: sobe devagar saindo do lago, balançando de leve.

use macroquad::math::Vec2;

/// Raio da bolha na tela virtual.
pub const RADIUS: f32 = 32.0;
/// Altura onde a bolha surge (no lago, logo acima do teclado).
pub const START_Y: f32 = 232.0;
/// Velocidade de subida normal (px/s): devagar, para dar tempo de achar a tecla.
pub const RISE_SPEED: f32 = 15.0;
/// Amplitude e velocidade do balanço lateral.
const SWAY: f32 = 10.0;
const SWAY_SPEED: f32 = 1.3;
/// Duração da animação de surgir (a bolha cresce).
pub const GROW_TIME: f32 = 0.5;

/// Uma bolha subindo.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bubble {
    pub letter: char,
    /// Posição horizontal do centro, sem o balanço.
    pub base_x: f32,
    /// Altura do centro.
    pub y: f32,
    /// Tempo desde que surgiu.
    pub age: f32,
    /// Fase do balanço e das cores do aro.
    pub phase: f32,
}

impl Bubble {
    /// Bolha nova no lago.
    #[must_use]
    pub const fn new(letter: char, base_x: f32, phase: f32) -> Self {
        Self {
            letter,
            base_x,
            y: START_Y,
            age: 0.0,
            phase,
        }
    }

    /// Centro atual (com o balanço).
    #[must_use]
    pub fn center(&self) -> Vec2 {
        let sway = self.age.mul_add(SWAY_SPEED, self.phase).sin() * SWAY;
        Vec2::new(self.base_x + sway, self.y)
    }

    /// Tamanho relativo (cresce ao surgir).
    #[must_use]
    pub fn scale(&self) -> f32 {
        let t = (self.age / GROW_TIME).min(1.0);
        // Cresce passando um pouco do tamanho e volta (efeito de "bolha").
        (t * std::f32::consts::PI).sin().mul_add(0.12, t)
    }

    /// Sobe `dt` segundos, a `speed` vezes a velocidade normal (o balanço não muda).
    pub fn update(&mut self, dt: f32, speed: f32) {
        self.age += dt;
        self.y = (RISE_SPEED * speed).mul_add(-dt, self.y);
    }

    /// Já saiu inteira pelo alto da tela.
    #[must_use]
    pub fn gone(&self) -> bool {
        self.y < -RADIUS
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sobe_e_sai_pelo_alto_depois_de_bastante_tempo() {
        let mut b = Bubble::new('A', 300.0, 0.0);
        let mut seconds = 0.0;
        while !b.gone() {
            b.update(0.1, 1.0);
            seconds += 0.1;
            assert!((b.center().x - 300.0).abs() <= SWAY + 0.01);
        }
        assert!(seconds > 15.0, "subiu rápido demais: {seconds} s");
    }

    #[test]
    fn velocidade_multiplica_so_a_subida() {
        let (mut slow, mut fast) = (Bubble::new('A', 300.0, 0.0), Bubble::new('A', 300.0, 0.0));
        slow.update(2.0, 0.5);
        fast.update(2.0, 2.0);
        assert!((START_Y - slow.y - RISE_SPEED).abs() < 1e-3);
        assert!(4.0f32.mul_add(-RISE_SPEED, START_Y - fast.y).abs() < 1e-3);
        assert!(
            (slow.center().x - fast.center().x).abs() < 1e-5,
            "o balanço é o mesmo"
        );
    }

    #[test]
    fn cresce_ao_surgir_e_fica_do_tamanho_normal() {
        let mut b = Bubble::new('A', 300.0, 0.0);
        assert!(b.scale() < 0.1);
        b.update(GROW_TIME / 2.0, 1.0);
        assert!(b.scale() > 0.5);
        b.update(GROW_TIME, 1.0);
        assert!((b.scale() - 1.0).abs() < 1e-5);
    }
}

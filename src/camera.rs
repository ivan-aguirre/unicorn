//! Câmera horizontal que acompanha o unicórnio com suavidade, olhando um pouco à frente.

use crate::player::Facing;
use crate::view::VIEW_W;

/// Quanto a câmera olha à frente do unicórnio.
const LOOK_AHEAD: f32 = VIEW_W * 0.14;
/// Rapidez com que a câmera alcança o alvo (maior = mais rápida).
const RESPONSIVENESS: f32 = 4.0;

/// Posição da borda esquerda da tela no mundo.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera {
    pub x: f32,
}

impl Camera {
    /// Câmera já centrada no alvo.
    #[must_use]
    pub fn new(player_x: f32, facing: Facing, length: f32) -> Self {
        Self {
            x: target(player_x, facing, length),
        }
    }

    /// Aproxima a câmera do alvo de forma suave e independente da taxa de quadros.
    pub fn follow(&mut self, player_x: f32, facing: Facing, length: f32, dt: f32) {
        let goal = target(player_x, facing, length);
        let blend = 1.0 - (-RESPONSIVENESS * dt).exp();
        self.x = (goal - self.x).mul_add(blend, self.x);
    }

    /// Posição arredondada ao pixel (evita tremulação na pixel art).
    #[must_use]
    pub const fn snapped(self) -> f32 {
        self.x.round()
    }
}

/// Onde a câmera quer estar, sem sair dos limites do nível.
fn target(player_x: f32, facing: Facing, length: f32) -> f32 {
    let wanted = facing.sign().mul_add(LOOK_AHEAD, player_x - VIEW_W / 2.0);
    wanted.clamp(0.0, (length - VIEW_W).max(0.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fica_presa_nas_bordas_do_nivel() {
        assert!(Camera::new(10.0, Facing::Right, 5000.0).x.abs() < f32::EPSILON);
        let end = Camera::new(4990.0, Facing::Right, 5000.0);
        assert!((end.x - (5000.0 - VIEW_W)).abs() < f32::EPSILON);
    }

    #[test]
    fn olha_para_onde_o_unicornio_vai() {
        let right = Camera::new(2000.0, Facing::Right, 5000.0);
        let left = Camera::new(2000.0, Facing::Left, 5000.0);
        assert!(right.x > left.x);
        // O unicórnio fica sempre visível.
        for cam in [right, left] {
            assert!((cam.x..cam.x + VIEW_W).contains(&2000.0));
        }
    }

    #[test]
    fn segue_suavemente_e_converge() {
        let mut cam = Camera::new(1000.0, Facing::Right, 5000.0);
        let start = cam.x;
        cam.follow(1200.0, Facing::Right, 5000.0, 1.0 / 60.0);
        assert!(cam.x > start && cam.x < start + 200.0);
        for _ in 0..600 {
            cam.follow(1200.0, Facing::Right, 5000.0, 1.0 / 60.0);
        }
        assert!((cam.x - target(1200.0, Facing::Right, 5000.0)).abs() < 0.01);
    }

    #[test]
    fn arredonda_ao_pixel() {
        assert!((Camera { x: 10.6 }.snapped() - 11.0).abs() < f32::EPSILON);
    }
}

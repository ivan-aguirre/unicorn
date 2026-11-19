//! Plataformas flutuantes de grama: dá para pular através delas por baixo e ficar em cima.

use fastrand::Rng;

use super::terrain::Terrain;

/// Espessura visual da plataforma (o corpo de terra abaixo da grama).
pub const THICKNESS: f32 = 16.0;
/// Alturas possíveis do topo em relação ao chão mais alto abaixo dela.
const HEIGHTS: [f32; 3] = [66.0, 72.0, 78.0];
/// Variação máxima do chão sob a plataforma (só em trechos quase planos).
const MAX_GROUND_VARIATION: f32 = 12.0;
/// Onde as plataformas começam e param (longe do início e do fim do nível).
const MARGIN: u16 = 1000;

/// Plataforma de mão única (só colide de cima para baixo).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Platform {
    /// Borda esquerda.
    pub x: f32,
    /// Topo (onde o unicórnio pisa).
    pub y: f32,
    /// Largura.
    pub w: f32,
}

impl Platform {
    /// Borda direita.
    #[must_use]
    pub fn right(&self) -> f32 {
        self.x + self.w
    }

    /// Se `x` está sobre a plataforma.
    #[must_use]
    pub fn covers(&self, x: f32) -> bool {
        (self.x..=self.right()).contains(&x)
    }
}

/// Espalha plataformas pelo nível, ordenadas por `x`, sempre alcançáveis com um pulo.
#[must_use]
pub fn generate(rng: &mut Rng, terrain: &Terrain, length: u16) -> Vec<Platform> {
    let mut platforms = Vec::new();
    let mut left = MARGIN;
    while left < length.saturating_sub(MARGIN) {
        let width = 16 * u16::from(rng.u8(5..=10));
        let (x, w) = (f32::from(left), f32::from(width));
        let (highest, lowest) = terrain.y_range(x, x + w);
        if lowest - highest <= MAX_GROUND_VARIATION {
            let height = HEIGHTS
                .get(rng.usize(..HEIGHTS.len()))
                .copied()
                .unwrap_or(66.0);
            platforms.push(Platform {
                x,
                y: highest - height,
                w,
            });
        }
        left = left.saturating_add(width + 16 * u16::from(rng.u8(32..=86)));
    }
    platforms
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::player::{HEIGHT, JUMP_HEIGHT};

    fn sample() -> (Terrain, Vec<Platform>) {
        let mut rng = Rng::with_seed(3);
        let terrain = Terrain::generate(&mut rng, 54_400);
        let platforms = generate(&mut rng, &terrain, 54_400);
        (terrain, platforms)
    }

    #[test]
    fn ha_muitas_plataformas_ordenadas_e_sem_sobreposicao() {
        let (_, platforms) = sample();
        assert!(platforms.len() > 20, "apenas {}", platforms.len());
        assert!(platforms.windows(2).all(|w| w[0].right() < w[1].x));
    }

    #[test]
    fn toda_plataforma_e_alcancavel_com_um_pulo() {
        let (terrain, platforms) = sample();
        for p in &platforms {
            let (_, lowest) = terrain.y_range(p.x, p.right());
            assert!(
                lowest - p.y < JUMP_HEIGHT - 4.0,
                "plataforma alta demais em x={}",
                p.x
            );
        }
    }

    #[test]
    fn da_para_passar_por_baixo_sem_bater_a_cabeca() {
        let (terrain, platforms) = sample();
        for p in &platforms {
            let (highest, _) = terrain.y_range(p.x, p.right());
            assert!(
                highest - (p.y + THICKNESS) > HEIGHT,
                "plataforma baixa em x={}",
                p.x
            );
        }
    }

    #[test]
    fn cobre_apenas_a_propria_extensao() {
        let p = Platform {
            x: 10.0,
            y: 100.0,
            w: 20.0,
        };
        assert!(p.covers(10.0) && p.covers(30.0) && p.covers(20.0));
        assert!(!p.covers(9.9) && !p.covers(30.1));
    }
}

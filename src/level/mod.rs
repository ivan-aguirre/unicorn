//! O único nível do jogo: chão com aclives e declives, plataformas e cenário.

pub mod platform;
pub mod scenery;
pub mod terrain;

use fastrand::Rng;

pub use platform::Platform;
pub use scenery::Scenery;
pub use terrain::Terrain;

/// Comprimento do nível, em pixels: 85 telas, alguns minutos de caminhada.
pub const LENGTH: u16 = 54_400;
/// Semente do nível: o cenário é sempre o mesmo, só os corações variam.
pub const SEED: u64 = 0x00C0_FFEE_0D1A_2025;

/// Nível completo.
#[derive(Debug, Clone)]
pub struct Level {
    pub terrain: Terrain,
    pub platforms: Vec<Platform>,
    pub scenery: Scenery,
}

impl Level {
    /// Gera o nível de forma determinística a partir da semente.
    #[must_use]
    pub fn generate(seed: u64) -> Self {
        let mut rng = Rng::with_seed(seed);
        let terrain = Terrain::generate(&mut rng, LENGTH);
        let platforms = platform::generate(&mut rng, &terrain, LENGTH);
        let scenery = Scenery::generate(&mut rng, LENGTH);
        Self {
            terrain,
            platforms,
            scenery,
        }
    }

    /// Comprimento do nível.
    #[must_use]
    pub fn length(&self) -> f32 {
        self.terrain.length()
    }

    /// Plataforma (índice) sob `x` cujo topo foi cruzado ao descer de `from_y` para `to_y`.
    #[must_use]
    pub fn landing_platform(&self, x: f32, from_y: f32, to_y: f32) -> Option<usize> {
        let lo = self.platforms.partition_point(|p| p.right() < x);
        self.platforms
            .get(lo..)?
            .iter()
            .take_while(|p| p.x <= x)
            .position(|p| p.covers(x) && from_y <= p.y && to_y >= p.y)
            .map(|i| lo + i)
    }

    /// Plataforma sob `x`, se houver (usada para posicionar corações). Busca binária.
    #[must_use]
    pub fn platform_at(&self, x: f32) -> Option<&Platform> {
        let lo = self.platforms.partition_point(|p| p.right() < x);
        self.platforms.get(lo).filter(|p| p.covers(x))
    }
}

/// Fatia de `items` (ordenados pela chave) com chave em `[from, to]`. Busca binária: O(log n).
pub fn span<T>(items: &[T], key: impl Fn(&T) -> f32, from: f32, to: f32) -> &[T] {
    let lo = items.partition_point(|i| key(i) < from);
    let hi = items.partition_point(|i| key(i) <= to);
    items.get(lo..hi.max(lo)).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn flat_level() -> Level {
        use macroquad::math::Vec2;
        Level {
            terrain: Terrain::from_points(vec![Vec2::new(0.0, 160.0), Vec2::new(1000.0, 160.0)]),
            platforms: vec![
                Platform {
                    x: 100.0,
                    y: 120.0,
                    w: 48.0,
                },
                Platform {
                    x: 300.0,
                    y: 116.0,
                    w: 64.0,
                },
            ],
            scenery: Scenery::generate(&mut Rng::with_seed(1), 1000),
        }
    }

    #[test]
    fn nivel_e_deterministico_e_longo() {
        let (a, b) = (Level::generate(SEED), Level::generate(SEED));
        assert!((a.length() - f32::from(LENGTH)).abs() < 1e-3);
        assert_eq!(a.terrain.points(), b.terrain.points());
        assert_eq!(a.platforms, b.platforms);
    }

    #[test]
    fn aterrissa_so_ao_cruzar_o_topo_descendo() {
        let level = flat_level();
        assert_eq!(level.landing_platform(320.0, 110.0, 118.0), Some(1));
        assert_eq!(level.landing_platform(120.0, 119.0, 121.0), Some(0));
        // Subindo por baixo: atravessa.
        assert_eq!(level.landing_platform(320.0, 125.0, 110.0), None);
        // Fora da plataforma.
        assert_eq!(level.landing_platform(200.0, 110.0, 130.0), None);
    }

    #[test]
    fn encontra_plataforma_sob_x() {
        let level = flat_level();
        assert_eq!(level.platform_at(330.0).map(|p| p.x), Some(300.0));
        assert!(level.platform_at(50.0).is_none());
        assert!(level.platform_at(5000.0).is_none());
    }

    #[test]
    fn faixa_por_busca_binaria() {
        let xs = [1.0, 2.0, 3.0, 5.0, 8.0];
        assert_eq!(span(&xs, |x| *x, 2.0, 5.0), &[2.0, 3.0, 5.0]);
        assert_eq!(span(&xs, |x| *x, 9.0, 10.0), []);
        assert_eq!(span(&xs, |x| *x, 6.0, 4.0), []);
    }
}

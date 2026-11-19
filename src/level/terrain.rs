//! Chão do nível: uma linha poligonal (aclives, declives e trechos planos).

use fastrand::Rng;
use macroquad::math::Vec2;

/// Ponto mais alto que o chão pode alcançar (menor `y`).
pub const GROUND_TOP: f32 = 220.0;
/// Ponto mais baixo que o chão pode alcançar (maior `y`).
pub const GROUND_BOTTOM: f32 = 306.0;
/// Altura do chão no começo e no fim do nível.
const BASE_Y: f32 = 280.0;
/// Trecho plano no início, onde o unicórnio aparece.
const FLAT_START: u16 = 752;
/// Trecho plano no fim do nível.
const FLAT_END: u16 = 640;
/// Inclinações possíveis (dy/dx; negativo sobe para a direita). Plano é o mais comum.
const SLOPES: [f32; 7] = [-0.5, -0.25, 0.0, 0.0, 0.0, 0.25, 0.5];
/// Inclinação máxima, em módulo: suave o bastante para andar sem esforço.
pub const MAX_SLOPE: f32 = 0.5;
/// Maior mudança de inclinação entre segmentos vizinhos (evita "quinas" bruscas).
const MAX_SLOPE_CHANGE: f32 = 0.5;

/// Superfície do chão, definida por pontos ordenados em `x` e interpolada linearmente.
#[derive(Debug, Clone)]
pub struct Terrain {
    points: Vec<Vec2>,
}

impl Terrain {
    /// Cria o chão a partir de pontos já ordenados por `x` (o primeiro deve estar em `x = 0`).
    #[must_use]
    pub const fn from_points(points: Vec<Vec2>) -> Self {
        Self { points }
    }

    /// Gera um chão aleatório (mas determinístico para a mesma semente) com o comprimento dado.
    ///
    /// Os segmentos têm largura múltipla de 16 px e inclinações múltiplas de 1/4: os pontos
    /// caem em pixels inteiros.
    #[must_use]
    pub fn generate(rng: &mut Rng, length: u16) -> Self {
        let mut points = vec![
            Vec2::new(0.0, BASE_Y),
            Vec2::new(f32::from(FLAT_START), BASE_Y),
        ];
        let (mut x, mut y, mut slope) = (FLAT_START, BASE_Y, 0.0);
        let end = length.saturating_sub(FLAT_END);
        while x < end {
            let run = (16 * u16::from(rng.u8(3..=13))).min(end - x);
            slope = next_slope(rng, slope, y, f32::from(run));
            x += run;
            y = slope
                .mul_add(f32::from(run), y)
                .clamp(GROUND_TOP, GROUND_BOTTOM);
            points.push(Vec2::new(f32::from(x), y));
        }
        // Termina com um trecho plano.
        points.push(Vec2::new(f32::from(length), y));
        Self { points }
    }

    /// Altura (coordenada `y`) da superfície em `x`. Fora do nível, repete a borda.
    #[must_use]
    pub fn height_at(&self, x: f32) -> f32 {
        let next = self.points.partition_point(|p| p.x <= x);
        let before = next.checked_sub(1).and_then(|prev| self.points.get(prev));
        match (before, self.points.get(next)) {
            (Some(left), Some(right)) => {
                let progress = (x - left.x) / (right.x - left.x);
                (right.y - left.y).mul_add(progress, left.y)
            }
            (Some(edge), None) | (None, Some(edge)) => edge.y,
            (None, None) => BASE_Y,
        }
    }

    /// Menor e maior `y` do chão no intervalo `[x0, x1]` (exato: o chão é poligonal).
    #[must_use]
    pub fn y_range(&self, x0: f32, x1: f32) -> (f32, f32) {
        let (h0, h1) = (self.height_at(x0), self.height_at(x1));
        let lo = self.points.partition_point(|p| p.x <= x0);
        let hi = self.points.partition_point(|p| p.x < x1);
        let inner = self.points.get(lo..hi).unwrap_or_default();
        inner
            .iter()
            .fold((h0.min(h1), h0.max(h1)), |(min, max), p| {
                (min.min(p.y), max.max(p.y))
            })
    }

    /// Comprimento total do chão.
    #[must_use]
    pub fn length(&self) -> f32 {
        self.points.last().map_or(0.0, |p| p.x)
    }

    /// Pontos que definem a superfície.
    #[must_use]
    pub fn points(&self) -> &[Vec2] {
        &self.points
    }
}

/// Escolhe a próxima inclinação respeitando os limites de altura e de mudança brusca.
fn next_slope(rng: &mut Rng, previous: f32, y: f32, seg: f32) -> f32 {
    let fits = |s: f32| {
        let end = s.mul_add(seg, y);
        (s - previous).abs() <= MAX_SLOPE_CHANGE && (GROUND_TOP..=GROUND_BOTTOM).contains(&end)
    };
    let start = rng.usize(..SLOPES.len());
    SLOPES
        .iter()
        .cycle()
        .skip(start)
        .take(SLOPES.len())
        .copied()
        .find(|s| fits(*s))
        .unwrap_or(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const LENGTH: u16 = 54_400;

    fn sample() -> Terrain {
        Terrain::generate(&mut Rng::with_seed(7), LENGTH)
    }

    #[test]
    fn interpola_entre_pontos_e_repete_as_bordas() {
        let t = Terrain::from_points(vec![Vec2::new(0.0, 100.0), Vec2::new(10.0, 120.0)]);
        assert!((t.height_at(5.0) - 110.0).abs() < 1e-4);
        assert!((t.height_at(-50.0) - 100.0).abs() < 1e-4);
        assert!((t.height_at(99.0) - 120.0).abs() < 1e-4);
        assert!((t.length() - 10.0).abs() < 1e-4);
    }

    #[test]
    fn gera_nivel_longo_com_comprimento_exato() {
        let t = sample();
        assert!((t.length() - f32::from(LENGTH)).abs() < 1e-3);
        assert!(t.points().windows(2).all(|w| w[0].x < w[1].x));
    }

    #[test]
    fn alturas_ficam_dentro_da_faixa_visivel() {
        let t = sample();
        assert!(
            t.points()
                .iter()
                .all(|p| (GROUND_TOP..=GROUND_BOTTOM).contains(&p.y))
        );
    }

    #[test]
    fn inclinacoes_sao_suaves() {
        let t = sample();
        for w in t.points().windows(2) {
            let slope = (w[1].y - w[0].y) / (w[1].x - w[0].x);
            assert!(
                slope.abs() <= MAX_SLOPE + 1e-3,
                "inclinação {slope} em x={}",
                w[0].x
            );
        }
    }

    #[test]
    fn tem_aclives_e_declives() {
        let t = sample();
        let slopes: Vec<f32> = t
            .points()
            .windows(2)
            .map(|w| (w[1].y - w[0].y) / (w[1].x - w[0].x))
            .collect();
        assert!(slopes.iter().filter(|s| **s < -0.1).count() > 20);
        assert!(slopes.iter().filter(|s| **s > 0.1).count() > 20);
    }

    #[test]
    fn mesma_semente_gera_o_mesmo_chao() {
        let (a, b) = (sample(), sample());
        assert_eq!(a.points(), b.points());
    }

    #[test]
    fn inicio_e_plano_para_o_unicornio_aparecer() {
        let t = sample();
        let (min, max) = t.y_range(0.0, f32::from(FLAT_START));
        assert!((max - min).abs() < f32::EPSILON);
    }

    #[test]
    fn faixa_de_alturas_inclui_pontos_internos() {
        let t = Terrain::from_points(vec![
            Vec2::new(0.0, 150.0),
            Vec2::new(10.0, 130.0),
            Vec2::new(20.0, 150.0),
        ]);
        let (min, max) = t.y_range(2.0, 18.0);
        assert!((min - 130.0).abs() < 1e-4);
        assert!((max - 146.0).abs() < 1e-4);
    }
}

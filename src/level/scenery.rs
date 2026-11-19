//! Cenário decorativo distribuído ao longo do nível: árvores, flores, montanhas com
//! cachoeiras, arco-íris e nuvens. Cada camada tem seu fator de paralaxe.

use fastrand::Rng;

use crate::view::VIEW_W_PX;

/// Fator de paralaxe, em porcentagem do movimento da câmera (inteiro: o cenário é gerado
/// em pixels exatos).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Parallax(pub u16);

impl Parallax {
    /// Fator multiplicador da posição da câmera.
    #[must_use]
    pub fn factor(self) -> f32 {
        f32::from(self.0) / 100.0
    }

    /// Até onde a camada precisa de conteúdo para um nível de comprimento `length`.
    #[must_use]
    pub fn extent(self, length: u16) -> u16 {
        let moved = u32::from(length) * u32::from(self.0) / 100;
        u16::try_from(moved + u32::from(VIEW_W_PX)).unwrap_or(u16::MAX)
    }
}

/// Paralaxe das montanhas distantes e do mar.
pub const FAR_PARALLAX: Parallax = Parallax(12);
/// Paralaxe dos arco-íris.
pub const RAINBOW_PARALLAX: Parallax = Parallax(20);
/// Paralaxe das colinas com cachoeiras.
pub const MESA_PARALLAX: Parallax = Parallax(40);
/// Período (em pixels de tela) em que cada camada de nuvens se repete.
pub const CLOUD_PERIOD: f32 = 1920.0;

/// Camada de nuvens que se repete pelo céu.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CloudLayer {
    pub parallax: f32,
    /// Deriva pelo vento, em px/s.
    pub drift: f32,
    pub top: f32,
    pub bottom: f32,
}

/// Camadas de nuvens: distante (lenta) e próxima.
pub const CLOUD_LAYERS: [CloudLayer; 2] = [
    CloudLayer {
        parallax: 0.06,
        drift: 3.0,
        top: 14.0,
        bottom: 66.0,
    },
    CloudLayer {
        parallax: 0.25,
        drift: 7.0,
        top: 40.0,
        bottom: 120.0,
    },
];
/// Nuvens por camada.
const CLOUDS_PER_LAYER: u8 = 7;
/// Distância mínima entre duas árvores.
const TREE_SPACING: u16 = 186;

/// Cor das flores.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlowerColor {
    Pink,
    Yellow,
    Purple,
}

/// Tipo de enfeite plantado no chão.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropKind {
    Tree,
    BlossomTree,
    Bush,
    Flower(FlowerColor),
}

/// Enfeite plantado no chão do nível (paralaxe 1).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Prop {
    pub x: f32,
    pub kind: PropKind,
}

/// Cachoeira descendo de uma colina.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Waterfall {
    /// Distância da borda esquerda da colina.
    pub offset: f32,
    pub w: f32,
}

/// Colina de topo plano ao fundo, opcionalmente com cachoeira.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mesa {
    pub x: f32,
    pub w: f32,
    pub top: f32,
    pub waterfall: Option<Waterfall>,
}

/// Uma cadeia de montanhas no horizonte: duas ondas triangulares somadas (picos pontudos) ou
/// suavizadas (colinas redondas).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Range {
    pub parallax: Parallax,
    /// Linha de base (onde a cadeia encontra a névoa do horizonte).
    pub base: f32,
    /// (altura, período) de cada onda.
    pub waves: [(f32, f32); 2],
    pub seed: f32,
    /// Colinas arredondadas em vez de picos.
    pub rounded: bool,
}

/// Cordilheira distante (picos com neve) e colinas verdes na frente dela.
pub const RANGES: [Range; 2] = [
    Range {
        parallax: Parallax(5),
        base: 204.0,
        waves: [(80.0, 190.0), (56.0, 113.0)],
        seed: 0.3,
        rounded: false,
    },
    Range {
        parallax: Parallax(12),
        base: 206.0,
        waves: [(24.0, 170.0), (9.0, 53.0)],
        seed: 1.7,
        rounded: true,
    },
];

impl Range {
    /// Altura (`y`) da crista na coordenada `x` da camada.
    #[must_use]
    pub fn top_at(&self, x: f32) -> f32 {
        let waves = self
            .waves
            .iter()
            .zip([1.0f32, 2.3])
            .map(|(&(height, period), k)| {
                let t = triangle(self.seed.mul_add(k, x / period));
                (height, t)
            });
        // Colinas: as ondas se somam, suavizadas. Picos: vale o mais alto dos dois, para os
        // picos ficarem sempre pontudos (somar ondas cria platôs onde elas se anulam).
        let lift = if self.rounded {
            waves
                .map(|(h, t)| h * t * t * 2.0f32.mul_add(-t, 3.0))
                .sum()
        } else {
            waves.map(|(h, t)| h * t).fold(0.0, f32::max)
        };
        self.base - lift
    }
}

/// Onda triangular de 0 a 1 (pico no meio de cada período).
fn triangle(t: f32) -> f32 {
    1.0 - 2.0f32.mul_add(t.rem_euclid(1.0), -1.0).abs()
}

/// Largura dos "ombros" arredondados de cada lado da colina.
const SHOULDER: f32 = 16.0;
/// Profundidade do sulco por onde a cachoeira escorre.
const GROOVE: f32 = 4.0;

impl Mesa {
    /// Altura do topo a `dx` pixels da borda esquerda: ondulado, com ombros arredondados
    /// nas laterais e um sulco plano onde nasce a cachoeira.
    #[must_use]
    pub fn top_at(&self, dx: f32) -> f32 {
        if let Some(w) = self.waterfall
            && (w.offset - 2.0..=w.offset + w.w + 2.0).contains(&dx)
        {
            return self.top + GROOVE;
        }
        let seed = self.x * 0.0137;
        let bumps = 3.0f32.mul_add(
            dx.mul_add(0.09, seed).sin(),
            1.5 * dx.mul_add(0.23, seed * 3.0).sin(),
        );
        let edge = dx.min(self.w - 1.0 - dx).max(0.0);
        let shoulder = (SHOULDER - edge).max(0.0).powi(2) / 10.0;
        self.top + bumps + shoulder
    }

    /// Onde a água da cachoeira começa a cair.
    #[must_use]
    pub fn waterfall_top(&self) -> f32 {
        self.top + GROOVE
    }
}

/// Nuvem de uma camada (coordenadas dentro do período da camada).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cloud {
    pub x: f32,
    pub y: f32,
    pub kind: usize,
}

/// Todo o cenário, com cada lista ordenada por `x` (para consultas por faixa visível).
#[derive(Debug, Clone)]
pub struct Scenery {
    pub props: Vec<Prop>,
    pub mesas: Vec<Mesa>,
    pub rainbows: Vec<f32>,
    pub clouds: [Vec<Cloud>; 2],
}

impl Scenery {
    /// Gera o cenário para um nível com o comprimento dado.
    #[must_use]
    pub fn generate(rng: &mut Rng, length: u16) -> Self {
        Self {
            props: props(rng, length),
            mesas: mesas(rng, MESA_PARALLAX.extent(length)),
            rainbows: rainbows(rng, RAINBOW_PARALLAX.extent(length)),
            clouds: [clouds(rng, &CLOUD_LAYERS[0]), clouds(rng, &CLOUD_LAYERS[1])],
        }
    }
}

/// Número aleatório em `[lo, hi)`.
pub fn range(rng: &mut Rng, lo: f32, hi: f32) -> f32 {
    (hi - lo).mul_add(rng.f32(), lo)
}

fn props(rng: &mut Rng, length: u16) -> Vec<Prop> {
    let mut out = Vec::new();
    let mut x: u16 = 66;
    let mut next_tree: u16 = 0;
    while x < length.saturating_sub(66) {
        let kind = prop_kind(rng, x >= next_tree);
        if matches!(kind, PropKind::Tree | PropKind::BlossomTree) {
            next_tree = x.saturating_add(TREE_SPACING);
        }
        out.push(Prop {
            x: f32::from(x),
            kind,
        });
        x = x.saturating_add(rng.u16(24..80));
    }
    out
}

fn prop_kind(rng: &mut Rng, tree_allowed: bool) -> PropKind {
    let roll = rng.u8(..100);
    match roll {
        0..=7 if tree_allowed => PropKind::BlossomTree,
        8..=19 if tree_allowed => PropKind::Tree,
        20..=39 => PropKind::Bush,
        _ => PropKind::Flower(flower_color(roll)),
    }
}

const fn flower_color(roll: u8) -> FlowerColor {
    match roll % 3 {
        0 => FlowerColor::Pink,
        1 => FlowerColor::Yellow,
        _ => FlowerColor::Purple,
    }
}

fn mesas(rng: &mut Rng, extent: u16) -> Vec<Mesa> {
    let mut out = Vec::new();
    let mut x = rng.u16(0..40);
    while x < extent {
        let width = rng.u16(80..200);
        let waterfall = (rng.u8(..100) < 45).then(|| Waterfall {
            offset: f32::from(rng.u16(14..width - 34)),
            w: f32::from(rng.u16(13..22)),
        });
        out.push(Mesa {
            x: f32::from(x),
            w: f32::from(width),
            top: f32::from(rng.u16(104..166)),
            waterfall,
        });
        x = x.saturating_add(width + rng.u16(34..200));
    }
    out
}

fn rainbows(rng: &mut Rng, extent: u16) -> Vec<f32> {
    let mut out = Vec::new();
    let mut x = rng.u16(230..430);
    while x < extent {
        out.push(f32::from(x));
        x = x.saturating_add(rng.u16(700..1270));
    }
    out
}

/// Nuvens espalhadas em fatias iguais do período, para não se amontoarem.
fn clouds(rng: &mut Rng, layer: &CloudLayer) -> Vec<Cloud> {
    let slot = CLOUD_PERIOD / f32::from(CLOUDS_PER_LAYER);
    (0..CLOUDS_PER_LAYER)
        .map(|i| Cloud {
            x: slot
                .mul_add(f32::from(i), range(rng, 0.0, slot * 0.6))
                .round(),
            y: range(rng, layer.top, layer.bottom).round(),
            kind: rng.usize(..3),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const LENGTH: u16 = 54_400;

    fn sample() -> Scenery {
        Scenery::generate(&mut Rng::with_seed(11), LENGTH)
    }

    fn sorted(xs: impl Iterator<Item = f32>) -> bool {
        let v: Vec<f32> = xs.collect();
        v.windows(2).all(|w| w[0] <= w[1])
    }

    #[test]
    fn listas_ficam_ordenadas_por_x() {
        let s = sample();
        assert!(sorted(s.props.iter().map(|p| p.x)));
        assert!(sorted(s.mesas.iter().map(|m| m.x)));
        assert!(sorted(s.rainbows.iter().copied()));
        assert!(
            s.clouds
                .iter()
                .all(|layer| sorted(layer.iter().map(|c| c.x)))
        );
    }

    #[test]
    fn camadas_cobrem_o_nivel_inteiro() {
        let s = sample();
        let last_mesa = s.mesas.last().map_or(0.0, |m| m.x + m.w);
        assert!(last_mesa >= f32::from(MESA_PARALLAX.extent(LENGTH)) - 260.0);
        let last_rainbow = s.rainbows.last().copied().unwrap_or(0.0);
        assert!(last_rainbow >= f32::from(RAINBOW_PARALLAX.extent(LENGTH)) - 1300.0);
        let last_prop = s.props.last().map_or(0.0, |p| p.x);
        assert!(last_prop >= f32::from(LENGTH) - 160.0);
    }

    #[test]
    fn ha_cachoeiras_e_arco_iris_espalhados() {
        let s = sample();
        let waterfalls = s.mesas.iter().filter(|m| m.waterfall.is_some()).count();
        assert!(waterfalls >= 20, "apenas {waterfalls} cachoeiras");
        assert!(
            s.rainbows.len() >= 8,
            "apenas {} arco-íris",
            s.rainbows.len()
        );
    }

    #[test]
    fn cachoeira_fica_dentro_da_colina() {
        for m in &sample().mesas {
            if let Some(w) = m.waterfall {
                assert!(
                    w.offset >= 0.0 && w.offset + w.w <= m.w,
                    "cachoeira fora em x={}",
                    m.x
                );
            }
        }
    }

    #[test]
    fn colinas_tem_ombros_redondos_e_topo_ondulado() {
        let m = Mesa {
            x: 1234.0,
            w: 160.0,
            top: 120.0,
            waterfall: None,
        };
        // Nas bordas o topo desce (ombro arredondado), no meio fica perto da altura base.
        assert!(m.top_at(0.0) > m.top + 15.0);
        assert!(m.top_at(m.w - 1.0) > m.top + 15.0);
        assert!((m.top_at(80.0) - m.top).abs() <= 4.5);
        // O topo não é reto: varia ao longo da colina.
        let tops: Vec<f32> = (20..140_u8).map(|d| m.top_at(f32::from(d))).collect();
        let lowest = tops.iter().copied().fold(f32::MAX, f32::min);
        let highest = tops.iter().copied().fold(f32::MIN, f32::max);
        assert!(highest - lowest > 3.0);
    }

    #[test]
    fn cachoeira_nasce_num_sulco_plano() {
        let fall = Waterfall {
            offset: 60.0,
            w: 16.0,
        };
        let m = Mesa {
            x: 50.0,
            w: 160.0,
            top: 120.0,
            waterfall: Some(fall),
        };
        for d in 60..76_u8 {
            assert!((m.top_at(f32::from(d)) - m.waterfall_top()).abs() < f32::EPSILON);
        }
    }

    #[test]
    fn montanhas_do_horizonte_ficam_no_ceu_e_tem_relevo() {
        for range in RANGES {
            let tops: Vec<f32> = (0..2000_u16).map(|x| range.top_at(f32::from(x))).collect();
            let highest = tops.iter().copied().fold(f32::MAX, f32::min);
            let lowest = tops.iter().copied().fold(f32::MIN, f32::max);
            assert!(highest > 100.0, "pico alto demais: {highest}");
            assert!(lowest <= range.base + f32::EPSILON);
            assert!(lowest - highest > 20.0, "cordilheira sem relevo");
        }
        // As colinas da frente ficam abaixo dos picos mais altos da cordilheira.
        let peaks = (0..2000_u16)
            .map(|x| RANGES[0].top_at(f32::from(x)))
            .fold(f32::MAX, f32::min);
        let hills = (0..2000_u16)
            .map(|x| RANGES[1].top_at(f32::from(x)))
            .fold(f32::MAX, f32::min);
        assert!(hills > peaks + 20.0);
    }

    #[test]
    fn picos_nao_formam_platos() {
        let range = RANGES[0];
        let longest_flat = (0..3000_u16)
            .map(|x| (range.top_at(f32::from(x)) - range.top_at(f32::from(x) + 1.0)).abs() < 0.05)
            .fold((0, 0), |(best, run), flat| {
                if flat {
                    (best.max(run + 1), run + 1)
                } else {
                    (best, 0)
                }
            })
            .0;
        assert!(longest_flat < 6, "platô de {longest_flat} px");
    }

    #[test]
    fn onda_triangular_vai_de_zero_a_um() {
        assert!(triangle(0.0).abs() < 1e-6);
        assert!((triangle(0.5) - 1.0).abs() < 1e-6);
        assert!((triangle(0.25) - 0.5).abs() < 1e-6);
        assert!((triangle(1.75) - 0.5).abs() < 1e-6);
    }

    #[test]
    fn arvores_nao_ficam_amontoadas() {
        let trees: Vec<f32> = sample()
            .props
            .iter()
            .filter(|p| matches!(p.kind, PropKind::Tree | PropKind::BlossomTree))
            .map(|p| p.x)
            .collect();
        assert!(trees.len() > 50);
        assert!(
            trees
                .windows(2)
                .all(|w| w[1] - w[0] >= f32::from(TREE_SPACING))
        );
    }

    #[test]
    fn nuvens_ficam_no_ceu_e_dentro_do_periodo() {
        let s = sample();
        for (layer, clouds) in CLOUD_LAYERS.iter().zip(&s.clouds) {
            assert_eq!(clouds.len(), usize::from(CLOUDS_PER_LAYER));
            assert!(
                clouds
                    .iter()
                    .all(|c| (layer.top..=layer.bottom).contains(&c.y))
            );
            assert!(clouds.iter().all(|c| (0.0..CLOUD_PERIOD).contains(&c.x)));
        }
    }

    #[test]
    fn paralaxe_inteira_calcula_fator_e_extensao() {
        assert!((MESA_PARALLAX.factor() - 0.4).abs() < 1e-6);
        assert_eq!(MESA_PARALLAX.extent(1000), 400 + VIEW_W_PX);
        assert_eq!(Parallax(100).extent(u16::MAX), u16::MAX);
    }

    #[test]
    fn faixa_aleatoria_respeita_limites() {
        let mut rng = Rng::with_seed(1);
        assert!(
            (0..1000)
                .map(|_| range(&mut rng, 2.0, 5.0))
                .all(|v| (2.0..5.0).contains(&v))
        );
    }
}

//! Imagens geradas por código: coração, sol, arco-íris, nuvens, árvores, arbustos e texturas
//! do chão.

use super::pixmap::{Pixmap, Rgba, rgb};
use crate::view::VIEW_H_PX;

/// Altura da faixa de grama acima da superfície do chão (pontas das folhas).
pub const GRASS_TIP: f32 = 5.0;
/// Largura do padrão repetido do chão.
pub const GROUND_TILE: u16 = 48;
/// Lado de cada quadrado do xadrez de terra.
const CHECKER: u16 = 24;

/// Cores das bandas do arco-íris, de fora para dentro.
const RAINBOW_BANDS: [u32; 7] = [
    0xff_5a_6e, 0xff_a0_4a, 0xff_e1_4d, 0x6f_d3_6a, 0x4f_b8_ff, 0x6a_7c_ff, 0xb0_7c_ff,
];
const RAINBOW_OUTER: f32 = 110.0;
const RAINBOW_BAND: f32 = 7.0;

/// Pinta um disco (centro e raio em pixels; testa o centro de cada pixel).
fn disc(p: &mut Pixmap, (cx, cy): (f32, f32), r: f32, color: Rgba) {
    for (x, y) in p.coords() {
        let (dx, dy) = (f32::from(x) + 0.5 - cx, f32::from(y) + 0.5 - cy);
        if dx.hypot(dy) <= r {
            p.set(x, y, color);
        }
    }
}

/// Pinta um retângulo `[x0, x1) x [y0, y1)`.
fn fill(p: &mut Pixmap, (x0, y0): (u16, u16), (x1, y1): (u16, u16), color: Rgba) {
    for y in y0..y1 {
        for x in x0..x1 {
            p.set(x, y, color);
        }
    }
}

/// Sombreia uma copa/arbusto: luz em cima à esquerda, sombra embaixo à direita.
fn shade(p: &mut Pixmap, base: Rgba, (light, dark): (Rgba, Rgba), (cx, cy): (f32, f32), size: f32) {
    for (x, y) in p.coords() {
        if p.get(x, y) != Some(base) {
            continue;
        }
        let (dx, dy) = (f32::from(x) - cx, f32::from(y) - cy);
        if dx + dy < -0.8 * size {
            p.set(x, y, light);
        } else if dx.mul_add(0.4, dy) > 0.45 * size {
            p.set(x, y, dark);
        }
    }
}

/// Coração liso (curva `(x²+y²-1)³ = x²y³`) com sombra, brilho e contorno.
/// `w` x `h` é o tamanho do recheio; o contorno acrescenta 1 pixel em volta.
#[must_use]
pub fn heart(w: u16, h: u16) -> Pixmap {
    let mut canvas = Pixmap::new(w + 2, h + 2);
    let size = (f32::from(w), f32::from(h));
    for (px, py) in canvas.coords() {
        // Plano da curva: x em [-1.25, 1.25] e y em [1.3, -1.15], com 1 pixel de margem.
        let u = ((f32::from(px) - 0.5) / size.0).mul_add(2.5, -1.25);
        let v = ((f32::from(py) - 0.5) / size.1).mul_add(-2.45, 1.3);
        let ring = (u.mul_add(u, v * v) - 1.0).powi(3);
        if (u * u).mul_add(-v.powi(3), ring) <= 0.0 {
            canvas.set(px, py, heart_color(u, v));
        }
    }
    canvas.outlined(rgb(0x8a_1f_4f))
}

fn heart_color(u: f32, v: f32) -> Rgba {
    let shine = (u + 0.55).hypot(v - 0.65);
    if shine < 0.12 {
        rgb(0xff_ff_ff)
    } else if shine < 0.3 {
        rgb(0xff_c6_dc)
    } else if u.mul_add(0.6, -v) > 0.55 {
        rgb(0xd6_28_6a)
    } else {
        rgb(0xff_4d_8d)
    }
}

/// Quadros da rotação dos raios do sol (um oitavo de volta entre raios, em passos suaves).
pub const SUN_FRAMES: u8 = 8;

/// Sol sorridente com raios; `frame` (0 a 7) gira os raios suavemente.
#[must_use]
pub fn sun(frame: u8) -> Pixmap {
    let mut p = Pixmap::new(72, 72);
    let center = (36.0, 36.0);
    let turn = f32::from(frame % SUN_FRAMES) / f32::from(SUN_FRAMES) * std::f32::consts::FRAC_PI_4;
    for (x, y) in p.coords() {
        let (dx, dy) = (f32::from(x) + 0.5 - center.0, f32::from(y) + 0.5 - center.1);
        let d = dx.hypot(dy);
        let sector = (dy.atan2(dx) - turn).rem_euclid(std::f32::consts::FRAC_PI_4);
        let width = (d - 25.0).mul_add(-0.012, 0.22);
        let on_ray = (sector - std::f32::consts::FRAC_PI_8).abs() < width;
        if (25.0..34.5).contains(&d) && on_ray {
            p.set(x, y, rgb(0xff_c9_3c));
        }
    }
    disc(&mut p, center, 21.5, rgb(0xf7_a5_31));
    disc(&mut p, center, 20.0, rgb(0xff_e1_4d));
    sun_shine(&mut p, center);
    sun_face(&mut p);
    p
}

/// Reflexo fininho na borda de cima, à esquerda (longe do rosto).
fn sun_shine(p: &mut Pixmap, center: (f32, f32)) {
    let light = rgb(0xff_f3_a8);
    for (x, y) in p.coords() {
        let (dx, dy) = (f32::from(x) + 0.5 - center.0, f32::from(y) + 0.5 - center.1);
        let along_rim = (15.5..18.0).contains(&dx.hypot(dy));
        let upper_left = (dy.atan2(dx) + 2.36).abs() < 0.5;
        if along_rim && upper_left {
            p.set(x, y, light);
        }
    }
}

/// Olhinhos fechados e felizes, bochechas e sorriso.
fn sun_face(p: &mut Pixmap) {
    let ink = rgb(0x8a_4a_1c);
    let blush = rgb(0xff_9e_7a);
    let eye = [(26, 35), (27, 34), (28, 33), (29, 33), (30, 34), (31, 35)];
    for (x, y) in eye {
        p.set(x, y, ink);
        p.set(x + 15, y, ink);
    }
    let smile = [
        (29, 42),
        (30, 43),
        (31, 44),
        (32, 44),
        (33, 44),
        (34, 44),
        (35, 44),
    ];
    for (x, y) in smile {
        p.set(x, y, ink);
        p.set(72 - x, y, ink);
    }
    for (x, y) in [(22, 40), (23, 40), (24, 40), (22, 41), (23, 41), (24, 41)] {
        p.set(x, y, blush);
        p.set(x + 26, y, blush);
    }
}

/// Arco-íris de sete cores (meio círculo), apoiado na base da imagem.
#[must_use]
pub fn rainbow() -> Pixmap {
    let mut p = Pixmap::new(222, 112);
    let (cx, cy) = (111.0, 111.5);
    for (x, y) in p.coords() {
        let d = (f32::from(x) + 0.5 - cx).hypot(f32::from(y) + 0.5 - cy);
        let band = RAINBOW_BANDS.iter().zip(0_u8..).find(|(_, i)| {
            let outer = f32::from(*i).mul_add(-RAINBOW_BAND, RAINBOW_OUTER);
            d < outer && d >= outer - RAINBOW_BAND
        });
        if let Some((color, _)) = band {
            p.set(x, y, rgb(*color));
        }
    }
    p
}

/// Disco de uma nuvem: centro e raio.
type Puff = ((f32, f32), f32);
/// Formato de nuvem: tamanho da imagem e discos.
type CloudShape = ((u16, u16), &'static [Puff]);

/// Formatos de nuvem.
const CLOUDS: [CloudShape; 3] = [
    (
        (84, 36),
        &[
            ((20.0, 23.0), 11.5),
            ((37.0, 17.0), 15.0),
            ((55.0, 18.0), 13.0),
            ((68.0, 25.0), 10.0),
        ],
    ),
    (
        (57, 27),
        &[
            ((15.0, 17.0), 9.0),
            ((28.0, 12.5), 11.0),
            ((43.0, 17.0), 9.0),
        ],
    ),
    (
        (110, 43),
        &[
            ((20.0, 28.0), 11.5),
            ((37.0, 20.0), 15.0),
            ((58.0, 17.0), 16.5),
            ((80.0, 22.0), 13.0),
            ((95.0, 30.0), 10.0),
        ],
    ),
];

/// Corpo de nuvem: os discos de cima e uma fileira de "bolinhas" embaixo, para a base ficar
/// redonda e ondulada (nunca reta); a parte de baixo ganha um tom de sombra.
fn fluffy(p: &mut Pixmap, puffs: &[Puff]) {
    let white = rgb(0xff_ff_ff);
    for &(center, r) in puffs {
        disc(p, center, r, white);
    }
    let h = f32::from(p.height());
    let r = (h * 0.24).max(4.0);
    let first = puffs.first().map_or(r, |((x, _), _)| *x);
    let last = puffs.last().map_or(r, |((x, _), _)| *x);
    let base = h - r - 2.0;
    let steps = (0..40_u8)
        .map(f32::from)
        .take_while(|k| k.mul_add(r * 1.25, first) <= last);
    for k in steps {
        disc(p, (k.mul_add(r * 1.25, first), base), r, white);
    }
    disc(p, (last, base), r, white);
    let shadow = rgb(0xdc_ee_ff);
    for (x, y) in p.coords() {
        if f32::from(y) > h * 0.68 && p.is_opaque(x, y) {
            p.set(x, y, shadow);
        }
    }
}

/// Nuvem fofa de um dos três formatos.
#[must_use]
pub fn cloud(kind: usize) -> Pixmap {
    let ((w, h), puffs) = CLOUDS.get(kind).copied().unwrap_or(CLOUDS[0]);
    let mut p = Pixmap::new(w, h);
    fluffy(&mut p, puffs);
    p.outlined(rgb(0xb8_dc_ff))
}

/// Carinha das nuvens da festa.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CloudFace {
    /// Olhinhos fechados de alegria (^^) e sorriso.
    Happy,
    /// Olhos abertos brilhando e boca aberta de festa.
    Cheering,
}

const CLOUD_INK: u32 = 0x5a_4a_7a;
const CLOUD_BLUSH: u32 = 0xff_a3_c8;

/// Nuvem sorridente que apoia a base do arco-íris na festa dos 100 corações.
#[must_use]
pub fn smiley_cloud(face: CloudFace) -> Pixmap {
    let mut p = Pixmap::new(120, 56);
    let puffs = [
        ((24.0, 32.0), 16.0),
        ((48.0, 23.0), 21.0),
        ((74.0, 21.0), 20.0),
        ((96.0, 31.0), 15.0),
    ];
    fluffy(&mut p, &puffs);
    // O rosto fica sobre branco, mesmo onde a sombra de baixo alcançaria.
    disc(&mut p, (60.0, 34.0), 13.0, rgb(0xff_ff_ff));
    for (x, y) in [
        (42, 37),
        (43, 37),
        (44, 37),
        (76, 37),
        (77, 37),
        (78, 37),
        (42, 38),
        (43, 38),
        (77, 38),
        (78, 38),
    ] {
        p.set(x, y, rgb(CLOUD_BLUSH));
    }
    match face {
        CloudFace::Happy => happy_face(&mut p),
        CloudFace::Cheering => cheering_face(&mut p),
    }
    p.outlined(rgb(0x8f_bf_e8))
}

/// Olhos "^" e sorriso fechado.
fn happy_face(p: &mut Pixmap) {
    let ink = rgb(CLOUD_INK);
    for (dx, dy) in [(0, 2), (1, 1), (2, 0), (3, 0), (4, 1), (5, 2)] {
        p.set(46 + dx, 29 + dy, ink);
        p.set(69 + dx, 29 + dy, ink);
    }
    let smile = [
        (53, 38),
        (54, 39),
        (55, 40),
        (56, 40),
        (57, 41),
        (58, 41),
        (59, 41),
    ];
    for (x, y) in smile {
        p.set(x, y, ink);
        p.set(119 - x, y, ink);
    }
}

/// Olhos abertos com brilho e boca aberta.
fn cheering_face(p: &mut Pixmap) {
    let ink = rgb(CLOUD_INK);
    for left in [47_u16, 70] {
        fill(p, (left, 28), (left + 4, 33), ink);
        p.set(left + 1, 29, rgb(0xff_ff_ff));
    }
    fill(p, (53, 37), (67, 38), ink);
    for (y, inset) in [(38_u16, 0_u16), (39, 0), (40, 1), (41, 2), (42, 4)] {
        fill(p, (53 + inset, y), (67 - inset, y + 1), ink);
    }
    fill(p, (55, 38), (65, 40), rgb(0xe0_55_9f));
    fill(p, (57, 40), (63, 42), rgb(0xff_8f_b8));
}

/// Árvore em duas camadas do mesmo tamanho: tronco e copa (a copa balança com o vento).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tree {
    pub trunk: Pixmap,
    pub crown: Pixmap,
}

/// Árvore redonda; `blossom` faz a versão de flores cor-de-rosa.
#[must_use]
pub fn tree(blossom: bool) -> Tree {
    let mut trunk = Pixmap::new(56, 84);
    fill(&mut trunk, (24, 40), (33, 82), rgb(0x9a_63_30));
    fill(&mut trunk, (29, 44), (31, 80), rgb(0x7a_4a_22));
    let (base, light, dark, outline) = if blossom {
        (0xff_b3_d9, 0xff_dc_ee, 0xf0_86_bd, 0xa8_4a_7c)
    } else {
        (0x4f_bf_45, 0x8b_e0_6a, 0x35_96_3a, 0x1f_5e_2a)
    };
    let mut crown = Pixmap::new(56, 84);
    let puffs = [
        ((28.0, 25.0), 18.0),
        ((15.0, 37.0), 12.5),
        ((42.0, 37.0), 12.5),
        ((28.0, 42.0), 13.0),
    ];
    for (c, r) in puffs {
        disc(&mut crown, c, r, rgb(base));
    }
    shade(
        &mut crown,
        rgb(base),
        (rgb(light), rgb(dark)),
        (28.0, 30.0),
        18.0,
    );
    if blossom {
        let petals = [
            (17, 30),
            (35, 20),
            (23, 40),
            (42, 33),
            (30, 12),
            (12, 38),
            (36, 44),
        ];
        for (x, y) in petals {
            crown.set(x, y, rgb(0xff_ff_ff));
            crown.set(x + 1, y, rgb(0xff_ff_ff));
        }
    }
    Tree {
        trunk: trunk.outlined(rgb(0x5a_34_14)),
        crown: crown.outlined(rgb(outline)),
    }
}

/// Quadros do passarinho: quatro batendo as asas e um planando.
pub const BIRD_FRAMES: u8 = 5;
/// Ângulo da asa em cada quadro (radianos; positivo = para cima).
const BIRD_WINGS: [f32; 5] = [1.1, 0.3, -0.85, -0.15, 0.12];

/// Passarinho azul de perfil (olhando para a direita): corpo gordinho, barriga creme, cauda
/// de duas penas, bico laranja, olho e bochecha; a asa muda de quadro para quadro.
#[must_use]
pub fn bird(frame: u8) -> Pixmap {
    let mut p = Pixmap::new(24, 22);
    let (blue, dark, light) = (rgb(0x5a_a9_e6), rgb(0x3b_78_b8), rgb(0x9f_d4_f7));
    feather(&mut p, (8.0, 13.0), (2.0, 10.0), 2.6, dark);
    feather(&mut p, (8.0, 13.0), (2.5, 15.0), 2.4, dark);
    for (x, y) in p.coords() {
        let (fx, fy) = (f32::from(x) + 0.5, f32::from(y) + 0.5);
        let body = ((fx - 11.0) / 6.0).hypot((fy - 13.5) / 4.2) <= 1.0;
        let head = (fx - 16.0).hypot(fy - 10.0) <= 3.8;
        if body || head {
            let belly = fy > 14.0 && fx > 9.0;
            p.set(x, y, if belly { rgb(0xff_f2_d6) } else { blue });
        }
    }
    for (x, y) in [(20, 9), (21, 10), (20, 10), (19, 10), (20, 11)] {
        p.set(x, y, rgb(0xff_a6_3c));
    }
    p.set(17, 9, rgb(0x2a_24_40));
    p.set(16, 11, rgb(0xff_a3_c8));
    let angle = BIRD_WINGS
        .get(usize::from(frame % BIRD_FRAMES))
        .copied()
        .unwrap_or(0.0);
    let tip = (
        8.0f32.mul_add(-angle.cos(), 11.0),
        8.0f32.mul_add(-angle.sin(), 12.0),
    );
    feather(&mut p, (11.5, 12.0), tip, 3.6, dark);
    feather(&mut p, tip, tip, 1.6, light);
    p.outlined(rgb(0x24_36_5c))
}

/// Pena (cápsula) de `from` a `to`.
fn feather(p: &mut Pixmap, from: (f32, f32), to: (f32, f32), width: f32, color: Rgba) {
    let (vx, vy) = (to.0 - from.0, to.1 - from.1);
    let length = vx.hypot(vy).max(f32::EPSILON);
    for (x, y) in p.coords() {
        let (dx, dy) = (f32::from(x) + 0.5 - from.0, f32::from(y) + 0.5 - from.1);
        let along = (dx.mul_add(vx, dy * vy) / length).clamp(0.0, length);
        let gap = (vx / length)
            .mul_add(-along, dx)
            .hypot((vy / length).mul_add(-along, dy));
        if gap <= width / 2.0 {
            p.set(x, y, color);
        }
    }
}

/// Pétala de flor de cerejeira, caindo.
#[must_use]
pub fn petal() -> Pixmap {
    let mut p = Pixmap::new(3, 3);
    for (x, y) in [(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)] {
        p.set(x, y, rgb(0xff_c6_e2));
    }
    p.set(1, 1, rgb(0xff_9c_cf));
    p
}

/// Arbusto baixo.
#[must_use]
pub fn bush() -> Pixmap {
    let mut p = Pixmap::new(42, 22);
    let base = rgb(0x4f_bf_45);
    for (c, r) in [
        ((12.0, 14.0), 8.5),
        ((21.0, 10.5), 10.0),
        ((31.0, 14.0), 8.5),
    ] {
        disc(&mut p, c, r, base);
    }
    fill(&mut p, (5, 14), (38, 21), base);
    shade(
        &mut p,
        base,
        (rgb(0x8b_e0_6a), rgb(0x35_96_3a)),
        (21.0, 14.0),
        11.0,
    );
    p.outlined(rgb(0x1f_5e_2a))
}

/// Terra em xadrez, repetida a cada [`GROUND_TILE`] pixels; a altura cobre a tela.
#[must_use]
pub fn soil() -> Pixmap {
    let mut p = Pixmap::new(GROUND_TILE, VIEW_H_PX);
    let (a, b) = (rgb(0xc8_74_2e), rgb(0xa8_5a_22));
    let dot = rgb(0x8f_4a_1a);
    let dots = [(6, 8), (7, 8), (18, 19), (18, 20), (13, 3)];
    for (x, y) in p.coords() {
        let odd = (x / CHECKER + y / CHECKER) % 2 == 1;
        p.set(x, y, if odd { b } else { a });
        if dots.contains(&(x % CHECKER, y % CHECKER)) {
            p.set(x, y, dot);
        }
    }
    p
}

/// Faixa de grama: pontas das folhas, gramado, borda escura e sombra sobre a terra.
#[must_use]
pub fn grass() -> Pixmap {
    const TIPS: [u16; 12] = [5, 2, 3, 1, 4, 2, 0, 3, 5, 1, 2, 4];
    let mut p = Pixmap::new(GROUND_TILE, 22);
    let rows: [(u16, u16, u32); 5] = [
        (5, 8, 0x8b_e0_5a),
        (8, 14, 0x57_c4_43),
        (14, 16, 0x46_ad_38),
        (16, 18, 0x2f_8a_2f),
        (18, 22, 0x8a_44_18),
    ];
    for (y0, y1, color) in rows {
        fill(&mut p, (0, y0), (GROUND_TILE, y1), rgb(color));
    }
    let tip = rgb(0x8b_e0_5a);
    for x in 0..GROUND_TILE {
        let height = TIPS.get(usize::from(x % 12)).copied().unwrap_or(0);
        for y in (5 - height)..5 {
            p.set(x, y, tip);
        }
        if x % 7 == 3 {
            p.set(x, 10, rgb(0x46_ad_38));
            p.set(x + 1, 11, rgb(0x46_ad_38));
        }
    }
    p
}

#[cfg(test)]
mod tests {
    use super::*;

    fn opaque(p: &Pixmap) -> usize {
        p.coords().filter(|(x, y)| p.is_opaque(*x, *y)).count()
    }

    #[test]
    fn imagens_tem_conteudo_e_tamanhos_esperados() {
        let all = [
            heart(21, 19),
            sun(0),
            sun(1),
            rainbow(),
            cloud(0),
            cloud(1),
            cloud(2),
            tree(false).crown,
            tree(true).crown,
            tree(false).trunk,
            bush(),
        ];
        for p in all {
            assert!(
                opaque(&p) > 100,
                "imagem quase vazia {}x{}",
                p.width(),
                p.height()
            );
        }
        assert_eq!(soil().height(), VIEW_H_PX);
        assert_eq!(soil().width(), GROUND_TILE);
        assert_eq!(grass().width(), GROUND_TILE);
    }

    #[test]
    fn coracao_e_simetrico_e_tem_o_tamanho_pedido() {
        let h = heart(21, 19);
        assert_eq!((h.width(), h.height()), (23, 21));
        // Simétrico em silhueta (o brilho fica só de um lado).
        for y in 0..h.height() {
            for x in 0..h.width() {
                assert_eq!(
                    h.is_opaque(x, y),
                    h.is_opaque(h.width() - 1 - x, y),
                    "({x},{y})"
                );
            }
        }
        // Ponta embaixo, dois lóbulos em cima com um vão no meio.
        assert!(h.is_opaque(11, 18));
        assert!(h.is_opaque(6, 3) && h.is_opaque(16, 3));
    }

    #[test]
    fn raios_do_sol_giram_suavemente_e_fecham_o_ciclo() {
        let frames: Vec<Pixmap> = (0..SUN_FRAMES).map(sun).collect();
        assert!(frames.windows(2).all(|w| w[0] != w[1]));
        assert_eq!(sun(SUN_FRAMES), frames[0]);
    }

    #[test]
    fn sol_sem_mancha_perto_dos_olhos() {
        let s = sun(0);
        let shine = rgb(0xff_f3_a8);
        // Nenhum pixel do reflexo na região dos olhos, bochechas e sorriso.
        for y in 30..46 {
            for x in 22..51 {
                assert_ne!(s.get(x, y), Some(shine), "reflexo no rosto em ({x},{y})");
            }
        }
        assert!(
            s.coords().any(|(x, y)| s.get(x, y) == Some(shine)),
            "o reflexo existe"
        );
    }

    #[test]
    fn passarinho_bate_as_asas() {
        let frames: Vec<Pixmap> = (0..BIRD_FRAMES).map(bird).collect();
        assert!(frames.windows(2).all(|w| w[0] != w[1]));
        // Asa para cima passa do topo do corpo; asa para baixo passa da barriga.
        let top = |p: &Pixmap| {
            p.coords()
                .filter(|(x, y)| p.is_opaque(*x, *y))
                .map(|(_, y)| y)
                .min()
        };
        let bottom = |p: &Pixmap| {
            p.coords()
                .filter(|(x, y)| p.is_opaque(*x, *y))
                .map(|(_, y)| y)
                .max()
        };
        assert!(top(&frames[0]) < top(&frames[2]));
        assert!(bottom(&frames[2]) > bottom(&frames[0]));
    }

    /// Maior trecho contínuo da linha de baixo da silhueta: numa base redonda, é curto.
    fn longest_bottom_run(p: &Pixmap) -> usize {
        let bottom = p
            .coords()
            .filter(|(x, y)| p.is_opaque(*x, *y))
            .map(|(_, y)| y)
            .max()
            .unwrap();
        (0..p.width())
            .map(|x| p.is_opaque(x, bottom))
            .fold((0, 0), |(best, run), on| {
                if on {
                    (best.max(run + 1), run + 1)
                } else {
                    (best, 0)
                }
            })
            .0
    }

    #[test]
    fn nuvens_tem_a_base_redonda() {
        for cloud in [cloud(0), cloud(1), cloud(2), smiley_cloud(CloudFace::Happy)] {
            let width = usize::from(cloud.width());
            assert!(
                longest_bottom_run(&cloud) * 6 < width,
                "base reta numa nuvem de {width} px"
            );
        }
    }

    #[test]
    fn nuvens_da_festa_tem_carinhas_diferentes() {
        let (happy, cheering) = (
            smiley_cloud(CloudFace::Happy),
            smiley_cloud(CloudFace::Cheering),
        );
        assert_ne!(happy, cheering);
        assert_eq!(
            (happy.width(), happy.height()),
            (cheering.width(), cheering.height())
        );
        let ink = rgb(CLOUD_INK);
        for cloud in [&happy, &cheering] {
            assert!(
                cloud
                    .coords()
                    .filter(|(x, y)| cloud.get(*x, *y) == Some(ink))
                    .count()
                    > 15
            );
        }
    }

    #[test]
    fn tronco_e_copa_se_encaixam() {
        let t = tree(true);
        assert_eq!(
            (t.trunk.width(), t.trunk.height()),
            (t.crown.width(), t.crown.height())
        );
        // O topo do tronco fica escondido sob a copa.
        assert!(t.crown.is_opaque(28, 42) && t.trunk.is_opaque(28, 42));
        assert!(petal().is_opaque(1, 1));
    }

    #[test]
    fn arco_iris_tem_sete_cores_e_buraco_no_meio() {
        let r = rainbow();
        for color in RAINBOW_BANDS {
            assert!(r.coords().any(|(x, y)| r.get(x, y) == Some(rgb(color))));
        }
        assert!(!r.is_opaque(111, 100), "o centro deve ser vazio");
    }

    #[test]
    fn terra_e_grama_cobrem_o_padrao_inteiro() {
        let s = soil();
        assert_eq!(opaque(&s), usize::from(s.width()) * usize::from(s.height()));
        let g = grass();
        assert!((0..g.width()).all(|x| (5..g.height()).all(|y| g.is_opaque(x, y))));
    }

    #[test]
    fn xadrez_repete_sem_emenda() {
        // A textura é repetida lado a lado: a largura precisa conter quadrados inteiros.
        assert_eq!(GROUND_TILE % (2 * CHECKER), 0);
        let s = soil();
        assert_ne!(s.get(0, 0), s.get(CHECKER, 0));
        assert_eq!(s.get(0, 0), s.get(CHECKER, CHECKER));
    }
}

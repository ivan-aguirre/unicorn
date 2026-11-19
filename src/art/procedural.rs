//! Imagens geradas por código: sol, arco-íris, nuvens, árvores, arbustos e texturas do chão.

use super::pixmap::{Pixmap, Rgba, rgb};
use crate::view::VIEW_H_PX;

/// Altura da faixa de grama acima da superfície do chão (pontas das folhas).
pub const GRASS_TIP: f32 = 3.0;
/// Largura do padrão repetido do chão.
pub const GROUND_TILE: u16 = 32;
/// Lado de cada quadrado do xadrez de terra.
const CHECKER: u16 = 16;

/// Cores das bandas do arco-íris, de fora para dentro.
const RAINBOW_BANDS: [u32; 7] = [
    0xff_5a_6e, 0xff_a0_4a, 0xff_e1_4d, 0x6f_d3_6a, 0x4f_b8_ff, 0x6a_7c_ff, 0xb0_7c_ff,
];
const RAINBOW_OUTER: f32 = 66.0;
const RAINBOW_BAND: f32 = 4.0;

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

/// Sombreia uma copa/nuvem: luz em cima à esquerda, sombra embaixo à direita.
fn shade(p: &mut Pixmap, base: Rgba, light: Rgba, dark: Rgba, (cx, cy): (f32, f32)) {
    for (x, y) in p.coords() {
        if p.get(x, y) != Some(base) {
            continue;
        }
        let (dx, dy) = (f32::from(x) - cx, f32::from(y) - cy);
        if dx + dy < -9.0 {
            p.set(x, y, light);
        } else if dx.mul_add(0.4, dy) > 5.0 {
            p.set(x, y, dark);
        }
    }
}

/// Sol sorridente com raios; `frame` gira os raios meio passo (cintilar).
#[must_use]
pub fn sun(frame: u8) -> Pixmap {
    let mut p = Pixmap::new(44, 44);
    let center = (22.0, 22.0);
    let turn = f32::from(frame % 2) * std::f32::consts::FRAC_PI_8;
    for (x, y) in p.coords() {
        let (dx, dy) = (f32::from(x) + 0.5 - center.0, f32::from(y) + 0.5 - center.1);
        let d = dx.hypot(dy);
        let sector = (dy.atan2(dx) - turn).rem_euclid(std::f32::consts::FRAC_PI_4);
        let on_ray = (sector - std::f32::consts::FRAC_PI_8).abs() < 0.2;
        if (15.0..20.5).contains(&d) && on_ray {
            p.set(x, y, rgb(0xff_c9_3c));
        }
    }
    disc(&mut p, center, 13.0, rgb(0xf7_a5_31));
    disc(&mut p, center, 12.0, rgb(0xff_e1_4d));
    disc(&mut p, (19.0, 18.0), 5.0, rgb(0xff_f3_a8));
    sun_face(&mut p);
    p
}

/// Olhinhos fechados e felizes, bochechas e sorriso.
fn sun_face(p: &mut Pixmap) {
    let ink = rgb(0x8a_4a_1c);
    let blush = rgb(0xff_9e_7a);
    for (x, y) in [(16, 21), (17, 20), (18, 21), (25, 21), (26, 20), (27, 21)] {
        p.set(x, y, ink);
    }
    for (x, y) in [
        (18, 25),
        (19, 26),
        (20, 26),
        (21, 26),
        (22, 26),
        (23, 26),
        (24, 26),
        (25, 25),
    ] {
        p.set(x, y, ink);
    }
    for (x, y) in [(14, 24), (15, 24), (28, 24), (29, 24)] {
        p.set(x, y, blush);
    }
}

/// Arco-íris de sete cores (meio círculo), apoiado na base da imagem.
#[must_use]
pub fn rainbow() -> Pixmap {
    let mut p = Pixmap::new(134, 68);
    let (cx, cy) = (67.0, 67.5);
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
        (50, 22),
        &[
            ((12.0, 14.0), 7.0),
            ((22.0, 10.0), 9.0),
            ((33.0, 11.0), 8.0),
            ((41.0, 15.0), 6.0),
        ],
    ),
    (
        (34, 16),
        &[((9.0, 10.0), 5.5), ((17.0, 7.5), 6.5), ((26.0, 10.0), 5.5)],
    ),
    (
        (66, 26),
        &[
            ((12.0, 17.0), 7.0),
            ((22.0, 12.0), 9.0),
            ((35.0, 10.0), 10.0),
            ((48.0, 13.0), 8.0),
            ((57.0, 18.0), 6.0),
        ],
    ),
];

/// Nuvem fofa de um dos três formatos.
#[must_use]
pub fn cloud(kind: usize) -> Pixmap {
    let ((w, h), puffs) = CLOUDS.get(kind).copied().unwrap_or(CLOUDS[0]);
    let mut p = Pixmap::new(w, h);
    let white = rgb(0xff_ff_ff);
    for &(center, r) in puffs {
        disc(&mut p, center, r, white);
    }
    fill(&mut p, (5, h - 8), (w - 5, h - 2), white);
    let shadow = rgb(0xdc_ee_ff);
    for (x, y) in p.coords() {
        if y + 5 >= h && p.is_opaque(x, y) {
            p.set(x, y, shadow);
        }
    }
    p.outlined(rgb(0xb8_dc_ff))
}

/// Árvore redonda; `blossom` faz a versão de flores cor-de-rosa.
#[must_use]
pub fn tree(blossom: bool) -> Pixmap {
    let mut p = Pixmap::new(34, 50);
    fill(&mut p, (15, 28), (20, 49), rgb(0x9a_63_30));
    fill(&mut p, (18, 30), (19, 48), rgb(0x7a_4a_22));
    let (base, light, dark, outline) = if blossom {
        (0xff_b3_d9, 0xff_dc_ee, 0xf0_86_bd, 0xa8_4a_7c)
    } else {
        (0x4f_bf_45, 0x8b_e0_6a, 0x35_96_3a, 0x1f_5e_2a)
    };
    for (c, r) in [
        ((17.0, 15.0), 11.0),
        ((9.0, 22.0), 7.5),
        ((25.0, 22.0), 7.5),
        ((17.0, 25.0), 8.0),
    ] {
        disc(&mut p, c, r, rgb(base));
    }
    shade(&mut p, rgb(base), rgb(light), rgb(dark), (17.0, 18.0));
    if blossom {
        for (x, y) in [(10, 18), (21, 12), (14, 24), (25, 20), (18, 7), (7, 23)] {
            p.set(x, y, rgb(0xff_ff_ff));
        }
    }
    p.outlined(rgb(outline))
}

/// Arbusto baixo.
#[must_use]
pub fn bush() -> Pixmap {
    let mut p = Pixmap::new(26, 14);
    let base = rgb(0x4f_bf_45);
    for (c, r) in [((7.0, 9.0), 5.5), ((13.0, 7.0), 6.5), ((19.0, 9.0), 5.5)] {
        disc(&mut p, c, r, base);
    }
    fill(&mut p, (3, 9), (23, 13), base);
    shade(&mut p, base, rgb(0x8b_e0_6a), rgb(0x35_96_3a), (13.0, 9.0));
    p.outlined(rgb(0x1f_5e_2a))
}

/// Terra em xadrez, repetida a cada [`GROUND_TILE`] pixels; a altura cobre a tela.
#[must_use]
pub fn soil() -> Pixmap {
    let mut p = Pixmap::new(GROUND_TILE, VIEW_H_PX);
    let (a, b) = (rgb(0xc8_74_2e), rgb(0xa8_5a_22));
    let dot = rgb(0x8f_4a_1a);
    for (x, y) in p.coords() {
        let odd = (x / CHECKER + y / CHECKER) % 2 == 1;
        p.set(x, y, if odd { b } else { a });
        if (x % CHECKER, y % CHECKER) == (4, 5) || (x % CHECKER, y % CHECKER) == (11, 12) {
            p.set(x, y, dot);
        }
    }
    p
}

/// Faixa de grama: pontas das folhas, gramado, borda escura e sombra sobre a terra.
#[must_use]
pub fn grass() -> Pixmap {
    const TIPS: [u16; 8] = [3, 1, 2, 0, 2, 3, 1, 2];
    let mut p = Pixmap::new(GROUND_TILE, 14);
    let rows: [(u16, u16, u32); 4] = [
        (3, 5, 0x8b_e0_5a),
        (5, 9, 0x57_c4_43),
        (9, 11, 0x2f_8a_2f),
        (11, 14, 0x8a_44_18),
    ];
    for (y0, y1, color) in rows {
        fill(&mut p, (0, y0), (GROUND_TILE, y1), rgb(color));
    }
    let tip = rgb(0x8b_e0_5a);
    for x in 0..GROUND_TILE {
        let height = TIPS.get(usize::from(x % 8)).copied().unwrap_or(0);
        for y in (3 - height)..3 {
            p.set(x, y, tip);
        }
        if x % 5 == 2 {
            p.set(x, 6, rgb(0x46_ad_38));
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
        for p in [
            sun(0),
            sun(1),
            rainbow(),
            cloud(0),
            cloud(1),
            cloud(2),
            tree(false),
            tree(true),
            bush(),
        ] {
            assert!(
                opaque(&p) > 50,
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
    fn raios_do_sol_giram_entre_quadros() {
        assert_ne!(sun(0), sun(1));
    }

    #[test]
    fn arco_iris_tem_sete_cores_e_buraco_no_meio() {
        let r = rainbow();
        for color in RAINBOW_BANDS {
            assert!(r.coords().any(|(x, y)| r.get(x, y) == Some(rgb(color))));
        }
        assert!(!r.is_opaque(67, 60), "o centro deve ser vazio");
    }

    #[test]
    fn terra_e_grama_cobrem_o_padrao_inteiro() {
        let s = soil();
        assert_eq!(opaque(&s), usize::from(s.width()) * usize::from(s.height()));
        let g = grass();
        // Abaixo das pontas das folhas, tudo é preenchido.
        assert!((0..g.width()).all(|x| (3..g.height()).all(|y| g.is_opaque(x, y))));
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

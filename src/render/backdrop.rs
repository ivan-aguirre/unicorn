//! Fundo: o lago (gradiente de profundidade, ondas, reflexos) e as colinas de pedra com
//! grama em cima, onde nascem as cachoeiras.

use macroquad::color::Color;
use macroquad::shapes::draw_rectangle;

use super::ambient;
use super::scene::{MESA_BASE, SEA_Y, color, columns};
use crate::level::scenery::{Mesa, RANGES, Range};
use crate::view::{VIEW_H, VIEW_W};

/// Faixas de profundidade do lago: (topo, cor). Mais escuro quanto mais perto.
const WATER: [(f32, u32); 5] = [
    (SEA_Y, 0x9f_d4_ff),
    (SEA_Y + 5.0, 0x5c_a2_ec),
    (SEA_Y + 22.0, 0x4c_93_e4),
    (SEA_Y + 48.0, 0x42_88_dc),
    (SEA_Y + 90.0, 0x38_7b_d2),
];

/// Fileiras de cristas de onda: (altura, velocidade relativa, espaço entre cristas, largura).
/// As de perto são maiores e andam mais rápido (paralaxe).
const CRESTS: [(f32, f32, f32, f32); 7] = [
    (214.0, 0.9, 70.0, 6.0),
    (222.0, 1.0, 64.0, 8.0),
    (233.0, 1.15, 88.0, 10.0),
    (247.0, 1.3, 78.0, 12.0),
    (263.0, 1.5, 98.0, 14.0),
    (283.0, 1.75, 108.0, 16.0),
    (306.0, 2.0, 120.0, 18.0),
];

/// Altura abaixo da qual a cordilheira tem neve.
const SNOW_LINE: f32 = 142.0;

/// Cores de uma cadeia de montanhas: lado iluminado, lado na sombra e realce da crista.
struct Palette {
    light: u32,
    shadow: u32,
    ridge: u32,
}

const RANGE_COLORS: [Palette; 2] = [
    Palette {
        light: 0xb6_cc_ee,
        shadow: 0x95_ad_da,
        ridge: 0xd2_e0_f6,
    },
    Palette {
        light: 0x9a_d4_b2,
        shadow: 0x77_b6_96,
        ridge: 0xc0_e8_cf,
    },
];

/// Montanhas no horizonte: cordilheira azulada com neve nos picos e colinas verdes na frente,
/// com luz do lado direito (onde está o sol) e névoa clara na base.
pub fn draw_mountains(cam: f32) {
    for (range, palette) in RANGES.iter().zip(&RANGE_COLORS) {
        let offset = cam * range.parallax.factor();
        for x in columns(1) {
            mountain_column(range, palette, x, x + offset);
        }
    }
    // Névoa do horizonte: fica mais densa perto da água, em faixas finas (sem listras).
    for band in columns(2).take_while(|b| *b < 28.0) {
        let alpha = (band / 28.0).powi(2) * 0.45;
        draw_rectangle(
            0.0,
            SEA_Y - 28.0 + band,
            VIEW_W,
            2.0,
            Color::new(0.9, 0.95, 1.0, alpha),
        );
    }
}

/// Uma coluna de 1 pixel de uma cadeia de montanhas.
fn mountain_column(range: &Range, palette: &Palette, x: f32, lx: f32) {
    let top = range.top_at(lx).round();
    // Encosta virada para a direita (descendo) recebe a luz do sol.
    let lit = range.top_at(lx + 1.0) >= range.top_at(lx);
    let body = if lit { palette.light } else { palette.shadow };
    draw_rectangle(x, top, 1.0, range.base - top, color(body));
    if !range.rounded && top < SNOW_LINE {
        let jag = ((lx * 0.9).sin() * 3.0).round();
        let snow_end = (SNOW_LINE + jag).min(range.base);
        let snow = if lit { 0xff_ff_ff } else { 0xdc_e6_f8 };
        draw_rectangle(x, top, 1.0, snow_end - top, color(snow));
    } else if lit {
        draw_rectangle(x, top, 1.0, 1.0, color(palette.ridge));
    }
}

/// Lago: faixas de profundidade, espuma no horizonte e cristas de onda em movimento.
pub fn draw_water(offset: f32, time: f32) {
    for (i, (top, hex)) in WATER.iter().enumerate() {
        let bottom = WATER.get(i + 1).map_or(VIEW_H, |(next, _)| *next);
        draw_rectangle(0.0, *top, VIEW_W, bottom - top, color(*hex));
    }
    draw_rectangle(0.0, SEA_Y, VIEW_W, 2.0, color(0xe4_f4_ff));
    for (row, (y, speed, gap, len)) in CRESTS.iter().enumerate() {
        let phase = f32::from(u8::try_from(row).unwrap_or(0));
        let drift = time.mul_add(6.0 * speed, offset * speed);
        let wobble = time.mul_add(1.3, phase).sin() * 3.0;
        let start = (wobble - drift).rem_euclid(*gap) - gap;
        let xs = columns(1)
            .map(|k| gap.mul_add(k, start))
            .take_while(|x| *x < VIEW_W);
        for x in xs {
            crest(phase.mul_add(7.0, x.round()), *y, *len);
        }
    }
}

/// Crista de onda em "︶": linha clara com as pontinhas erguidas.
fn crest(x: f32, y: f32, len: f32) {
    let light = Color::new(0.85, 0.94, 1.0, 0.85);
    draw_rectangle(x, y, len, 1.0, light);
    draw_rectangle(x - 2.0, y - 1.0, 2.0, 1.0, light);
    draw_rectangle(x + len, y - 1.0, 2.0, 1.0, light);
}

/// Colina de pedra: topo ondulado com grama (e tufos pendurados), rocha em camadas com fendas,
/// luz à esquerda, sombra à direita, reflexo no lago e, se houver, a cachoeira.
pub fn draw_mesa(m: &Mesa, sx: f32, time: f32) {
    reflection(m, sx, time);
    let seed = m.x * 0.0137;
    for dx in columns(1).take_while(|dx| *dx < m.w) {
        cliff_column(m, sx + dx, dx, seed);
    }
    waterline(m, sx, time);
    if let Some(w) = m.waterfall {
        ambient::draw_waterfall(sx + w.offset, w.w, m.waterfall_top(), time);
    }
}

/// Uma coluna de 1 pixel da colina.
fn cliff_column(m: &Mesa, x: f32, dx: f32, seed: f32) {
    let top = m.top_at(dx).round();
    let edge_light = dx < 4.0;
    let edge_dark = dx > m.w - 6.0;
    let rock = match (edge_light, edge_dark) {
        (true, _) => 0x6c_b8_90,
        (_, true) => 0x3a_76_5a,
        _ => 0x4f_9c_78,
    };
    draw_rectangle(x, top, 1.0, MESA_BASE - top, color(rock));
    // Camadas da rocha, levemente onduladas.
    let strata = color(0x44_8a_6a);
    let layers = columns(13)
        .map(|k| m.top + 16.0 + k)
        .take_while(|y| *y < MESA_BASE - 3.0);
    for y in layers {
        let wave = dx.mul_add(0.12, y * 0.3).sin() * 2.0;
        draw_rectangle(x, (y + wave).round(), 1.0, 1.0, strata);
    }
    // Fendas verticais aqui e ali.
    if dx.mul_add(0.71, seed).sin() > 0.97 {
        let depth = 22.0f32.mul_add(dx.mul_add(0.37, seed).sin().abs(), 14.0);
        draw_rectangle(x, top + 12.0, 1.0, depth, color(0x35_6e_54));
    }
    grass_cap(x, dx, top, seed);
}

/// Grama no topo: linha clara em cima, gramado, borda escura e tufos pendurados.
fn grass_cap(x: f32, dx: f32, top: f32, seed: f32) {
    let hang = dx.mul_add(1.7, seed).sin();
    let tuft = if hang > 0.55 {
        3.0
    } else if hang > 0.1 {
        1.0
    } else {
        0.0
    };
    draw_rectangle(x, top, 1.0, 6.0 + tuft, color(0x6f_cf_72));
    draw_rectangle(x, top, 1.0, 1.0, color(0xa8_ec_9c));
    draw_rectangle(x, top + 6.0 + tuft, 1.0, 1.0, color(0x2f_7a_52));
}

/// Espuma ondulante onde a pedra encontra a água (o pé da colina não fica reto).
fn waterline(m: &Mesa, sx: f32, time: f32) {
    let foam = Color::new(1.0, 1.0, 1.0, 0.8);
    for dx in columns(1).take_while(|dx| *dx < m.w + 4.0) {
        let lap = time.mul_add(2.5, dx * 0.45).sin();
        if lap > -0.3 {
            draw_rectangle(sx + dx - 2.0, MESA_BASE - 1.0 + lap.round(), 1.0, 1.0, foam);
        }
    }
}

/// Reflexo da colina no lago, ondulando fileira a fileira.
fn reflection(m: &Mesa, sx: f32, time: f32) {
    let shade = Color::new(0.16, 0.38, 0.36, 0.32);
    for row in columns(2).take_while(|row| *row < 26.0) {
        let wobble = (time.mul_add(2.0, row * 0.7).sin() * 2.0).round();
        let inset = row.mul_add(0.4, 3.0);
        draw_rectangle(
            sx + inset + wobble,
            MESA_BASE + row,
            2.0f32.mul_add(-inset, m.w),
            1.0,
            shade,
        );
        if let Some(w) = m.waterfall {
            let streak = Color::new(1.0, 1.0, 1.0, 0.35 * (1.0 - row / 26.0));
            draw_rectangle(sx + w.offset + wobble, MESA_BASE + row, w.w, 1.0, streak);
        }
    }
}

//! Desenho da vida no cenário: pássaros, peixes, brilhos no lago, cachoeiras animadas,
//! brilhos no arco-íris, pétalas e borboletas. As trajetórias vêm de [`crate::ambient`].

use macroquad::color::{Color, WHITE};
use macroquad::math::Vec2;
use macroquad::shapes::{draw_circle, draw_ellipse_lines, draw_rectangle};
use macroquad::texture::{DrawTextureParams, Texture2D, draw_texture_ex};

use super::scene::{MESA_BASE, SEA_Y, color, draw_centered, offsets};
use super::textures::Textures;
use crate::ambient::{
    FLIGHTS, Fish, bird, butterfly, fish, fish_arc, fish_landing, flight_size, petal, twinkle,
};
use crate::level::scenery::FAR_PARALLAX;
use crate::view::{VIEW_H, VIEW_W};

/// Peixes no lago (um a cada `FISH_GAP` pixels da camada distante).
const FISH_SLOTS: u16 = 24;
const FISH_GAP: f32 = 330.0;
/// Brilhos no lago (um a cada `GLINT_GAP` pixels da camada distante).
const GLINT_SLOTS: u16 = 90;
const GLINT_GAP: f32 = 83.0;
/// Pétalas por árvore florida.
const PETALS: u8 = 7;

const fn white(alpha: f32) -> Color {
    Color::new(1.0, 1.0, 1.0, alpha.clamp(0.0, 1.0))
}

fn draw_at(t: &Texture2D, at: Vec2, params: DrawTextureParams) {
    draw_texture_ex(t, at.x.round(), at.y.round(), WHITE, params);
}

/// Passarinhos voando sozinhos ou em grupinhos, batendo as asas e planando.
pub fn draw_birds(tex: &Textures, cam: f32, time: f32) {
    for flight in 0..FLIGHTS {
        for member in 0..flight_size(flight) {
            let b = bird(flight, member, time, cam);
            if let Some(t) = tex.bird.get(usize::from(b.frame)) {
                let params = DrawTextureParams {
                    flip_x: b.facing_left,
                    ..Default::default()
                };
                draw_at(t, b.at - t.size() / 2.0, params);
            }
        }
    }
}

/// Brilhos que piscam na superfície do lago.
pub fn draw_glints(cam: f32, time: f32) {
    let left = cam * FAR_PARALLAX.factor();
    for slot in 0..GLINT_SLOTS {
        let k = f32::from(slot);
        let x = k.mul_add(GLINT_GAP, (k * 1.9).sin() * 30.0) - left;
        if !(-4.0..VIEW_W + 4.0).contains(&x) {
            continue;
        }
        let y = (k * 2.7).sin().mul_add(0.5, 0.5).mul_add(70.0, SEA_Y + 9.0);
        let glow = twinkle(k, time);
        if glow > 0.05 {
            let (x, y) = (x.round(), y.round());
            draw_rectangle(x - 2.0, y, 5.0, 1.0, white(glow));
            draw_rectangle(x, y - 2.0, 1.0, 5.0, white(glow));
        }
    }
}

/// Peixinhos pulando no lago, com respingo na saída e ondinhas na volta.
pub fn draw_fish(tex: &Textures, cam: f32, time: f32) {
    let left = cam * FAR_PARALLAX.factor();
    for slot in 0..FISH_SLOTS {
        let k = f32::from(slot);
        let start = Vec2::new(
            k.mul_add(FISH_GAP, 140.0) - left,
            (k % 3.0).mul_add(11.0, SEA_Y + 24.0),
        );
        if !(-60.0..VIEW_W + 20.0).contains(&start.x) {
            continue;
        }
        match fish(k, time) {
            Fish::Jumping(p) => jumping_fish(tex, start, p),
            Fish::Ripple(r) => ripple(fish_landing(start), r),
            Fish::Hidden => {}
        }
    }
}

fn jumping_fish(tex: &Textures, start: Vec2, progress: f32) {
    let (at, angle) = fish_arc(start, progress);
    let size = tex.fish.size();
    let params = DrawTextureParams {
        rotation: angle,
        ..Default::default()
    };
    draw_at(&tex.fish, at - size / 2.0, params);
    if progress < 0.2 {
        // Respingo: gotinhas subindo de onde o peixe saiu.
        let rise = progress * 40.0;
        for dx in [-4.0, 0.0, 4.0] {
            draw_rectangle(
                (start.x + dx).round(),
                (start.y - rise - dx.abs()).round(),
                2.0,
                2.0,
                white(1.0),
            );
        }
    }
}

fn ripple(at: Vec2, growth: f32) {
    let alpha = 1.0 - growth;
    let w = 8.0f32.mul_add(growth, 4.0);
    draw_ellipse_lines(at.x, at.y, w, w * 0.35, 0.0, 1.0, white(alpha));
    draw_ellipse_lines(at.x, at.y, w * 0.5, w * 0.18, 0.0, 1.0, white(alpha * 0.8));
}

/// Cachoeira: a água passa curvando pela borda, abre um pouco ao cair, desce em três
/// faixas de velocidades diferentes com brilhos, e na base há pedras, espuma pulando, gotas
/// saltando e névoa.
pub fn draw_waterfall(left: f32, width: f32, top: f32, time: f32) {
    let height = MESA_BASE - top;
    fall_body(left, width, top, height);
    fall_lanes(left, width, top, height, time);
    fall_sparkles(left, width, (top, height), time);
    // Borda curva: a água "rola" por cima da pedra antes de cair.
    draw_rectangle(left - 3.0, top - 2.0, width + 6.0, 3.0, WHITE);
    draw_rectangle(left - 1.0, top - 3.0, width + 2.0, 1.0, color(0xe4_f6_ff));
    draw_rectangle(left - 4.0, top, 2.0, 2.0, color(0xc4_ec_ff));
    draw_rectangle(left + width + 2.0, top, 2.0, 2.0, color(0xc4_ec_ff));
    waterfall_foot(
        left + width / 2.0,
        spread_at(height).mul_add(2.0, width),
        time,
    );
}

/// Quanto a água já se abriu (para cada lado) depois de cair `drop` pixels.
fn spread_at(drop: f32) -> f32 {
    (drop / 16.0).floor()
}

/// Corpo da queda: mais claro no alto, abrindo um pouco e com as bordas mais escuras.
fn fall_body(left: f32, width: f32, top: f32, height: f32) {
    for row in offsets(2, height) {
        let spread = spread_at(row);
        let body = if row < 8.0 { 0xc8_f0_ff } else { 0x8f_d6_ff };
        let (x, w) = (left - spread, 2.0f32.mul_add(spread, width));
        draw_rectangle(x, top + row, w, 2.0, color(body));
        draw_rectangle(x, top + row, 2.0, 2.0, color(0x5f_b8_f0));
        draw_rectangle(x + w - 2.0, top + row, 2.0, 2.0, color(0x5f_b8_f0));
    }
}

/// Três faixas de água descendo em velocidades diferentes.
fn fall_lanes(left: f32, width: f32, top: f32, height: f32, time: f32) {
    let lane = width / 3.0;
    let lanes: [(f32, f32, usize, u32); 3] = [
        (0.0, 70.0, 12, 0xc4_ec_ff),
        (1.0, 105.0, 17, 0xff_ff_ff),
        (2.0, 85.0, 14, 0xc4_ec_ff),
    ];
    for (i, speed, gap, hex) in lanes {
        for offset in offsets(gap, height) {
            let drop = time
                .mul_add(speed, i.mul_add(5.0, offset))
                .rem_euclid(height);
            let spread = spread_at(drop);
            let x = (i - 1.0).mul_add(spread, i.mul_add(lane, left + 1.0));
            draw_rectangle(
                x.round(),
                (top + drop).round(),
                (lane - 1.0).round(),
                3.0,
                color(hex),
            );
        }
    }
}

/// Brilhos descendo junto com a água.
fn fall_sparkles(left: f32, width: f32, (top, height): (f32, f32), time: f32) {
    for j in [0.0f32, 1.0, 2.0] {
        let drop = time.mul_add(60.0, j * height / 3.0).rem_euclid(height);
        let x = width.mul_add(j.mul_add(0.25, 0.25), left).round();
        let glow = twinkle(j.mul_add(2.1, left), time);
        if glow > 0.1 {
            draw_rectangle(x, (top + drop).round(), 1.0, 2.0, white(glow));
            draw_rectangle(
                x - 1.0,
                (top + drop).round() + 1.0,
                3.0,
                1.0,
                white(glow * 0.7),
            );
        }
    }
}

fn waterfall_foot(center: f32, width: f32, time: f32) {
    // Pedras arredondadas dos dois lados da queda.
    for (dx, r) in [(-0.5f32, 5.0f32), (0.5, 4.0)] {
        let x = dx.signum().mul_add(4.0, dx.mul_add(width, center));
        draw_circle(x, MESA_BASE + 1.0, r, color(0x7d_8a_a0));
        draw_circle(x - 1.0, MESA_BASE - 1.0, r - 2.0, color(0xa8_b4_c8));
    }
    // Névoa: nuvenzinhas que sobem, crescem e somem.
    for j in [0.0f32, 1.0, 2.0] {
        let p = time.mul_add(0.45, j / 3.0).rem_euclid(1.0);
        let x = p.mul_add(5.0, (j - 1.0).mul_add(width * 0.5, center));
        let y = 18.0f32.mul_add(-p, MESA_BASE - 6.0);
        draw_circle(x, y, 4.0f32.mul_add(p, 4.0), white(0.55 * (1.0 - p)));
    }
    foam_and_drops(center, width, time);
}

/// Bolhas de espuma pulando e gotas saltando em arco para os lados.
fn foam_and_drops(center: f32, width: f32, time: f32) {
    for j in [0.0f32, 1.0, 2.0, 3.0, 4.0, 5.0] {
        let x = (j - 2.5).mul_add((width + 8.0) / 5.0, center);
        let hop = time.mul_add(5.0, j * 1.3).sin().abs() * 4.0;
        draw_circle(
            x.round(),
            (MESA_BASE - 2.0 - hop).round(),
            3.0 + (j % 2.0),
            WHITE,
        );
        let p = time.mul_add(0.9, j / 6.0).rem_euclid(1.0);
        let side = if j % 2.0 < 1.0 { -1.0 } else { 1.0 };
        let arc = 4.0 * p * (1.0 - p);
        let drop_x = (side * p).mul_add(14.0, x);
        let drop_y = 14.0f32.mul_add(-arc, MESA_BASE - 4.0);
        draw_rectangle(drop_x.round(), drop_y.round(), 2.0, 2.0, white(1.0 - p));
    }
}

/// Estrelinhas cintilando ao longo de um arco-íris (canto esquerdo em `left`).
pub fn draw_rainbow_sparkles(tex: &Textures, left: f32, time: f32) {
    let center = Vec2::new(left + tex.rainbow.width() / 2.0, SEA_Y + 3.0);
    for k in 0..6_u8 {
        let kf = f32::from(k);
        let angle = std::f32::consts::PI * kf.mul_add(0.15, 0.12);
        let radius = (kf % 3.0).mul_add(-14.0, 100.0);
        let at = center + Vec2::new(-radius * angle.cos(), -radius * angle.sin());
        let glow = twinkle(left.mul_add(0.01, kf), time);
        if glow > 0.05 {
            draw_centered(&tex.sparkle, at, 1.0, white(glow));
        }
    }
}

/// Pétalas caindo de uma copa florida (`crown` na tela).
pub fn draw_petals(tex: &Textures, crown: Vec2, time: f32) {
    for i in 0..PETALS {
        let (at, alpha) = petal(crown, i, time);
        if at.y < VIEW_H {
            draw_texture_ex(
                &tex.petal,
                at.x.round(),
                at.y.round(),
                white(alpha),
                DrawTextureParams::default(),
            );
        }
    }
}

/// Borboleta voando sobre uma flor (`flower` na tela; `world_x` escolhe a cor e o ritmo).
pub fn draw_butterfly(tex: &Textures, flower: Vec2, world_x: f32, time: f32) {
    let (at, open) = butterfly(Vec2::new(world_x, flower.y), time);
    let at = Vec2::new(at.x - world_x + flower.x, at.y);
    let hue = (world_x * 0.31).sin();
    let kind = usize::from(hue > -0.3) + usize::from(hue > 0.4);
    let frames = tex.butterflies.get(kind).unwrap_or(&tex.butterflies[0]);
    if let Some(t) = frames.get(usize::from(!open)) {
        let facing_left = time.mul_add(0.9, world_x * 0.137).cos() < 0.0;
        let params = DrawTextureParams {
            flip_x: facing_left,
            ..Default::default()
        };
        draw_at(t, at - t.size() / 2.0, params);
    }
}

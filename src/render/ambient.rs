//! Desenho da vida no cenário: pássaros, peixes, brilhos no lago, cachoeiras animadas,
//! brilhos no arco-íris, pétalas e borboletas. As trajetórias vêm de [`crate::ambient`].

use macroquad::color::{Color, WHITE};
use macroquad::math::Vec2;
use macroquad::shapes::{draw_circle, draw_ellipse_lines, draw_rectangle};
use macroquad::texture::{DrawTextureParams, Texture2D, draw_texture_ex};

use super::scene::{MESA_BASE, SEA_Y, color, draw_centered, offsets};
use super::textures::Textures;
use crate::ambient::{Fish, bird, butterfly, fish, fish_arc, fish_landing, petal, twinkle};
use crate::level::scenery::FAR_PARALLAX;
use crate::view::{VIEW_H, VIEW_W};

/// Bandos e pássaros por bando.
const FLOCKS: u8 = 3;
const BIRDS_PER_FLOCK: u8 = 5;
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

/// Bandos de pássaros em "V" atravessando o céu, batendo as asas.
pub fn draw_birds(tex: &Textures, cam: f32, time: f32) {
    for flock in 0..FLOCKS {
        for k in 0..BIRDS_PER_FLOCK {
            let (at, up) = bird(flock, k, time, cam);
            if let Some(t) = tex.bird.get(usize::from(!up)) {
                draw_at(t, at, DrawTextureParams::default());
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

/// Cachoeira: três faixas de água em velocidades diferentes, borda brilhante no alto,
/// bolhas de espuma pulando na base e névoa subindo e sumindo.
pub fn draw_waterfall(left: f32, width: f32, top: f32, time: f32) {
    let height = MESA_BASE - top;
    draw_rectangle(left, top, width, height, color(0x8f_d6_ff));
    let lane = width / 3.0;
    let lanes: [(f32, f32, usize, u32); 3] = [
        (0.0, 70.0, 12, 0xc4_ec_ff),
        (1.0, 105.0, 17, 0xff_ff_ff),
        (2.0, 85.0, 14, 0xc4_ec_ff),
    ];
    for (i, speed, gap, hex) in lanes {
        let x = i.mul_add(lane, left + 1.0);
        for offset in offsets(gap, height) {
            let y = top
                + time
                    .mul_add(speed, i.mul_add(5.0, offset))
                    .rem_euclid(height);
            draw_rectangle(x.round(), y.round(), (lane - 1.0).round(), 3.0, color(hex));
        }
    }
    let edge = color(0x5f_b8_f0);
    draw_rectangle(left, top, 2.0, height, edge);
    draw_rectangle(left + width - 2.0, top, 2.0, height, edge);
    draw_rectangle(left - 2.0, top - 2.0, width + 4.0, 3.0, WHITE);
    waterfall_foot(left + width / 2.0, width, time);
}

fn waterfall_foot(center: f32, width: f32, time: f32) {
    // Névoa: nuvenzinhas que sobem, crescem e somem.
    for j in [0.0f32, 1.0, 2.0] {
        let p = time.mul_add(0.45, j / 3.0).rem_euclid(1.0);
        let x = p.mul_add(5.0, (j - 1.0).mul_add(width * 0.5, center));
        let y = 18.0f32.mul_add(-p, MESA_BASE - 6.0);
        draw_circle(x, y, 4.0f32.mul_add(p, 4.0), white(0.55 * (1.0 - p)));
    }
    // Bolhas de espuma pulando na base.
    for j in [0.0f32, 1.0, 2.0, 3.0, 4.0, 5.0] {
        let x = (j - 2.5).mul_add((width + 8.0) / 5.0, center);
        let hop = time.mul_add(5.0, j * 1.3).sin().abs() * 4.0;
        draw_circle(
            x.round(),
            (MESA_BASE - 2.0 - hop).round(),
            3.0 + (j % 2.0),
            WHITE,
        );
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

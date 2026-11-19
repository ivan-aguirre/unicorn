//! Placar de corações, barra até 100, festa e tela de pausa.

use macroquad::color::{Color, WHITE};
use macroquad::math::Vec2;
use macroquad::shapes::draw_rectangle;
use macroquad::texture::{DrawTextureParams, draw_texture_ex};

use super::scene::{color, draw_centered};
use super::textures::Textures;
use crate::effects::CELEBRATION_TIME;
use crate::game::{Game, Mode};
use crate::hearts::HeartCounter;
use crate::view::{VIEW_H, VIEW_W};

/// Escala do texto do placar e da pausa (letras grandes para crianças).
const TEXT_SCALE: f32 = 3.0;
/// Arco-íris da festa: ampliação e topo na tela.
const PARTY_RAINBOW_SCALE: f32 = 2.0;
const PARTY_RAINBOW_TOP: f32 = 40.0;
/// Distância da borda do arco-íris até o meio de cada base (na imagem original).
const RAINBOW_FOOT: f32 = 25.5;
/// Escala do "100" da festa.
const PARTY_SCALE: f32 = 5.0;
/// Largura da barra de progresso até 100.
const BAR_W: f32 = 108.0;
/// Escala do aviso de como sair, na pausa.
const HINT_SCALE: f32 = 2.0;
/// Largura de um espaço entre palavras.
const SPACE: f32 = 4.0;

/// Desenha o placar e as camadas sobre o jogo (festa, pausa e saída).
pub fn draw(tex: &Textures, game: &Game, quit_progress: f32) {
    draw_counter(tex, game.counter);
    let party = game.effects.celebration();
    if party > 0.0 {
        draw_celebration(tex, party);
    }
    if game.mode == Mode::Paused {
        draw_pause(tex);
    }
    if quit_progress > 0.0 {
        draw_quit(tex, quit_progress);
    }
}

fn draw_counter(tex: &Textures, counter: HeartCounter) {
    draw_rectangle(6.0, 6.0, 128.0, 52.0, Color::new(1.0, 1.0, 1.0, 0.55));
    draw_centered(&tex.heart, Vec2::new(26.0, 27.0), 1.0, WHITE);
    draw_text(
        tex,
        &counter.value().to_string(),
        Vec2::new(46.0, 13.0),
        TEXT_SCALE,
    );
    draw_rectangle(14.0, 45.0, BAR_W, 6.0, color(0x6b_4a_8a));
    let filled = BAR_W * f32::from(counter.value()) / f32::from(HeartCounter::GOAL);
    draw_rectangle(14.0, 45.0, filled, 6.0, color(0xff_4d_8d));
}

/// Arco-íris gigante surgindo e sumindo durante a festa, com o "100" no meio.
fn draw_celebration(tex: &Textures, remaining: f32) {
    let elapsed = CELEBRATION_TIME - remaining;
    let alpha = (elapsed * 3.0).min(remaining * 1.5).min(1.0);
    let tint = Color::new(1.0, 1.0, 1.0, alpha);
    let size = tex.rainbow.size() * PARTY_RAINBOW_SCALE;
    let left = ((VIEW_W - size.x) / 2.0).round();
    let params = DrawTextureParams {
        dest_size: Some(size),
        ..Default::default()
    };
    draw_texture_ex(&tex.rainbow, left, PARTY_RAINBOW_TOP, tint, params);
    // Uma nuvem sorridente em cada base, balançando em sentidos opostos.
    let base_y = PARTY_RAINBOW_TOP + size.y - 10.0;
    let feet = [
        RAINBOW_FOOT.mul_add(PARTY_RAINBOW_SCALE, left),
        RAINBOW_FOOT.mul_add(-PARTY_RAINBOW_SCALE, left + size.x),
    ];
    for ((cloud, foot), phase) in tex
        .party_clouds
        .iter()
        .zip(feet)
        .zip([0.0, std::f32::consts::PI])
    {
        let bob = (elapsed.mul_add(3.0, phase).sin() * 2.0).round();
        draw_centered(cloud, Vec2::new(foot, base_y + bob), 1.0, tint);
    }
    draw_centered_text(tex, "100", 150.0, PARTY_SCALE);
}

fn draw_pause(tex: &Textures) {
    draw_rectangle(0.0, 0.0, VIEW_W, VIEW_H, Color::new(0.16, 0.1, 0.3, 0.55));
    let (cx, cy) = (VIEW_W / 2.0, VIEW_H / 2.0 - 30.0);
    let outline = color(0x4b_3b_63);
    for dx in [-24.0, 10.0] {
        draw_rectangle(cx + dx - 2.0, cy - 36.0, 18.0, 72.0, outline);
        draw_rectangle(cx + dx, cy - 34.0, 14.0, 68.0, WHITE);
    }
    draw_centered_text(tex, "PAUSA", cy + 50.0, TEXT_SCALE);
    draw_centered_text(tex, "SEGURE ESC PARA SAIR", VIEW_H - 40.0, HINT_SCALE);
}

/// Aviso de saída enquanto Esc está segurado, com a barra enchendo.
fn draw_quit(tex: &Textures, progress: f32) {
    let (w, h) = (220.0, 70.0);
    let (x, y) = ((VIEW_W - w) / 2.0, 20.0);
    draw_rectangle(x, y, w, h, Color::new(0.16, 0.1, 0.3, 0.85));
    draw_centered_text(tex, "SAIR", y + 12.0, TEXT_SCALE);
    draw_rectangle(x + 20.0, y + 48.0, w - 40.0, 8.0, color(0x6b_4a_8a));
    draw_rectangle(x + 20.0, y + 48.0, (w - 40.0) * progress, 8.0, WHITE);
}

/// Texto centralizado na horizontal.
fn draw_centered_text(tex: &Textures, text: &str, y: f32, scale: f32) {
    let x = ((VIEW_W - text_width(tex, text, scale)) / 2.0).round();
    draw_text(tex, text, Vec2::new(x, y), scale);
}

/// Texto na fonte de pixel, ampliado em `scale`, com 1 pixel de espaço entre letras.
fn draw_text(tex: &Textures, text: &str, at: Vec2, scale: f32) {
    let mut x = at.x;
    for c in text.chars() {
        let Some(g) = tex.glyph(c) else {
            x = SPACE.mul_add(scale, x);
            continue;
        };
        draw_texture_ex(
            g,
            x,
            at.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(g.size() * scale),
                ..Default::default()
            },
        );
        x = (g.width() + 1.0).mul_add(scale, x);
    }
}

fn text_width(tex: &Textures, text: &str, scale: f32) -> f32 {
    let advance: f32 = text
        .chars()
        .map(|c| tex.glyph(c).map_or(SPACE, |g| g.width() + 1.0) * scale)
        .sum();
    advance - scale
}

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
const TEXT_SCALE: f32 = 2.0;
/// Escala do "100" da festa.
const PARTY_SCALE: f32 = 3.0;
/// Largura da barra de progresso até 100.
const BAR_W: f32 = 64.0;

/// Desenha o placar e as camadas sobre o jogo (festa e pausa).
pub fn draw(tex: &Textures, game: &Game) {
    draw_counter(tex, game.counter);
    let party = game.effects.celebration();
    if party > 0.0 {
        draw_celebration(tex, party);
    }
    if game.mode == Mode::Paused {
        draw_pause(tex);
    }
}

fn draw_counter(tex: &Textures, counter: HeartCounter) {
    draw_rectangle(4.0, 4.0, 76.0, 31.0, Color::new(1.0, 1.0, 1.0, 0.55));
    draw_centered(&tex.heart, Vec2::new(14.0, 16.0), 1.5, WHITE);
    draw_text(
        tex,
        &counter.value().to_string(),
        Vec2::new(28.0, 8.0),
        TEXT_SCALE,
    );
    draw_rectangle(8.0, 28.0, BAR_W, 4.0, color(0x6b_4a_8a));
    let filled = BAR_W * f32::from(counter.value()) / f32::from(HeartCounter::GOAL);
    draw_rectangle(8.0, 28.0, filled, 4.0, color(0xff_4d_8d));
}

/// Arco-íris gigante surgindo e sumindo durante a festa, com o "100" no meio.
fn draw_celebration(tex: &Textures, remaining: f32) {
    let elapsed = CELEBRATION_TIME - remaining;
    let alpha = (elapsed * 3.0).min(remaining * 1.5).min(1.0);
    let size = tex.rainbow.size() * 2.0;
    draw_texture_ex(
        &tex.rainbow,
        ((VIEW_W - size.x) / 2.0).round(),
        24.0,
        Color::new(1.0, 1.0, 1.0, alpha),
        DrawTextureParams {
            dest_size: Some(size),
            ..Default::default()
        },
    );
    draw_centered_text(tex, "100", 92.0, PARTY_SCALE);
}

fn draw_pause(tex: &Textures) {
    draw_rectangle(0.0, 0.0, VIEW_W, VIEW_H, Color::new(0.16, 0.1, 0.3, 0.55));
    let (cx, cy) = (VIEW_W / 2.0, VIEW_H / 2.0 - 14.0);
    let outline = color(0x4b_3b_63);
    for dx in [-14.0, 6.0] {
        draw_rectangle(cx + dx - 1.0, cy - 21.0, 10.0, 42.0, outline);
        draw_rectangle(cx + dx, cy - 20.0, 8.0, 40.0, WHITE);
    }
    draw_centered_text(tex, "PAUSA", cy + 30.0, TEXT_SCALE);
}

/// Texto centralizado na horizontal.
fn draw_centered_text(tex: &Textures, text: &str, y: f32, scale: f32) {
    let x = ((VIEW_W - text_width(tex, text, scale)) / 2.0).round();
    draw_text(tex, text, Vec2::new(x, y), scale);
}

/// Texto na fonte de pixel, ampliado em `scale`, com 1 pixel de espaço entre letras.
fn draw_text(tex: &Textures, text: &str, at: Vec2, scale: f32) {
    let mut x = at.x;
    for g in text.chars().filter_map(|c| tex.glyph(c)) {
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
        .filter_map(|c| tex.glyph(c))
        .map(|g| (g.width() + 1.0) * scale)
        .sum();
    advance - scale
}

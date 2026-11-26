//! Tela inicial: o cenário ao fundo e dois cartões grandes, sem texto. À esquerda, o passeio
//! (o unicórnio e corações); à direita, as letras (o desenho de uma bola e as teclas de BOLA).

use macroquad::color::{Color, WHITE};
use macroquad::math::{Rect, Vec2};

use super::letters::draw_key;
use super::scene::{self, color};
use super::smooth::Painter;
use super::textures::Textures;
use crate::letters::KeyState;
use crate::letters::keyboard::{KEY_H, KEY_W};
use crate::letters::words::Picture;
use crate::level::Level;
use crate::menu::{Choice, Menu};

/// Lado do desenho da bola no cartão das letras.
const ICON_PICTURE: f32 = 84.0;
/// Tamanho e centro dos cartões.
const CARD: Vec2 = Vec2::new(200.0, 170.0);
const CENTERS: [(Choice, Vec2); 2] = [
    (Choice::Walk, Vec2::new(190.0, 170.0)),
    (Choice::Letters, Vec2::new(450.0, 170.0)),
];

/// Parte em pixel art: só o cenário.
pub fn draw_pixels(tex: &Textures, level: &Level, menu: &Menu) {
    scene::draw_scenery(tex, level, 0.0, menu.time);
}

/// Os dois cartões; o escolhido fica maior, mais claro e com borda pulsando.
pub fn draw_smooth(p: &Painter<'_>, tex: &Textures, menu: &Menu) {
    for (choice, center) in CENTERS {
        let selected = choice == menu.choice;
        let pulse = if selected {
            (menu.time * 4.0).sin().mul_add(0.03, 1.06)
        } else {
            1.0
        };
        draw_card(p, center, pulse, selected, menu.time);
        match choice {
            Choice::Walk => draw_walk_icon(p, tex, center, pulse),
            Choice::Letters => draw_letters_icon(p, tex, center, pulse, menu.time),
        }
    }
}

fn draw_card(p: &Painter<'_>, center: Vec2, zoom: f32, selected: bool, time: f32) {
    let size = CARD * zoom;
    let rect = Rect::new(
        center.x - size.x / 2.0,
        center.y - size.y / 2.0,
        size.x,
        size.y,
    );
    if selected {
        let glow = (time * 4.0).sin().mul_add(0.2, 0.8);
        let border = Rect::new(rect.x - 5.0, rect.y - 5.0, rect.w + 10.0, rect.h + 10.0);
        p.rounded_rect(
            border,
            22.0,
            Color {
                a: glow,
                ..color(0xff_5f_a2)
            },
        );
    }
    let alpha = if selected { 0.9 } else { 0.55 };
    p.rounded_rect(rect, 18.0, Color::new(1.0, 1.0, 1.0, alpha));
}

/// Passeio: a cabeça do unicórnio com corações em volta.
fn draw_walk_icon(p: &Painter<'_>, tex: &Textures, center: Vec2, zoom: f32) {
    if let Some(head) = tex.heads.first() {
        p.texture(head, center + Vec2::new(0.0, 4.0) * zoom, 3.0 * zoom, WHITE);
    }
    for offset in [
        Vec2::new(-68.0, -50.0),
        Vec2::new(66.0, -40.0),
        Vec2::new(60.0, 52.0),
    ] {
        p.texture(&tex.heart, center + offset * zoom, zoom, WHITE);
    }
}

/// Letras: o desenho da bola e as teclas da palavra BOLA embaixo, com a primeira acesa.
fn draw_letters_icon(p: &Painter<'_>, tex: &Textures, center: Vec2, zoom: f32, time: f32) {
    let bob = (time * 2.0).sin() * 3.0;
    if let Some(t) = tex.picture(Picture::Ball) {
        let at = center + Vec2::new(0.0, bob - 22.0) * zoom;
        p.texture(t, at, ICON_PICTURE * zoom / t.width().max(1.0), WHITE);
    }
    for (i, c) in "BOLA".chars().enumerate() {
        let x = f32::from(u8::try_from(i).unwrap_or(0)).mul_add(34.0, -66.0);
        let at = center + Vec2::new(x, 40.0) * zoom;
        let state = if i == 0 {
            KeyState::Target
        } else {
            KeyState::Normal
        };
        let rect = Rect::new(at.x, at.y, KEY_W, KEY_H);
        draw_key(p, c, rect, state, 0.7);
    }
}

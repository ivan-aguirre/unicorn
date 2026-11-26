//! Desenho do modo Letras: cenário e unicórnio em pixel art (tela virtual) e, por cima, em
//! desenho suave, o cartão da palavra, o teclado e o contador de estrelas.

use macroquad::color::{Color, WHITE};
use macroquad::math::{Rect, Vec2};

use super::hud;
use super::scene::{self, color};
use super::smooth::{Painter, TextStyle};
use super::textures::Textures;
use crate::letters::keyboard;
use crate::letters::words::{SHOW_TIME, WordTask};
use crate::letters::{KeyState, Letters};
use crate::view::VIEW_W;

/// Cor das letras escuras e dos contornos (a mesma do contorno da pixel art).
const INK: u32 = 0x4b_3b_63;
/// Altura da letra na tecla.
const KEY_LETTER: f32 = 18.0;
/// Cartão da palavra: lado da figura, margem interna e altura do topo.
const PICTURE: f32 = 124.0;
const CARD_PAD: f32 = 14.0;
const CARD_TOP: f32 = 56.0;
/// Espaço de cada letra a digitar, o intervalo entre eles e a altura das letras.
const SLOT: Vec2 = Vec2::new(40.0, 46.0);
const SLOT_GAP: f32 = 6.0;
const MODEL_LETTER: f32 = 40.0;
const SLOT_LETTER: f32 = 34.0;

/// Parte em pixel art (antes de ampliar a tela virtual).
pub fn draw_pixels(tex: &Textures, l: &Letters) {
    scene::draw_scenery(tex, &l.level, 0.0, l.time());
    scene::draw_player(tex, &l.player, 0.0);
    scene::draw_particles(tex, &l.effects, 0.0);
    let party = l.effects.celebration();
    if party > 0.0 {
        hud::draw_party(tex, party, None);
    }
}

/// Parte suave, na resolução da janela.
pub fn draw_smooth(p: &Painter<'_>, tex: &Textures, l: &Letters) {
    draw_keyboard(p, l);
    draw_stars(p, tex, l.stars());
    draw_word(p, tex, &l.words, l.time());
}

const fn with_alpha(hex: u32, a: f32) -> Color {
    Color { a, ..color(hex) }
}

/// Teclado: base escura e as teclas, cada uma no seu estado.
fn draw_keyboard(p: &Painter<'_>, l: &Letters) {
    p.rounded_rect(keyboard::plate(), 12.0, with_alpha(INK, 0.68));
    let glow = (l.time() * 6.0).sin().mul_add(0.25, 0.65);
    for (c, rect) in keyboard::keys() {
        draw_key(p, c, rect, l.key_state(c), glow);
    }
}

/// Cores de uma tecla: face, lateral e letra.
const fn key_colors(state: KeyState) -> (Color, Color, Color) {
    match state {
        KeyState::Normal => (color(0xff_f8_ee), color(0xd8_cc_e6), color(INK)),
        KeyState::Target | KeyState::Hit => (color(0xff_5f_a2), color(0xc8_34_7a), WHITE),
        KeyState::Miss => (color(0xb8_b4_c8), color(0x8a_86_9c), color(INK)),
    }
}

/// Uma tecla com lateral (efeito 3D): a apertada afunda; a da próxima letra brilha em volta.
pub fn draw_key(p: &Painter<'_>, c: char, rect: Rect, state: KeyState, glow: f32) {
    let depth = if state == KeyState::Hit { 1.0 } else { 3.5 };
    let top = rect.y + 3.5 - depth;
    let body = Rect::new(rect.x, top, rect.w, rect.bottom() - top);
    if state == KeyState::Target {
        let halo = Rect::new(body.x - 3.0, body.y - 3.0, body.w + 6.0, body.h + 6.0);
        p.rounded_rect(halo, 8.0, with_alpha(0xff_e1_4d, glow));
    }
    let (face, side, ink) = key_colors(state);
    let edge = Rect::new(body.x - 0.8, body.y - 0.8, body.w + 1.6, body.h + 1.6);
    p.rounded_rect(edge, 6.0, color(INK));
    p.rounded_rect(body, 5.5, side);
    let face_rect = Rect::new(body.x, body.y, body.w, body.h - depth);
    p.rounded_rect(face_rect, 5.5, face);
    let style = TextStyle {
        size: KEY_LETTER,
        fill: ink,
        outline: (ink, 0.0),
    };
    p.text(
        &c.to_string(),
        face_rect.center() + Vec2::new(0.0, 0.5),
        style,
    );
}

/// Contador de estrelas no canto: uma estrela e o número.
fn draw_stars(p: &Painter<'_>, tex: &Textures, stars: u32) {
    p.rounded_rect(
        Rect::new(6.0, 6.0, 96.0, 40.0),
        10.0,
        Color::new(1.0, 1.0, 1.0, 0.55),
    );
    p.texture(&tex.star, Vec2::new(28.0, 26.0), 2.0, WHITE);
    let style = TextStyle {
        size: 26.0,
        fill: WHITE,
        outline: (color(INK), 1.6),
    };
    let text = stars.to_string();
    p.text(&text, Vec2::new(70.0, 27.0), style);
}

/// Palavra da vez num cartão: a figura, a palavra escrita (a letra da vez marcada) e, embaixo,
/// um espaço por letra que a criança enche digitando. Completa, a figura pula e o fundo do
/// cartão fica transparente para a festa aparecer atrás.
fn draw_word(p: &Painter<'_>, tex: &Textures, w: &WordTask, time: f32) {
    let word = w.word();
    let len = f32::from(u8::try_from(word.text.len()).unwrap_or(6));
    let letters_w = len.mul_add(SLOT.x + SLOT_GAP, -SLOT_GAP);
    let card_w = 3.0f32.mul_add(CARD_PAD, PICTURE + letters_w);
    let card_h = 2.0f32.mul_add(CARD_PAD, PICTURE);
    let card = Rect::new(((VIEW_W - card_w) / 2.0).round(), CARD_TOP, card_w, card_h);
    let party = (SHOW_TIME - w.showing()).min(1.0) * f32::from(u8::from(w.showing() > 0.0));
    let shadow = Rect::new(card.x, card.y + 4.0, card.w, card.h);
    p.rounded_rect(shadow, 18.0, with_alpha(INK, party.mul_add(-0.25, 0.25)));
    p.rounded_rect(
        card,
        18.0,
        with_alpha(0xff_ff_ff, party.mul_add(-0.6, 0.92)),
    );
    let hop = (time * 10.0).sin().abs() * 6.0 * party;
    if let Some(t) = tex.picture(word.picture) {
        let center = Vec2::new(card.x + CARD_PAD + PICTURE / 2.0, card.center().y - hop);
        p.texture(t, center, PICTURE / t.width().max(1.0), WHITE);
    }
    let left = 2.0f32.mul_add(CARD_PAD, card.x + PICTURE);
    let blink = (time * 5.0).sin().mul_add(0.35, 0.65);
    for (i, c) in word.text.chars().enumerate() {
        let x = f32::from(u8::try_from(i).unwrap_or(0)).mul_add(SLOT.x + SLOT_GAP, left);
        let order = i.cmp(&w.typed());
        let model = Vec2::new(
            x + SLOT.x / 2.0,
            card.y + CARD_PAD + MODEL_LETTER / 2.0 + 6.0,
        );
        draw_model(p, c, model, order.is_eq(), blink);
        let slot = Rect::new(x, card.bottom() - CARD_PAD - SLOT.y, SLOT.x, SLOT.y);
        draw_slot(p, slot, c, order, blink);
    }
}

/// Uma letra da palavra escrita; a da vez ganha um fundo amarelo piscando.
fn draw_model(p: &Painter<'_>, c: char, center: Vec2, current: bool, blink: f32) {
    if current {
        let back = Rect::new(center.x - SLOT.x / 2.0, center.y - 25.0, SLOT.x, 50.0);
        p.rounded_rect(back, 10.0, with_alpha(0xff_e1_4d, blink));
    }
    let style = TextStyle {
        size: MODEL_LETTER,
        fill: color(INK),
        outline: (color(INK), 0.0),
    };
    p.text(&c.to_string(), center + Vec2::new(0.0, 1.0), style);
}

/// Um espaço para digitar: com a letra (já digitada), piscando (o próximo) ou vazio.
fn draw_slot(p: &Painter<'_>, rect: Rect, c: char, order: std::cmp::Ordering, blink: f32) {
    use std::cmp::Ordering;
    let (back, letter) = match order {
        Ordering::Less => (color(0xff_f8_ee), Some(color(0xff_5f_a2))),
        Ordering::Equal => (with_alpha(0xff_e1_4d, blink), None),
        Ordering::Greater => (with_alpha(0xd8_cc_e6, 0.9), None),
    };
    let edge = Rect::new(rect.x - 1.0, rect.y - 1.0, rect.w + 2.0, rect.h + 2.0);
    p.rounded_rect(edge, 9.0, with_alpha(INK, 0.35));
    p.rounded_rect(rect, 8.0, back);
    if let Some(fill) = letter {
        let style = TextStyle {
            size: SLOT_LETTER,
            fill,
            outline: (color(INK), 1.6),
        };
        p.text(&c.to_string(), rect.center() + Vec2::new(0.0, 1.0), style);
    }
}

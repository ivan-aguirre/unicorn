//! Desenho do modo Letras: cenário e unicórnio em pixel art (tela virtual) e, por cima, em
//! desenho suave, a bolha com a letra, o teclado, o contador de estrelas e a palavra da vez.

use macroquad::color::{Color, WHITE};
use macroquad::math::{Rect, Vec2};

use super::hud;
use super::scene::{self, color};
use super::smooth::{Painter, Ring, TextStyle};
use super::textures::Textures;
use crate::letters::bubble::RADIUS;
use crate::letters::keyboard;
use crate::letters::words::WordTask;
use crate::letters::{KeyState, Letters, POP_TIME, Pop, SPEED_SHOWN, SPEEDS};

/// Cor das letras escuras e dos contornos (a mesma do contorno da pixel art).
const INK: u32 = 0x4b_3b_63;
/// Cores do aro de arco-íris das bolhas.
const RAINBOW: [u32; 5] = [0xff_7e_c0, 0xff_e1_4d, 0x6f_d3_6a, 0x4f_b8_ff, 0xb0_7c_ff];
/// Altura da letra dentro da bolha e na tecla.
const BUBBLE_LETTER: f32 = 40.0;
const KEY_LETTER: f32 = 18.0;

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
    if let Some(b) = l.bubble() {
        draw_bubble(p, b.center(), RADIUS * b.scale(), b.phase, Some(b.letter));
    }
    if let Some(pop) = l.pop() {
        draw_pop(p, pop);
    }
    draw_stars(p, tex, l.stars());
    draw_speed(p, l.speed(), l.speed_shown());
    if let Some(w) = &l.words {
        draw_word(p, tex, w, l.time());
    }
}

const fn with_alpha(hex: u32, a: f32) -> Color {
    Color { a, ..color(hex) }
}

const fn ring(center: Vec2, radius: f32, width: f32) -> Ring {
    Ring {
        center,
        radius,
        width,
    }
}

fn rainbow(i: usize) -> u32 {
    RAINBOW.get(i % RAINBOW.len()).copied().unwrap_or(INK)
}

/// Bolha transparente com aro de arco-íris, brilhos e, se houver, a letra dentro.
pub fn draw_bubble(p: &Painter<'_>, center: Vec2, radius: f32, phase: f32, letter: Option<char>) {
    if radius < 1.0 {
        return;
    }
    p.disc(center, radius, Color::new(0.88, 0.95, 1.0, 0.25));
    let turn = phase.to_degrees();
    for i in 0..30_u8 {
        let start = f32::from(i).mul_add(12.0, turn);
        let hue = rainbow(usize::from(i) / 6);
        p.arc(
            ring(center, radius, 3.5),
            (start, 12.5),
            with_alpha(hue, 0.9),
        );
        p.arc(
            ring(center, radius - 4.0, 2.5),
            (start, 12.5),
            with_alpha(hue, 0.3),
        );
    }
    p.disc(
        center + Vec2::new(-0.42, -0.45) * radius,
        0.17 * radius,
        with_alpha(0xff_ff_ff, 0.92),
    );
    p.disc(
        center + Vec2::new(-0.16, -0.68) * radius,
        0.07 * radius,
        with_alpha(0xff_ff_ff, 0.8),
    );
    p.arc(
        ring(center, 0.72 * radius, 2.0),
        (20.0, 50.0),
        with_alpha(0xff_ff_ff, 0.75),
    );
    if let Some(c) = letter {
        let style = TextStyle {
            size: BUBBLE_LETTER * radius / RADIUS,
            fill: WHITE,
            outline: (color(INK), 1.8),
        };
        p.text(&c.to_string(), center + Vec2::new(0.0, 1.0), style);
    }
}

/// Estouro: o aro se abre e some, a letra cresce e some, e gotinhas voam para fora.
fn draw_pop(p: &Painter<'_>, pop: &Pop) {
    let t = (pop.age / POP_TIME).min(1.0);
    let fade = 1.0 - t;
    let radius = RADIUS * t.mul_add(0.8, 1.0);
    for i in 0..10_u8 {
        let start = f32::from(i).mul_add(36.0, pop.phase.to_degrees());
        p.arc(
            ring(pop.center, radius, 3.0 * fade),
            (start, 18.0),
            with_alpha(rainbow(usize::from(i)), fade),
        );
    }
    for i in 0..8_u8 {
        let angle = std::f32::consts::TAU * f32::from(i) / 8.0 + pop.phase;
        let at = pop.center + Vec2::from_angle(angle) * radius * 1.15;
        p.disc(at, 3.0 * fade, with_alpha(0xff_ff_ff, fade));
    }
    let style = TextStyle {
        size: BUBBLE_LETTER * t.mul_add(0.6, 1.0),
        fill: Color::new(1.0, 1.0, 1.0, fade),
        outline: (with_alpha(INK, fade), 1.8),
    };
    p.text(&pop.letter.to_string(), pop.center, style);
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

/// Uma tecla com lateral (efeito 3D): a apertada afunda; a da bolha brilha em volta.
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

/// Indicador de velocidade embaixo do contador: barrinhas crescentes, acesas até o nível
/// atual. Fica maior e mais forte por um instante depois de uma mudança.
fn draw_speed(p: &Painter<'_>, level: usize, shown: f32) {
    let highlight = (shown / SPEED_SHOWN).min(1.0);
    let alpha = highlight.mul_add(0.4, 0.55);
    let zoom = highlight.mul_add(0.35, 1.0);
    let bars = u8::try_from(SPEEDS.len()).unwrap_or(5);
    let origin = Vec2::new(14.0, 52.0);
    let back = Rect::new(
        origin.x - 6.0,
        origin.y - 3.0,
        54.0f32.mul_add(zoom, 12.0),
        22.0f32.mul_add(zoom, 6.0),
    );
    p.rounded_rect(back, 8.0, Color::new(1.0, 1.0, 1.0, alpha * 0.6));
    for i in 0..bars {
        let x = f32::from(i).mul_add(11.0 * zoom, origin.x);
        let h = f32::from(i).mul_add(4.0, 6.0) * zoom;
        let rect = Rect::new(x, 22.0f32.mul_add(zoom, origin.y) - h, 8.0 * zoom, h);
        let lit = usize::from(i) <= level;
        let color = if lit {
            with_alpha(0xff_5f_a2, alpha + 0.1)
        } else {
            with_alpha(INK, alpha * 0.4)
        };
        p.rounded_rect(rect, 2.5, color);
    }
}

/// Palavra da vez no alto: figura e um espaço por letra. As digitadas aparecem, a próxima
/// pisca; completa, a figura pula.
fn draw_word(p: &Painter<'_>, tex: &Textures, w: &WordTask, time: f32) {
    let word = w.word();
    let len = word.text.chars().count();
    let slot = Vec2::new(28.0, 34.0);
    let letters_w = f32::from(u8::try_from(len).unwrap_or(6)).mul_add(slot.x + 4.0, -4.0);
    let panel = Rect::new(
        (640.0 - (letters_w + 84.0)) / 2.0,
        6.0,
        letters_w + 84.0,
        52.0,
    );
    p.rounded_rect(panel, 12.0, Color::new(1.0, 1.0, 1.0, 0.7));
    let hop = if w.showing() > 0.0 {
        (time * 10.0).sin().abs() * 4.0
    } else {
        0.0
    };
    if let Some(t) = tex.picture(word.picture) {
        p.texture(
            t,
            Vec2::new(panel.x + 34.0, panel.center().y - hop),
            2.0,
            WHITE,
        );
    }
    let blink = (time * 5.0).sin().mul_add(0.35, 0.65);
    for (i, c) in word.text.chars().enumerate() {
        let x = f32::from(u8::try_from(i).unwrap_or(0)).mul_add(slot.x + 4.0, panel.x + 66.0);
        let rect = Rect::new(x, panel.y + 9.0, slot.x, slot.y);
        draw_slot(p, rect, c, i.cmp(&w.typed()), blink);
    }
}

/// Um espaço da palavra: com a letra (já digitada), piscando (a próxima) ou vazio.
fn draw_slot(p: &Painter<'_>, rect: Rect, c: char, order: std::cmp::Ordering, blink: f32) {
    use std::cmp::Ordering;
    let (back, letter) = match order {
        Ordering::Less => (color(0xff_f8_ee), Some(color(0xff_5f_a2))),
        Ordering::Equal => (with_alpha(0xff_e1_4d, blink), None),
        Ordering::Greater => (with_alpha(0xd8_cc_e6, 0.8), None),
    };
    p.rounded_rect(rect, 6.0, back);
    if let Some(fill) = letter {
        let style = TextStyle {
            size: 24.0,
            fill,
            outline: (color(INK), 1.2),
        };
        p.text(&c.to_string(), rect.center() + Vec2::new(0.0, 1.0), style);
    }
}

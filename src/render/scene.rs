//! Desenho do mundo na tela virtual, do fundo para a frente, com paralaxe.
//!
//! Só o que está visível é desenhado (buscas binárias nas listas ordenadas por `x`).

use macroquad::color::{Color, WHITE};
use macroquad::math::{Rect, Vec2};
use macroquad::shapes::draw_rectangle;
use macroquad::texture::{DrawTextureParams, Texture2D, draw_texture, draw_texture_ex};

use super::ambient;
use super::textures::Textures;
use crate::ambient as ambient_motion;
use crate::animation::frame_at;
use crate::art::flower_index;
use crate::art::procedural::SUN_FRAMES;
use crate::art::procedural::{GRASS_TIP, GROUND_TILE};
use crate::art::unicorn::{FEET, FRAME_W, HEAD_AT};
use crate::effects::ParticleKind;
use crate::game::Game;
use crate::level::platform::THICKNESS;
use crate::level::scenery::{
    CLOUD_LAYERS, CLOUD_PERIOD, FAR_PARALLAX, FlowerColor, MESA_PARALLAX, Mesa, Parallax, Prop,
    PropKind, RAINBOW_PARALLAX,
};
use crate::level::{Level, span};
use crate::player::Facing;
use crate::view::{VIEW_H, VIEW_H_PX, VIEW_W, VIEW_W_PX};

/// Linha do horizonte (onde começa o mar).
pub const SEA_Y: f32 = 206.0;
/// Base das colinas do fundo (dentro do lago).
pub const MESA_BASE: f32 = 250.0;
/// Faixas do céu: (topo, base, cor).
const SKY: [(f32, f32, u32); 5] = [
    (0.0, 40.0, 0x3a_78_d8),
    (40.0, 84.0, 0x4a_8a_e8),
    (84.0, 126.0, 0x62_a2_f2),
    (126.0, 166.0, 0x82_bb_f7),
    (166.0, 206.0, 0xa8_d4_fb),
];

/// Converte `0xRRGGBB` numa cor opaca.
#[must_use]
pub const fn color(hex: u32) -> Color {
    let [_, r, g, b] = hex.to_be_bytes();
    Color::from_rgba(r, g, b, 255)
}

/// Contexto do quadro: câmera arredondada ao pixel e relógio do jogo.
#[derive(Clone, Copy)]
struct View {
    cam: f32,
    time: f32,
}

impl View {
    /// Início visível de uma camada com o fator de paralaxe dado.
    fn layer(self, parallax: Parallax) -> f32 {
        (self.cam * parallax.factor()).round()
    }
}

/// Colunas da tela virtual (0, 1, 2, ...) em passos de `step` pixels.
fn columns(step: usize) -> impl Iterator<Item = f32> {
    (0..VIEW_W_PX).step_by(step).map(f32::from)
}

/// Deslocamentos `0, step, 2*step, ...` menores que `limit` (até a altura da tela).
pub fn offsets(step: usize, limit: f32) -> impl Iterator<Item = f32> {
    (0..VIEW_H_PX)
        .step_by(step)
        .map(f32::from)
        .take_while(move |v| *v < limit)
}

/// Desenha o mundo inteiro.
pub fn draw(tex: &Textures, game: &Game) {
    let view = View {
        cam: game.camera.snapped(),
        time: game.time,
    };
    draw_sky(tex, view);
    draw_clouds(tex, game, view, 0);
    ambient::draw_birds(tex, view.cam, view.time);
    draw_far(view);
    ambient::draw_fish(tex, view.cam, view.time);
    draw_rainbows(tex, game, view);
    draw_clouds(tex, game, view, 1);
    draw_mesas(game, view);
    draw_trees(tex, &game.level, view);
    draw_ground(tex, &game.level, view);
    draw_platforms(tex, &game.level, view);
    draw_plants(tex, &game.level, view);
    draw_hearts(tex, game, view);
    draw_player(tex, game, view);
    draw_particles(tex, game, view);
}

fn draw_sky(tex: &Textures, view: View) {
    for (top, bottom, hex) in SKY {
        draw_rectangle(0.0, top, VIEW_W, bottom - top, color(hex));
    }
    let frame = frame_at(view.time, 0.14, SUN_FRAMES);
    if let Some(sun) = tex.sun.get(usize::from(frame)) {
        draw_texture(sun, VIEW_W - 100.0, -4.0, WHITE);
    }
}

fn draw_clouds(tex: &Textures, game: &Game, view: View, layer: usize) {
    let (Some(layer), Some(clouds)) = (
        CLOUD_LAYERS.get(layer),
        game.level.scenery.clouds.get(layer),
    ) else {
        return;
    };
    for c in clouds {
        let moved = view.cam.mul_add(layer.parallax, view.time * layer.drift);
        let x = (c.x - moved).rem_euclid(CLOUD_PERIOD) - 120.0;
        let bob = view.time.mul_add(0.5, c.x).sin().round();
        if let Some(t) = tex.clouds.get(c.kind).filter(|_| x < VIEW_W) {
            draw_texture(t, x.round(), c.y + bob, WHITE);
        }
    }
}

/// Montanhas distantes e o mar cintilante.
fn draw_far(view: View) {
    let offset = view.cam * FAR_PARALLAX.factor();
    let (body, top) = (color(0x8c_c9_b4), color(0xb8_e4_d0));
    for x in columns(2) {
        let lx = x + offset;
        let peaks = 10.0f32.mul_add(
            lx.mul_add(0.034, 1.3).sin(),
            15.0f32.mul_add((lx * 0.0126).sin(), 173.0),
        );
        let peak = peaks.round();
        draw_rectangle(x, peak, 2.0, SEA_Y - peak, body);
        draw_rectangle(x, peak, 2.0, 3.0, top);
    }
    draw_rectangle(0.0, SEA_Y, VIEW_W, VIEW_H - SEA_Y, color(0x3f_86_e0));
    draw_rectangle(0.0, SEA_Y, VIEW_W, 3.0, color(0x9f_d4_ff));
    draw_sea_shimmer(view, offset);
    ambient::draw_glints(view.cam, view.time);
}

fn draw_sea_shimmer(view: View, offset: f32) {
    let light = color(0xc4_e6_ff);
    for (row, y) in [
        (0.0f32, 215.0f32),
        (1.0, 225.0),
        (2.0, 238.0),
        (3.0, 255.0),
        (4.0, 277.0),
    ] {
        let wobble = view.time.mul_add(1.5, row).sin() * 5.0;
        let speed = 0.2f32.mul_add(row, 1.0);
        let start = offset.mul_add(-speed, wobble).rem_euclid(80.0) - 80.0;
        for x in columns(80) {
            let sx = row.mul_add(15.0, start + x);
            draw_rectangle(sx.round(), y, 2.0f32.mul_add(-row, 16.0), 2.0, light);
        }
    }
}

fn draw_rainbows(tex: &Textures, game: &Game, view: View) {
    let left = view.layer(RAINBOW_PARALLAX);
    let width = tex.rainbow.width();
    for &x in span(
        &game.level.scenery.rainbows,
        |x| *x,
        left - width,
        left + VIEW_W,
    ) {
        let sx = x - left;
        draw_texture(&tex.rainbow, sx, SEA_Y - tex.rainbow.height() + 3.0, WHITE);
        ambient::draw_rainbow_sparkles(tex, sx, view.time);
        if let Some(puff) = tex.clouds.get(1) {
            draw_texture(puff, sx - 20.0, SEA_Y - 23.0, WHITE);
            draw_texture(puff, sx + width - 37.0, SEA_Y - 23.0, WHITE);
        }
    }
}

fn draw_mesas(game: &Game, view: View) {
    let left = view.layer(MESA_PARALLAX);
    for m in span(
        &game.level.scenery.mesas,
        |m| m.x,
        left - 210.0,
        left + VIEW_W,
    ) {
        let sx = m.x - left;
        draw_mesa(m, sx);
        if let Some(w) = m.waterfall {
            ambient::draw_waterfall(sx + w.offset, w.w, m.top, view.time);
        }
    }
}

/// Colina de topo gramado com cantos arredondados, luz à esquerda e sombra à direita.
fn draw_mesa(m: &Mesa, sx: f32) {
    let (top, height) = (m.top, MESA_BASE - m.top);
    draw_rectangle(sx, top + 3.0, m.w, height - 3.0, color(0x4f_9c_78));
    let stripe = color(0x43_88_68);
    for dx in columns(12).take_while(|dx| *dx < m.w - 13.0) {
        draw_rectangle(sx + dx + 8.0, top + 12.0, 3.0, height - 12.0, stripe);
    }
    draw_rectangle(sx, top + 10.0, 3.0, height - 10.0, color(0x68_b8_8e));
    draw_rectangle(
        sx + m.w - 3.0,
        top + 10.0,
        3.0,
        height - 10.0,
        color(0x3a_78_5c),
    );
    let grass = color(0x7a_d8_7c);
    draw_rectangle(sx + 3.0, top, m.w - 6.0, 1.0, grass);
    draw_rectangle(sx + 1.0, top + 1.0, m.w - 2.0, 2.0, grass);
    draw_rectangle(sx, top + 3.0, m.w, 5.0, grass);
    draw_rectangle(sx, top + 8.0, m.w, 3.0, color(0x2f_7a_52));
}

/// Enfeites do chão visíveis (com margem para os que entram pela borda).
fn visible_props(level: &Level, view: View) -> &[Prop] {
    span(
        &level.scenery.props,
        |p| p.x,
        view.cam - 60.0,
        view.cam + VIEW_W + 60.0,
    )
}

/// Canto superior esquerdo de um sprite plantado em `x`, afundando `sink` pixels no chão.
fn planted(t: &Texture2D, x: f32, ground: f32, sink: f32) -> Vec2 {
    Vec2::new(x - (t.width() / 2.0).floor(), ground - t.height() + sink)
}

/// Árvores atrás do chão: tronco parado e copa balançando com o vento; as floridas soltam
/// pétalas.
fn draw_trees(tex: &Textures, level: &Level, view: View) {
    for p in visible_props(level, view) {
        let tree = match p.kind {
            PropKind::Tree => &tex.tree,
            PropKind::BlossomTree => &tex.blossom,
            PropKind::Bush | PropKind::Flower(_) => continue,
        };
        let ground = level.terrain.height_at(p.x).round();
        let at = planted(&tree.trunk, p.x - view.cam, ground, 6.0);
        draw_texture(&tree.trunk, at.x, at.y, WHITE);
        let sway = ambient_motion::sway(p.x, view.time);
        draw_texture(&tree.crown, at.x + sway, at.y, WHITE);
        if p.kind == PropKind::BlossomTree {
            let crown = at + Vec2::new(tree.crown.width() / 2.0 + sway, 34.0);
            ambient::draw_petals(tex, crown, view.time);
        }
    }
}

/// Arbustos e flores na frente do chão; algumas flores têm uma borboleta voando em cima.
fn draw_plants(tex: &Textures, level: &Level, view: View) {
    let mut butterflies = Vec::new();
    for p in visible_props(level, view) {
        let (t, sink) = match p.kind {
            PropKind::Bush => (&tex.bush, 4.0),
            PropKind::Flower(c) => (flower(tex, c, p.x, view.time), 3.0),
            PropKind::Tree | PropKind::BlossomTree => continue,
        };
        let ground = level.terrain.height_at(p.x).round();
        let at = planted(t, p.x - view.cam, ground, sink);
        draw_texture(t, at.x, at.y, WHITE);
        if matches!(p.kind, PropKind::Flower(_)) && ambient_motion::has_butterfly(p.x) {
            butterflies.push((Vec2::new(p.x - view.cam, ground), p.x));
        }
    }
    for (flower_at, world_x) in butterflies {
        ambient::draw_butterfly(tex, flower_at, world_x, view.time);
    }
}

fn flower(tex: &Textures, c: FlowerColor, x: f32, time: f32) -> &Texture2D {
    let frame = usize::from(x.mul_add(0.03, time * 2.0).sin() > 0.0);
    let pair = tex.flowers.get(flower_index(c)).unwrap_or(&tex.flowers[0]);
    pair.get(frame).unwrap_or(&pair[0])
}

/// Chão coluna a coluna: terra em xadrez e grama por cima (duas passadas, mesmo atlas).
fn draw_ground(tex: &Textures, level: &Level, view: View) {
    for x in columns(1) {
        let wx = view.cam + x;
        let gy = level.terrain.height_at(wx).round();
        column(
            &tex.soil,
            x,
            gy + 6.0,
            Rect::new(tile_x(wx), gy + 6.0, 1.0, VIEW_H - gy - 6.0),
        );
    }
    let grass_h = tex.grass.height();
    for x in columns(1) {
        let wx = view.cam + x;
        let gy = level.terrain.height_at(wx).round();
        column(
            &tex.grass,
            x,
            gy - GRASS_TIP,
            Rect::new(tile_x(wx), 0.0, 1.0, grass_h),
        );
    }
}

/// Plataformas: grama em cima e terra afinando nas pontas, como ilhas flutuantes.
fn draw_platforms(tex: &Textures, level: &Level, view: View) {
    let grass_h = tex.grass.height();
    let underside = color(0x6b_3a_14);
    for p in span(
        &level.platforms,
        |p| p.x,
        view.cam - 170.0,
        view.cam + VIEW_W,
    ) {
        for c in columns(1).take_while(|c| *c < p.w) {
            let (wx, sx) = (p.x + c, p.x + c - view.cam);
            let edge = c.min(p.w - 1.0 - c);
            let depth = (6.0 - edge).max(0.0).mul_add(-2.0, THICKNESS + 6.0);
            column(
                &tex.soil,
                sx,
                p.y + 6.0,
                Rect::new(tile_x(wx), p.y + 6.0, 1.0, depth - 6.0),
            );
            draw_rectangle(sx, p.y + depth, 1.0, 2.0, underside);
            column(
                &tex.grass,
                sx,
                p.y - GRASS_TIP,
                Rect::new(tile_x(wx), 0.0, 1.0, grass_h),
            );
        }
    }
}

fn tile_x(world_x: f32) -> f32 {
    world_x.rem_euclid(f32::from(GROUND_TILE))
}

fn column(t: &Texture2D, x: f32, y: f32, source: Rect) {
    draw_texture_ex(
        t,
        x,
        y,
        WHITE,
        DrawTextureParams {
            source: Some(source),
            dest_size: Some(Vec2::new(1.0, source.h)),
            ..Default::default()
        },
    );
}

fn draw_hearts(tex: &Textures, game: &Game, view: View) {
    let (w, h) = (tex.heart.width(), tex.heart.height());
    for heart in game.hearts.in_span(view.cam - w, view.cam + VIEW_W + w) {
        let bob = (view.time.mul_add(3.0, heart.phase)).sin() * 3.0;
        let center = Vec2::new(heart.pos.x - view.cam, heart.pos.y + bob.round());
        let grow = (heart.age / 0.35).min(1.0);
        if grow < 1.0 {
            draw_centered(&tex.sparkle, center, 1.0 + grow, WHITE);
        }
        let size = Vec2::new(w, h) * grow.max(0.3);
        draw_texture_ex(
            &tex.heart,
            (center.x - size.x / 2.0).round(),
            (center.y - size.y / 2.0).round(),
            WHITE,
            DrawTextureParams {
                dest_size: Some(size),
                ..Default::default()
            },
        );
    }
}

/// Pônei em camadas: corpo (patas, cauda, barriga) e cabeça (pela expressão).
fn draw_player(tex: &Textures, game: &Game, view: View) {
    let player = &game.player;
    let frame = tex.unicorn.get(player.pose().index());
    let head = tex.heads.get(player.expression().index());
    let (Some(frame), Some(head)) = (frame, head) else {
        return;
    };
    let flip = player.facing == Facing::Left;
    let origin = Vec2::new(
        (player.pos.x - view.cam - FEET.0).round(),
        (player.pos.y - FEET.1).round(),
    );
    // Espelhado, a cabeça vai para o outro lado do quadro.
    let head_x = if flip {
        f32::from(FRAME_W - HEAD_AT.0) - head.width()
    } else {
        f32::from(HEAD_AT.0)
    };
    let head_at = origin + Vec2::new(head_x, f32::from(HEAD_AT.1) + frame.head_drop);
    for (t, at) in [(&frame.body, origin), (head, head_at)] {
        let params = DrawTextureParams {
            flip_x: flip,
            ..Default::default()
        };
        draw_texture_ex(t, at.x, at.y, WHITE, params);
    }
}

fn draw_particles(tex: &Textures, game: &Game, view: View) {
    for p in game.effects.particles() {
        let t = match p.kind {
            ParticleKind::Sparkle => &tex.sparkle,
            ParticleKind::Heart => &tex.heart_small,
            ParticleKind::Star => &tex.star,
        };
        let fade = Color::new(1.0, 1.0, 1.0, (p.life * 3.0).min(1.0));
        draw_centered(t, Vec2::new(p.pos.x - view.cam, p.pos.y), 1.0, fade);
    }
}

/// Desenha centralizado em `center` com escala `scale`, alinhado ao pixel.
pub fn draw_centered(t: &Texture2D, center: Vec2, scale: f32, tint: Color) {
    let size = t.size() * scale;
    draw_texture_ex(
        t,
        (center.x - size.x / 2.0).round(),
        (center.y - size.y / 2.0).round(),
        tint,
        DrawTextureParams {
            dest_size: Some(size),
            ..Default::default()
        },
    );
}

//! Desenho suave, na resolução da janela, por cima da tela virtual ampliada.
//!
//! As letras, as bolhas e o teclado do modo Letras ficariam serrilhados na tela virtual
//! pequena: aqui eles são desenhados direto na janela (formas com antisserrilhamento e uma
//! fonte vetorial), em coordenadas da tela virtual convertidas pela mesma escala da imagem.

use macroquad::color::Color;
use macroquad::math::{Rect, Vec2};
use macroquad::shapes::{draw_arc, draw_poly, draw_rectangle};
use macroquad::text::{Font, TextParams, draw_text_ex, measure_text};
use macroquad::texture::{DrawTextureParams, Texture2D, draw_texture_ex};
use macroquad::window::{screen_height, screen_width};

use super::viewport;
use crate::view::VIEW_W;

/// Lados dos círculos (suficiente para bolhas grandes sem facetas visíveis).
const CIRCLE_SIDES: u8 = 72;
/// Direções do contorno do texto (desenho repetido em volta).
const OUTLINE_STEPS: u8 = 16;

/// Aparência de um texto.
#[derive(Debug, Clone, Copy)]
pub struct TextStyle {
    /// Altura da fonte, em pixels da tela virtual.
    pub size: f32,
    pub fill: Color,
    /// Contorno: cor e espessura em pixels da tela virtual (0 = sem contorno).
    pub outline: (Color, f32),
}

/// Anel: centro, raio da borda de fora e espessura (na tela virtual).
#[derive(Debug, Clone, Copy)]
pub struct Ring {
    pub center: Vec2,
    pub radius: f32,
    pub width: f32,
}

/// Pinta em coordenadas da tela virtual, convertidas para a janela.
pub struct Painter<'a> {
    font: Option<&'a Font>,
    origin: Vec2,
    scale: f32,
}

impl<'a> Painter<'a> {
    /// Pintor para o tamanho atual da janela.
    #[must_use]
    pub fn new(font: Option<&'a Font>) -> Self {
        let v = viewport::fit(screen_width(), screen_height());
        Self {
            font,
            origin: Vec2::new(v.x, v.y),
            scale: v.w / VIEW_W,
        }
    }

    fn at(&self, p: Vec2) -> Vec2 {
        self.origin + p * self.scale
    }

    /// Disco cheio.
    pub fn disc(&self, center: Vec2, radius: f32, color: Color) {
        let c = self.at(center);
        draw_poly(c.x, c.y, CIRCLE_SIDES, radius * self.scale, 0.0, color);
    }

    /// Trecho de anel de `start` a `start + sweep` graus (0° à direita, sentido horário).
    pub fn arc(&self, ring: Ring, (start, sweep): (f32, f32), color: Color) {
        let c = self.at(ring.center);
        let inner = (ring.radius - ring.width).max(0.0) * self.scale;
        let w = ring.width.min(ring.radius) * self.scale;
        draw_arc(c.x, c.y, CIRCLE_SIDES, inner, start, w, sweep, color);
    }

    /// Retângulo de cantos arredondados (sem sobreposição: funciona com transparência).
    pub fn rounded_rect(&self, rect: Rect, radius: f32, color: Color) {
        let o = self.at(rect.point());
        let (w, h) = (rect.w * self.scale, rect.h * self.scale);
        let r = (radius * self.scale).min(w / 2.0).min(h / 2.0);
        let (inner_w, inner_h) = (2.0f32.mul_add(-r, w), 2.0f32.mul_add(-r, h));
        draw_rectangle(o.x + r, o.y, inner_w, h, color);
        draw_rectangle(o.x, o.y + r, r, inner_h, color);
        draw_rectangle(o.x + w - r, o.y + r, r, inner_h, color);
        let corners = [
            (o.x + r, o.y + r, 180.0),
            (o.x + w - r, o.y + r, 270.0),
            (o.x + w - r, o.y + h - r, 0.0),
            (o.x + r, o.y + h - r, 90.0),
        ];
        for (x, y, rotation) in corners {
            draw_arc(x, y, CIRCLE_SIDES, 0.0, rotation, r, 90.0, color);
        }
    }

    /// Textura de pixel art centralizada em `center`, ampliada `zoom` vezes (na tela virtual).
    pub fn texture(&self, t: &Texture2D, center: Vec2, zoom: f32, tint: Color) {
        let size = t.size() * zoom * self.scale;
        let c = self.at(center);
        let params = DrawTextureParams {
            dest_size: Some(size),
            ..Default::default()
        };
        draw_texture_ex(
            t,
            (c.x - size.x / 2.0).round(),
            (c.y - size.y / 2.0).round(),
            tint,
            params,
        );
    }

    /// Texto centralizado em `center` (letras maiúsculas: centro da altura das letras).
    pub fn text(&self, text: &str, center: Vec2, style: TextStyle) {
        let size = pixel_size(style.size * self.scale);
        let dims = measure_text(text, self.font, size, 1.0);
        let c = self.at(center);
        let base = Vec2::new(c.x - dims.width / 2.0, c.y + dims.offset_y / 2.0);
        let (outline, thickness) = style.outline;
        if thickness > 0.0 {
            let t = thickness * self.scale;
            for i in 0..OUTLINE_STEPS {
                let angle = std::f32::consts::TAU * f32::from(i) / f32::from(OUTLINE_STEPS);
                let at = base + Vec2::from_angle(angle) * t;
                self.raw_text(text, at, size, outline);
            }
        }
        self.raw_text(text, base, size, style.fill);
    }

    fn raw_text(&self, text: &str, at: Vec2, size: u16, color: Color) {
        let params = TextParams {
            font: self.font,
            font_size: size,
            color,
            ..Default::default()
        };
        draw_text_ex(text, at.x, at.y, params);
    }
}

/// Tamanho de fonte inteiro mais próximo de `size` (entre 1 e 512 pixels).
#[must_use]
pub fn pixel_size(size: f32) -> u16 {
    let target = size.clamp(1.0, 512.0);
    (1..=512_u16)
        .find(|s| f32::from(*s) + 0.5 > target)
        .unwrap_or(512)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tamanho_de_fonte_arredonda_e_respeita_limites() {
        assert_eq!(pixel_size(12.2), 12);
        assert_eq!(pixel_size(12.6), 13);
        assert_eq!(pixel_size(0.0), 1);
        assert_eq!(pixel_size(-5.0), 1);
        assert_eq!(pixel_size(10_000.0), 512);
    }
}

//! Desenho vetorial calculado na CPU, para figuras grandes e nítidas (sem pixels aparentes).
//!
//! Cada forma é um campo de distância: cada pixel recebe a fração coberta pela forma (bordas
//! suaves) e, se pedido, um contorno escuro em volta, como num desenho animado. As camadas são
//! pintadas em ordem, umas sobre as outras. Puro: testável sem janela.

use macroquad::math::Vec2;

use super::pixmap::{Pixmap, Rgba, rgb};
use super::sprites::OUTLINE;

/// Lado da área de desenho, em unidades: as formas usam coordenadas de 0 a 100 (y para baixo).
pub const CANVAS: f32 = 100.0;
/// Espessura do contorno, em unidades.
pub const BORDER: f32 = 2.6;

/// Forma básica, em unidades da área de desenho. Ângulos em graus, no sentido horário.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shape {
    Circle {
        center: (f32, f32),
        radius: f32,
    },
    Ellipse {
        center: (f32, f32),
        radii: (f32, f32),
        angle: f32,
    },
    /// Retângulo de cantos arredondados.
    Rect {
        center: (f32, f32),
        half: (f32, f32),
        corner: f32,
        angle: f32,
    },
    /// Polígono fechado qualquer (regra par-ímpar: pode cruzar a si mesmo).
    Polygon(&'static [(f32, f32)]),
    /// Traço de pontas redondas passando pelos pontos.
    Line {
        points: &'static [(f32, f32)],
        width: f32,
    },
}

/// Círculo de centro `(x, y)`.
#[must_use]
pub const fn circle(x: f32, y: f32, radius: f32) -> Shape {
    Shape::Circle {
        center: (x, y),
        radius,
    }
}

/// Elipse de centro `(x, y)` e semieixos `rx` e `ry`.
#[must_use]
pub const fn ellipse(x: f32, y: f32, rx: f32, ry: f32) -> Shape {
    Shape::Ellipse {
        center: (x, y),
        radii: (rx, ry),
        angle: 0.0,
    }
}

/// Retângulo de centro `(x, y)`, largura `w`, altura `h` e cantos de raio `corner`.
#[must_use]
pub const fn rect(x: f32, y: f32, w: f32, h: f32, corner: f32) -> Shape {
    Shape::Rect {
        center: (x, y),
        half: (w / 2.0, h / 2.0),
        corner,
        angle: 0.0,
    }
}

/// Polígono fechado.
#[must_use]
pub const fn poly(points: &'static [(f32, f32)]) -> Shape {
    Shape::Polygon(points)
}

/// Traço de espessura `width`.
#[must_use]
pub const fn line(points: &'static [(f32, f32)], width: f32) -> Shape {
    Shape::Line { points, width }
}

impl Shape {
    /// A mesma forma girada `degrees` em volta do próprio centro (só elipses e retângulos).
    #[must_use]
    pub const fn turned(self, degrees: f32) -> Self {
        match self {
            Self::Ellipse { center, radii, .. } => Self::Ellipse {
                center,
                radii,
                angle: degrees,
            },
            Self::Rect {
                center,
                half,
                corner,
                ..
            } => Self::Rect {
                center,
                half,
                corner,
                angle: degrees,
            },
            other => other,
        }
    }

    /// Distância até a borda (negativa dentro), em unidades.
    #[must_use]
    pub fn distance(&self, p: Vec2) -> f32 {
        match *self {
            Self::Circle { center, radius } => p.distance(v(center)) - radius,
            Self::Ellipse {
                center,
                radii,
                angle,
            } => ellipse_distance(local(p, center, angle), v(radii)),
            Self::Rect {
                center,
                half,
                corner,
                angle,
            } => rect_distance(local(p, center, angle), v(half), corner),
            Self::Polygon(points) => polygon_distance(p, points),
            Self::Line { points, width } => polyline_distance(p, points) - width / 2.0,
        }
    }

    /// Caixa que contém a forma (pode sobrar espaço), em unidades.
    #[must_use]
    pub fn bounds(&self) -> (Vec2, Vec2) {
        match *self {
            Self::Circle { center, radius } => around(v(center), radius),
            Self::Ellipse { center, radii, .. } => around(v(center), radii.0.max(radii.1)),
            Self::Rect { center, half, .. } => around(v(center), v(half).length()),
            Self::Polygon(points) => points_bounds(points, 0.0),
            Self::Line { points, width } => points_bounds(points, width / 2.0),
        }
    }
}

/// Uma forma pintada: cor, opacidade, contorno e recortes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Layer {
    pub shape: Shape,
    /// Parte tirada da forma (lua crescente, sombras).
    pub cut: Option<Shape>,
    /// A forma só aparece dentro desta (listras de uma bola, sombra de um corpo).
    pub within: Option<Shape>,
    pub color: u32,
    pub alpha: f32,
    pub border: bool,
}

/// Forma com contorno escuro.
#[must_use]
pub const fn solid(shape: Shape, color: u32) -> Layer {
    Layer {
        shape,
        cut: None,
        within: None,
        color,
        alpha: 1.0,
        border: true,
    }
}

/// Forma sem contorno (detalhes: olhos, brilhos, sombras).
#[must_use]
pub const fn flat(shape: Shape, color: u32) -> Layer {
    Layer {
        border: false,
        ..solid(shape, color)
    }
}

impl Layer {
    /// Tira `cut` da forma.
    #[must_use]
    pub const fn minus(self, cut: Shape) -> Self {
        Self {
            cut: Some(cut),
            ..self
        }
    }

    /// Mostra a forma só dentro de `clip`.
    #[must_use]
    pub const fn inside(self, clip: Shape) -> Self {
        Self {
            within: Some(clip),
            ..self
        }
    }

    /// Transparência (1 = opaca).
    #[must_use]
    pub const fn faded(self, alpha: f32) -> Self {
        Self { alpha, ..self }
    }

    /// Distância até a borda da forma já recortada, em unidades.
    #[must_use]
    pub fn distance(&self, p: Vec2) -> f32 {
        let mut d = self.shape.distance(p);
        if let Some(cut) = &self.cut {
            d = d.max(-cut.distance(p));
        }
        if let Some(clip) = &self.within {
            d = d.max(clip.distance(p));
        }
        d
    }
}

/// Pinta as camadas, em ordem, numa imagem de `size` x `size` pixels.
#[must_use]
pub fn render(layers: &[Layer], size: u16) -> Pixmap {
    let mut canvas = Canvas::new(size);
    for layer in layers {
        canvas.paint(layer);
    }
    canvas.finish()
}

/// Cores com alfa pré-multiplicado, em ponto flutuante, enquanto as camadas são pintadas.
struct Canvas {
    size: u16,
    /// Pixels por unidade.
    scale: f32,
    pixels: Vec<[f32; 4]>,
}

impl Canvas {
    fn new(size: u16) -> Self {
        let len = usize::from(size) * usize::from(size);
        Self {
            size,
            scale: f32::from(size) / CANVAS,
            pixels: vec![[0.0; 4]; len],
        }
    }

    /// Pinta uma camada, só na caixa que ela pode cobrir.
    fn paint(&mut self, layer: &Layer) {
        let margin = if layer.border { BORDER } else { 0.0 } + 1.0;
        let (lo, hi) = layer.shape.bounds();
        let (x0, x1) = (self.to_px(lo.x - margin), self.to_px(hi.x + margin));
        let (y0, y1) = (self.to_px(lo.y - margin), self.to_px(hi.y + margin));
        for y in y0..y1 {
            for x in x0..x1 {
                self.shade(layer, x, y);
            }
        }
    }

    /// Contorno e cor de uma camada num pixel, pela fração coberta.
    fn shade(&mut self, layer: &Layer, x: u16, y: u16) {
        let center = Vec2::new(f32::from(x) + 0.5, f32::from(y) + 0.5) / self.scale;
        let dist = layer.distance(center) * self.scale;
        let index = usize::from(y) * usize::from(self.size) + usize::from(x);
        let Some(px) = self.pixels.get_mut(index) else {
            return;
        };
        if layer.border {
            over(
                px,
                rgb(OUTLINE),
                layer.alpha * coverage(BORDER.mul_add(-self.scale, dist)),
            );
        }
        over(px, rgb(layer.color), layer.alpha * coverage(dist));
    }

    /// Unidade para índice de pixel, dentro da imagem.
    fn to_px(&self, units: f32) -> u16 {
        let v = units * self.scale;
        (0..=self.size)
            .rev()
            .find(|i| f32::from(*i) <= v)
            .unwrap_or(0)
    }

    /// Converte para RGBA de 8 bits (alfa comum). Os pixels transparentes levam a cor do
    /// contorno, para a filtragem linear não escurecer nem clarear as bordas.
    fn finish(self) -> Pixmap {
        let mut out = Pixmap::new(self.size, self.size);
        let [ir, ig, ib, _] = rgb(OUTLINE);
        for ((x, y), [r, g, b, a]) in out.coords().zip(self.pixels) {
            let color = if a > 0.0 {
                [byte(r / a), byte(g / a), byte(b / a), byte(a)]
            } else {
                [ir, ig, ib, 0]
            };
            out.set(x, y, color);
        }
        out
    }
}

/// Pinta `color` com opacidade `alpha` por cima do pixel (alfa pré-multiplicado).
fn over(px: &mut [f32; 4], color: Rgba, alpha: f32) {
    if alpha <= 0.0 {
        return;
    }
    let [r, g, b, _] = color.map(|c| f32::from(c) / 255.0);
    let keep = 1.0 - alpha;
    let [pr, pg, pb, pa] = *px;
    *px = [
        r.mul_add(alpha, pr * keep),
        g.mul_add(alpha, pg * keep),
        b.mul_add(alpha, pb * keep),
        pa.mul_add(keep, alpha),
    ];
}

/// Fração do pixel coberta, a partir da distância em pixels até a borda.
fn coverage(d: f32) -> f32 {
    (0.5 - d).clamp(0.0, 1.0)
}

/// Valor de 0 a 1 para um canal de 8 bits (busca binária: sem conversões com perda).
fn byte(value: f32) -> u8 {
    let target = value.clamp(0.0, 1.0) * 255.0;
    let (mut lo, mut hi) = (0_u8, 255_u8);
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if f32::from(mid) + 0.5 <= target {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}

const fn v((x, y): (f32, f32)) -> Vec2 {
    Vec2::new(x, y)
}

fn around(center: Vec2, radius: f32) -> (Vec2, Vec2) {
    (center - radius, center + radius)
}

fn points_bounds(points: &[(f32, f32)], pad: f32) -> (Vec2, Vec2) {
    let lo = points.iter().fold(Vec2::INFINITY, |m, p| m.min(v(*p)));
    let hi = points.iter().fold(Vec2::NEG_INFINITY, |m, p| m.max(v(*p)));
    (lo - pad, hi + pad)
}

/// O ponto nas coordenadas da forma: centro na origem e sem a rotação.
fn local(p: Vec2, center: (f32, f32), degrees: f32) -> Vec2 {
    Vec2::from_angle(-degrees.to_radians()).rotate(p - v(center))
}

/// Distância aproximada até uma elipse centrada na origem (boa perto da borda, que é o que
/// importa para o antisserrilhamento e o contorno).
fn ellipse_distance(p: Vec2, radii: Vec2) -> f32 {
    let k0 = (p / radii).length();
    let k1 = (p / (radii * radii)).length();
    if k1 <= f32::EPSILON {
        return -radii.min_element();
    }
    k0 * (k0 - 1.0) / k1
}

fn rect_distance(p: Vec2, half: Vec2, corner: f32) -> f32 {
    let corner = corner.min(half.min_element());
    let q = p.abs() - half + corner;
    q.max(Vec2::ZERO).length() + q.max_element().min(0.0) - corner
}

fn segment_distance(p: Vec2, a: Vec2, b: Vec2) -> f32 {
    let ab = b - a;
    let t = ((p - a).dot(ab) / ab.length_squared().max(f32::EPSILON)).clamp(0.0, 1.0);
    p.distance(ab.mul_add(Vec2::splat(t), a))
}

fn polygon_distance(p: Vec2, points: &[(f32, f32)]) -> f32 {
    let Some(&last) = points.last() else {
        return f32::MAX;
    };
    let mut a = v(last);
    let mut best = f32::MAX;
    let mut inside = false;
    for &point in points {
        let b = v(point);
        best = best.min(segment_distance(p, a, b));
        if (a.y > p.y) != (b.y > p.y) && p.x < (b.x - a.x) * (p.y - a.y) / (b.y - a.y) + a.x {
            inside = !inside;
        }
        a = b;
    }
    if inside { -best } else { best }
}

fn polyline_distance(p: Vec2, points: &[(f32, f32)]) -> f32 {
    let single = points.first().map_or(f32::MAX, |a| p.distance(v(*a)));
    points
        .windows(2)
        .filter_map(|w| Some(segment_distance(p, v(*w.first()?), v(*w.get(1)?))))
        .fold(single, f32::min)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn alpha(p: &Pixmap, x: u16, y: u16) -> u8 {
        p.get(x, y).map_or(0, |c| c[3])
    }

    #[test]
    fn distancias_negativas_dentro_e_positivas_fora() {
        let shapes = [
            circle(50.0, 50.0, 20.0),
            ellipse(50.0, 50.0, 30.0, 10.0).turned(30.0),
            rect(50.0, 50.0, 40.0, 20.0, 5.0).turned(-15.0),
            poly(&[(30.0, 30.0), (70.0, 30.0), (50.0, 70.0)]),
            line(&[(30.0, 50.0), (70.0, 50.0)], 6.0),
        ];
        for s in shapes {
            assert!(s.distance(Vec2::new(50.0, 48.0)) < 0.0, "{s:?}");
            assert!(s.distance(Vec2::new(5.0, 95.0)) > 0.0, "{s:?}");
            let (lo, hi) = s.bounds();
            assert!(lo.x <= 30.0 && hi.x >= 70.0 - 1e-3, "{s:?}");
        }
        let c = circle(0.0, 0.0, 10.0);
        assert!((c.distance(Vec2::new(13.0, 0.0)) - 3.0).abs() < 1e-4);
    }

    #[test]
    fn elipse_girada_troca_os_eixos() {
        let e = ellipse(0.0, 0.0, 20.0, 5.0);
        let up = Vec2::new(0.0, 15.0);
        assert!(e.distance(up) > 0.0);
        assert!(e.turned(90.0).distance(up) < 0.0);
    }

    #[test]
    fn recorte_e_mascara() {
        let moon = solid(circle(50.0, 50.0, 20.0), 0xff_ff_00).minus(circle(60.0, 50.0, 18.0));
        assert!(
            moon.distance(Vec2::new(60.0, 50.0)) > 0.0,
            "o miolo foi tirado"
        );
        assert!(moon.distance(Vec2::new(33.0, 50.0)) < 0.0, "a borda ficou");
        let stripe = flat(rect(50.0, 50.0, 100.0, 10.0, 0.0), 0).inside(circle(50.0, 50.0, 20.0));
        assert!(stripe.distance(Vec2::new(50.0, 50.0)) < 0.0);
        assert!(
            stripe.distance(Vec2::new(90.0, 50.0)) > 0.0,
            "fora da máscara"
        );
    }

    #[test]
    fn poligono_que_cruza_a_si_mesmo_vira_dois_triangulos() {
        let bow = poly(&[(0.0, 0.0), (20.0, 10.0), (20.0, 0.0), (0.0, 10.0)]);
        assert!(bow.distance(Vec2::new(3.0, 5.0)) < 0.0);
        assert!(bow.distance(Vec2::new(17.0, 5.0)) < 0.0);
        assert!(
            bow.distance(Vec2::new(10.0, 1.0)) > 0.0,
            "o meio de cima fica vazio"
        );
    }

    #[test]
    fn pinta_com_contorno_e_bordas_suaves() {
        let p = render(&[solid(circle(50.0, 50.0, 30.0), 0xff_00_00)], 100);
        assert_eq!(
            p.get(50, 50),
            Some([255, 0, 0, 255]),
            "miolo na cor da forma"
        );
        assert_eq!(
            p.get(81, 50),
            Some(rgb(OUTLINE)),
            "contorno por fora da borda"
        );
        assert_eq!(alpha(&p, 95, 50), 0, "fora fica transparente");
        assert_eq!(alpha(&p, 2, 2), 0);
        let soft = (0..100).any(|x| (1..255).contains(&alpha(&p, x, 50)));
        assert!(
            soft,
            "a borda tem pixels semitransparentes (sem serrilhado)"
        );
    }

    #[test]
    fn camadas_de_cima_cobrem_e_transparencia_mistura() {
        let layers = [
            flat(rect(50.0, 50.0, 100.0, 100.0, 0.0), 0x00_00_ff),
            flat(circle(50.0, 50.0, 10.0), 0xff_ff_ff).faded(0.5),
        ];
        let p = render(&layers, 50);
        assert_eq!(p.get(1, 1), Some([0, 0, 255, 255]));
        let [r, _, b, a] = p.get(25, 25).unwrap();
        assert!(
            (120..=135).contains(&r) && b == 255 && a == 255,
            "{r} {b} {a}"
        );
    }

    #[test]
    fn canal_de_oito_bits_arredonda() {
        assert_eq!(byte(0.0), 0);
        assert_eq!(byte(1.0), 255);
        assert_eq!(byte(0.5), 128);
        assert_eq!(byte(-3.0), 0);
        assert_eq!(byte(7.0), 255);
    }
}

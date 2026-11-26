//! Figuras das palavras do modo Letras, em desenho vetorial (ver [`super::vector`]): aparecem
//! grandes no cartão da palavra, então precisam de traço limpo, não de pixel art.

mod animals;
mod food;
mod things;

use super::pixmap::Pixmap;
use super::vector::{Layer, circle, ellipse, flat, render};
use crate::letters::words::Picture;

/// Lado das imagens das figuras, em pixels (nítidas até em tela cheia).
pub const PICTURE_PX: u16 = 384;

const INK: u32 = 0x4b_3b_63;
const WHITE: u32 = 0xff_ff_ff;
const SHADE: u32 = 0x4b_3b_63;
const PINK: u32 = 0xff_8f_b8;
const RED: u32 = 0xff_5a_5f;
const ORANGE: u32 = 0xff_a1_3d;
const YELLOW: u32 = 0xff_d8_4d;
const GREEN: u32 = 0x6f_d3_6a;
const GREEN_DARK: u32 = 0x3f_a8_4f;
const BLUE: u32 = 0x4f_b8_ff;
const PURPLE: u32 = 0xa8_6f_e0;
const BROWN: u32 = 0x9b_6a_45;
const CREAM: u32 = 0xff_f3_dc;
const GRAY: u32 = 0xc4_bf_d4;
const GOLD: u32 = 0xff_c9_3d;
const SPOT: u32 = 0x3d_35_50;

/// Imagem de uma figura, `size` x `size` pixels.
#[must_use]
pub fn picture(p: Picture, size: u16) -> Pixmap {
    render(layers(p), size)
}

/// Camadas de cada figura, de trás para a frente.
#[must_use]
pub const fn layers(p: Picture) -> &'static [Layer] {
    match p {
        Picture::Ball => things::BALL,
        Picture::Duck => animals::DUCK,
        Picture::Cat => animals::CAT,
        Picture::House => things::HOUSE,
        Picture::Frog => animals::FROG,
        Picture::Moon => things::MOON,
        Picture::Grapes => food::GRAPES,
        Picture::Kite => things::KITE,
        Picture::Cake => food::CAKE,
        Picture::Die => things::DIE,
        Picture::Banana => food::BANANA,
        Picture::Tomato => food::TOMATO,
        Picture::Cup => things::CUP,
        Picture::Pear => food::PEAR,
        Picture::Bell => things::BELL,
        Picture::Seal => animals::SEAL,
        Picture::Mouse => animals::MOUSE,
        Picture::Window => things::WINDOW,
        Picture::Shoe => things::SHOE,
        Picture::Popcorn => food::POPCORN,
        Picture::Cow => animals::COW,
        Picture::Monkey => animals::MONKEY,
        Picture::Rooster => animals::ROOSTER,
        Picture::Giraffe => animals::GIRAFFE,
        Picture::Toucan => animals::TOUCAN,
        Picture::Whale => animals::WHALE,
        Picture::Coconut => food::COCONUT,
        Picture::Lollipop => food::LOLLIPOP,
        Picture::Soup => food::SOUP,
        Picture::Suitcase => things::SUITCASE,
        Picture::Table => things::TABLE,
        Picture::Bed => things::BED,
        Picture::Candle => things::CANDLE,
        Picture::Boot => things::BOOT,
        Picture::Ship => things::SHIP,
        Picture::Magnifier => things::MAGNIFIER,
        Picture::Teapot => things::TEAPOT,
        Picture::Doll => things::DOLL,
        Picture::Fire => things::FIRE,
        Picture::Mug => things::MUG,
    }
}

/// Brilho branco (reflexo de luz).
const fn shine(x: f32, y: f32, rx: f32, ry: f32) -> Layer {
    flat(ellipse(x, y, rx, ry).turned(-35.0), WHITE).faded(0.85)
}

/// Olho: bolinha escura com reflexo.
const fn eye(x: f32, y: f32, r: f32) -> [Layer; 2] {
    [
        flat(circle(x, y, r), INK),
        flat(circle(x + r * 0.35, y - r * 0.4, r * 0.38), WHITE),
    ]
}

/// Bochecha rosada.
const fn cheek(x: f32, y: f32, r: f32) -> Layer {
    flat(ellipse(x, y, r, r * 0.65), PINK).faded(0.7)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toda_figura_desenha_algo_no_meio_e_deixa_margem() {
        for p in Picture::ALL {
            let img = picture(p, 100);
            let opaque = img.coords().filter(|(x, y)| img.is_opaque(*x, *y)).count();
            assert!(opaque > 1500, "{p:?} quase vazia: {opaque}");
            let edges = (0..100).flat_map(|i| [(i, 0), (i, 99), (0, i), (99, i)]);
            for (x, y) in edges {
                assert!(!img.is_opaque(x, y), "{p:?} encosta na borda em ({x}, {y})");
            }
        }
    }

    #[test]
    fn figuras_sao_diferentes_entre_si() {
        let all: Vec<Pixmap> = Picture::ALL.iter().map(|p| picture(*p, 48)).collect();
        for (i, a) in all.iter().enumerate() {
            for b in all.iter().skip(i + 1) {
                assert_ne!(a, b);
            }
        }
    }
}

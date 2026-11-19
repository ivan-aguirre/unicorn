//! Toda a arte do jogo, montada em memória na inicialização (nenhum arquivo externo).

pub mod pixmap;
pub mod procedural;
pub mod sprites;

use pixmap::{ArtError, Pixmap, rgb};
use sprites::{EYE_AT, EYE_CLOSED, LEGS_Y, OUTLINE, PALETTE, UNICORN_BODY};

use crate::level::scenery::FlowerColor;

/// Imagens prontas para virar texturas.
#[derive(Debug, Clone)]
pub struct Art {
    /// Um quadro por [`Pose`](crate::player::Pose), na ordem de `Pose::index`.
    pub unicorn: Vec<Pixmap>,
    pub heart: Pixmap,
    pub heart_small: Pixmap,
    pub star: Pixmap,
    pub sparkle: Pixmap,
    /// Flores rosa, amarela e roxa, dois quadros cada.
    pub flowers: [[Pixmap; 2]; 3],
    pub tree: Pixmap,
    pub blossom: Pixmap,
    pub bush: Pixmap,
    pub clouds: [Pixmap; 3],
    pub sun: [Pixmap; 2],
    pub rainbow: Pixmap,
    pub soil: Pixmap,
    pub grass: Pixmap,
    pub glyphs: Vec<(char, Pixmap)>,
}

impl Art {
    /// Monta toda a arte.
    ///
    /// # Errors
    /// Algum sprite ASCII inválido (coberto pelos testes).
    pub fn build() -> Result<Self, ArtError> {
        let outline = rgb(OUTLINE);
        let sprite = |rows: &[&str]| Pixmap::from_ascii(rows, PALETTE).map(|p| p.outlined(outline));
        Ok(Self {
            unicorn: unicorn_frames()?,
            heart: sprite(sprites::HEART)?,
            heart_small: sprite(sprites::HEART_SMALL)?,
            star: sprite(sprites::STAR)?,
            sparkle: Pixmap::from_ascii(sprites::SPARKLE, PALETTE)?,
            flowers: [
                flower(FlowerColor::Pink)?,
                flower(FlowerColor::Yellow)?,
                flower(FlowerColor::Purple)?,
            ],
            tree: procedural::tree(false),
            blossom: procedural::tree(true),
            bush: procedural::bush(),
            clouds: [
                procedural::cloud(0),
                procedural::cloud(1),
                procedural::cloud(2),
            ],
            sun: [procedural::sun(0), procedural::sun(1)],
            rainbow: procedural::rainbow(),
            soil: procedural::soil(),
            grass: procedural::grass(),
            glyphs: sprites::GLYPHS
                .iter()
                .map(|(c, rows)| {
                    Pixmap::from_ascii(rows, PALETTE).map(|p| (*c, p.shadowed(outline)))
                })
                .collect::<Result<_, _>>()?,
        })
    }
}

/// Índice da flor de cada cor em [`Art::flowers`].
#[must_use]
pub const fn flower_index(color: FlowerColor) -> usize {
    match color {
        FlowerColor::Pink => 0,
        FlowerColor::Yellow => 1,
        FlowerColor::Purple => 2,
    }
}

/// Quadros do unicórnio: pernas + corpo (com balanço) + olho, e o contorno por fim.
fn unicorn_frames() -> Result<Vec<Pixmap>, ArtError> {
    use sprites::{LEGS_JUMP, LEGS_PASS, LEGS_STAND, LEGS_STRIDE_A, LEGS_STRIDE_B};
    // Na ordem de `Pose::index`.
    let frames = [
        Frame::new(LEGS_STAND),
        Frame {
            blink: true,
            ..Frame::new(LEGS_STAND)
        },
        Frame::bobbing(LEGS_STRIDE_A),
        Frame::new(LEGS_PASS),
        Frame::bobbing(LEGS_STRIDE_B),
        Frame::new(LEGS_PASS),
        Frame::new(LEGS_JUMP),
        Frame::new(LEGS_STAND),
    ];
    frames.iter().map(Frame::render).collect()
}

/// Receita de um quadro do unicórnio.
struct Frame {
    legs: &'static [&'static str],
    /// Corpo 1 pixel mais baixo (pernas abertas na passada).
    bob: bool,
    blink: bool,
}

impl Frame {
    const fn new(legs: &'static [&'static str]) -> Self {
        Self {
            legs,
            bob: false,
            blink: false,
        }
    }

    const fn bobbing(legs: &'static [&'static str]) -> Self {
        Self {
            bob: true,
            ..Self::new(legs)
        }
    }

    fn render(&self) -> Result<Pixmap, ArtError> {
        let mut body = Pixmap::from_ascii(UNICORN_BODY, PALETTE)?;
        if self.blink {
            body.overlay(
                &Pixmap::from_ascii(EYE_CLOSED, PALETTE)?,
                EYE_AT.0,
                EYE_AT.1,
            );
        }
        let legs = Pixmap::from_ascii(self.legs, PALETTE)?;
        let mut frame = Pixmap::new(body.width(), LEGS_Y + legs.height());
        frame.overlay(&legs, 0, LEGS_Y);
        frame.overlay(&body, 0, u16::from(self.bob));
        Ok(frame.outlined(rgb(OUTLINE)))
    }
}

/// Ícone quadrado (RGBA, `size` x `size`) com a cabeça do unicórnio, ampliada sem borrar.
#[must_use]
pub fn icon(frame: &Pixmap, size: u16) -> Vec<u8> {
    let crop = frame.width().min(frame.height()).max(1);
    let mut out = Pixmap::new(size, size);
    for (x, y) in out.coords() {
        let source = (x * crop / size.max(1), y * crop / size.max(1));
        if let Some(c) = frame.get(source.0, source.1) {
            out.set(x, y, c);
        }
    }
    out.bytes().to_vec()
}

fn flower(color: FlowerColor) -> Result<[Pixmap; 2], ArtError> {
    let (petal, center) = match color {
        FlowerColor::Pink => (0xff_8c_cf, 0xff_e1_4d),
        FlowerColor::Yellow => (0xff_e1_4d, 0xf0_8a_3a),
        FlowerColor::Purple => (0xb9_8c_ff, 0xff_f3_a8),
    };
    let swaps = [
        (rgb(0xff_8c_cf), rgb(petal)),
        (rgb(0xff_e1_4d), rgb(center)),
    ];
    let frame = |rows: &[&str]| -> Result<Pixmap, ArtError> {
        let base = Pixmap::from_ascii(rows, PALETTE)?;
        Ok(base.recolored(&swaps).outlined(rgb(OUTLINE)))
    };
    Ok([frame(sprites::FLOWER[0])?, frame(sprites::FLOWER[1])?])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::player::Pose;

    #[test]
    fn monta_toda_a_arte() {
        let art = Art::build().unwrap();
        assert_eq!(art.unicorn.len(), Pose::COUNT);
        assert_eq!(art.glyphs.len(), sprites::GLYPHS.len());
    }

    #[test]
    fn quadros_do_unicornio_tem_o_mesmo_tamanho_e_variam() {
        let art = Art::build().unwrap();
        let first = &art.unicorn[0];
        assert!(
            art.unicorn
                .iter()
                .all(|f| (f.width(), f.height()) == (first.width(), first.height()))
        );
        assert_ne!(art.unicorn[0], art.unicorn[1], "piscar muda o quadro");
        assert_ne!(art.unicorn[2], art.unicorn[4], "passadas diferentes");
    }

    #[test]
    fn icone_tem_o_tamanho_pedido_e_conteudo() {
        let art = Art::build().unwrap();
        for size in [16_u16, 32, 64] {
            let bytes = icon(&art.unicorn[0], size);
            assert_eq!(bytes.len(), usize::from(size) * usize::from(size) * 4);
            assert!(bytes.as_chunks::<4>().0.iter().any(|px| px[3] > 0));
        }
    }

    #[test]
    fn flores_de_cores_diferentes() {
        let art = Art::build().unwrap();
        assert_ne!(art.flowers[0][0], art.flowers[1][0]);
        assert_ne!(art.flowers[1][0], art.flowers[2][0]);
        assert_ne!(art.flowers[0][0], art.flowers[0][1], "a flor balança");
    }
}

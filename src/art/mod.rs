//! Toda a arte do jogo, montada em memória na inicialização (nenhum arquivo externo).

pub mod pixmap;
pub mod procedural;
pub mod sprites;
pub mod unicorn;

use pixmap::{ArtError, Pixmap, rgb};
use sprites::{OUTLINE, PALETTE};

use crate::level::scenery::FlowerColor;

/// Tamanho do recheio do coração coletável (o contorno acrescenta 1 pixel em volta).
pub const HEART_SIZE: (u16, u16) = (21, 19);

/// Imagens prontas para virar texturas.
#[derive(Debug, Clone)]
pub struct Art {
    /// Camadas do corpo, uma por [`Pose`](crate::animation::Pose), na ordem de `Pose::index`.
    pub unicorn: Vec<unicorn::BodyFrame>,
    /// Cabeças, uma por [`Expression`](crate::animation::Expression).
    pub heads: Vec<Pixmap>,
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
            unicorn: unicorn::body_frames(),
            heads: unicorn::heads()?,
            heart: procedural::heart(HEART_SIZE.0, HEART_SIZE.1),
            heart_small: procedural::heart(11, 10),
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

/// Ícone quadrado (RGBA, `size` x `size`) com a cabeça do unicórnio, ampliada sem borrar.
#[must_use]
pub fn icon(head: &Pixmap, size: u16) -> Vec<u8> {
    let crop = head.width().min(head.height()).max(1);
    let mut out = Pixmap::new(size, size);
    for (x, y) in out.coords() {
        let source = (x * crop / size.max(1), y * crop / size.max(1));
        if let Some(c) = head.get(source.0, source.1) {
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
    use crate::animation::{Expression, Pose};

    #[test]
    fn monta_toda_a_arte() {
        let art = Art::build().unwrap();
        assert_eq!(art.unicorn.len(), Pose::COUNT);
        assert_eq!(art.heads.len(), Expression::COUNT);
        assert_eq!(art.glyphs.len(), sprites::GLYPHS.len());
    }

    #[test]
    fn coracoes_sao_grandes() {
        let art = Art::build().unwrap();
        assert!(art.heart.width() > 20 && art.heart.height() > 18);
    }

    #[test]
    fn icone_tem_o_tamanho_pedido_e_conteudo() {
        let art = Art::build().unwrap();
        for size in [16_u16, 32, 64] {
            let bytes = icon(&art.heads[0], size);
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

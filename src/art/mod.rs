//! Toda a arte do jogo, montada em memória na inicialização (nenhum arquivo externo).

pub mod pixmap;
pub mod procedural;
pub mod sprites;
pub mod unicorn;

use pixmap::{ArtError, Pixmap, rgb};
use sprites::{OUTLINE, PALETTE};

use crate::letters::words::Picture;
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
    pub tree: procedural::Tree,
    pub blossom: procedural::Tree,
    pub bush: Pixmap,
    pub clouds: [Pixmap; 3],
    /// Sol com os raios girando, [`procedural::SUN_FRAMES`] quadros.
    pub sun: Vec<Pixmap>,
    pub critters: Critters,
    pub rainbow: Pixmap,
    /// Nuvens sorridentes nas bases do arco-íris da festa (esquerda e direita).
    pub party_clouds: [Pixmap; 2],
    pub soil: Pixmap,
    pub grass: Pixmap,
    pub glyphs: Vec<(char, Pixmap)>,
    /// Figuras das palavras, na ordem de [`Picture::index`].
    pub pictures: Vec<Pixmap>,
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
                recolored_pair(sprites::FLOWER, flower_colors(FlowerColor::Pink))?,
                recolored_pair(sprites::FLOWER, flower_colors(FlowerColor::Yellow))?,
                recolored_pair(sprites::FLOWER, flower_colors(FlowerColor::Purple))?,
            ],
            tree: procedural::tree(false),
            blossom: procedural::tree(true),
            bush: procedural::bush(),
            clouds: [
                procedural::cloud(0),
                procedural::cloud(1),
                procedural::cloud(2),
            ],
            sun: (0..procedural::SUN_FRAMES).map(procedural::sun).collect(),
            critters: Critters::build()?,
            rainbow: procedural::rainbow(),
            party_clouds: [
                procedural::smiley_cloud(procedural::CloudFace::Happy),
                procedural::smiley_cloud(procedural::CloudFace::Cheering),
            ],
            soil: procedural::soil(),
            grass: procedural::grass(),
            glyphs: sprites::GLYPHS
                .iter()
                .map(|(c, rows)| {
                    Pixmap::from_ascii(rows, PALETTE).map(|p| (*c, p.shadowed(outline)))
                })
                .collect::<Result<_, _>>()?,
            pictures: Picture::ALL
                .iter()
                .map(|p| picture(*p, outline))
                .collect::<Result<_, _>>()?,
        })
    }
}

/// Figura de uma palavra, com 1 pixel de margem para o contorno não ser cortado nas bordas.
fn picture(p: Picture, outline: pixmap::Rgba) -> Result<Pixmap, ArtError> {
    let drawn = Pixmap::from_ascii(sprites::picture(p), PALETTE)?;
    let mut padded = Pixmap::new(drawn.width() + 2, drawn.height() + 2);
    padded.overlay(&drawn, 1, 1);
    Ok(padded.outlined(outline))
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

/// Bichinhos do cenário.
#[derive(Debug, Clone)]
pub struct Critters {
    /// Passarinho: quatro quadros batendo as asas e um planando.
    pub bird: Vec<Pixmap>,
    /// Rosa, amarela e azul; asas abertas e fechadas.
    pub butterflies: [[Pixmap; 2]; 3],
    pub fish: Pixmap,
    pub petal: Pixmap,
}

impl Critters {
    fn build() -> Result<Self, ArtError> {
        let plain = |rows: &[&str]| Pixmap::from_ascii(rows, PALETTE);
        let wings = |petal: u32, spot: u32| recolored_pair(sprites::BUTTERFLY, (petal, spot));
        Ok(Self {
            bird: (0..procedural::BIRD_FRAMES).map(procedural::bird).collect(),
            butterflies: [
                wings(0xff_8c_cf, 0xff_e1_4d)?,
                wings(0xff_e1_4d, 0xf0_8a_3a)?,
                wings(0x7e_c8_ff, 0xff_ff_ff)?,
            ],
            fish: plain(sprites::FISH)?.outlined(rgb(OUTLINE)),
            petal: procedural::petal(),
        })
    }
}

/// Cores de pétala e miolo de cada flor.
const fn flower_colors(color: FlowerColor) -> (u32, u32) {
    match color {
        FlowerColor::Pink => (0xff_8c_cf, 0xff_e1_4d),
        FlowerColor::Yellow => (0xff_e1_4d, 0xf0_8a_3a),
        FlowerColor::Purple => (0xb9_8c_ff, 0xff_f3_a8),
    }
}

/// Dois quadros com as cores `P`/`p` da paleta trocadas por `(main, detail)`, contornados.
fn recolored_pair(
    frames: [&[&str]; 2],
    (main, detail): (u32, u32),
) -> Result<[Pixmap; 2], ArtError> {
    let swaps = [(rgb(0xff_8c_cf), rgb(main)), (rgb(0xff_e1_4d), rgb(detail))];
    let frame = |rows: &[&str]| -> Result<Pixmap, ArtError> {
        let base = Pixmap::from_ascii(rows, PALETTE)?;
        Ok(base.recolored(&swaps).outlined(rgb(OUTLINE)))
    };
    Ok([frame(frames[0])?, frame(frames[1])?])
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
        assert_eq!(art.pictures.len(), Picture::ALL.len());
    }

    #[test]
    fn figuras_tem_contorno_inteiro_e_tamanho_parecido() {
        let art = Art::build().unwrap();
        for p in &art.pictures {
            assert!(
                (16..=22).contains(&p.width()),
                "{}x{}",
                p.width(),
                p.height()
            );
            assert!(
                (13..=22).contains(&p.height()),
                "{}x{}",
                p.width(),
                p.height()
            );
            // A margem garante o contorno nas bordas: nenhum pixel colorido encosta nelas.
            for x in 0..p.width() {
                assert!(!p.is_opaque(x, 0) || p.get(x, 0) == Some(rgb(OUTLINE)));
            }
        }
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
        let wings = &art.critters.butterflies;
        assert_ne!(wings[0][0], wings[1][0], "borboletas de cores diferentes");
        assert_ne!(wings[0][0], wings[0][1], "a borboleta bate as asas");
        assert_eq!(
            art.critters.bird.len(),
            usize::from(procedural::BIRD_FRAMES)
        );
    }
}

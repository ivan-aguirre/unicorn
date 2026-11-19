//! Envia a arte para a GPU (filtro "nearest" para manter os pixels nítidos) e monta o atlas.

use macroquad::texture::{FilterMode, Texture2D, build_textures_atlas};

use crate::art::Art;
use crate::art::pixmap::Pixmap;

/// Camadas de um quadro do unicórnio na GPU.
pub struct UnicornFrame {
    pub body: Texture2D,
    pub back_arm: Texture2D,
    pub front_arm: Texture2D,
    pub head_drop: f32,
}

/// Texturas do jogo, espelhando [`Art`].
pub struct Textures {
    pub unicorn: Vec<UnicornFrame>,
    pub heads: Vec<Texture2D>,
    pub heart: Texture2D,
    pub heart_small: Texture2D,
    pub star: Texture2D,
    pub sparkle: Texture2D,
    pub flowers: [[Texture2D; 2]; 3],
    pub tree: Texture2D,
    pub blossom: Texture2D,
    pub bush: Texture2D,
    pub clouds: [Texture2D; 3],
    pub sun: [Texture2D; 2],
    pub rainbow: Texture2D,
    pub soil: Texture2D,
    pub grass: Texture2D,
    pub glyphs: Vec<(char, Texture2D)>,
}

impl Textures {
    /// Cria as texturas e junta todas num atlas (poucas trocas de textura por quadro).
    #[must_use]
    pub fn upload(art: &Art) -> Self {
        let pair = |p: &[Pixmap; 2]| [texture(&p[0]), texture(&p[1])];
        let textures = Self {
            unicorn: art
                .unicorn
                .iter()
                .map(|f| UnicornFrame {
                    body: texture(&f.body),
                    back_arm: texture(&f.back_arm),
                    front_arm: texture(&f.front_arm),
                    head_drop: f32::from(f.head_drop),
                })
                .collect(),
            heads: art.heads.iter().map(texture).collect(),
            heart: texture(&art.heart),
            heart_small: texture(&art.heart_small),
            star: texture(&art.star),
            sparkle: texture(&art.sparkle),
            flowers: [
                pair(&art.flowers[0]),
                pair(&art.flowers[1]),
                pair(&art.flowers[2]),
            ],
            tree: texture(&art.tree),
            blossom: texture(&art.blossom),
            bush: texture(&art.bush),
            clouds: [
                texture(&art.clouds[0]),
                texture(&art.clouds[1]),
                texture(&art.clouds[2]),
            ],
            sun: pair(&art.sun),
            rainbow: texture(&art.rainbow),
            soil: texture(&art.soil),
            grass: texture(&art.grass),
            glyphs: art.glyphs.iter().map(|(c, p)| (*c, texture(p))).collect(),
        };
        build_textures_atlas();
        textures
    }

    /// Textura de um caractere da fonte, se existir.
    #[must_use]
    pub fn glyph(&self, c: char) -> Option<&Texture2D> {
        self.glyphs.iter().find(|(g, _)| *g == c).map(|(_, t)| t)
    }
}

fn texture(p: &Pixmap) -> Texture2D {
    let t = Texture2D::from_rgba8(p.width(), p.height(), p.bytes());
    t.set_filter(FilterMode::Nearest);
    t
}

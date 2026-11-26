//! Envia a arte para a GPU (filtro "nearest" para manter os pixels nítidos) e monta o atlas.

use macroquad::texture::{FilterMode, Texture2D, build_textures_atlas, set_default_filter_mode};

use crate::art::Art;
use crate::art::pictures::{PICTURE_PX, picture};
use crate::art::pixmap::Pixmap;
use crate::art::procedural::Tree;
use crate::letters::words::Picture;

/// Camadas de um quadro do unicórnio na GPU.
pub struct UnicornFrame {
    pub body: Texture2D,
    pub head_drop: f32,
}

/// Tronco e copa de uma árvore.
pub struct TreeTextures {
    pub trunk: Texture2D,
    pub crown: Texture2D,
}

impl TreeTextures {
    fn upload(tree: &Tree) -> Self {
        Self {
            trunk: texture(&tree.trunk),
            crown: texture(&tree.crown),
        }
    }
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
    pub tree: TreeTextures,
    pub blossom: TreeTextures,
    pub bush: Texture2D,
    pub clouds: [Texture2D; 3],
    pub sun: Vec<Texture2D>,
    pub bird: Vec<Texture2D>,
    pub butterflies: [[Texture2D; 2]; 3],
    pub fish: Texture2D,
    pub petal: Texture2D,
    pub rainbow: Texture2D,
    pub party_clouds: [Texture2D; 2],
    pub soil: Texture2D,
    pub grass: Texture2D,
    pub glyphs: Vec<(char, Texture2D)>,
    /// Figuras das palavras em alta definição, na ordem de [`Picture::index`]. Ficam fora do
    /// atlas e com filtro linear: são desenhadas grandes e suaves, não como pixel art.
    pub pictures: Vec<Texture2D>,
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
            tree: TreeTextures::upload(&art.tree),
            blossom: TreeTextures::upload(&art.blossom),
            bush: texture(&art.bush),
            clouds: [
                texture(&art.clouds[0]),
                texture(&art.clouds[1]),
                texture(&art.clouds[2]),
            ],
            sun: art.sun.iter().map(texture).collect(),
            bird: art.critters.bird.iter().map(texture).collect(),
            butterflies: [
                pair(&art.critters.butterflies[0]),
                pair(&art.critters.butterflies[1]),
                pair(&art.critters.butterflies[2]),
            ],
            fish: texture(&art.critters.fish),
            petal: texture(&art.critters.petal),
            rainbow: texture(&art.rainbow),
            party_clouds: pair(&art.party_clouds),
            soil: texture(&art.soil),
            grass: texture(&art.grass),
            glyphs: art.glyphs.iter().map(|(c, p)| (*c, texture(p))).collect(),
            pictures: Vec::new(),
        };
        // O atlas tem filtro próprio (linear por padrão): sem isto, tudo o que é desenhado
        // ampliado sai borrado.
        set_default_filter_mode(FilterMode::Nearest);
        build_textures_atlas();
        Self {
            pictures: pictures(),
            ..textures
        }
    }

    /// Figura de uma palavra.
    #[must_use]
    pub fn picture(&self, p: Picture) -> Option<&Texture2D> {
        self.pictures.get(p.index())
    }

    /// Textura de um caractere da fonte, se existir.
    #[must_use]
    pub fn glyph(&self, c: char) -> Option<&Texture2D> {
        self.glyphs.iter().find(|(g, _)| *g == c).map(|(_, t)| t)
    }
}

/// Figuras das palavras: criadas depois do atlas para ficarem de fora dele, com filtro linear.
fn pictures() -> Vec<Texture2D> {
    Picture::ALL
        .iter()
        .map(|p| {
            let img = picture(*p, PICTURE_PX);
            let t = Texture2D::from_rgba8(img.width(), img.height(), img.bytes());
            t.set_filter(FilterMode::Linear);
            t
        })
        .collect()
}

fn texture(p: &Pixmap) -> Texture2D {
    let t = Texture2D::from_rgba8(p.width(), p.height(), p.bytes());
    t.set_filter(FilterMode::Nearest);
    t
}

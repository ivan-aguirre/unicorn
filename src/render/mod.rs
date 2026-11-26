//! Desenho: tudo vai para uma tela virtual pequena (pixel art), ampliada na janela no fim.

pub mod ambient;
pub mod backdrop;
pub mod hud;
pub mod letters;
pub mod menu;
pub mod scene;
pub mod smooth;
pub mod textures;
pub mod viewport;

use macroquad::camera::{Camera2D, set_camera, set_default_camera};
use macroquad::color::WHITE;
use macroquad::math::{Rect, Vec2};
use macroquad::text::{Font, load_ttf_font_from_bytes};
use macroquad::texture::{
    DrawTextureParams, FilterMode, RenderTarget, draw_texture_ex, render_target,
};
use macroquad::window::{clear_background, screen_height, screen_width};

use crate::art::Art;
use crate::game::Game;
use crate::letters::Letters;
use crate::level::Level;
use crate::menu::Menu;
use crate::view::{VIEW_H, VIEW_H_PX, VIEW_W, VIEW_W_PX};
use smooth::Painter;
use textures::Textures;

/// Cor da moldura em volta da tela virtual.
const LETTERBOX: u32 = 0x1b_14_30;

/// Fonte das letras suaves (Baloo 2, SIL Open Font License: ver `assets/fontes/OFL.txt`).
const FONT: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/fontes/Baloo2-ExtraBold.ttf"
));

/// Dono das texturas, da fonte e da tela virtual.
pub struct Renderer {
    textures: Textures,
    font: Option<Font>,
    target: RenderTarget,
    camera: Camera2D,
}

impl Renderer {
    /// Envia a arte para a GPU e prepara a tela virtual.
    #[must_use]
    pub fn new(art: &Art) -> Self {
        let target = render_target(u32::from(VIEW_W_PX), u32::from(VIEW_H_PX));
        target.texture.set_filter(FilterMode::Nearest);
        let mut camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, VIEW_W, VIEW_H));
        camera.render_target = Some(target.clone());
        Self {
            textures: Textures::upload(art),
            font: load_ttf_font_from_bytes(FONT).ok(),
            target,
            camera,
        }
    }

    /// Desenha um quadro do jogo e o amplia na janela. `quit_progress` (0 a 1) mostra o
    /// aviso de saída enquanto Esc está segurado.
    pub fn draw(&self, game: &Game, quit_progress: f32) {
        set_camera(&self.camera);
        scene::draw(&self.textures, game);
        hud::draw(&self.textures, game, quit_progress);
        self.present();
    }

    /// Desenha um quadro do modo Letras: cenário em pixel art e, por cima, o cartão da palavra
    /// e o teclado suaves na resolução da janela.
    pub fn draw_letters(&self, letters: &Letters, quit_progress: f32) {
        set_camera(&self.camera);
        letters::draw_pixels(&self.textures, letters);
        self.quit(quit_progress);
        self.present();
        letters::draw_smooth(&self.painter(), &self.textures, letters);
    }

    /// Desenha a tela inicial (`level` dá o cenário de fundo).
    pub fn draw_menu(&self, menu: &Menu, level: &Level, quit_progress: f32) {
        set_camera(&self.camera);
        menu::draw_pixels(&self.textures, level, menu);
        self.quit(quit_progress);
        self.present();
        menu::draw_smooth(&self.painter(), &self.textures, menu);
    }

    fn painter(&self) -> Painter<'_> {
        Painter::new(self.font.as_ref())
    }

    fn quit(&self, progress: f32) {
        if progress > 0.0 {
            hud::draw_quit(&self.textures, progress);
        }
    }

    /// Imagem da tela virtual (usada pelo exemplo de capturas de tela).
    #[must_use]
    pub const fn target(&self) -> &RenderTarget {
        &self.target
    }

    /// Texturas carregadas (usadas pelo exemplo de capturas de tela).
    #[must_use]
    pub const fn textures(&self) -> &Textures {
        &self.textures
    }

    fn present(&self) {
        set_default_camera();
        clear_background(scene::color(LETTERBOX));
        let v = viewport::fit(screen_width(), screen_height());
        draw_texture_ex(
            &self.target.texture,
            v.x,
            v.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(v.w, v.h)),
                // A tela virtual é gerada de cabeça para baixo.
                flip_y: true,
                ..Default::default()
            },
        );
    }
}

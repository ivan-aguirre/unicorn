//! Gera capturas da tela virtual em cenas típicas, para revisar a arte sem jogar.
//!
//! Uso: `cargo run --example snapshot -- <pasta>` (padrão: `target/snapshots`).
//! Precisa de uma janela (contexto OpenGL); fecha sozinho ao terminar.

use fastrand::Rng;
use macroquad::prelude::*;
use unicorn::art::Art;
use unicorn::art::pixmap::Pixmap;
use unicorn::camera::Camera;
use unicorn::clock::STEP;
use unicorn::game::{Game, Mode};
use unicorn::input::Input;
use unicorn::letters::Letters;
use unicorn::letters::words::{WORDS, Word};
use unicorn::level::{Level, SEED};
use unicorn::menu::{Choice, Menu, MenuInput};
use unicorn::player::Player;
use unicorn::render::Renderer;
use unicorn::render::smooth::{Painter, TextStyle};

/// Posições do nível a fotografar (x do unicórnio).
const SPOTS: [f32; 6] = [160.0, 2500.0, 7000.0, 15_000.0, 28_000.0, 54_200.0];

fn conf() -> Conf {
    Conf {
        window_title: String::from("Unicórnio - capturas"),
        window_width: 1280,
        window_height: 720,
        sample_count: 4,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let dir = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "target/snapshots".into());
    let (Ok(()), Ok(art)) = (std::fs::create_dir_all(&dir), Art::build()) else {
        return;
    };
    let renderer = Renderer::new(&art);
    let mut game = Game::new(Level::generate(SEED), Rng::with_seed(1));

    for (i, x) in SPOTS.iter().enumerate() {
        teleport(&mut game, *x);
        settle(&mut game, &renderer, Input::RIGHT, 40).await;
        save(&renderer, &format!("{dir}/cena_{i}.png"));
    }

    // Sequência da mesma cena, para ver as animações do cenário em movimento.
    teleport(&mut game, 9000.0);
    for i in 0..6 {
        settle(&mut game, &renderer, Input::default(), 9).await;
        save(&renderer, &format!("{dir}/sequencia_{i}.png"));
    }

    teleport(&mut game, 4300.0);
    game.effects.celebrate(game.camera.x, unicorn::view::VIEW_W);
    settle(&mut game, &renderer, Input::default(), 50).await;
    save(&renderer, &format!("{dir}/festa.png"));

    game.mode = Mode::Paused;
    settle(&mut game, &renderer, Input::default(), 2).await;
    save(&renderer, &format!("{dir}/pausa.png"));

    renderer.draw(&game, 0.6);
    next_frame().await;
    save(&renderer, &format!("{dir}/sair.png"));

    menu_and_letters(&renderer, &dir).await;
    word_sheets(&renderer, &dir).await;

    sprite_sheet(&art, &format!("{dir}/sprites.png"));
}

/// Tela inicial e o modo Letras (capturas da janela: inclui o desenho suave).
async fn menu_and_letters(renderer: &Renderer, dir: &str) {
    menu_shot(renderer, dir).await;
    let mut l = Letters::new(Level::generate(SEED), Rng::with_seed(3));
    word_shots(&mut l, renderer, dir).await;
}

async fn menu_shot(renderer: &Renderer, dir: &str) {
    let level = Level::generate(SEED);
    let mut menu = Menu::new(Choice::Letters);
    for _ in 0..30 {
        menu.update(MenuInput::default(), 1.0 / 60.0);
        renderer.draw_menu(&menu, &level, 0.0);
        next_frame().await;
    }
    renderer.draw_menu(&menu, &level, 0.0);
    save_screen(&format!("{dir}/menu.png"));
    next_frame().await;
}

/// O cartão da palavra vazio, pela metade com uma tecla errada, completo (festa) e o seguinte.
async fn word_shots(l: &mut Letters, renderer: &Renderer, dir: &str) {
    letters_frames(l, renderer, 30).await;
    shot(l, renderer, &format!("{dir}/letras_palavra.png")).await;
    let word = l.words.word().text;
    let (start, rest) = word.split_at(2);
    type_text(l, start);
    type_text(l, "Q");
    letters_frames(l, renderer, 6).await;
    shot(l, renderer, &format!("{dir}/letras_palavra_metade.png")).await;
    type_text(l, rest);
    letters_frames(l, renderer, 50).await;
    shot(l, renderer, &format!("{dir}/letras_palavra_completa.png")).await;
    letters_frames(l, renderer, 240).await;
    shot(l, renderer, &format!("{dir}/letras_palavra_seguinte.png")).await;
}

/// Digita o texto, uma letra por passo.
fn type_text(l: &mut Letters, text: &str) {
    for c in text.chars() {
        l.step(STEP, &[c]);
    }
}

/// Prévia de todas as palavras do modo Letras, 20 por página.
async fn word_sheets(renderer: &Renderer, dir: &str) {
    for (page, words) in (1_u8..).zip(WORDS.chunks(20)) {
        word_sheet(renderer, words, &format!("{dir}/palavras_{page}.png")).await;
    }
}

/// Prévia de até 20 palavras do modo Letras, cada uma com o desenho dela.
async fn word_sheet(renderer: &Renderer, words: &[Word], path: &str) {
    let font = load_ttf_font_from_bytes(include_bytes!("../assets/fontes/Baloo2-ExtraBold.ttf"));
    let painter = Painter::new(font.as_ref().ok());
    clear_background(Color::from_hex(0x9e_d8_ff));
    let ink = Color::from_hex(0x4b_3b_63);
    let style = TextStyle {
        size: 17.0,
        fill: ink,
        outline: (ink, 0.0),
    };
    for (i, word) in (0_u8..).zip(words) {
        let cell = Vec2::new(f32::from(i % 5), f32::from(i / 5));
        let center = cell * Vec2::new(126.0, 88.0) + Vec2::new(68.0, 48.0);
        let card = Rect::new(center.x - 58.0, center.y - 42.0, 116.0, 84.0);
        painter.rounded_rect(card, 12.0, WHITE);
        if let Some(t) = renderer.textures().picture(word.picture) {
            painter.texture(t, center - Vec2::new(0.0, 10.0), 56.0 / t.width(), WHITE);
        }
        painter.text(word.text, center + Vec2::new(0.0, 29.0), style);
    }
    save_screen(path);
    next_frame().await;
}

/// Simula e desenha `frames` quadros do modo Letras.
async fn letters_frames(l: &mut Letters, renderer: &Renderer, frames: u32) {
    for _ in 0..frames {
        l.step(1.0 / 60.0, &[]);
        renderer.draw_letters(l, 0.0);
        next_frame().await;
    }
}

/// Desenha o modo Letras e salva a janela.
async fn shot(l: &Letters, renderer: &Renderer, path: &str) {
    renderer.draw_letters(l, 0.0);
    save_screen(path);
    next_frame().await;
}

/// Salva a janela inteira (a tela virtual ampliada e o desenho suave por cima).
fn save_screen(path: &str) {
    get_screen_data().export_png(path);
}

fn teleport(game: &mut Game, x: f32) {
    game.player = Player::spawn(&game.level, x);
    game.camera = Camera::new(x, game.player.facing, game.level.length());
    game.mode = Mode::Playing;
}

/// Simula e desenha alguns quadros (anima o cenário e assenta a câmera).
async fn settle(game: &mut Game, renderer: &Renderer, input: Input, frames: u32) {
    for _ in 0..frames {
        game.step(input, 1.0 / 60.0);
        renderer.draw(game, 0.0);
        next_frame().await;
    }
}

fn save(renderer: &Renderer, path: &str) {
    let image = renderer.target().texture.get_texture_data();
    image.export_png(path);
}

/// Todos os quadros do unicórnio e alguns sprites lado a lado (montado na CPU).
fn sprite_sheet(art: &Art, path: &str) {
    let frames: Vec<Pixmap> = art
        .unicorn
        .iter()
        .flat_map(|f| {
            art.heads
                .iter()
                .take(1)
                .map(|h| unicorn::art::unicorn::compose(f, h))
        })
        .chain(art.heads.iter().cloned())
        .collect();
    let sprites: Vec<&Pixmap> = frames
        .iter()
        .chain([&art.heart, &art.star, &art.flowers[0][0], &art.bush])
        .chain(art.critters.bird.iter())
        .chain(art.sun.iter().take(1))
        .collect();
    let width = sprites.iter().map(|p| p.width() + 2).sum::<u16>() + 2;
    let height = sprites.iter().map(|p| p.height()).max().unwrap_or(1) + 4;
    let mut sheet = Pixmap::new(width, height);
    for (x, y) in sheet.coords() {
        sheet.set(x, y, [130, 187, 247, 255]);
    }
    let mut x = 2;
    for p in sprites {
        sheet.overlay(p, x, 2);
        x += p.width() + 2;
    }
    // `export_png` inverte as linhas (espera dados da GPU); inverte antes para compensar.
    let row = usize::from(sheet.width()) * 4;
    let flipped: Vec<u8> = sheet.bytes().chunks(row).rev().flatten().copied().collect();
    let image = Image {
        bytes: flipped,
        width: sheet.width(),
        height: sheet.height(),
    };
    image.export_png(path);
}

//! Ligação com a janela: configuração, leitura do teclado e o laço principal.

use fastrand::Rng;
use macroquad::input::{KeyCode, is_key_down, is_key_pressed};
use macroquad::miniquad::conf::Icon;
use macroquad::time::get_frame_time;
use macroquad::window::{Conf, next_frame, set_fullscreen};

use crate::art::{self, Art};
use crate::audio::Sfx;
use crate::clock::{FixedStep, STEP};
use crate::game::{Events, Game};
use crate::input::{Button, Input, InputLatch, QuitHold};
use crate::level::{Level, SEED};
use crate::render::Renderer;

/// Teclas de teste (F9 começa a festa dos corações): só nos builds de desenvolvimento.
const TEST_KEYS: bool = cfg!(debug_assertions);

/// Configuração da janela: 2x a tela virtual, redimensionável.
#[must_use]
pub fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Unicórnio"),
        window_width: 1280,
        window_height: 720,
        window_resizable: true,
        icon: Art::build().ok().and_then(|a| window_icon(&a)),
        ..Default::default()
    }
}

fn window_icon(art: &Art) -> Option<Icon> {
    let head = art.heads.first()?;
    Some(Icon {
        small: art::icon(head, 16).try_into().ok()?,
        medium: art::icon(head, 32).try_into().ok()?,
        big: art::icon(head, 64).try_into().ok()?,
    })
}

/// Lê o teclado: setas andam, espaço pula, Enter pausa (Esc é tratado à parte: sair).
fn read_input() -> Input {
    Input {
        left: is_key_down(KeyCode::Left),
        right: is_key_down(KeyCode::Right),
        jump: Button {
            pressed: is_key_pressed(KeyCode::Space),
            held: is_key_down(KeyCode::Space),
        },
        pause: Button {
            pressed: is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::KpEnter),
            held: is_key_down(KeyCode::Enter) || is_key_down(KeyCode::KpEnter),
        },
    }
}

/// Roda o jogo até a janela ser fechada ou Esc ser segurado.
pub async fn run() {
    let Ok(art) = Art::build() else {
        return;
    };
    let renderer = Renderer::new(&art);
    let sfx = Sfx::load().await;
    let mut game = Game::new(Level::generate(SEED), Rng::new());
    let mut clock = FixedStep::default();
    let mut latch = InputLatch::default();
    let mut fullscreen = false;
    let mut quit = QuitHold::default();
    loop {
        if quit.update(is_key_down(KeyCode::Escape), get_frame_time()) {
            break;
        }
        if is_key_pressed(KeyCode::F11) {
            fullscreen = !fullscreen;
            set_fullscreen(fullscreen);
        }
        let mut events = simulate(&mut game, &mut clock, &mut latch, read_input());
        if TEST_KEYS && is_key_pressed(KeyCode::F9) {
            events = events.merge(game.celebrate_now());
        }
        if let Some(sfx) = &sfx {
            sfx.play(events);
        }
        renderer.draw(&game, quit.progress());
        next_frame().await;
    }
}

/// Roda os passos fixos devidos neste quadro; as bordas de tecla valem só no primeiro.
fn simulate(game: &mut Game, clock: &mut FixedStep, latch: &mut InputLatch, raw: Input) -> Events {
    let mut input = latch.feed(raw);
    let mut events = Events::default();
    for _ in 0..clock.advance(get_frame_time()) {
        events = events.merge(game.step(input, STEP));
        latch.consume();
        input = input.without_edges();
    }
    events
}

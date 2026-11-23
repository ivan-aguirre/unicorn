//! Ligação com a janela: configuração, leitura do teclado e o laço principal, com a tela
//! inicial e os dois modos (passeio e letras).

use fastrand::Rng;
use macroquad::input::{KeyCode, get_char_pressed, is_key_down, is_key_pressed};
use macroquad::miniquad::conf::Icon;
use macroquad::time::get_frame_time;
use macroquad::window::{Conf, next_frame, set_fullscreen};

use crate::art::{self, Art};
use crate::audio::Sfx;
use crate::clock::{FixedStep, STEP};
use crate::game::{Events, Game};
use crate::input::{Button, Input, InputLatch, QuitHold};
use crate::letters::{LetterEvents, Letters, keyboard};
use crate::level::{Level, SEED};
use crate::menu::{Choice, Menu, MenuInput};
use crate::render::Renderer;

/// Teclas de teste (F9 começa a festa dos corações): só nos builds de desenvolvimento.
const TEST_KEYS: bool = cfg!(debug_assertions);
/// Amostras por pixel do antisserrilhamento (bordas suaves das letras, bolhas e teclas).
const MSAA_SAMPLES: i32 = 4;

/// Configuração da janela: 2x a tela virtual, redimensionável.
#[must_use]
pub fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Unicórnio"),
        window_width: 1280,
        window_height: 720,
        window_resizable: true,
        sample_count: MSAA_SAMPLES,
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

/// Lê o teclado do menu: setas escolhem, Enter ou espaço confirmam.
fn read_menu_input() -> MenuInput {
    MenuInput {
        left: is_key_pressed(KeyCode::Left),
        right: is_key_pressed(KeyCode::Right),
        confirm: is_key_pressed(KeyCode::Enter)
            || is_key_pressed(KeyCode::KpEnter)
            || is_key_pressed(KeyCode::Space),
    }
}

/// Em que tela o jogo está.
enum Screen {
    Menu(Menu),
    Walk(Box<Game>),
    Letters(Box<Letters>),
}

/// Tudo o que vive enquanto a janela está aberta.
struct App {
    renderer: Renderer,
    sfx: Option<Sfx>,
    /// Cenário de fundo do menu.
    backdrop: Level,
    clock: FixedStep,
    latch: InputLatch,
    quit: QuitHold,
    /// Esc só conta depois de solto: segurar para sair de um modo não fecha o jogo em seguida.
    esc_armed: bool,
    /// Letras digitadas ainda não vistas por um passo de simulação.
    typed: Vec<char>,
    fullscreen: bool,
}

/// Roda o jogo até a janela ser fechada ou Esc ser segurado na tela inicial.
pub async fn run() {
    let Ok(art) = Art::build() else {
        return;
    };
    let mut app = App {
        renderer: Renderer::new(&art),
        sfx: Sfx::load().await,
        backdrop: Level::generate(SEED),
        clock: FixedStep::default(),
        latch: InputLatch::default(),
        quit: QuitHold::default(),
        esc_armed: true,
        typed: Vec::new(),
        fullscreen: false,
    };
    let mut screen = Screen::Menu(Menu::new(Choice::Walk));
    while let Some(next) = app.frame(screen) {
        screen = next;
        next_frame().await;
    }
}

impl App {
    /// Um quadro: devolve a tela seguinte, ou `None` para fechar o jogo.
    fn frame(&mut self, screen: Screen) -> Option<Screen> {
        let esc = is_key_down(KeyCode::Escape);
        self.esc_armed |= !esc;
        if self.quit.update(esc && self.esc_armed, get_frame_time()) {
            self.quit = QuitHold::default();
            self.esc_armed = false;
            return match screen {
                Screen::Menu(_) => None,
                Screen::Walk(_) | Screen::Letters(_) => Some(menu_after(&screen)),
            };
        }
        if is_key_pressed(KeyCode::F11) {
            self.fullscreen = !self.fullscreen;
            set_fullscreen(self.fullscreen);
        }
        self.typed
            .extend(std::iter::from_fn(get_char_pressed).filter_map(keyboard::normalize));
        Some(match screen {
            Screen::Menu(menu) => self.menu(menu),
            Screen::Walk(game) => Screen::Walk(self.walk(game)),
            Screen::Letters(letters) => Screen::Letters(self.letters(letters)),
        })
    }

    fn menu(&mut self, mut menu: Menu) -> Screen {
        self.typed.clear();
        let chosen = menu.update(read_menu_input(), get_frame_time());
        self.renderer
            .draw_menu(&menu, &self.backdrop, self.quit.progress());
        self.clock = FixedStep::default();
        self.latch = InputLatch::default();
        match chosen {
            Some(Choice::Walk) => {
                Screen::Walk(Box::new(Game::new(Level::generate(SEED), Rng::new())))
            }
            Some(Choice::Letters) => {
                Screen::Letters(Box::new(Letters::new(Level::generate(SEED), Rng::new())))
            }
            None => Screen::Menu(menu),
        }
    }

    fn walk(&mut self, mut game: Box<Game>) -> Box<Game> {
        self.typed.clear();
        let mut events = self.simulate(&mut game, read_input());
        if TEST_KEYS && is_key_pressed(KeyCode::F9) {
            events = events.merge(game.celebrate_now());
        }
        if let Some(sfx) = &self.sfx {
            sfx.play(events);
        }
        self.renderer.draw(&game, self.quit.progress());
        game
    }

    fn letters(&mut self, mut letters: Box<Letters>) -> Box<Letters> {
        if pressed_any(&[KeyCode::Up, KeyCode::KpAdd, KeyCode::Equal]) {
            letters.faster();
        }
        if pressed_any(&[KeyCode::Down, KeyCode::KpSubtract, KeyCode::Minus]) {
            letters.slower();
        }
        let mut events = LetterEvents::default();
        for _ in 0..self.clock.advance(get_frame_time()) {
            events = events.merge(letters.step(STEP, &self.typed));
            self.typed.clear();
        }
        if let Some(sfx) = &self.sfx {
            sfx.play_letters(events);
        }
        self.renderer.draw_letters(&letters, self.quit.progress());
        letters
    }

    /// Roda os passos fixos devidos neste quadro; as bordas de tecla valem só no primeiro.
    fn simulate(&mut self, game: &mut Game, raw: Input) -> Events {
        let mut input = self.latch.feed(raw);
        let mut events = Events::default();
        for _ in 0..self.clock.advance(get_frame_time()) {
            events = events.merge(game.step(input, STEP));
            self.latch.consume();
            input = input.without_edges();
        }
        events
    }
}

/// Se alguma das teclas foi apertada neste quadro.
fn pressed_any(keys: &[KeyCode]) -> bool {
    keys.iter().any(|k| is_key_pressed(*k))
}

/// Volta ao menu com o modo de onde se saiu já escolhido.
const fn menu_after(screen: &Screen) -> Screen {
    let choice = match screen {
        Screen::Letters(_) => Choice::Letters,
        Screen::Menu(_) | Screen::Walk(_) => Choice::Walk,
    };
    Screen::Menu(Menu::new(choice))
}

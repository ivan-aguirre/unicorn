//! Partidas simuladas do começo ao fim, sem janela: o jogo inteiro funciona junto
//! (nível, física, corações, contador, festa, pausa e limites de memória).

use fastrand::Rng;
use macroquad::math::Vec2;
use unicorn::camera::Camera;
use unicorn::clock::STEP;
use unicorn::effects::MAX_PARTICLES;
use unicorn::game::{Events, Game, Mode};
use unicorn::hearts::MAX_HEARTS;
use unicorn::input::{Button, Input};
use unicorn::level::{Level, SEED};
use unicorn::player::Player;
use unicorn::view::VIEW_W;

/// Passos de simulação em `seconds` segundos de jogo.
const fn steps(seconds: u32) -> u32 {
    seconds * 120
}

fn new_game(seed: u64) -> Game {
    Game::new(Level::generate(SEED), Rng::with_seed(seed))
}

/// Uma criança que segura a seta para a direita e aperta espaço de vez em quando.
const fn child_input(step: u32) -> Input {
    let jump = step.is_multiple_of(90);
    Input {
        jump: Button {
            pressed: jump,
            held: jump,
        },
        ..Input::RIGHT
    }
}

/// Joga por `seconds` segundos, somando os eventos e checando invariantes a cada passo.
fn play(game: &mut Game, seconds: u32, input: impl Fn(u32) -> Input) -> Events {
    let mut total = Events::default();
    for step in 0..steps(seconds) {
        total = total.merge(game.step(input(step), STEP));
        assert!(
            game.counter.value() < 100,
            "o contador deve voltar a zero em 100"
        );
        assert!(game.hearts.len() <= MAX_HEARTS);
        assert!(game.effects.particles().len() <= MAX_PARTICLES);
        let cam = game.camera.x;
        assert!(
            (cam..=cam + VIEW_W).contains(&game.player.pos.x),
            "unicórnio fora da tela"
        );
    }
    total
}

#[test]
fn atravessa_o_nivel_inteiro_coletando_coracoes_e_fazendo_festa() {
    let mut game = new_game(1);
    let length = game.level.length();
    let events = play(&mut game, 360, child_input);
    assert!(
        game.player.pos.x > length - 20.0,
        "parou em x={}",
        game.player.pos.x
    );
    assert!(
        events.hearts >= 150,
        "coletou só {} corações",
        events.hearts
    );
    assert!(events.celebrated, "deveria ter completado 100 corações");
    assert!(events.jumped);
    assert_eq!(
        game.counter.value(),
        u8::try_from(events.hearts % 100).unwrap()
    );
}

/// Um coração no alto (exige pulo), longe de plataformas e de outros corações.
fn high_heart(game: &Game) -> Option<Vec2> {
    let ground = |x: f32| game.level.terrain.height_at(x);
    let hearts = game.hearts.in_span(1000.0, 30_000.0);
    hearts.iter().map(|h| h.pos).find(|pos| {
        let alone = hearts
            .iter()
            .filter(|o| (o.pos.x - pos.x).abs() < 60.0)
            .count()
            == 1;
        ground(pos.x) - pos.y > 40.0 && game.level.platform_at(pos.x).is_none() && alone
    })
}

fn teleport(game: &mut Game, x: f32) {
    game.player = Player::spawn(&game.level, x);
    game.camera = Camera::new(x, game.player.facing, game.level.length());
}

fn still_there(game: &Game, pos: Vec2) -> bool {
    game.hearts
        .in_span(pos.x - 0.5, pos.x + 0.5)
        .iter()
        .any(|h| h.pos == pos)
}

#[test]
fn pular_alcanca_coracao_no_alto_que_andando_nao_alcanca() {
    let mut game = new_game(7);
    let target = high_heart(&game).expect("o nível tem corações no alto");

    teleport(&mut game, target.x - 80.0);
    play(&mut game, 2, |_| Input::RIGHT);
    assert!(game.player.pos.x > target.x + 20.0);
    assert!(still_there(&game, target), "andando não deveria alcançar");

    teleport(&mut game, target.x - 80.0);
    let mut jumped = false;
    for _ in 0..steps(2) {
        let jump = !jumped && game.player.pos.x >= target.x - 12.0;
        jumped |= jump;
        let input = Input {
            jump: Button {
                pressed: jump,
                held: jump,
            },
            ..Input::RIGHT
        };
        game.step(input, STEP);
    }
    assert!(!still_there(&game, target), "pulando deveria pegar");
}

#[test]
fn coracoes_continuam_aparecendo_durante_o_jogo() {
    let mut game = new_game(3);
    let at_start = game.hearts.len();
    // Vai e volta várias vezes no começo do nível, pegando tudo por ali.
    let back_and_forth = |step: u32| {
        if (step / steps(8)).is_multiple_of(2) {
            child_input(step)
        } else {
            Input {
                left: true,
                right: false,
                ..child_input(step)
            }
        }
    };
    let events = play(&mut game, 120, back_and_forth);
    assert!(events.hearts > 10, "coletou {}", events.hearts);
    // Os coletados reapareceram em outro lugar: o total se mantém.
    let now = game.hearts.len() + game.hearts.pending();
    assert_eq!(now, at_start);
}

#[test]
fn pausa_congela_tudo_e_enter_retoma() {
    let mut game = new_game(5);
    play(&mut game, 5, child_input);
    let enter = Input {
        pause: Button {
            pressed: true,
            held: true,
        },
        ..Input::default()
    };
    assert_eq!(game.step(enter, STEP).mode_changed, Some(Mode::Paused));
    let frozen = (game.player.pos, game.hearts.len(), game.counter, game.time);
    play(&mut game, 10, child_input);
    assert_eq!(
        frozen,
        (game.player.pos, game.hearts.len(), game.counter, game.time)
    );
    assert_eq!(game.step(enter, STEP).mode_changed, Some(Mode::Playing));
    play(&mut game, 1, child_input);
    assert!(game.player.pos.x > frozen.0.x);
}

#[test]
fn coracoes_mudam_a_cada_partida_mas_o_nivel_nao() {
    let (a, b) = (new_game(1), new_game(2));
    assert_eq!(a.level.terrain.points(), b.level.terrain.points());
    let first = |g: &Game| {
        g.hearts
            .in_span(0.0, 3000.0)
            .iter()
            .map(|h| h.pos)
            .collect::<Vec<_>>()
    };
    assert_ne!(first(&a), first(&b));
}

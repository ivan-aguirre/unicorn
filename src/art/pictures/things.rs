//! Coisas: bola, casa, lua, pipa, dado, copo, sino, janela, sapato e as outras.

use super::{
    BLUE, BROWN, CREAM, GOLD, GRAY, GREEN, INK, ORANGE, PINK, RED, SHADE, WHITE, YELLOW, cheek,
    eye, shine,
};
use crate::art::vector::{Layer, Shape, circle, ellipse, flat, line, poly, rect, solid};

const BALL_BODY: Shape = circle(50.0, 52.0, 40.0);
/// Gomos escuros da bola de futebol: o do meio e os cinco da borda (cortados pela bola).
const BALL_PATCHES: [Shape; 6] = [
    poly(&[
        (50.0, 40.0),
        (61.4, 48.3),
        (57.1, 61.7),
        (42.9, 61.7),
        (38.6, 48.3),
    ]),
    poly(&[
        (50.0, 25.0),
        (37.6, 16.0),
        (42.4, 1.5),
        (57.6, 1.5),
        (62.4, 16.0),
    ]),
    poly(&[
        (75.7, 43.7),
        (80.4, 29.1),
        (95.7, 29.1),
        (100.4, 43.7),
        (88.0, 52.6),
    ]),
    poly(&[
        (65.9, 73.8),
        (81.2, 73.8),
        (85.9, 88.4),
        (73.5, 97.4),
        (61.1, 88.4),
    ]),
    poly(&[
        (34.1, 73.8),
        (38.9, 88.4),
        (26.5, 97.4),
        (14.1, 88.4),
        (18.8, 73.8),
    ]),
    poly(&[
        (24.3, 43.7),
        (12.0, 52.6),
        (-0.4, 43.7),
        (4.3, 29.1),
        (19.6, 29.1),
    ]),
];
/// Costuras: do gomo do meio aos da borda e entre os da borda (formando os hexágonos).
const BALL_SEAMS: [Shape; 10] = [
    line(&[(50.0, 40.0), (50.0, 25.0)], 2.0),
    line(&[(61.4, 48.3), (75.7, 43.7)], 2.0),
    line(&[(57.1, 61.7), (65.9, 73.8)], 2.0),
    line(&[(42.9, 61.7), (34.1, 73.8)], 2.0),
    line(&[(38.6, 48.3), (24.3, 43.7)], 2.0),
    line(&[(62.4, 16.0), (80.4, 29.1)], 2.0),
    line(&[(88.0, 52.6), (81.2, 73.8)], 2.0),
    line(&[(61.1, 88.4), (38.9, 88.4)], 2.0),
    line(&[(18.8, 73.8), (12.0, 52.6)], 2.0),
    line(&[(19.6, 29.1), (37.6, 16.0)], 2.0),
];
/// Traço escuro (gomo ou costura) cortado pela bola.
const fn on_ball(shape: Shape) -> Layer {
    flat(shape, INK).inside(BALL_BODY)
}
/// Bola de futebol: branca com gomos escuros, sombra embaixo e brilho em cima.
pub const BALL: &[Layer] = &[
    solid(BALL_BODY, WHITE),
    on_ball(BALL_SEAMS[0]),
    on_ball(BALL_SEAMS[1]),
    on_ball(BALL_SEAMS[2]),
    on_ball(BALL_SEAMS[3]),
    on_ball(BALL_SEAMS[4]),
    on_ball(BALL_SEAMS[5]),
    on_ball(BALL_SEAMS[6]),
    on_ball(BALL_SEAMS[7]),
    on_ball(BALL_SEAMS[8]),
    on_ball(BALL_SEAMS[9]),
    on_ball(BALL_PATCHES[0]),
    on_ball(BALL_PATCHES[1]),
    on_ball(BALL_PATCHES[2]),
    on_ball(BALL_PATCHES[3]),
    on_ball(BALL_PATCHES[4]),
    on_ball(BALL_PATCHES[5]),
    flat(BALL_BODY, SHADE)
        .minus(circle(44.0, 45.0, 40.0))
        .faded(0.15),
    shine(34.0, 30.0, 8.0, 4.5),
];

/// Casinha com telhado vermelho, porta e janelas.
pub const HOUSE: &[Layer] = &[
    solid(rect(71.0, 24.0, 11.0, 22.0, 1.5), BROWN),
    solid(rect(50.0, 66.0, 62.0, 50.0, 2.0), CREAM),
    solid(poly(&[(8.0, 48.0), (50.0, 10.0), (92.0, 48.0)]), RED),
    flat(poly(&[(8.0, 48.0), (50.0, 10.0), (92.0, 48.0)]), SHADE)
        .minus(poly(&[(4.0, 44.0), (50.0, 2.0), (96.0, 44.0)]))
        .faded(0.18),
    solid(rect(50.0, 76.0, 17.0, 30.0, 4.0), BROWN),
    flat(circle(54.5, 78.0, 1.8), YELLOW),
    solid(rect(30.0, 62.0, 15.0, 15.0, 2.0), BLUE),
    solid(rect(70.0, 62.0, 15.0, 15.0, 2.0), BLUE),
    flat(line(&[(30.0, 55.0), (30.0, 69.0)], 1.8), INK),
    flat(line(&[(23.0, 62.0), (37.0, 62.0)], 1.8), INK),
    flat(line(&[(70.0, 55.0), (70.0, 69.0)], 1.8), INK),
    flat(line(&[(63.0, 62.0), (77.0, 62.0)], 1.8), INK),
    shine(26.5, 58.5, 2.5, 1.5),
    shine(66.5, 58.5, 2.5, 1.5),
];

/// Lua crescente dormindo, com estrelinhas.
pub const MOON: &[Layer] = &[
    solid(circle(46.0, 52.0, 38.0), YELLOW).minus(circle(66.0, 40.0, 32.0)),
    flat(circle(46.0, 52.0, 38.0), SHADE)
        .minus(circle(54.0, 44.0, 38.0))
        .faded(0.15),
    flat(line(&[(22.0, 52.0), (26.0, 55.5), (30.0, 52.0)], 2.4), INK),
    cheek(30.0, 62.0, 4.5),
    solid(
        poly(&[
            (72.0, 48.0),
            (74.6, 54.4),
            (81.5, 54.9),
            (76.3, 59.4),
            (77.9, 66.1),
            (72.0, 62.5),
            (66.1, 66.1),
            (67.7, 59.4),
            (62.5, 54.9),
            (69.4, 54.4),
        ]),
        YELLOW,
    ),
    solid(
        poly(&[
            (84.0, 15.0),
            (85.9, 19.4),
            (90.7, 19.8),
            (87.0, 23.0),
            (88.1, 27.7),
            (84.0, 25.2),
            (79.9, 27.7),
            (81.0, 23.0),
            (77.3, 19.8),
            (82.1, 19.4),
        ]),
        YELLOW,
    ),
];

const KITE_BODY: Shape = poly(&[(50.0, 6.0), (80.0, 38.0), (50.0, 72.0), (20.0, 38.0)]);
/// Pipa em losango com rabiola de lacinhos.
pub const KITE: &[Layer] = &[
    flat(
        line(
            &[(50.0, 72.0), (45.0, 80.0), (54.0, 87.0), (47.0, 95.0)],
            2.0,
        ),
        INK,
    ),
    solid(
        poly(&[(38.0, 76.0), (52.0, 84.0), (52.0, 76.0), (38.0, 84.0)]),
        RED,
    ),
    solid(
        poly(&[(44.0, 87.0), (58.0, 95.0), (58.0, 87.0), (44.0, 95.0)]),
        YELLOW,
    ),
    solid(KITE_BODY, BLUE),
    flat(poly(&[(50.0, 6.0), (50.0, 38.0), (20.0, 38.0)]), PINK).inside(KITE_BODY),
    flat(poly(&[(50.0, 38.0), (80.0, 38.0), (50.0, 72.0)]), PINK).inside(KITE_BODY),
    flat(line(&[(50.0, 8.0), (50.0, 70.0)], 1.8), INK),
    flat(line(&[(22.0, 38.0), (78.0, 38.0)], 1.8), INK),
    shine(40.0, 24.0, 4.0, 2.5),
];

const DIE_BODY: Shape = rect(50.0, 52.0, 68.0, 68.0, 13.0).turned(-10.0);
/// Dado branco mostrando o cinco.
pub const DIE: &[Layer] = &[
    solid(DIE_BODY, WHITE),
    flat(DIE_BODY, 0xb9_b4_c9)
        .minus(rect(47.0, 49.0, 68.0, 68.0, 13.0).turned(-10.0))
        .faded(0.8),
    flat(circle(30.3, 38.2, 6.5), RED),
    flat(circle(63.8, 32.3, 6.5), RED),
    flat(circle(50.0, 52.0, 6.5), RED),
    flat(circle(36.2, 71.7, 6.5), RED),
    flat(circle(69.7, 65.8, 6.5), RED),
    shine(28.0, 27.0, 5.0, 2.5),
];

const GLASS: Shape = poly(&[(26.0, 20.0), (74.0, 20.0), (67.0, 90.0), (33.0, 90.0)]);
/// Copo de suco de laranja com canudinho.
pub const CUP: &[Layer] = &[
    solid(line(&[(57.0, 60.0), (69.0, 8.0)], 5.0), PINK),
    solid(GLASS, 0xe4_f5_ff).faded(0.85),
    flat(rect(50.0, 68.0, 60.0, 46.0, 0.0), ORANGE).inside(GLASS),
    flat(rect(50.0, 46.5, 60.0, 3.0, 0.0), 0xff_c2_7a).inside(GLASS),
    flat(circle(43.0, 62.0, 2.2), WHITE).faded(0.6),
    flat(circle(55.0, 74.0, 1.8), WHITE).faded(0.6),
    flat(line(&[(34.0, 27.0), (38.5, 82.0)], 3.0), WHITE).faded(0.75),
];

const BELL_BODY: Shape = poly(&[
    (50.0, 19.0),
    (53.9, 19.5),
    (57.5, 21.0),
    (60.6, 23.4),
    (63.0, 26.5),
    (64.5, 30.1),
    (65.0, 34.0),
    (65.1, 38.2),
    (65.6, 42.4),
    (66.5, 46.6),
    (67.8, 50.8),
    (69.6, 55.0),
    (71.8, 59.2),
    (74.6, 63.4),
    (77.9, 67.6),
    (81.7, 71.8),
    (86.0, 76.0),
    (14.0, 76.0),
    (18.3, 71.8),
    (22.1, 67.6),
    (25.4, 63.4),
    (28.2, 59.2),
    (30.4, 55.0),
    (32.2, 50.8),
    (33.5, 46.6),
    (34.4, 42.4),
    (34.9, 38.2),
    (35.0, 34.0),
    (35.5, 30.1),
    (37.0, 26.5),
    (39.4, 23.4),
    (42.5, 21.0),
    (46.1, 19.5),
]);
/// Sino dourado com laço vermelho.
pub const BELL: &[Layer] = &[
    solid(circle(50.0, 84.0, 6.0), BROWN),
    solid(BELL_BODY, GOLD),
    flat(ellipse(74.0, 56.0, 14.0, 34.0), 0xe8_a9_2a).inside(BELL_BODY),
    solid(rect(50.0, 77.0, 78.0, 9.0, 4.5), 0xf0_b0_30),
    flat(ellipse(41.0, 42.0, 3.5, 10.0).turned(10.0), WHITE).faded(0.8),
    solid(poly(&[(50.0, 16.0), (35.0, 8.0), (35.0, 24.0)]), RED),
    solid(poly(&[(50.0, 16.0), (65.0, 8.0), (65.0, 24.0)]), RED),
    solid(circle(50.0, 16.0, 4.5), RED),
];

const PANE: Shape = rect(50.0, 50.0, 50.0, 58.0, 2.0);
/// Janela aberta para o céu, com cortinas cor-de-rosa.
pub const WINDOW: &[Layer] = &[
    solid(rect(50.0, 52.0, 62.0, 70.0, 4.0), CREAM),
    solid(PANE, 0x9e_d8_ff),
    flat(circle(64.0, 32.0, 6.0), YELLOW).inside(PANE),
    flat(circle(38.0, 64.0, 5.5), WHITE).inside(PANE),
    flat(circle(45.0, 61.0, 7.0), WHITE).inside(PANE),
    flat(circle(52.0, 64.0, 5.0), WHITE).inside(PANE),
    solid(rect(50.0, 50.0, 5.0, 58.0, 0.0), CREAM),
    solid(rect(50.0, 50.0, 50.0, 5.0, 0.0), CREAM),
    solid(
        poly(&[
            (10.0, 14.0),
            (29.0, 14.0),
            (25.0, 34.0),
            (21.0, 50.0),
            (24.0, 64.0),
            (19.0, 86.0),
            (10.0, 86.0),
        ]),
        PINK,
    ),
    solid(
        poly(&[
            (90.0, 14.0),
            (71.0, 14.0),
            (75.0, 34.0),
            (79.0, 50.0),
            (76.0, 64.0),
            (81.0, 86.0),
            (90.0, 86.0),
        ]),
        PINK,
    ),
    flat(line(&[(15.0, 17.0), (14.5, 83.0)], 1.6), 0xe0_70_9a),
    flat(line(&[(85.0, 17.0), (85.5, 83.0)], 1.6), 0xe0_70_9a),
    solid(ellipse(21.0, 51.0, 5.0, 3.0), 0xff_5f_8f),
    solid(ellipse(79.0, 51.0, 5.0, 3.0), 0xff_5f_8f),
    solid(line(&[(6.0, 13.0), (94.0, 13.0)], 4.0), BROWN),
    solid(rect(50.0, 89.0, 72.0, 7.0, 3.0), BROWN),
];

const SHOE_UPPER: Shape = poly(&[
    (14.0, 74.0),
    (14.0, 46.0),
    (20.0, 36.0),
    (36.0, 34.0),
    (44.0, 46.0),
    (58.0, 50.0),
    (76.0, 54.0),
    (88.0, 62.0),
    (91.0, 74.0),
]);
/// Tênis vermelho com cadarço branco.
pub const SHOE: &[Layer] = &[
    solid(SHOE_UPPER, RED),
    flat(ellipse(82.0, 68.0, 14.0, 10.0), 0xff_8a_8d).inside(SHOE_UPPER),
    flat(ellipse(28.0, 36.5, 8.0, 2.8), INK),
    flat(line(&[(43.0, 51.0), (50.0, 44.0)], 2.4), WHITE),
    flat(line(&[(50.0, 53.0), (57.0, 47.0)], 2.4), WHITE),
    flat(line(&[(57.0, 55.0), (64.0, 50.0)], 2.4), WHITE),
    flat(
        line(&[(20.0, 64.0), (40.0, 60.0), (62.0, 64.0)], 3.0),
        WHITE,
    )
    .inside(SHOE_UPPER),
    solid(rect(52.0, 77.0, 82.0, 9.0, 4.5), WHITE),
    flat(line(&[(17.0, 77.0), (87.0, 77.0)], 1.5), 0xd8_cc_e6),
];

const SUITCASE_BODY: Shape = rect(50.0, 60.0, 72.0, 50.0, 8.0);
/// Mala de viagem com alça e cintas.
pub const SUITCASE: &[Layer] = &[
    solid(rect(50.0, 30.0, 26.0, 14.0, 5.0), BROWN).minus(rect(50.0, 33.0, 15.0, 10.0, 3.0)),
    solid(SUITCASE_BODY, 0xe0_8a_4a),
    flat(rect(32.0, 60.0, 7.0, 50.0, 0.0), 0xb8_6a_34).inside(SUITCASE_BODY),
    flat(rect(68.0, 60.0, 7.0, 50.0, 0.0), 0xb8_6a_34).inside(SUITCASE_BODY),
    solid(rect(32.0, 44.0, 9.0, 6.0, 1.5), GOLD),
    solid(rect(68.0, 44.0, 9.0, 6.0, 1.5), GOLD),
    flat(circle(50.0, 66.0, 6.0), YELLOW),
    flat(circle(80.0, 74.0, 4.0), BLUE),
    shine(22.0, 44.0, 3.0, 2.0),
];

/// Mesa de madeira com uma maçã em cima.
pub const TABLE: &[Layer] = &[
    solid(rect(20.0, 72.0, 7.0, 40.0, 2.0), BROWN),
    solid(rect(80.0, 72.0, 7.0, 40.0, 2.0), BROWN),
    solid(rect(50.0, 56.0, 72.0, 8.0, 2.0), 0x7d_52_33),
    solid(rect(50.0, 48.0, 88.0, 10.0, 3.0), 0xb9_80_52),
    solid(line(&[(56.0, 26.0), (58.0, 19.0)], 2.5), BROWN),
    solid(circle(56.0, 33.0, 9.0), RED),
    solid(ellipse(64.0, 21.0, 5.0, 2.5).turned(-20.0), GREEN),
    shine(52.0, 30.0, 2.5, 1.6),
];

/// Cama com travesseiro e cobertor.
pub const BED: &[Layer] = &[
    solid(rect(14.0, 60.0, 10.0, 56.0, 4.0), BROWN),
    solid(rect(86.0, 72.0, 8.0, 32.0, 3.0), BROWN),
    solid(rect(51.0, 72.0, 70.0, 14.0, 4.0), WHITE),
    solid(ellipse(29.0, 58.0, 11.0, 6.5), WHITE),
    solid(rect(58.0, 66.0, 54.0, 20.0, 5.0), BLUE),
    flat(line(&[(36.0, 62.0), (82.0, 62.0)], 2.0), 0x9e_d8_ff),
    solid(rect(18.0, 92.0, 5.0, 8.0, 1.5), BROWN),
    solid(rect(84.0, 92.0, 5.0, 8.0, 1.5), BROWN),
];

/// Vela acesa num pires dourado.
pub const CANDLE: &[Layer] = &[
    flat(circle(50.0, 24.0, 18.0), YELLOW).faded(0.25),
    solid(ellipse(50.0, 88.0, 28.0, 6.0), GOLD),
    solid(rect(50.0, 62.0, 24.0, 50.0, 4.0), 0xff_d6_e6),
    flat(ellipse(44.0, 40.0, 4.0, 6.0), 0xff_ee_f4),
    flat(line(&[(44.0, 48.0), (44.0, 80.0)], 3.0), WHITE).faded(0.7),
    solid(line(&[(50.0, 37.0), (50.0, 31.0)], 2.0), INK),
    solid(
        poly(&[
            (50.0, 6.0),
            (57.0, 17.0),
            (56.0, 25.0),
            (50.0, 30.0),
            (44.0, 25.0),
            (43.0, 17.0),
        ]),
        ORANGE,
    ),
    flat(ellipse(50.0, 22.0, 3.5, 5.0), YELLOW),
];

const BOOT_SHAPE: Shape = poly(&[
    (30.0, 14.0),
    (58.0, 14.0),
    (58.0, 60.0),
    (82.0, 64.0),
    (90.0, 74.0),
    (90.0, 86.0),
    (30.0, 86.0),
]);
/// Bota de chuva amarela.
pub const BOOT: &[Layer] = &[
    solid(BOOT_SHAPE, YELLOW),
    flat(rect(50.0, 80.0, 70.0, 12.0, 0.0), 0xf0_b0_30).inside(BOOT_SHAPE),
    solid(rect(44.0, 15.0, 34.0, 9.0, 3.0), 0xf0_b0_30),
    solid(rect(60.0, 89.0, 66.0, 7.0, 3.0), BROWN),
    flat(line(&[(37.0, 26.0), (37.0, 66.0)], 3.0), WHITE).faded(0.75),
];

const HULL: Shape = poly(&[(8.0, 60.0), (92.0, 60.0), (80.0, 82.0), (20.0, 82.0)]);
/// Navio com cabine, chaminé e ondas.
pub const SHIP: &[Layer] = &[
    flat(circle(72.0, 14.0, 5.0), GRAY).faded(0.8),
    flat(circle(80.0, 9.0, 4.0), GRAY).faded(0.6),
    solid(rect(66.0, 34.0, 11.0, 20.0, 2.0), RED),
    flat(rect(66.0, 27.0, 11.0, 5.0, 0.0), INK).inside(rect(66.0, 34.0, 11.0, 20.0, 2.0)),
    solid(rect(46.0, 50.0, 44.0, 20.0, 3.0), WHITE),
    flat(circle(33.0, 49.0, 3.5), BLUE),
    flat(circle(46.0, 49.0, 3.5), BLUE),
    flat(circle(59.0, 49.0, 3.5), BLUE),
    solid(HULL, RED),
    flat(rect(50.0, 66.0, 100.0, 4.0, 0.0), WHITE).inside(HULL),
    solid(
        line(
            &[
                (6.0, 88.0),
                (16.0, 85.0),
                (28.0, 89.0),
                (40.0, 85.0),
                (52.0, 89.0),
                (64.0, 85.0),
                (76.0, 89.0),
                (88.0, 85.0),
                (94.0, 87.0),
            ],
            3.0,
        ),
        BLUE,
    ),
];

/// Lupa com cabo de madeira.
pub const MAGNIFIER: &[Layer] = &[
    solid(rect(74.0, 74.0, 11.0, 36.0, 5.0).turned(-45.0), BROWN),
    flat(circle(42.0, 42.0, 23.0), 0xcf_ee_ff).faded(0.8),
    flat(
        line(&[(30.0, 36.0), (34.0, 28.0), (41.0, 24.0)], 3.0),
        WHITE,
    ),
    solid(circle(42.0, 42.0, 30.0), GRAY).minus(circle(42.0, 42.0, 23.0)),
];

const TEAPOT_BODY: Shape = ellipse(48.0, 62.0, 30.0, 23.0);
/// Bule azul de bolinhas.
pub const TEAPOT: &[Layer] = &[
    solid(ellipse(20.0, 60.0, 12.0, 14.0), BLUE).minus(ellipse(20.0, 60.0, 6.0, 8.0)),
    solid(
        poly(&[(70.0, 64.0), (90.0, 40.0), (96.0, 44.0), (80.0, 74.0)]),
        BLUE,
    ),
    solid(rect(48.0, 86.0, 32.0, 6.0, 3.0), 0x2f_8f_d8),
    solid(TEAPOT_BODY, BLUE),
    flat(circle(36.0, 58.0, 3.0), WHITE).inside(TEAPOT_BODY),
    flat(circle(52.0, 54.0, 3.0), WHITE).inside(TEAPOT_BODY),
    flat(circle(62.0, 68.0, 3.0), WHITE).inside(TEAPOT_BODY),
    flat(circle(44.0, 72.0, 3.0), WHITE).inside(TEAPOT_BODY),
    solid(ellipse(48.0, 40.0, 18.0, 5.0), 0x2f_8f_d8),
    solid(circle(48.0, 32.0, 4.5), 0x2f_8f_d8),
];

const DOLL_HAIR: u32 = 0xb5_65_3a;
/// Boneca de maria-chiquinha e vestido rosa.
pub const DOLL: &[Layer] = &[
    solid(
        poly(&[(36.0, 62.0), (64.0, 62.0), (80.0, 95.0), (20.0, 95.0)]),
        PINK,
    ),
    flat(line(&[(28.0, 88.0), (72.0, 88.0)], 2.0), WHITE),
    solid(circle(19.0, 44.0, 9.0), DOLL_HAIR),
    solid(circle(81.0, 44.0, 9.0), DOLL_HAIR),
    solid(circle(24.0, 33.0, 4.5), RED),
    solid(circle(76.0, 33.0, 4.5), RED),
    solid(circle(50.0, 38.0, 26.0), DOLL_HAIR),
    solid(circle(50.0, 45.0, 20.0), 0xff_d9_c2),
    flat(ellipse(50.0, 29.0, 21.0, 9.0), DOLL_HAIR),
    eye(42.0, 46.0, 3.4)[0],
    eye(42.0, 46.0, 3.4)[1],
    eye(58.0, 46.0, 3.4)[0],
    eye(58.0, 46.0, 3.4)[1],
    flat(line(&[(45.0, 55.0), (50.0, 58.0), (55.0, 55.0)], 2.2), INK),
    cheek(37.0, 53.0, 3.5),
    cheek(63.0, 53.0, 3.5),
];

/// Fogueira: chamas sobre dois troncos cruzados.
pub const FIRE: &[Layer] = &[
    solid(rect(50.0, 82.0, 60.0, 9.0, 4.5).turned(12.0), BROWN),
    solid(rect(50.0, 82.0, 60.0, 9.0, 4.5).turned(-12.0), 0x7d_52_33),
    solid(
        poly(&[
            (50.0, 8.0),
            (60.0, 28.0),
            (70.0, 20.0),
            (72.0, 44.0),
            (80.0, 40.0),
            (82.0, 60.0),
            (76.0, 74.0),
            (62.0, 82.0),
            (38.0, 82.0),
            (24.0, 74.0),
            (18.0, 60.0),
            (22.0, 42.0),
            (30.0, 48.0),
            (32.0, 26.0),
            (42.0, 34.0),
        ]),
        0xff_6a_3d,
    ),
    flat(
        poly(&[
            (50.0, 34.0),
            (58.0, 50.0),
            (64.0, 46.0),
            (66.0, 62.0),
            (60.0, 76.0),
            (40.0, 76.0),
            (34.0, 62.0),
            (38.0, 50.0),
            (44.0, 54.0),
        ]),
        YELLOW,
    ),
];

const MUG_BODY: Shape = rect(44.0, 58.0, 48.0, 52.0, 6.0);
/// Caneca com um coração, soltando fumacinha.
pub const MUG: &[Layer] = &[
    solid(rect(70.0, 58.0, 24.0, 30.0, 11.0), RED).minus(rect(70.0, 58.0, 11.0, 16.0, 5.0)),
    solid(MUG_BODY, RED),
    flat(circle(39.0, 55.0, 6.0), WHITE),
    flat(circle(49.0, 55.0, 6.0), WHITE),
    flat(poly(&[(33.5, 57.5), (54.5, 57.5), (44.0, 70.0)]), WHITE),
    flat(
        line(&[(36.0, 26.0), (33.0, 18.0), (37.0, 10.0)], 2.4),
        0xc9_c2_db,
    ),
    flat(
        line(&[(50.0, 26.0), (47.0, 18.0), (51.0, 10.0)], 2.4),
        0xc9_c2_db,
    ),
    flat(line(&[(26.0, 40.0), (26.0, 76.0)], 3.0), WHITE).faded(0.5),
];

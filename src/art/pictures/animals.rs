//! Bichos: pato, gato, sapo, foca, rato e os outros.

use super::{
    BLUE, BROWN, CREAM, GRAY, GREEN, INK, ORANGE, PINK, RED, SHADE, SPOT, WHITE, YELLOW, cheek,
    eye, shine,
};
use crate::art::vector::{Layer, Shape, circle, ellipse, flat, line, poly, rect, solid};

const DUCK_BODY: Shape = ellipse(46.0, 66.0, 36.0, 22.0);
/// Patinho amarelo de lado, olhando para a direita.
pub const DUCK: &[Layer] = &[
    solid(poly(&[(16.0, 62.0), (6.0, 38.0), (30.0, 52.0)]), YELLOW),
    solid(DUCK_BODY, YELLOW),
    flat(DUCK_BODY, SHADE)
        .minus(ellipse(46.0, 60.0, 36.0, 22.0))
        .faded(0.15),
    solid(ellipse(85.0, 41.0, 10.0, 6.0), ORANGE),
    solid(circle(64.0, 36.0, 19.0), YELLOW),
    solid(ellipse(42.0, 63.0, 17.0, 10.0).turned(-12.0), 0xf5_c0_2c),
    eye(68.0, 31.0, 4.2)[0],
    eye(68.0, 31.0, 4.2)[1],
    cheek(73.0, 43.0, 4.0),
    shine(56.0, 26.0, 5.0, 3.0),
];

/// Carinha de gato laranja, de frente.
pub const CAT: &[Layer] = &[
    solid(poly(&[(18.0, 46.0), (22.0, 10.0), (46.0, 32.0)]), ORANGE),
    solid(poly(&[(82.0, 46.0), (78.0, 10.0), (54.0, 32.0)]), ORANGE),
    flat(poly(&[(25.0, 38.0), (26.0, 19.0), (38.0, 31.0)]), PINK),
    flat(poly(&[(75.0, 38.0), (74.0, 19.0), (62.0, 31.0)]), PINK),
    solid(ellipse(50.0, 58.0, 38.0, 30.0), ORANGE),
    flat(line(&[(50.0, 31.0), (50.0, 39.0)], 3.4), 0xe0_7b_2a),
    flat(line(&[(41.0, 32.5), (42.0, 39.0)], 3.4), 0xe0_7b_2a),
    flat(line(&[(59.0, 32.5), (58.0, 39.0)], 3.4), 0xe0_7b_2a),
    flat(ellipse(50.0, 72.0, 15.0, 10.0), CREAM),
    flat(ellipse(37.0, 55.0, 4.2, 6.0), INK),
    flat(circle(38.5, 52.5, 1.7), WHITE),
    flat(ellipse(63.0, 55.0, 4.2, 6.0), INK),
    flat(circle(64.5, 52.5, 1.7), WHITE),
    flat(
        poly(&[(46.0, 65.0), (54.0, 65.0), (50.0, 69.5)]),
        0xff_6f_a5,
    ),
    flat(
        line(
            &[
                (43.0, 71.0),
                (46.5, 74.0),
                (50.0, 70.5),
                (53.5, 74.0),
                (57.0, 71.0),
            ],
            2.0,
        ),
        INK,
    ),
    flat(line(&[(8.0, 62.0), (28.0, 65.0)], 1.8), INK),
    flat(line(&[(8.0, 72.0), (28.0, 70.0)], 1.8), INK),
    flat(line(&[(92.0, 62.0), (72.0, 65.0)], 1.8), INK),
    flat(line(&[(92.0, 72.0), (72.0, 70.0)], 1.8), INK),
    cheek(27.0, 66.0, 5.0),
    cheek(73.0, 66.0, 5.0),
];

const FROG_BODY: Shape = ellipse(50.0, 64.0, 38.0, 26.0);
/// Sapinho verde sorrindo, de frente.
pub const FROG: &[Layer] = &[
    solid(ellipse(28.0, 88.0, 11.0, 5.0), GREEN),
    solid(ellipse(72.0, 88.0, 11.0, 5.0), GREEN),
    solid(FROG_BODY, GREEN),
    flat(ellipse(50.0, 76.0, 22.0, 11.0), 0xc8_f0_a0).inside(FROG_BODY),
    solid(circle(31.0, 38.0, 14.0), GREEN),
    solid(circle(69.0, 38.0, 14.0), GREEN),
    solid(circle(31.0, 37.0, 8.5), WHITE),
    solid(circle(69.0, 37.0, 8.5), WHITE),
    eye(32.0, 38.0, 4.5)[0],
    eye(32.0, 38.0, 4.5)[1],
    eye(68.0, 38.0, 4.5)[0],
    eye(68.0, 38.0, 4.5)[1],
    flat(
        line(
            &[
                (32.0, 60.0),
                (41.0, 66.0),
                (50.0, 68.0),
                (59.0, 66.0),
                (68.0, 60.0),
            ],
            2.6,
        ),
        INK,
    ),
    cheek(24.0, 60.0, 5.0),
    cheek(76.0, 60.0, 5.0),
];

const SEAL_COLOR: u32 = 0x8f_a3_c0;
const SEAL_BODY: Shape = ellipse(42.0, 70.0, 30.0, 19.0).turned(-28.0);
const SEAL_HEAD: Shape = circle(64.0, 42.0, 15.0);
/// Foca equilibrando uma bola no focinho.
pub const SEAL: &[Layer] = &[
    solid(poly(&[(22.0, 80.0), (6.0, 72.0), (8.0, 92.0)]), 0x7a_8f_ae),
    solid(SEAL_BODY, SEAL_COLOR),
    solid(SEAL_HEAD, SEAL_COLOR),
    flat(SEAL_BODY, SEAL_COLOR),
    flat(SEAL_HEAD, SEAL_COLOR),
    flat(ellipse(50.0, 74.0, 17.0, 10.0).turned(-28.0), 0xc9_d6_ea).inside(SEAL_BODY),
    solid(ellipse(58.0, 82.0, 10.0, 4.5).turned(35.0), 0x7a_8f_ae),
    flat(ellipse(74.0, 46.0, 7.5, 5.5), 0xdd_e6_f3),
    flat(ellipse(79.5, 43.0, 3.2, 2.4), INK),
    eye(63.0, 37.0, 3.3)[0],
    eye(63.0, 37.0, 3.3)[1],
    flat(line(&[(74.0, 48.0), (86.0, 50.0)], 1.2), INK),
    flat(line(&[(73.0, 50.0), (84.0, 55.0)], 1.2), INK),
    cheek(70.0, 50.0, 3.0),
    solid(circle(84.0, 30.0, 8.0), RED),
    flat(line(&[(76.5, 30.0), (91.5, 30.0)], 2.4), WHITE).inside(circle(84.0, 30.0, 8.0)),
    shine(81.0, 26.5, 2.5, 1.5),
];

/// Carinha de ratinho cinza, de frente, com orelhas redondas.
pub const MOUSE: &[Layer] = &[
    solid(circle(26.0, 32.0, 16.0), GRAY),
    solid(circle(74.0, 32.0, 16.0), GRAY),
    flat(circle(26.0, 32.0, 10.0), PINK),
    flat(circle(74.0, 32.0, 10.0), PINK),
    solid(ellipse(50.0, 60.0, 31.0, 26.0), GRAY),
    eye(39.0, 56.0, 4.2)[0],
    eye(39.0, 56.0, 4.2)[1],
    eye(61.0, 56.0, 4.2)[0],
    eye(61.0, 56.0, 4.2)[1],
    solid(rect(50.0, 79.0, 7.0, 6.0, 1.5), WHITE),
    flat(
        line(
            &[
                (44.0, 74.0),
                (47.0, 77.0),
                (50.0, 74.0),
                (53.0, 77.0),
                (56.0, 74.0),
            ],
            2.0,
        ),
        INK,
    ),
    solid(circle(50.0, 68.0, 4.5), 0xff_6f_a5),
    flat(line(&[(33.0, 68.0), (12.0, 64.0)], 1.6), INK),
    flat(line(&[(33.0, 72.0), (12.0, 76.0)], 1.6), INK),
    flat(line(&[(67.0, 68.0), (88.0, 64.0)], 1.6), INK),
    flat(line(&[(67.0, 72.0), (88.0, 76.0)], 1.6), INK),
    cheek(30.0, 66.0, 4.5),
    cheek(70.0, 66.0, 4.5),
];

const COW_HEAD: Shape = ellipse(50.0, 50.0, 27.0, 30.0);
/// Carinha de vaca malhada, de frente.
pub const COW: &[Layer] = &[
    solid(ellipse(35.0, 18.0, 4.0, 8.0).turned(-20.0), CREAM),
    solid(ellipse(65.0, 18.0, 4.0, 8.0).turned(20.0), CREAM),
    solid(ellipse(16.0, 40.0, 12.0, 6.0).turned(20.0), WHITE),
    solid(ellipse(84.0, 40.0, 12.0, 6.0).turned(-20.0), WHITE),
    flat(ellipse(17.0, 40.5, 7.0, 3.0).turned(20.0), PINK),
    flat(ellipse(83.0, 40.5, 7.0, 3.0).turned(-20.0), PINK),
    solid(COW_HEAD, WHITE),
    flat(ellipse(34.0, 34.0, 11.0, 9.0).turned(20.0), SPOT).inside(COW_HEAD),
    flat(ellipse(70.0, 56.0, 8.0, 10.0), SPOT).inside(COW_HEAD),
    eye(39.0, 47.0, 4.0)[0],
    eye(39.0, 47.0, 4.0)[1],
    eye(61.0, 47.0, 4.0)[0],
    eye(61.0, 47.0, 4.0)[1],
    solid(ellipse(50.0, 71.0, 22.0, 14.0), 0xff_b8_c8),
    flat(ellipse(42.0, 70.0, 3.0, 4.0), INK),
    flat(ellipse(58.0, 70.0, 3.0, 4.0), INK),
];

const MONKEY_BROWN: u32 = 0x9b_6a_45;
const MONKEY_FACE: u32 = 0xf2_c7_9a;
const MONKEY_HEAD: Shape = circle(50.0, 50.0, 30.0);
/// Carinha de macaco, de frente.
pub const MONKEY: &[Layer] = &[
    solid(circle(19.0, 50.0, 11.0), MONKEY_BROWN),
    solid(circle(81.0, 50.0, 11.0), MONKEY_BROWN),
    flat(circle(19.0, 50.0, 6.0), MONKEY_FACE),
    flat(circle(81.0, 50.0, 6.0), MONKEY_FACE),
    solid(MONKEY_HEAD, MONKEY_BROWN),
    flat(circle(40.0, 46.0, 12.0), MONKEY_FACE),
    flat(circle(60.0, 46.0, 12.0), MONKEY_FACE),
    flat(ellipse(50.0, 64.0, 21.0, 15.0), MONKEY_FACE),
    eye(42.0, 46.0, 4.0)[0],
    eye(42.0, 46.0, 4.0)[1],
    eye(58.0, 46.0, 4.0)[0],
    eye(58.0, 46.0, 4.0)[1],
    flat(circle(47.0, 59.0, 1.6), INK),
    flat(circle(53.0, 59.0, 1.6), INK),
    flat(
        line(
            &[
                (40.0, 66.0),
                (45.0, 70.0),
                (50.0, 71.0),
                (55.0, 70.0),
                (60.0, 66.0),
            ],
            2.4,
        ),
        INK,
    ),
    cheek(33.0, 60.0, 4.0),
    cheek(67.0, 60.0, 4.0),
];

const ROOSTER_BODY: Shape = ellipse(44.0, 62.0, 26.0, 21.0);
const ROOSTER_HEAD: Shape = circle(66.0, 36.0, 13.0);
/// Galo de lado, com crista vermelha e rabo colorido.
pub const ROOSTER: &[Layer] = &[
    solid(ellipse(25.0, 38.0, 16.0, 6.5).turned(-65.0), 0xff_5a_5f),
    solid(ellipse(21.0, 50.0, 16.0, 6.5).turned(-35.0), 0xff_a1_3d),
    solid(ellipse(22.0, 63.0, 14.0, 6.0).turned(-5.0), GREEN),
    solid(
        line(&[(40.0, 80.0), (40.0, 90.0), (34.0, 92.0)], 3.0),
        ORANGE,
    ),
    solid(
        line(&[(52.0, 80.0), (52.0, 90.0), (58.0, 92.0)], 3.0),
        ORANGE,
    ),
    solid(circle(60.0, 23.0, 5.0), RED),
    solid(circle(66.0, 20.5, 5.5), RED),
    solid(circle(72.0, 24.0, 5.0), RED),
    solid(ROOSTER_BODY, WHITE),
    solid(ROOSTER_HEAD, WHITE),
    flat(ROOSTER_BODY, WHITE),
    flat(ROOSTER_HEAD, WHITE),
    solid(ellipse(42.0, 62.0, 14.0, 9.0).turned(-12.0), 0xf0_e8_dc),
    solid(poly(&[(77.0, 32.0), (90.0, 37.0), (77.0, 41.0)]), ORANGE),
    solid(ellipse(75.0, 47.0, 4.0, 6.0), RED),
    eye(68.0, 33.0, 3.0)[0],
    eye(68.0, 33.0, 3.0)[1],
];

const GIRAFFE_COLOR: u32 = 0xff_d2_7a;
const GIRAFFE_SPOT: u32 = 0xc9_85_3e;
const GIRAFFE_NECK: Shape = rect(44.0, 70.0, 20.0, 46.0, 4.0);
const GIRAFFE_HEAD: Shape = ellipse(56.0, 34.0, 21.0, 13.0).turned(18.0);
/// Cabeça e pescoço de girafa.
pub const GIRAFFE: &[Layer] = &[
    solid(line(&[(46.0, 26.0), (42.0, 13.0)], 4.0), GIRAFFE_SPOT),
    solid(line(&[(56.0, 24.0), (58.0, 11.0)], 4.0), GIRAFFE_SPOT),
    solid(circle(42.0, 12.0, 3.8), BROWN),
    solid(circle(58.0, 10.0, 3.8), BROWN),
    solid(ellipse(34.0, 28.0, 9.0, 4.0).turned(-25.0), GIRAFFE_COLOR),
    solid(line(&[(33.0, 48.0), (33.0, 91.0)], 5.0), BROWN),
    solid(GIRAFFE_NECK, GIRAFFE_COLOR),
    solid(GIRAFFE_HEAD, GIRAFFE_COLOR),
    flat(GIRAFFE_NECK, GIRAFFE_COLOR),
    flat(GIRAFFE_HEAD, GIRAFFE_COLOR),
    flat(ellipse(70.0, 41.0, 10.0, 8.0).turned(18.0), 0xff_e8_b8),
    flat(circle(41.0, 60.0, 5.0), GIRAFFE_SPOT).inside(GIRAFFE_NECK),
    flat(circle(51.0, 74.0, 4.5), GIRAFFE_SPOT).inside(GIRAFFE_NECK),
    flat(circle(40.0, 82.0, 4.0), GIRAFFE_SPOT).inside(GIRAFFE_NECK),
    flat(circle(50.0, 87.0, 3.5), GIRAFFE_SPOT).inside(GIRAFFE_NECK),
    flat(circle(46.0, 33.0, 3.5), GIRAFFE_SPOT).inside(GIRAFFE_HEAD),
    flat(circle(74.0, 39.0, 1.6), INK),
    eye(56.0, 31.0, 3.4)[0],
    eye(56.0, 31.0, 3.4)[1],
    cheek(62.0, 40.0, 3.0),
];

const TOUCAN_BODY: Shape = ellipse(38.0, 58.0, 19.0, 27.0);
/// Tucano preto de bico laranja num galho.
pub const TOUCAN: &[Layer] = &[
    solid(rect(50.0, 88.0, 84.0, 7.0, 3.5), BROWN),
    solid(rect(31.0, 86.0, 10.0, 18.0, 3.0).turned(15.0), 0x2f_2a_3d),
    solid(TOUCAN_BODY, 0x2f_2a_3d),
    flat(ellipse(46.0, 50.0, 10.0, 13.0), WHITE).inside(TOUCAN_BODY),
    flat(ellipse(47.0, 60.0, 8.0, 5.0), YELLOW).inside(TOUCAN_BODY),
    solid(
        poly(&[
            (50.0, 30.0),
            (78.0, 31.0),
            (94.0, 42.0),
            (84.0, 47.0),
            (50.0, 45.0),
        ]),
        ORANGE,
    ),
    flat(
        poly(&[(50.0, 30.0), (78.0, 31.0), (90.0, 39.0), (50.0, 36.0)]),
        YELLOW,
    ),
    flat(
        poly(&[(88.0, 40.0), (94.0, 42.0), (84.0, 47.0), (82.0, 45.0)]),
        0x2f_2a_3d,
    ),
    solid(circle(43.0, 34.0, 5.0), BLUE),
    flat(circle(43.5, 34.0, 2.6), INK),
    flat(circle(44.5, 33.0, 1.0), WHITE),
    solid(ellipse(32.0, 62.0, 9.0, 15.0).turned(10.0), 0x3d_37_52),
];

const WHALE_BODY: Shape = ellipse(46.0, 60.0, 36.0, 24.0);
/// Baleia azul soltando água pelo alto.
pub const WHALE: &[Layer] = &[
    solid(
        poly(&[
            (74.0, 58.0),
            (90.0, 34.0),
            (93.0, 42.0),
            (88.0, 54.0),
            (94.0, 62.0),
            (90.0, 70.0),
        ]),
        BLUE,
    ),
    solid(WHALE_BODY, BLUE),
    flat(ellipse(42.0, 74.0, 30.0, 10.0), 0xbf_e6_ff).inside(WHALE_BODY),
    flat(line(&[(42.0, 36.0), (42.0, 24.0)], 4.0), 0x6f_c4_ff),
    solid(circle(34.0, 18.0, 4.5), 0x9e_d8_ff),
    solid(circle(50.0, 17.0, 4.5), 0x9e_d8_ff),
    solid(circle(42.0, 12.0, 4.0), 0x9e_d8_ff),
    eye(28.0, 54.0, 3.6)[0],
    eye(28.0, 54.0, 3.6)[1],
    flat(line(&[(14.0, 63.0), (22.0, 67.0), (31.0, 66.0)], 2.2), INK),
    cheek(33.0, 62.0, 3.5),
    solid(ellipse(52.0, 70.0, 10.0, 4.0).turned(20.0), 0x2f_8f_d8),
];

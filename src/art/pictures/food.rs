//! Comidas: uva, bolo, banana, tomate, pera, pipoca e as outras.

use super::{
    BLUE, BROWN, GRAY, GREEN, GREEN_DARK, ORANGE, PINK, PURPLE, RED, SHADE, WHITE, YELLOW, shine,
};
use crate::art::vector::{Layer, Shape, circle, ellipse, flat, line, poly, rect, solid};

/// Uma uva com brilho.
const fn grape(x: f32, y: f32) -> [Layer; 2] {
    [
        solid(circle(x, y, 12.0), PURPLE),
        flat(circle(x - 4.5, y - 4.5, 3.2), WHITE).faded(0.75),
    ]
}

/// Cacho de uvas com cabinho e folha.
pub const GRAPES: &[Layer] = &[
    solid(ellipse(65.0, 17.0, 13.0, 6.5).turned(-20.0), GREEN),
    flat(line(&[(56.0, 19.0), (73.0, 14.0)], 1.6), GREEN_DARK),
    solid(line(&[(50.0, 30.0), (53.0, 12.0)], 4.0), BROWN),
    grape(30.0, 40.0)[0],
    grape(30.0, 40.0)[1],
    grape(50.0, 38.0)[0],
    grape(50.0, 38.0)[1],
    grape(70.0, 40.0)[0],
    grape(70.0, 40.0)[1],
    grape(40.0, 58.0)[0],
    grape(40.0, 58.0)[1],
    grape(60.0, 58.0)[0],
    grape(60.0, 58.0)[1],
    grape(50.0, 76.0)[0],
    grape(50.0, 76.0)[1],
];

/// Bolo de aniversário com cobertura escorrendo e uma vela.
pub const CAKE: &[Layer] = &[
    solid(ellipse(50.0, 88.0, 42.0, 6.0), 0xe8_e4_f4),
    solid(rect(50.0, 68.0, 64.0, 38.0, 5.0), 0xe8_a4_6a),
    flat(rect(50.0, 80.0, 64.0, 3.0, 0.0), 0xc9_7f_4a).inside(rect(50.0, 68.0, 64.0, 38.0, 5.0)),
    solid(
        poly(&[
            (16.0, 47.0),
            (16.2, 45.2),
            (16.9, 43.5),
            (18.1, 42.1),
            (19.5, 40.9),
            (21.2, 40.2),
            (23.0, 40.0),
            (77.0, 40.0),
            (78.8, 40.2),
            (80.5, 40.9),
            (81.9, 42.1),
            (83.1, 43.5),
            (83.8, 45.2),
            (84.0, 47.0),
            (84.0, 56.0),
            (81.0, 56.0),
            (79.5, 59.6),
            (78.0, 60.6),
            (76.5, 61.0),
            (75.0, 60.9),
            (73.5, 60.3),
            (72.0, 59.0),
            (70.5, 56.0),
            (69.0, 56.0),
            (67.5, 58.6),
            (66.0, 60.9),
            (64.5, 62.1),
            (63.0, 62.7),
            (61.5, 63.0),
            (60.0, 62.9),
            (58.5, 62.5),
            (57.0, 61.7),
            (55.5, 60.3),
            (54.0, 56.0),
            (49.5, 56.0),
            (48.0, 59.8),
            (46.5, 60.9),
            (45.0, 61.4),
            (43.5, 61.5),
            (42.0, 61.1),
            (40.5, 60.2),
            (39.0, 58.3),
            (37.5, 56.0),
            (34.5, 56.0),
            (33.0, 58.5),
            (31.5, 60.7),
            (30.0, 61.8),
            (28.5, 62.3),
            (27.0, 62.5),
            (25.5, 62.3),
            (24.0, 61.8),
            (22.5, 60.7),
            (21.0, 58.5),
            (19.5, 56.0),
            (16.0, 56.0),
        ]),
        PINK,
    ),
    flat(rect(28.0, 47.0, 5.0, 2.2, 1.1).turned(30.0), YELLOW),
    flat(rect(42.0, 50.0, 5.0, 2.2, 1.1).turned(-25.0), BLUE),
    flat(rect(60.0, 46.0, 5.0, 2.2, 1.1).turned(15.0), GREEN),
    flat(rect(73.0, 50.0, 5.0, 2.2, 1.1).turned(-40.0), WHITE),
    solid(rect(50.0, 30.0, 8.0, 20.0, 2.0), 0xb8_e0_ff),
    flat(line(&[(46.5, 25.0), (53.5, 22.0)], 2.0), WHITE),
    flat(line(&[(46.5, 33.0), (53.5, 30.0)], 2.0), WHITE),
    solid(
        poly(&[
            (50.0, 6.0),
            (55.0, 14.0),
            (54.0, 18.0),
            (50.0, 20.0),
            (46.0, 18.0),
            (45.0, 14.0),
        ]),
        ORANGE,
    ),
    flat(ellipse(50.0, 16.0, 2.5, 3.0), YELLOW),
];

/// Banana curva, com cabinho e ponta escura.
pub const BANANA: &[Layer] = &[
    solid(rect(87.0, 27.0, 6.0, 14.0, 2.0).turned(35.0), 0x8a_b8_4a),
    solid(
        poly(&[
            (84.4, 32.1),
            (83.5, 36.1),
            (82.4, 40.0),
            (80.9, 43.8),
            (79.2, 47.5),
            (77.2, 51.0),
            (74.9, 54.4),
            (72.4, 57.6),
            (69.6, 60.6),
            (66.6, 63.4),
            (63.5, 66.0),
            (60.1, 68.3),
            (56.6, 70.4),
            (52.9, 72.1),
            (49.1, 73.6),
            (45.2, 74.8),
            (41.2, 75.7),
            (37.2, 76.4),
            (33.1, 76.6),
            (29.0, 76.6),
            (25.0, 76.3),
            (20.9, 75.7),
            (16.9, 74.8),
            (13.0, 73.5),
            (9.3, 72.0),
            (14.6, 69.2),
            (19.1, 66.8),
            (23.1, 64.4),
            (26.6, 62.0),
            (29.7, 59.6),
            (32.4, 57.3),
            (34.8, 55.1),
            (37.0, 53.1),
            (38.9, 51.4),
            (40.7, 49.8),
            (42.5, 48.5),
            (44.4, 47.4),
            (46.3, 46.5),
            (48.4, 45.7),
            (50.7, 45.1),
            (53.3, 44.5),
            (56.1, 43.8),
            (59.2, 43.0),
            (62.7, 42.1),
            (66.4, 40.9),
            (70.4, 39.3),
            (74.6, 37.3),
            (79.1, 34.9),
        ]),
        YELLOW,
    ),
    flat(
        line(
            &[
                (66.7, 44.6),
                (60.7, 47.7),
                (55.2, 50.2),
                (50.2, 52.6),
                (45.4, 55.1),
                (40.7, 57.9),
                (35.5, 61.1),
                (29.5, 64.4),
            ],
            3.0,
        ),
        0xff_f0_a0,
    ),
    flat(circle(10.5, 72.0, 3.0), BROWN),
];

const TOMATO_BODY: Shape = ellipse(50.0, 58.0, 40.0, 33.0);
/// Tomate vermelho com folhinhas verdes.
pub const TOMATO: &[Layer] = &[
    solid(TOMATO_BODY, RED),
    flat(TOMATO_BODY, SHADE)
        .minus(ellipse(45.0, 52.0, 40.0, 33.0))
        .faded(0.18),
    solid(
        poly(&[
            (50.0, 15.0),
            (54.1, 22.9),
            (69.0, 22.6),
            (56.7, 27.2),
            (61.8, 34.9),
            (50.0, 29.9),
            (38.2, 34.9),
            (43.3, 27.2),
            (31.0, 22.6),
            (45.9, 22.9),
        ]),
        GREEN,
    ),
    solid(rect(50.0, 16.0, 4.5, 10.0, 2.0), GREEN_DARK),
    shine(32.0, 44.0, 8.0, 5.0),
];

const PEAR_PARTS: [Shape; 3] = [
    circle(50.0, 64.0, 27.0),
    circle(50.0, 36.0, 15.0),
    ellipse(50.0, 50.0, 20.0, 16.0),
];
const PEAR_COLOR: u32 = 0xc6_dd_52;
/// Pera verde-amarelada com cabinho e folha (contorno só por fora: os recheios cobrem as
/// bordas de dentro).
pub const PEAR: &[Layer] = &[
    solid(line(&[(50.0, 22.0), (54.0, 9.0)], 4.0), BROWN),
    solid(ellipse(64.0, 13.0, 10.0, 5.0).turned(-25.0), GREEN),
    solid(PEAR_PARTS[0], PEAR_COLOR),
    solid(PEAR_PARTS[1], PEAR_COLOR),
    solid(PEAR_PARTS[2], PEAR_COLOR),
    flat(PEAR_PARTS[0], PEAR_COLOR),
    flat(PEAR_PARTS[1], PEAR_COLOR),
    flat(PEAR_PARTS[2], PEAR_COLOR),
    flat(PEAR_PARTS[0], SHADE)
        .minus(circle(44.0, 58.0, 27.0))
        .faded(0.15),
    shine(38.0, 52.0, 6.0, 3.5),
];

const BUCKET: Shape = poly(&[(22.0, 48.0), (78.0, 48.0), (70.0, 92.0), (30.0, 92.0)]);
/// Pipoca: um balde listrado cheio de pipocas.
pub const POPCORN: &[Layer] = &[
    solid(circle(36.0, 26.0, 8.0), 0xff_f4_d6),
    solid(circle(50.0, 19.0, 9.0), 0xff_f4_d6),
    solid(circle(64.0, 25.0, 8.0), 0xff_f4_d6),
    solid(circle(27.0, 40.0, 8.0), 0xff_f4_d6),
    solid(circle(42.0, 33.0, 9.5), 0xff_f4_d6),
    solid(circle(57.0, 33.0, 9.5), 0xff_f4_d6),
    solid(circle(72.0, 39.0, 8.0), 0xff_f4_d6),
    solid(circle(35.0, 44.0, 8.0), 0xff_f4_d6),
    solid(circle(50.0, 44.0, 9.0), 0xff_f4_d6),
    solid(circle(65.0, 45.0, 8.0), 0xff_f4_d6),
    flat(circle(46.0, 30.0, 2.5), YELLOW),
    flat(circle(60.0, 22.0, 2.0), YELLOW),
    flat(circle(31.0, 38.0, 2.0), YELLOW),
    flat(circle(69.0, 37.0, 2.0), YELLOW),
    solid(BUCKET, WHITE),
    flat(rect(31.0, 70.0, 9.0, 50.0, 0.0), RED).inside(BUCKET),
    flat(rect(50.0, 70.0, 9.0, 50.0, 0.0), RED).inside(BUCKET),
    flat(rect(69.0, 70.0, 9.0, 50.0, 0.0), RED).inside(BUCKET),
    solid(rect(50.0, 49.0, 62.0, 7.0, 3.5), RED),
];

const COCONUT_SHELL: Shape = circle(50.0, 58.0, 34.0);
/// Coco verde com canudinho.
pub const COCONUT: &[Layer] = &[
    solid(line(&[(54.0, 36.0), (68.0, 7.0)], 5.0), PINK),
    solid(COCONUT_SHELL, 0x6f_bf_4a),
    flat(COCONUT_SHELL, SHADE)
        .minus(circle(44.0, 52.0, 34.0))
        .faded(0.18),
    solid(ellipse(50.0, 32.0, 15.0, 6.0), 0xf4_f9_e8),
    flat(ellipse(50.0, 33.0, 9.0, 3.5), 0xdd_ee_c4),
    flat(line(&[(54.0, 33.0), (57.0, 27.0)], 5.0), PINK),
    shine(34.0, 52.0, 6.0, 4.0),
];

const CANDY: Shape = circle(50.0, 38.0, 28.0);
/// Pirulito redondo em espiral, com palito.
pub const LOLLIPOP: &[Layer] = &[
    solid(line(&[(50.0, 62.0), (50.0, 91.0)], 5.0), WHITE),
    solid(CANDY, PINK),
    flat(
        line(
            &[
                (52.0, 38.0),
                (52.3, 39.4),
                (51.5, 41.0),
                (49.7, 42.0),
                (47.3, 41.8),
                (45.1, 40.2),
                (44.0, 37.2),
                (44.8, 33.9),
                (47.4, 31.1),
                (51.5, 30.1),
                (55.8, 31.6),
                (58.9, 35.3),
                (59.7, 40.5),
                (57.5, 45.6),
                (52.6, 49.0),
                (46.3, 49.4),
                (40.4, 46.3),
                (36.9, 40.2),
                (37.0, 32.8),
                (41.1, 26.3),
                (48.4, 22.8),
                (56.8, 23.5),
                (63.9, 28.8),
                (67.3, 37.3),
                (65.8, 46.7),
                (59.3, 54.2),
                (49.6, 57.3),
                (39.3, 54.9),
                (31.5, 47.2),
                (28.7, 36.2),
                (32.2, 25.1),
            ],
            4.5,
        ),
        WHITE,
    )
    .inside(CANDY),
    shine(38.0, 24.0, 5.0, 3.0),
];

const BOWL: Shape = ellipse(50.0, 56.0, 40.0, 30.0);
/// Tigela de sopa quentinha, com colher.
pub const SOUP: &[Layer] = &[
    solid(rect(50.0, 86.0, 30.0, 6.0, 3.0), 0x2f_8f_d8),
    solid(BOWL, BLUE).minus(rect(50.0, 26.0, 100.0, 60.0, 0.0)),
    solid(ellipse(50.0, 56.0, 40.0, 9.0), BLUE),
    flat(ellipse(50.0, 57.0, 35.0, 6.5), 0xff_b0_4d),
    flat(circle(38.0, 57.0, 2.5), GREEN),
    flat(circle(52.0, 59.0, 2.2), 0xff_6f_3d),
    flat(circle(62.0, 56.0, 2.5), GREEN),
    solid(line(&[(64.0, 56.0), (86.0, 30.0)], 5.0), GRAY),
    flat(
        line(
            &[(36.0, 44.0), (33.0, 36.0), (37.0, 28.0), (34.0, 20.0)],
            2.4,
        ),
        0xc9_c2_db,
    ),
    flat(
        line(
            &[(50.0, 44.0), (47.0, 36.0), (51.0, 28.0), (48.0, 20.0)],
            2.4,
        ),
        0xc9_c2_db,
    ),
    flat(ellipse(30.0, 70.0, 4.0, 2.2).turned(30.0), WHITE).faded(0.7),
];

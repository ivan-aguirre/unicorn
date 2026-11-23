//! Sprites desenhados à mão em ASCII: uma letra por pixel, `.` é transparente.
//! O contorno escuro é acrescentado automaticamente (ver `Pixmap::outlined`).

use super::pixmap::Palette;
use crate::letters::words::Picture;

/// Cores dos sprites.
pub const PALETTE: &Palette = &[
    // Unicórnio
    (b'o', 0x4b_3b_63),
    (b'W', 0xff_ff_ff),
    (b'g', 0xe2_d8_f2),
    (b'M', 0xff_7e_c0),
    (b'm', 0xe0_55_9f),
    (b'L', 0xff_c4_e2),
    (b'Y', 0xff_e2_7a),
    (b'D', 0xf0_b0_40),
    (b'y', 0xff_9f_d2),
    (b'E', 0x3a_24_62),
    (b'I', 0x8f_5f_f0),
    (b'i', 0xc3_a2_ff),
    (b'C', 0xff_a3_c8),
    (b'h', 0xf5_8c_c2),
    // Estrela e brilho
    (b'S', 0xff_e1_4d),
    (b's', 0xf5_b8_2e),
    // Bichinhos: corpo da borboleta, peixe
    (b'k', 0x4b_3b_63),
    (b'O', 0xff_9a_3c),
    (b'Q', 0xff_d2_9a),
    // Plantas
    (b'F', 0x3a_a2_4a),
    (b'P', 0xff_8c_cf),
    (b'p', 0xff_e1_4d),
    // Figuras das palavras
    (b'R', 0xf0_44_4a),
    (b'r', 0xb8_28_3a),
    (b'B', 0x4a_8a_f0),
    (b'b', 0x2f_62_c0),
    (b'N', 0x9a_5a_2a),
    (b'n', 0xd9_a0_60),
    (b'V', 0x9a_52_d8),
    (b'v', 0xc8_9a_f5),
    (b'T', 0x62_c8_50),
    (b't', 0x2f_8a_3a),
    (b'u', 0xc4_f0_9a),
    (b'c', 0xff_f1_d6),
    (b'G', 0xb8_b4_c8),
];

/// Cor do contorno automático.
pub const OUTLINE: u32 = 0x4b_3b_63;

/// Estrela da festa. 11x11.
#[rustfmt::skip]
pub const STAR: &[&str] = &[
    ".....S.....",
    "....SSS....",
    "....SWS....",
    "SSSSSWSSSSS",
    ".SSSSSSSSs.",
    "..SSSSSSs..",
    "...SSSSs...",
    "..SSSsSSs..",
    "..SSs.sSs..",
    ".SSs...sSs.",
    ".s.......s.",
];

/// Brilho ao pegar um coração (sem contorno). 11x11.
#[rustfmt::skip]
pub const SPARKLE: &[&str] = &[
    ".....S.....",
    ".....S.....",
    ".....S.....",
    "....SWS....",
    "...SWWWS...",
    "SSSWWWWWSSS",
    "...SWWWS...",
    "....SWS....",
    ".....S.....",
    ".....S.....",
    ".....S.....",
];

/// Flor balançando (dois quadros). 11x14. Pétalas `P`, miolo `p`.
#[rustfmt::skip]
pub const FLOWER: [&[&str]; 2] = [
    &[
        "...PP.PP...",
        "..PPPPPPP..",
        ".PPPPpPPPP.",
        "PPPPpppPPPP",
        ".PPPPpPPPP.",
        "..PPPPPPP..",
        "...PP.PP...",
        ".....F.....",
        ".....F..FF.",
        "..FF.F.FF..",
        "...FFFF....",
        ".....F.....",
        ".....F.....",
        ".....F.....",
    ],
    &[
        "....PP.PP..",
        "...PPPPPPP.",
        "..PPPPpPPPP",
        ".PPPPpppPPP",
        "..PPPPpPPPP",
        "...PPPPPPP.",
        "....PP.PP..",
        "......F....",
        ".....F..FF.",
        "..FF.F.FF..",
        "...FFFF....",
        ".....F.....",
        ".....F.....",
        ".....F.....",
    ],
];

/// Borboleta (asas abertas e fechadas). Asas `P`, pintinhas `p`, corpo `k`. 11x8.
#[rustfmt::skip]
pub const BUTTERFLY: [&[&str]; 2] = [
    &[
        "..PP...PP..",
        ".PPPP.PPPP.",
        "PPpPPkPPpPP",
        "PPPPPkPPPPP",
        ".PPPPkPPPP.",
        "..PPPkPPP..",
        ".PPP.k.PPP.",
        "..P.....P..",
    ],
    &[
        "....P.P....",
        "...PP.PP...",
        "...PPkPP...",
        "..PPPkPPP..",
        "...PPkPP...",
        "....PkP....",
        "....PkP....",
        ".....k.....",
    ],
];

/// Peixinho do lago, nadando para a direita. 9x5.
#[rustfmt::skip]
pub const FISH: &[&str] = &[
    "...OOO...",
    "..OOOOO.O",
    ".WEOOOOOO",
    "OOQQQOO.O",
    "..OOOO...",
];

// Figuras das palavras do modo Letras (ver `letters::words::Picture`).

/// Bola. 16x16.
#[rustfmt::skip]
const BALL: &[&str] = &[
    ".....RRRRRR.....",
    "...LRRRWWWBBB...",
    "..LRRRWWWWWBBB..",
    ".RRRRRWWWWWBBBB.",
    ".RRRRWWWWWWWBBB.",
    "RRRRRWWWWWWWBBBB",
    "RRRRWWWYYWWWBBBB",
    "RRRRWWYYYYWWBBBB",
    "RRRRWWYYYYWWBBBB",
    "RRRRWWWYYWWWBBBB",
    "RRRRRWWWWWWWBBBb",
    ".RRRRWWWWWWWBBb.",
    ".rRRRRWWWWWBBBb.",
    "..rRRRWWWWWBBb..",
    "...rrRRWWWBbb...",
    ".....rrrWbb.....",
];

/// Pato. 18x17.
#[rustfmt::skip]
const DUCK: &[&str] = &[
    "..........YYYY....",
    ".........YYYYYY...",
    ".........YYYWEY...",
    ".........YYYEEYOOO",
    ".........YYYYYYOOO",
    "..........YYYYY...",
    "..........YYYY....",
    "YY.......YYYYYY...",
    "YYY....YYYYYYYYY..",
    "YYYYYYYYYYYYYYYYY.",
    "YYYYYYDDDDDYYYYYY.",
    ".YYYYYYDDDDYYYYYY.",
    "..YYYYYYYYYYYYYY..",
    "...YYYYYYYYYYYY...",
    ".....YYYYYYYYY....",
    "........O..O......",
    ".......OO.OO......",
];

/// Gato. 17x17.
#[rustfmt::skip]
const CAT: &[&str] = &[
    "..O........O.....",
    "..OO......OO.....",
    "..OLO....OLO.....",
    "..OOOOOOOOOO.....",
    ".OOOOOOOOOOOO....",
    ".OOWEOOOOWEOO....",
    ".OOEEOOOOEEOO....",
    ".OOOOOCCOOOOO....",
    ".OOQOOEEOOQOO....",
    "..OOOOOOOOOO.....",
    "...OOOOOOOO....O.",
    "..OOOQQQQOOO...O.",
    ".OOOQQQQQQOOO..O.",
    ".OOOQQQQQQOOO.OO.",
    ".OOOQQQQQQOOOOO..",
    ".OOOQQQQQQOOOO...",
    "..QQ.QQ.QQ.QQ....",
];

/// Casa. 18x16.
#[rustfmt::skip]
const HOUSE: &[&str] = &[
    "........rr........",
    ".......rRRr.......",
    "......rRRRRr......",
    ".....rRRRRRRr.....",
    "....rRRRRRRRRr....",
    "...rRRRRRRRRRRr...",
    "..rRRRRRRRRRRRRr..",
    ".rrrrrrrrrrrrrrrr.",
    "..cccccccccccccc..",
    "..cBBBcccccccccc..",
    "..cBWBccccNNNccc..",
    "..cBBBccccNNNccc..",
    "..ccccccccNNNccc..",
    "..ccccccccNnNccc..",
    "..ccccccccNNNccc..",
    "FFFFFFFFFFFFFFFFFF",
];

/// Sapo. 17x14.
#[rustfmt::skip]
const FROG: &[&str] = &[
    "..WWW.......WWW..",
    ".WWEWW.....WWEWW.",
    ".WWEWWTTTTTWWEWW.",
    "..WWWTTTTTTTWWW..",
    ".TTTTTTTTTTTTTTT.",
    ".TTCCTTTTTTTCCTT.",
    ".TTTtTTTTTTTtTTT.",
    "..TTTtttttttTTT..",
    "..TTTTTTTTTTTTT..",
    ".TTTTuuuuuuuTTTT.",
    "TTTTuuuuuuuuuTTTT",
    "TTTTTuuuuuuuTTTTT",
    "TTT.TTTTTTTTT.TTT",
    "TT...TT...TT...TT",
];

/// Lua. 15x16.
#[rustfmt::skip]
const MOON: &[&str] = &[
    ".....YYYY......",
    "...YYYYY.......",
    "..YYYYY........",
    ".YYYYY.........",
    ".YYYY..........",
    "YYYYY..........",
    "YYYY...........",
    "YYEY...........",
    "YYYE...........",
    "YYYYY..........",
    "YCYYY.........Y",
    ".YYYYY.......Y.",
    ".DYYYYYY...YYY.",
    "..DDYYYYYYYYY..",
    "....DDYYYYYY...",
    "......DDDD.....",
];

/// Uva. 16x17.
#[rustfmt::skip]
const GRAPES: &[&str] = &[
    "......N.FF......",
    "......NFFFF.....",
    "......N.FFFF....",
    "..vVV.N.vVV.....",
    ".vVVVVvVVVVV....",
    ".VVVVVvVVVVVvVV.",
    "..VVvVVVvVVvVVVV",
    "..vVVVVvVVVVVVVV",
    ".vVVVVVVVVvVVVV.",
    ".VVVVvVVVVVVVV..",
    "..VVVVVVvVVVV...",
    "...vVVVVVVVVV...",
    "...VVVVVvVVV....",
    "....VVVVVVVV....",
    ".....vVVVVV.....",
    ".....VVVVV......",
    "......VVV.......",
];

/// Pipa. 14x20.
#[rustfmt::skip]
const KITE: &[&str] = &[
    "......RB......",
    ".....RRBB.....",
    "....RRRBBB....",
    "...RRRRBBBB...",
    "..RRRRRBBBBB..",
    ".RRRRRRBBBBBB.",
    "YYYYYYYTTTTTTT",
    ".YYYYYYTTTTTT.",
    "..YYYYYTTTTT..",
    "...YYYYTTTT...",
    "....YYYTTT....",
    ".....YYTT.....",
    "......YT......",
    "......o.......",
    ".......o......",
    "......LoL.....",
    "......o.......",
    ".....o........",
    "....LoL.......",
    ".....o........",
];

/// Bolo. 18x16.
#[rustfmt::skip]
const CAKE: &[&str] = &[
    "........Y.........",
    "........R.........",
    "........W.........",
    "........W.........",
    "..LLLLLLLLLLLLLL..",
    ".LLLLLLRRLLLLLLLL.",
    ".LLLLLLRRLLLLLLLL.",
    ".LNLLLNNLLLNLLLNL.",
    ".NNNNNNNNNNNNNNNN.",
    ".NNNNNNNNNNNNNNNN.",
    ".cccccccccccccccc.",
    ".NNNNNNNNNNNNNNNN.",
    ".NNNNNNNNNNNNNNNN.",
    ".NNNNNNNNNNNNNNNN.",
    "GGGGGGGGGGGGGGGGGG",
    ".GGGGGGGGGGGGGGGG.",
];

/// Dado. 15x14.
#[rustfmt::skip]
const DIE: &[&str] = &[
    ".WWWWWWWWWWWWW.",
    "WWWWWWWWWWWWWWG",
    "WWEEWWWWWWWEEWG",
    "WWEEWWWWWWWEEWG",
    "WWWWWWWWWWWWWWG",
    "WWWWWWWWWWWWWWG",
    "WWWWWWEEWWWWWWG",
    "WWWWWWEEWWWWWWG",
    "WWWWWWWWWWWWWWG",
    "WWWWWWWWWWWWWWG",
    "WWEEWWWWWWWEEWG",
    "WWEEWWWWWWWEEWG",
    "WWWWWWWWWWWWWWG",
    ".GGGGGGGGGGGGG.",
];

/// Banana. 18x11.
#[rustfmt::skip]
const BANANA: &[&str] = &[
    "..............NN..",
    ".............NYN..",
    "............YYY...",
    "...........YYYD...",
    "..........YYYYD...",
    ".........YYYYD....",
    "N.......YYYYYD....",
    "NYY...YYYYYYD.....",
    ".YYYYYYYYYYDD.....",
    "..DYYYYYYYDD......",
    "...DDDDDDDD.......",
];

/// Tomate. 16x15.
#[rustfmt::skip]
const TOMATO: &[&str] = &[
    ".......FF.......",
    ".......F........",
    "...FFFFFFFFF....",
    "..RRFFRFFRFFRR..",
    ".RRRRRRRRRRRRRR.",
    ".RWWRRRRRRRRRRR.",
    "RRWRRRRRRRRRRRRR",
    "RRRRRRRRRRRRRRRr",
    "RRRRRRRRRRRRRRRr",
    "RRRRRRRRRRRRRRrr",
    "RRRRRRRRRRRRRRrr",
    ".RRRRRRRRRRRRrr.",
    ".rRRRRRRRRRRrrr.",
    "..rrRRRRRRrrrr..",
    "....rrrrrrrr....",
];

/// Desenho de cada figura das palavras.
#[must_use]
pub const fn picture(p: Picture) -> &'static [&'static str] {
    match p {
        Picture::Ball => BALL,
        Picture::Duck => DUCK,
        Picture::Cat => CAT,
        Picture::House => HOUSE,
        Picture::Frog => FROG,
        Picture::Moon => MOON,
        Picture::Grapes => GRAPES,
        Picture::Kite => KITE,
        Picture::Cake => CAKE,
        Picture::Die => DIE,
        Picture::Banana => BANANA,
        Picture::Tomato => TOMATO,
    }
}

/// Dígitos e as letras usadas nos avisos, em fonte 5x7 (branca; a sombra vem depois).
#[rustfmt::skip]
pub const GLYPHS: [(char, &[&str]); 19] = [
    ('0', &[
        ".WWW.",
        "W...W",
        "W...W",
        "W...W",
        "W...W",
        "W...W",
        ".WWW.",
    ]),
    ('1', &[
        "..W..",
        ".WW..",
        "..W..",
        "..W..",
        "..W..",
        "..W..",
        ".WWW.",
    ]),
    ('2', &[
        ".WWW.",
        "W...W",
        "....W",
        "...W.",
        "..W..",
        ".W...",
        "WWWWW",
    ]),
    ('3', &[
        "WWWW.",
        "....W",
        "....W",
        ".WWW.",
        "....W",
        "....W",
        "WWWW.",
    ]),
    ('4', &[
        "...W.",
        "..WW.",
        ".W.W.",
        "W..W.",
        "WWWWW",
        "...W.",
        "...W.",
    ]),
    ('5', &[
        "WWWWW",
        "W....",
        "WWWW.",
        "....W",
        "....W",
        "W...W",
        ".WWW.",
    ]),
    ('6', &[
        "..WW.",
        ".W...",
        "W....",
        "WWWW.",
        "W...W",
        "W...W",
        ".WWW.",
    ]),
    ('7', &[
        "WWWWW",
        "....W",
        "...W.",
        "..W..",
        ".W...",
        ".W...",
        ".W...",
    ]),
    ('8', &[
        ".WWW.",
        "W...W",
        "W...W",
        ".WWW.",
        "W...W",
        "W...W",
        ".WWW.",
    ]),
    ('9', &[
        ".WWW.",
        "W...W",
        "W...W",
        ".WWWW",
        "....W",
        "...W.",
        ".WW..",
    ]),
    ('A', &[
        ".WWW.",
        "W...W",
        "W...W",
        "WWWWW",
        "W...W",
        "W...W",
        "W...W",
    ]),
    ('C', &[
        ".WWWW",
        "W....",
        "W....",
        "W....",
        "W....",
        "W....",
        ".WWWW",
    ]),
    ('E', &[
        "WWWWW",
        "W....",
        "W....",
        "WWWW.",
        "W....",
        "W....",
        "WWWWW",
    ]),
    ('G', &[
        ".WWWW",
        "W....",
        "W....",
        "W..WW",
        "W...W",
        "W...W",
        ".WWW.",
    ]),
    ('I', &[
        ".WWW.",
        "..W..",
        "..W..",
        "..W..",
        "..W..",
        "..W..",
        ".WWW.",
    ]),
    ('P', &[
        "WWWW.",
        "W...W",
        "W...W",
        "WWWW.",
        "W....",
        "W....",
        "W....",
    ]),
    ('R', &[
        "WWWW.",
        "W...W",
        "W...W",
        "WWWW.",
        "W.W..",
        "W..W.",
        "W...W",
    ]),
    ('S', &[
        ".WWWW",
        "W....",
        "W....",
        ".WWW.",
        "....W",
        "....W",
        "WWWW.",
    ]),
    ('U', &[
        "W...W",
        "W...W",
        "W...W",
        "W...W",
        "W...W",
        "W...W",
        ".WWW.",
    ]),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::art::pixmap::Pixmap;

    #[test]
    fn todos_os_sprites_sao_validos() {
        let mut all: Vec<&[&str]> = vec![
            STAR,
            SPARKLE,
            FLOWER[0],
            FLOWER[1],
            BUTTERFLY[0],
            BUTTERFLY[1],
            FISH,
        ];
        all.extend(GLYPHS.iter().map(|(_, g)| *g));
        all.extend(Picture::ALL.map(picture));
        for rows in all {
            assert!(
                Pixmap::from_ascii(rows, PALETTE).is_ok(),
                "sprite inválido: {rows:?}"
            );
        }
    }

    #[test]
    fn fonte_tem_glifos_unicos_e_cobre_os_avisos() {
        let mut chars: Vec<char> = GLYPHS.iter().map(|(c, _)| *c).collect();
        chars.sort_unstable();
        chars.dedup();
        assert_eq!(chars.len(), GLYPHS.len());
        for text in ["PAUSA", "SEGURE ESC PARA SAIR", "0123456789"] {
            assert!(
                text.chars().all(|c| c == ' ' || chars.contains(&c)),
                "{text}"
            );
        }
    }
}

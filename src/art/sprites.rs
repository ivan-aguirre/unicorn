//! Sprites desenhados à mão em ASCII: uma letra por pixel, `.` é transparente.
//! O contorno escuro é acrescentado automaticamente (ver `Pixmap::outlined`).

use super::pixmap::Palette;

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
    (b'y', 0xff_9f_d2),
    (b'E', 0x3a_24_62),
    (b'I', 0x8f_5f_f0),
    (b'C', 0xff_a3_c8),
    (b'h', 0xf5_8c_c2),
    // Coração
    (b'R', 0xff_4d_8d),
    (b'r', 0xd6_28_6a),
    (b'l', 0xff_d1_e3),
    // Estrela e brilho
    (b'S', 0xff_e1_4d),
    (b's', 0xf5_b8_2e),
    // Plantas
    (b'F', 0x3a_a2_4a),
    (b'P', 0xff_8c_cf),
    (b'p', 0xff_e1_4d),
];

/// Cor do contorno automático.
pub const OUTLINE: u32 = 0x4b_3b_63;

/// Cabeça, corpo, braço e cauda do unicórnio (olhando para a direita). 24x26.
pub const UNICORN_BODY: &[&str] = &[
    "........................",
    "..................Yy....",
    "..................YY....",
    "......MM.........yYY....",
    "....MMMMMM.WW....YYy....",
    "...MMLLMMmMWLW..yYY.....",
    "..MMLLMMmMMMWWWWWWW.....",
    "..MLLMMmMMMMMMMWWWWWW...",
    "..MLMMmMMMMMMMMWWWWWWW..",
    "..MLMmMMMMMMmMmWWWWWWW..",
    "..MLMmMMMMMmWWEEWWWWWW..",
    "..MMmMMMMMmWWEWIEWWWWW..",
    "..MLmMMMMmWWWEIIEWWWWWW.",
    "...LmMMMmWWWWEIWEWWWWoW.",
    "...MmMMmWWWWWWEEWWWWWWW.",
    "...MmMmWWWWWWWWWCCWWWWW.",
    "....MmmWWWWWWWWWWWWooW..",
    "....MLmgWWWWWWWWWWWW....",
    "....Mm..gggWWWWWgg......",
    "..MM....WWWWWWWW........",
    ".MMLM..WWWWWWWWWWWh.....",
    ".MLLMM.WWWWWWWgWWWh.....",
    ".MMMMMMWWWWWWWggg.......",
    ".MMmMMgWWWWWWWWW........",
    "..MmMM.WWWWWWWWW........",
    "..mMm...WWWWWWWg........",
];

/// Linha do corpo onde as pernas começam.
pub const LEGS_Y: u16 = 26;

/// Pernas paradas. 24x6.
pub const LEGS_STAND: &[&str] = &[
    ".........WWW.ggg........",
    ".........WWW.ggg........",
    ".........WWW.ggg........",
    ".........hhh.hhh........",
    "........hhhh.hhhh.......",
    "........................",
];

/// Passada: perna da frente (branca) adiante. O corpo desce 1 pixel.
pub const LEGS_STRIDE_A: &[&str] = &[
    "........................",
    "........ggg..WWW........",
    ".......ggg....WWW.......",
    "......hhh......hhh......",
    ".....hhhh......hhhh.....",
    "........................",
];

/// Pernas se cruzando no meio do passo.
pub const LEGS_PASS: &[&str] = &[
    ".........WWWgg..........",
    ".........WWWgg..........",
    ".........WWWgg..........",
    ".........hhhhh..........",
    "........hhhhh...........",
    "........................",
];

/// Passada: perna de trás (sombreada) adiante. O corpo desce 1 pixel.
pub const LEGS_STRIDE_B: &[&str] = &[
    "........................",
    "........WWW..ggg........",
    ".......WWW....ggg.......",
    "......hhh......hhh......",
    ".....hhhh......hhhh.....",
    "........................",
];

/// Pernas encolhidas no pulo.
pub const LEGS_JUMP: &[&str] = &[
    "........WWW..ggg........",
    ".......hhhW..ggh........",
    ".......hhh....hhh.......",
    "........................",
    "........................",
    "........................",
];

/// Olho fechado e feliz (^), sobreposto ao olho aberto para piscar. 4x5.
#[rustfmt::skip]
pub const EYE_CLOSED: &[&str] = &[
    "WWWW",
    "WWWW",
    "WEEW",
    "EWWE",
    "WWWW",
];
/// Posição do olho no corpo.
pub const EYE_AT: (u16, u16) = (13, 10);

/// Coração para coletar. 9x8.
pub const HEART: &[&str] = &[
    ".RR...RR.",
    "RlRR.RRRR",
    "RlRRRRRRR",
    "RRRRRRRRr",
    ".RRRRRRr.",
    "..RRRRr..",
    "...RRr...",
    "....r....",
];

/// Coraçãozinho da festa. 7x6.
#[rustfmt::skip]
pub const HEART_SMALL: &[&str] = &[
    ".RR.RR.",
    "RlRRRRR",
    "RRRRRRr",
    ".RRRRr.",
    "..RRr..",
    "...r...",
];

/// Estrela da festa. 7x7.
#[rustfmt::skip]
pub const STAR: &[&str] = &[
    "...S...",
    "..SSS..",
    "SSSWSSS",
    ".SSSSs.",
    "..SSs..",
    ".Ss.Ss.",
    ".s...s.",
];

/// Brilho ao pegar um coração (sem contorno). 7x7.
#[rustfmt::skip]
pub const SPARKLE: &[&str] = &[
    "...S...",
    "...S...",
    "..SWS..",
    "SSWWWSS",
    "..SWS..",
    "...S...",
    "...S...",
];

/// Flor balançando (dois quadros). 7x10. Pétalas `P`, miolo `p`.
#[rustfmt::skip]
pub const FLOWER: [&[&str]; 2] = [
    &[
        "..PPP..",
        ".PPpPP.",
        ".PpppP.",
        ".PPpPP.",
        "..PPP..",
        "...F...",
        "..FF...",
        "...F.F.",
        "...FF..",
        "...F...",
    ],
    &[
        "...PPP.",
        "..PPpPP",
        "..PpppP",
        "..PPpPP",
        "...PPP.",
        "...F...",
        "..FF...",
        "...F.F.",
        "...FF..",
        "...F...",
    ],
];

/// Dígitos e letras de "PAUSA" em fonte 5x7 (branca; a sombra é acrescentada depois).
#[rustfmt::skip]
pub const GLYPHS: [(char, &[&str]); 14] = [
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
    ('P', &[
        "WWWW.",
        "W...W",
        "W...W",
        "WWWW.",
        "W....",
        "W....",
        "W....",
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
    ('U', &[
        "W...W",
        "W...W",
        "W...W",
        "W...W",
        "W...W",
        "W...W",
        ".WWW.",
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
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::art::pixmap::Pixmap;

    fn all() -> Vec<(&'static str, &'static [&'static str])> {
        let mut v: Vec<(&str, &[&str])> = vec![
            ("corpo", UNICORN_BODY),
            ("pernas paradas", LEGS_STAND),
            ("passada A", LEGS_STRIDE_A),
            ("cruzando", LEGS_PASS),
            ("passada B", LEGS_STRIDE_B),
            ("pulo", LEGS_JUMP),
            ("olho", EYE_CLOSED),
            ("coração", HEART),
            ("coraçãozinho", HEART_SMALL),
            ("estrela", STAR),
            ("brilho", SPARKLE),
            ("flor 0", FLOWER[0]),
            ("flor 1", FLOWER[1]),
        ];
        v.extend(GLYPHS.iter().map(|(_, g)| ("glifo", *g)));
        v
    }

    #[test]
    fn todos_os_sprites_sao_validos() {
        for (name, rows) in all() {
            assert!(
                Pixmap::from_ascii(rows, PALETTE).is_ok(),
                "sprite inválido: {name}"
            );
        }
    }

    #[test]
    fn partes_do_unicornio_se_encaixam() {
        let body = Pixmap::from_ascii(UNICORN_BODY, PALETTE).unwrap();
        assert_eq!(body.height(), LEGS_Y);
        for legs in [
            LEGS_STAND,
            LEGS_STRIDE_A,
            LEGS_PASS,
            LEGS_STRIDE_B,
            LEGS_JUMP,
        ] {
            let legs = Pixmap::from_ascii(legs, PALETTE).unwrap();
            assert_eq!(legs.width(), body.width());
        }
        let eye = Pixmap::from_ascii(EYE_CLOSED, PALETTE).unwrap();
        assert!(EYE_AT.0 + eye.width() <= body.width());
        assert!(EYE_AT.1 + eye.height() <= body.height());
    }

    #[test]
    fn borda_do_sprite_fica_livre_para_o_contorno() {
        let body = Pixmap::from_ascii(UNICORN_BODY, PALETTE).unwrap();
        let w = body.width();
        for y in 0..body.height() {
            assert!(
                !body.is_opaque(0, y) && !body.is_opaque(w - 1, y),
                "linha {y}"
            );
        }
        for x in 0..w {
            assert!(!body.is_opaque(x, 0));
        }
    }

    #[test]
    fn fonte_tem_glifos_unicos() {
        let mut chars: Vec<char> = GLYPHS.iter().map(|(c, _)| *c).collect();
        chars.sort_unstable();
        chars.dedup();
        assert_eq!(chars.len(), GLYPHS.len());
    }
}

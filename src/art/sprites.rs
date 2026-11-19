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
    // Plantas
    (b'F', 0x3a_a2_4a),
    (b'P', 0xff_8c_cf),
    (b'p', 0xff_e1_4d),
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
        let mut all: Vec<&[&str]> = vec![STAR, SPARKLE, FLOWER[0], FLOWER[1]];
        all.extend(GLYPHS.iter().map(|(_, g)| *g));
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

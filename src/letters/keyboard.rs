//! Teclado desenhado na tela: layout ABNT2 (as três fileiras de letras) e a posição de cada
//! tecla na tela virtual. Puro: testável sem janela.

use macroquad::math::{Rect, Vec2};

use crate::view::{VIEW_H, VIEW_W};

/// Fileiras de letras do teclado ABNT2.
pub const ROWS: [&str; 3] = ["QWERTYUIOP", "ASDFGHJKLÇ", "ZXCVBNM"];
/// Recuo de cada fileira, como no teclado de verdade.
const ROW_OFFSETS: [f32; 3] = [0.0, 10.0, 26.0];
/// Tamanho de uma tecla e o espaço entre teclas.
pub const KEY_W: f32 = 30.0;
pub const KEY_H: f32 = 29.0;
const GAP: f32 = 3.0;
/// Margem da base escura em volta das teclas.
const PLATE_PAD: f32 = 7.0;

/// Canto superior esquerdo da primeira tecla (teclado centralizado embaixo da tela).
#[must_use]
pub fn origin() -> Vec2 {
    let width = 10.0f32.mul_add(KEY_W + GAP, -GAP);
    let height = 3.0f32.mul_add(KEY_H + GAP, -GAP);
    Vec2::new(
        ((VIEW_W - width) / 2.0).round(),
        VIEW_H - height - PLATE_PAD,
    )
}

/// Base escura atrás das teclas (aberta embaixo: encosta na borda da tela).
#[must_use]
pub fn plate() -> Rect {
    let o = origin();
    let width = 10.0f32.mul_add(KEY_W + GAP, -GAP);
    Rect::new(
        o.x - PLATE_PAD - 4.0,
        o.y - PLATE_PAD,
        2.0f32.mul_add(PLATE_PAD, width) + 8.0,
        VIEW_H - o.y + PLATE_PAD + 12.0,
    )
}

/// Todas as teclas com o retângulo de cada uma.
pub fn keys() -> impl Iterator<Item = (char, Rect)> {
    let o = origin();
    ROWS.iter()
        .zip(ROW_OFFSETS)
        .zip([0.0_f32, 1.0, 2.0])
        .flat_map(move |((row, offset), r)| {
            let y = r.mul_add(KEY_H + GAP, o.y);
            row.chars().zip((0_u8..).map(f32::from)).map(move |(c, i)| {
                let x = i.mul_add(KEY_W + GAP, o.x + offset);
                (c, Rect::new(x, y, KEY_W, KEY_H))
            })
        })
}

/// Retângulo da tecla `c`, se existir.
#[must_use]
pub fn key_rect(c: char) -> Option<Rect> {
    keys().find(|(k, _)| *k == c).map(|(_, r)| r)
}

/// Letra digitada como aparece no teclado (maiúscula), se for uma das teclas de letra.
#[must_use]
pub fn normalize(c: char) -> Option<char> {
    let upper = c.to_uppercase().next()?;
    keys().any(|(k, _)| k == upper).then_some(upper)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn todas_as_letras_tem_tecla() {
        for c in ('A'..='Z').chain(['Ç']) {
            assert!(key_rect(c).is_some(), "{c}");
        }
        assert_eq!(keys().count(), 27);
    }

    #[test]
    fn teclas_cabem_na_tela_e_nao_se_sobrepoem() {
        let all: Vec<(char, Rect)> = keys().collect();
        let screen = Rect::new(0.0, 0.0, VIEW_W, VIEW_H);
        for (c, r) in &all {
            assert!(
                screen.contains(r.point()) && screen.contains(r.point() + r.size()),
                "{c}"
            );
            assert!(plate().contains(r.point()), "{c} fora da base");
        }
        for (i, (a, ra)) in all.iter().enumerate() {
            for (b, rb) in &all[i + 1..] {
                assert!(!ra.overlaps(rb), "{a} e {b} se sobrepõem");
            }
        }
    }

    #[test]
    fn fileiras_na_ordem_do_teclado() {
        let q = key_rect('Q').unwrap();
        let a = key_rect('A').unwrap();
        let z = key_rect('Z').unwrap();
        assert!(q.y < a.y && a.y < z.y);
        assert!(q.x < a.x && a.x < z.x, "fileiras recuadas");
        assert!(key_rect('W').unwrap().x > q.x);
    }

    #[test]
    fn normaliza_minusculas_e_ignora_o_resto() {
        assert_eq!(normalize('a'), Some('A'));
        assert_eq!(normalize('ç'), Some('Ç'));
        assert_eq!(normalize('Z'), Some('Z'));
        for c in ['1', ' ', 'á', '\n', '-'] {
            assert_eq!(normalize(c), None, "{c:?}");
        }
    }
}

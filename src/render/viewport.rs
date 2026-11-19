//! Onde a tela virtual aparece na janela: escala inteira quando possível (pixels iguais),
//! senão a maior escala que cabe. O resto vira moldura.

use crate::view::{VIEW_H, VIEW_W};

/// Escala inteira só vale a pena se ocupar ao menos esta fração da escala máxima.
const INTEGER_SCALE_MIN_FILL: f32 = 0.85;

/// Retângulo da tela virtual dentro da janela.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

/// Calcula o retângulo para uma janela de `screen_w` x `screen_h`.
#[must_use]
pub fn fit(screen_w: f32, screen_h: f32) -> Viewport {
    let max_scale = (screen_w / VIEW_W).min(screen_h / VIEW_H).max(f32::EPSILON);
    let integer = max_scale.floor();
    let scale = if integer >= 1.0 && integer >= max_scale * INTEGER_SCALE_MIN_FILL {
        integer
    } else {
        max_scale
    };
    let (w, h) = (VIEW_W * scale, VIEW_H * scale);
    Viewport {
        x: ((screen_w - w) / 2.0).floor(),
        y: ((screen_h - h) / 2.0).floor(),
        w,
        h,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_hd_usa_escala_cinco_exata() {
        let v = fit(1920.0, 1080.0);
        assert_eq!(
            v,
            Viewport {
                x: 0.0,
                y: 0.0,
                w: 1920.0,
                h: 1080.0
            }
        );
    }

    #[test]
    fn janela_media_usa_escala_inteira_centralizada() {
        let v = fit(1400.0, 800.0);
        assert!((v.w / VIEW_W - 2.0).abs() < f32::EPSILON);
        assert!((v.x - 60.0).abs() < f32::EPSILON);
        assert!((v.y - 40.0).abs() < f32::EPSILON);
    }

    #[test]
    fn janela_pequena_usa_escala_fracionaria() {
        let v = fit(900.0, 600.0);
        assert!(v.w <= 900.0 && v.h <= 600.0);
        assert!(v.w > 850.0, "deve aproveitar a janela");
    }

    #[test]
    fn mantem_a_proporcao() {
        for (w, h) in [(800.0, 600.0), (2560.0, 1080.0), (300.0, 900.0)] {
            let v = fit(w, h);
            assert!((v.w / v.h - VIEW_W / VIEW_H).abs() < 1e-3);
            assert!(v.x >= 0.0 && v.y >= 0.0);
        }
    }
}

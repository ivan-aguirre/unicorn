//! Vida no cenário: borboletas, pássaros, peixes, pétalas, brilhos e o vento nas árvores.
//!
//! Tudo é função do tempo e da posição, sem estado guardado: barato (nada a atualizar por
//! quadro), determinístico e testável sem janela.

use macroquad::math::Vec2;

/// A cada quantos segundos cada peixe pula.
pub const FISH_PERIOD: f32 = 5.5;
/// Duração do salto do peixe.
const FISH_JUMP: f32 = 0.9;
/// Duração das ondinhas depois que o peixe volta para a água.
const FISH_RIPPLE: f32 = 0.6;
/// Alcance do salto: distância e altura.
const FISH_REACH: (f32, f32) = (36.0, 34.0);
/// Tempo de queda de uma pétala, da copa até o chão.
const PETAL_FALL: f32 = 4.0;
/// Quanto a pétala cai.
const PETAL_DROP: f32 = 72.0;
/// Pássaros: tamanho do laço na tela e paralaxe do céu.
const SKY_LOOP: f32 = 1500.0;
const SKY_PARALLAX: f32 = 0.15;

/// Borboleta voando em "oito" sobre uma flor. Devolve a posição e se a asa está aberta.
#[must_use]
pub fn butterfly(flower: Vec2, t: f32) -> (Vec2, bool) {
    let seed = flower.x * 0.137;
    let phase = t.mul_add(0.9, seed);
    let hover = 5.0f32.mul_add(t.mul_add(0.6, seed).sin(), flower.y - 26.0);
    let loop_ = Vec2::new(16.0 * phase.sin(), 7.0 * (2.0 * phase).sin());
    let open = t.mul_add(14.0, seed).sin() > 0.0;
    (Vec2::new(flower.x, hover) + loop_, open)
}

/// Se há uma borboleta nesta flor (umas poucas, escolhidas pela posição).
#[must_use]
pub fn has_butterfly(flower_x: f32) -> bool {
    (flower_x * 0.0173).sin() > 0.75
}

/// Momento de um peixe do lago.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Fish {
    /// Embaixo d'água (não aparece).
    Hidden,
    /// No ar: progresso do salto de 0 a 1.
    Jumping(f32),
    /// De volta à água: ondinhas crescendo de 0 a 1.
    Ripple(f32),
}

/// Estado do peixe `slot` no tempo `t` (cada peixe pula num momento diferente).
#[must_use]
pub fn fish(slot: f32, t: f32) -> Fish {
    let local = slot.mul_add(2.3, t).rem_euclid(FISH_PERIOD);
    if local < FISH_JUMP {
        Fish::Jumping(local / FISH_JUMP)
    } else if local < FISH_JUMP + FISH_RIPPLE {
        Fish::Ripple((local - FISH_JUMP) / FISH_RIPPLE)
    } else {
        Fish::Hidden
    }
}

/// Posição do peixe no arco do salto (saindo de `start`) e ângulo do corpo, em radianos.
#[must_use]
pub fn fish_arc(start: Vec2, progress: f32) -> (Vec2, f32) {
    let (reach, height) = FISH_REACH;
    let lift = 4.0 * height * progress * (1.0 - progress);
    let slope = -4.0 * height * 2.0f32.mul_add(-progress, 1.0) / reach;
    (
        Vec2::new(reach.mul_add(progress, start.x), start.y - lift),
        slope.atan(),
    )
}

/// Onde o peixe volta para a água.
#[must_use]
pub const fn fish_landing(start: Vec2) -> Vec2 {
    Vec2::new(start.x + FISH_REACH.0, start.y)
}

/// Pétala `i` caindo de uma copa florida: posição e opacidade (aparece e some suave).
#[must_use]
pub fn petal(crown: Vec2, i: u8, t: f32) -> (Vec2, f32) {
    let index = f32::from(i);
    let fall = (index.mul_add(0.83, t) / PETAL_FALL).rem_euclid(1.0);
    let spread = (index * 2.399).sin() * 16.0;
    let flutter = t.mul_add(2.2, index).sin() * 5.0;
    let at = Vec2::new(
        crown.x + spread + 18.0f32.mul_add(fall, flutter),
        PETAL_DROP.mul_add(fall, crown.y),
    );
    let alpha = (fall * 5.0).min((1.0 - fall) * 5.0).min(1.0);
    (at, alpha)
}

/// Pássaro `k` do bando `flock`, na tela: posição e asa para cima. Bandos em "V".
#[must_use]
pub fn bird(flock: u8, k: u8, t: f32, cam: f32) -> (Vec2, bool) {
    let group = f32::from(flock);
    let speed = 6.0f32.mul_add(group, 18.0);
    let travel = t.mul_add(speed, group.mul_add(470.0, -cam * SKY_PARALLAX));
    let rank = f32::from(k.div_ceil(2));
    let side: f32 = if k % 2 == 1 { -1.0 } else { 1.0 };
    let glide = t.mul_add(0.8, group).sin() * 4.0;
    let at = Vec2::new(
        8.0f32.mul_add(-rank, travel.rem_euclid(SKY_LOOP) - 100.0),
        (5.0 * side).mul_add(rank, 22.0f32.mul_add(group, 46.0) + glide),
    );
    let wing = t.mul_add(7.0, f32::from(k) * 1.3).sin() > 0.0;
    (at, wing)
}

/// Intensidade (0 a 1) de um brilho que pisca, com fases diferentes por `k`.
#[must_use]
pub fn twinkle(k: f32, t: f32) -> f32 {
    t.mul_add(2.5, k * 1.7).sin().max(0.0).powi(3)
}

/// Balanço da copa de uma árvore com o vento, em pixels inteiros.
#[must_use]
pub fn sway(x: f32, t: f32) -> f32 {
    (t.mul_add(1.3, x * 0.013).sin() * 1.6).round()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn times() -> impl Iterator<Item = f32> {
        (0..2000_u16).map(|i| f32::from(i) * 0.01)
    }

    #[test]
    fn borboleta_fica_perto_da_flor_e_bate_as_asas() {
        let flower = Vec2::new(500.0, 300.0);
        let mut flaps = 0;
        let mut last = false;
        for t in times() {
            let (pos, open) = butterfly(flower, t);
            assert!((pos.x - flower.x).abs() <= 16.0);
            assert!(pos.y < flower.y - 10.0 && pos.y > flower.y - 40.0);
            flaps += u32::from(open != last);
            last = open;
        }
        assert!(flaps > 50, "asas paradas");
    }

    #[test]
    fn so_algumas_flores_tem_borboleta() {
        let with = (0..1000_u16)
            .filter(|x| has_butterfly(f32::from(*x) * 40.0))
            .count();
        assert!(
            (50..350).contains(&with),
            "{with} borboletas em 1000 flores"
        );
    }

    #[test]
    fn peixe_pula_volta_e_some_em_ciclo() {
        let states: Vec<Fish> = times().map(|t| fish(0.0, t)).collect();
        assert!(states.iter().any(|s| matches!(s, Fish::Jumping(_))));
        assert!(states.iter().any(|s| matches!(s, Fish::Ripple(_))));
        let hidden = states.iter().filter(|s| **s == Fish::Hidden).count();
        assert!(
            hidden > states.len() / 2,
            "o lago não pode virar um aquário"
        );
        let progress = states.iter().filter_map(|s| match s {
            Fish::Jumping(p) | Fish::Ripple(p) => Some(*p),
            Fish::Hidden => None,
        });
        assert!(progress.into_iter().all(|p| (0.0..=1.0).contains(&p)));
    }

    #[test]
    fn peixes_diferentes_pulam_em_momentos_diferentes() {
        let together = times()
            .filter(|t| {
                matches!(
                    (fish(0.0, *t), fish(1.0, *t)),
                    (Fish::Jumping(_), Fish::Jumping(_))
                )
            })
            .count();
        assert_eq!(together, 0);
    }

    #[test]
    fn arco_do_peixe_sai_e_volta_para_a_agua() {
        let start = Vec2::new(100.0, 250.0);
        let (begin, up) = fish_arc(start, 0.0);
        let (top, level) = fish_arc(start, 0.5);
        let (end, down) = fish_arc(start, 1.0);
        assert!((begin - start).length() < 1e-3);
        assert!((end - fish_landing(start)).length() < 1e-3);
        assert!((start.y - top.y - FISH_REACH.1).abs() < 1e-3);
        assert!(up < 0.0 && down > 0.0 && level.abs() < 1e-3);
    }

    #[test]
    fn petalas_caem_da_copa_ate_o_chao_e_somem_suave() {
        let crown = Vec2::new(300.0, 200.0);
        let petals = times().flat_map(|t| (0..6).map(move |i| petal(crown, i, t)));
        for (pos, alpha) in petals {
            assert!(pos.y >= crown.y && pos.y <= crown.y + PETAL_DROP);
            assert!((0.0..=1.0).contains(&alpha));
            let landing = pos.y > crown.y + PETAL_DROP - 1.0;
            assert!(!landing || alpha < 0.1, "pétala some ao tocar o chão");
        }
    }

    #[test]
    fn passaros_voam_no_ceu_em_formacao() {
        for t in times() {
            let (leader, _) = bird(0, 0, t, 1234.0);
            for k in 1..5 {
                let (b, _) = bird(0, k, t, 1234.0);
                assert!(b.y < 150.0, "pássaro baixo demais");
                assert!(b.x <= leader.x + 1e-3 || b.x > leader.x + SKY_LOOP - 200.0);
            }
        }
    }

    #[test]
    fn brilho_e_balanco_ficam_nos_limites() {
        for t in times() {
            assert!((0.0..=1.0).contains(&twinkle(3.0, t)));
            assert!(sway(t * 100.0, t).abs() <= 2.0);
        }
    }
}

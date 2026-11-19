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

/// Voos de passarinhos no céu ao mesmo tempo.
pub const FLIGHTS: u8 = 4;
/// Quantos passarinhos em cada voo: sozinhos ou em grupinhos.
const FLIGHT_SIZES: [u8; 4] = [1, 2, 1, 3];
/// Ciclo de voo: bate as asas por um tempo e depois plana.
const FLAP_CYCLE: f32 = 2.6;
const FLAPPING: f32 = 1.2;
const FLAP_FRAME_TIME: f32 = 0.1;
/// Quadro de planar (depois dos 4 de bater as asas).
pub const GLIDE_FRAME: u8 = 4;

/// Quantos passarinhos há no voo `flight`.
#[must_use]
pub fn flight_size(flight: u8) -> u8 {
    FLIGHT_SIZES.get(usize::from(flight)).copied().unwrap_or(1)
}

/// Um passarinho na tela.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BirdView {
    /// Centro do passarinho.
    pub at: Vec2,
    /// Quadro da asa (0 a 3 batendo, 4 planando).
    pub frame: u8,
    pub facing_left: bool,
}

/// Passarinho `member` do voo `flight`: atravessa o céu (uns para a direita, outros para a
/// esquerda), ondulando, batendo as asas e planando; os do grupinho vão um pouco atrás.
#[must_use]
pub fn bird(flight: u8, member: u8, t: f32, cam: f32) -> BirdView {
    let (group, place) = (f32::from(flight), f32::from(member));
    let rightward = flight.is_multiple_of(2);
    let along = t
        .mul_add(9.0f32.mul_add(group, 34.0), group * 610.0)
        .rem_euclid(SKY_LOOP);
    let heading = if rightward { along } else { SKY_LOOP - along };
    let behind = 18.0f32.mul_add(place, 4.0 * t.mul_add(0.9, place).sin());
    let lead_x = cam.mul_add(-SKY_PARALLAX, heading).rem_euclid(SKY_LOOP) - 120.0;
    let side: f32 = match member % 3 {
        0 => 0.0,
        1 => -10.0,
        _ => 9.0,
    };
    let bob = 2.0f32.mul_add(t.mul_add(1.4, place * 2.0).sin(), side);
    let altitude = 8.0f32.mul_add(t.mul_add(0.6, group).sin(), 26.0f32.mul_add(group, 40.0));
    let local = place
        .mul_add(0.23, group.mul_add(0.7, t))
        .rem_euclid(FLAP_CYCLE);
    let frame = if local < FLAPPING {
        crate::animation::frame_at(local, FLAP_FRAME_TIME, GLIDE_FRAME)
    } else {
        GLIDE_FRAME
    };
    BirdView {
        at: Vec2::new(
            if rightward {
                lead_x - behind
            } else {
                lead_x + behind
            },
            altitude + bob,
        ),
        frame,
        facing_left: !rightward,
    }
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
    fn passarinhos_voam_no_ceu_sozinhos_ou_em_grupinhos() {
        assert!((0..FLIGHTS).all(|f| (1..=3).contains(&flight_size(f))));
        assert!(
            (0..FLIGHTS).any(|f| flight_size(f) == 1),
            "há quem voe sozinho"
        );
        let all = times().flat_map(|t| {
            (0..FLIGHTS).flat_map(move |f| (0..flight_size(f)).map(move |m| (t, f, m)))
        });
        for (t, flight, member) in all {
            let leader = bird(flight, 0, t, 500.0);
            let b = bird(flight, member, t, 500.0);
            assert!(b.at.y > 20.0 && b.at.y < 160.0, "fora do céu");
            assert!(b.frame <= GLIDE_FRAME);
            let near = (b.at.x - leader.at.x).abs() < 60.0;
            let wrapped = (b.at.x - leader.at.x).abs() > SKY_LOOP - 200.0;
            assert!(near || wrapped, "o grupinho se desfez");
        }
    }

    #[test]
    fn passarinho_bate_as_asas_e_depois_plana() {
        let frames: Vec<u8> = times().map(|t| bird(0, 0, t, 0.0).frame).collect();
        assert!(frames.contains(&GLIDE_FRAME));
        assert!((0..GLIDE_FRAME).all(|f| frames.contains(&f)));
    }

    #[test]
    fn passarinho_olha_para_onde_voa() {
        for flight in 0..FLIGHTS {
            let (a, b) = (bird(flight, 0, 10.0, 0.0), bird(flight, 0, 10.1, 0.0));
            let moving_left = b.at.x < a.at.x;
            assert_eq!(a.facing_left, moving_left, "voo {flight}");
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

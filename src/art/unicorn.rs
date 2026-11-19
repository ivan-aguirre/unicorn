//! O unicórnio: cabeça desenhada à mão (ASCII) e corpo, pernas, braços e cauda gerados por
//! ângulos, o que permite uma animação fluida com poucos dados.

use super::pixmap::{ArtError, Pixmap, Rgba, rgb};
use super::sprites::{OUTLINE, PALETTE};

/// Largura de um quadro.
pub const FRAME_W: u16 = 46;
/// Altura de um quadro.
pub const FRAME_H: u16 = 64;
/// Ponto do quadro que fica sobre a posição dos pés (centro, linha do chão).
pub const FEET: (f32, f32) = (23.0, 62.0);
/// Comprimento das pernas no ar (no chão, elas se esticam até a linha do chão).
const LEG: f32 = 9.5;
/// Raio dos cascos (meia largura das pernas).
const LEG_RADIUS: f32 = 2.5;
/// Onde a cabeça é colada no quadro.
const HEAD_AT: (u16, u16) = (2, 0);
/// Olho na cabeça (canto superior esquerdo) e tamanho, para piscar.
const EYE_AT: (u16, u16) = (23, 21);
const EYE_SIZE: (u16, u16) = (6, 8);
/// Pálpebra fechada e feliz (^), em coordenadas da cabeça.
const CLOSED_EYE: [(u16, u16); 6] = [(23, 26), (24, 25), (25, 25), (26, 25), (27, 25), (28, 26)];

/// Cores do corpo.
const WHITE: u32 = 0xff_ff_ff;
const SHADE: u32 = 0xe2_d8_f2;
const MANE: u32 = 0xff_7e_c0;
const MANE_DARK: u32 = 0xe0_55_9f;
const MANE_LIGHT: u32 = 0xff_c4_e2;
const HOOF: u32 = 0xf5_8c_c2;
const HOOF_BACK: u32 = 0xd9_60_9e;
const EYE_DARK: u32 = 0x3a_24_62;

/// Receita de um quadro: ângulos (radianos; 0 = para baixo, positivo = para frente) e balanços.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FrameSpec {
    /// Perna da frente e de trás.
    pub legs: (f32, f32),
    /// Quanto cada casco sobe do chão (a perna que avança no passo).
    pub lift: (f32, f32),
    /// Braço da frente e de trás.
    pub arms: (f32, f32),
    /// Balanço da cauda (-1 a 1).
    pub sway: f32,
    /// Pixels que o corpo desce (passada larga).
    pub bob: u16,
    /// Pixels que só a cabeça desce (respiração).
    pub nod: u16,
    pub blink: bool,
    /// No ar (pulo/queda): as pernas não precisam alcançar o chão.
    pub airborne: bool,
}

impl FrameSpec {
    const REST: Self = Self {
        legs: (0.0, 0.0),
        lift: (0.0, 0.0),
        arms: (0.3, 0.2),
        sway: 0.0,
        bob: 0,
        nod: 0,
        blink: false,
        airborne: false,
    };
}

/// Quadros na ordem de `Pose::index`: 4 parado (respirando), piscando, 6 andando, pulo, queda.
#[must_use]
pub fn specs() -> Vec<FrameSpec> {
    let mut out = Vec::with_capacity(13);
    let breathing = [
        (-0.6, 0, 0.3),
        (0.0, 0, 0.35),
        (0.6, 1, 0.4),
        (0.0, 1, 0.35),
    ];
    out.extend(breathing.iter().map(|&(sway, nod, arm)| FrameSpec {
        sway,
        nod,
        arms: (arm, arm - 0.1),
        ..FrameSpec::REST
    }));
    out.push(FrameSpec {
        blink: true,
        ..FrameSpec::REST
    });
    out.extend((0..6_u8).map(walk));
    out.push(FrameSpec {
        legs: (-0.9, 0.6),
        arms: (2.4, 2.0),
        sway: 1.0,
        airborne: true,
        ..FrameSpec::REST
    });
    out.push(FrameSpec {
        legs: (0.25, -0.2),
        arms: (1.6, 1.3),
        sway: -1.0,
        airborne: true,
        ..FrameSpec::REST
    });
    out
}

/// Quadro `i` da caminhada: pernas e braços alternados, a perna que avança levanta o casco e
/// o corpo desce nas passadas largas.
fn walk(i: u8) -> FrameSpec {
    let phase = std::f32::consts::TAU * f32::from(i) / 6.0;
    let (swing, step) = phase.sin_cos();
    FrameSpec {
        legs: (0.55 * swing, -0.55 * swing),
        lift: (2.5 * step.max(0.0), 2.5 * (-step).max(0.0)),
        arms: (0.3f32.mul_add(-swing, 0.35), 0.3f32.mul_add(swing, 0.35)),
        sway: swing,
        bob: u16::from(swing.abs() > 0.5),
        ..FrameSpec::REST
    }
}

/// Monta todos os quadros do unicórnio.
///
/// # Errors
/// Cabeça ASCII inválida (coberta pelos testes).
pub fn frames() -> Result<Vec<Pixmap>, ArtError> {
    let head = Pixmap::from_ascii(HEAD, PALETTE)?;
    Ok(specs().iter().map(|s| render(&head, s)).collect())
}

/// Desenha um quadro, de trás para a frente, e contorna.
#[must_use]
pub fn render(head: &Pixmap, spec: &FrameSpec) -> Pixmap {
    let mut p = Pixmap::new(FRAME_W, FRAME_H);
    let bob = f32::from(spec.bob);
    let hip = 50.0 + bob;
    // No chão, cada perna se estica até a linha do chão (menos o quanto o casco subiu).
    let reach = |angle: f32, lift: f32| {
        if spec.airborne {
            LEG
        } else {
            (FEET.1 - 0.1 - LEG_RADIUS - hip) / angle.cos() - lift
        }
    };
    let (front, back) = spec.legs;
    let back_leg = reach(back, spec.lift.1);
    limb(
        &mut p,
        Limb::leg((20.0, hip), back, back_leg, (SHADE, HOOF_BACK)),
    );
    limb(
        &mut p,
        Limb::arm((23.0, 41.0 + bob), spec.arms.1, (SHADE, HOOF_BACK)),
    );
    tail(&mut p, (15.0, 47.0 + bob), spec.sway);
    torso(&mut p, bob);
    let front_leg = reach(front, spec.lift.0);
    limb(
        &mut p,
        Limb::leg((25.0, hip), front, front_leg, (WHITE, HOOF)),
    );
    let mut face = head.clone();
    if spec.blink {
        close_eye(&mut face);
    }
    p.overlay(&face, HEAD_AT.0, HEAD_AT.1 + spec.bob + spec.nod);
    limb(
        &mut p,
        Limb::arm((28.0, 41.0 + bob), spec.arms.0, (WHITE, HOOF)),
    );
    p.outlined(rgb(OUTLINE))
}

fn close_eye(face: &mut Pixmap) {
    for y in EYE_AT.1..EYE_AT.1 + EYE_SIZE.1 {
        for x in EYE_AT.0..EYE_AT.0 + EYE_SIZE.0 {
            face.set(x, y, rgb(WHITE));
        }
    }
    for (x, y) in CLOSED_EYE {
        face.set(x, y, rgb(EYE_DARK));
    }
}

/// Tronco: elipse branca com sombra na parte de trás, embaixo.
fn torso(p: &mut Pixmap, bob: f32) {
    let (white, shade) = (rgb(WHITE), rgb(SHADE));
    for (x, y) in p.coords() {
        let (fx, fy) = (f32::from(x) + 0.5, f32::from(y) + 0.5);
        if ((fx - 22.0) / 8.5).hypot((fy - 44.0 - bob) / 8.0) <= 1.0 {
            let back = fy > 48.0 + bob && fx < 19.5;
            p.set(x, y, if back { shade } else { white });
        }
    }
}

/// Membro em forma de cápsula, com o casco na ponta.
#[derive(Debug, Clone, Copy)]
struct Limb {
    from: (f32, f32),
    angle: f32,
    length: f32,
    width: f32,
    hoof_len: f32,
    color: Rgba,
    hoof: Rgba,
}

impl Limb {
    const fn leg(from: (f32, f32), angle: f32, length: f32, (color, hoof): (u32, u32)) -> Self {
        Self {
            from,
            angle,
            length,
            width: LEG_RADIUS * 2.0,
            hoof_len: 3.0,
            color: rgb(color),
            hoof: rgb(hoof),
        }
    }

    const fn arm(from: (f32, f32), angle: f32, (color, hoof): (u32, u32)) -> Self {
        Self {
            from,
            angle,
            length: 8.0,
            width: 4.0,
            hoof_len: 2.0,
            color: rgb(color),
            hoof: rgb(hoof),
        }
    }
}

fn limb(p: &mut Pixmap, l: Limb) {
    let dir = (l.angle.sin(), l.angle.cos());
    for (x, y) in p.coords() {
        let (dx, dy) = (f32::from(x) + 0.5 - l.from.0, f32::from(y) + 0.5 - l.from.1);
        let along = dx.mul_add(dir.0, dy * dir.1).clamp(0.0, l.length);
        let gap = along.mul_add(-dir.0, dx).hypot(along.mul_add(-dir.1, dy));
        if gap <= l.width / 2.0 {
            let tip = along > l.length - l.hoof_len;
            p.set(x, y, if tip { l.hoof } else { l.color });
        }
    }
}

/// Cauda cacheada: discos que afinam ao longo de uma curva, com fios e brilho.
fn tail(p: &mut Pixmap, base: (f32, f32), sway: f32) {
    let points: Vec<(f32, f32, f32)> = (0..18_u8)
        .map(|i| {
            let t = f32::from(i) / 17.0;
            let curl = t * 4.2;
            let along = 12.0f32.mul_add(-t, base.0);
            let x = (sway * 1.2).mul_add(-t, (2.5 * t * t).mul_add(curl.sin(), along));
            let arc = 3.0f32.mul_add(-(t * 3.2).sin(), base.1);
            let y = (sway * 1.5).mul_add(t, (4.0 * t * t).mul_add(curl.cos(), arc));
            (x, y, 2.6f32.mul_add(-t, 4.6))
        })
        .collect();
    for &(x, y, r) in &points {
        disc(p, (x, y), r, rgb(MANE));
    }
    for &(x, y, r) in points.iter().skip(1).take(14) {
        p.plot(x, 0.45f32.mul_add(r, y), rgb(MANE_DARK));
    }
    for &(x, y, r) in points.iter().skip(1).take(9) {
        p.plot(x + 1.0, 0.55f32.mul_add(-r, y), rgb(MANE_LIGHT));
    }
}

fn disc(p: &mut Pixmap, (cx, cy): (f32, f32), r: f32, color: Rgba) {
    for (x, y) in p.coords() {
        if (f32::from(x) + 0.5 - cx).hypot(f32::from(y) + 0.5 - cy) <= r {
            p.set(x, y, color);
        }
    }
}

/// Cabeça olhando para a direita: crina farta com franja até a testa e o chifre saindo do
/// meio do cabelo. 40x38.
#[rustfmt::skip]
pub const HEAD: &[&str] = &[
    "........................................",
    ".......................Y................",
    "......................YD................",
    "......................yD................",
    ".....................YYD................",
    ".............WW......YyD................",
    ".............WW.....yYYD................",
    "............WWWW....YYyD................",
    "............WLLWMMMMYyYD................",
    "..........MMWLLWMMMyYYyD................",
    "........MMMWWLLWWMmMMMMMMM..............",
    ".......MMMMWWLLWWmMMmMMMMMM.............",
    "......MMMMMWWLLWWmMMmMmMMMMM............",
    ".....MMMMMMMMmMMmMMmMMmMMmMMM...........",
    "....MMMMMMMMmLLmmLLLLMmMMmMMMM..........",
    "....MMMMMMLmmMMmmMMmLLmMMmMMMM..........",
    "...MMMMMLLmmmMmMmMmMMMmLMmMMMMM.........",
    "...MMMMMLmMmMmMmMMmMMmMMMmMMMMM.........",
    "...MMMMMMmMmmMMmMmMMMmMMMmMMMMM.........",
    "...MMMMMmMmmMMMmmMMMMmMMMMMMMMMW........",
    "...MMMmmMMmMMMmMMMMMMWMMMWMMWMEW........",
    "...MMMMMMmmMMMmMmWMMMWMMEEEEWEWW........",
    "....MMMMmmMMMMmmWWMMMWMEEEEEEWWW........",
    "....MMMmMmMMMmmMWWMMMWWEIIWWEWWWW.......",
    "....MMmMmMMMMmMMWWWMWWWEIIWIEWWWWWW.....",
    "....MmMMmMMMmmMMWWWWWWWEIIIIEWWWWWWW....",
    "....mMMmMMMMmMMMWWWWWWWEiiiIEWWWWWWW....",
    "....MMMmMMMmmMMMWWWWWWWEiiiiEWWWWWooW...",
    "....MMmMMMmmMMMMWWWWWWWWEEEEWWWWWWWWW...",
    "....MMmMMMmmMMMMWWWWWWWWWWWWWWWWWWWWW...",
    "....MmMMmMmMMMMMWWWWWWWWWCCCWWWWWWWWW...",
    ".....mMmMMmMMMMWWWWWWWWWWWWWWWoWWoWW....",
    ".....MMMMmMMMMMWWWWWWWWWWWWWWWWooWWW....",
    "......mMMmMMMMWWWWWWWWWWWWWWWWWWWWW.....",
    ".....M..MMMM...WWWWWWWWWW..WWWWWW.......",
    "........M..M............................",
    "........................................",
    "........................................",
];

#[cfg(test)]
mod tests {
    use super::*;

    fn all() -> Vec<Pixmap> {
        frames().unwrap()
    }

    #[test]
    fn treze_quadros_do_mesmo_tamanho() {
        let f = all();
        assert_eq!(f.len(), 13);
        assert!(
            f.iter()
                .all(|p| (p.width(), p.height()) == (FRAME_W, FRAME_H))
        );
    }

    #[test]
    fn cabeca_cabe_no_quadro() {
        let head = Pixmap::from_ascii(HEAD, PALETTE).unwrap();
        assert!(HEAD_AT.0 + head.width() <= FRAME_W);
        assert!(HEAD_AT.1 + head.height() + 2 <= FRAME_H);
    }

    #[test]
    fn caminhada_tem_quadros_diferentes() {
        let f = all();
        let walk = &f[5..11];
        for (i, a) in walk.iter().enumerate() {
            for b in &walk[i + 1..] {
                assert_ne!(a, b, "quadros repetidos na caminhada");
            }
        }
    }

    #[test]
    fn parado_respira_e_pisca() {
        let f = all();
        assert_ne!(f[0], f[2], "a respiração mexe a cabeça e a cauda");
        assert_ne!(f[0], f[4], "piscar muda o quadro");
    }

    #[test]
    fn pes_tocam_a_linha_do_chao_em_todo_quadro_no_chao() {
        // Linha logo acima do chão tem casco; a do chão, só contorno ou nada.
        let ground = 62_u16;
        for (i, f) in all().iter().enumerate().take(11) {
            let hoof_row = (0..FRAME_W).any(|x| {
                f.get(x, ground - 1)
                    .is_some_and(|c| c[3] > 0 && c != rgb(OUTLINE))
            });
            assert!(hoof_row, "quadro {i} flutuando");
            let below = (0..FRAME_W).all(|x| {
                f.get(x, ground)
                    .is_none_or(|c| c[3] == 0 || c == rgb(OUTLINE))
            });
            assert!(below, "quadro {i} afundando no chão");
        }
    }

    #[test]
    fn borda_livre_para_o_contorno() {
        for f in all() {
            assert!((0..FRAME_W).all(|x| f.get(x, 0) == Some([0, 0, 0, 0]) || f.get(x, 0) == Some(rgb(OUTLINE))));
            assert!((0..FRAME_H).all(|y| f.get(0, y) != Some(rgb(WHITE))));
        }
    }
}

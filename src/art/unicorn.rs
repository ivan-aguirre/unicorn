//! O unicórnio, desenhado em camadas montadas na hora: corpo (pernas, cauda e tronco),
//! braço de trás, cabeça (uma por expressão) e braço da frente.
//!
//! A cabeça é desenhada à mão (ASCII); o resto é gerado por ângulos, o que dá uma animação
//! fluida com poucos dados. Cada camada tem contorno próprio, então os dois braços aparecem
//! bem definidos sobre o corpo.

use super::pixmap::{ArtError, Pixmap, Rgba, rgb};
use super::sprites::{OUTLINE, PALETTE};
use crate::animation::{Expression, IDLE_FRAMES, Pose, WALK_FRAMES, WAVE_FRAMES};

/// Largura de um quadro.
pub const FRAME_W: u16 = 46;
/// Altura de um quadro.
pub const FRAME_H: u16 = 64;
/// Ponto do quadro que fica sobre a posição dos pés (centro, linha do chão).
pub const FEET: (f32, f32) = (23.0, 62.0);
/// Onde a cabeça é colada no quadro (antes do balanço).
pub const HEAD_AT: (u16, u16) = (2, 0);
/// Comprimento das pernas no ar (no chão, elas se esticam até a linha do chão).
const LEG: f32 = 9.5;
/// Raio dos cascos (meia largura das pernas).
const LEG_RADIUS: f32 = 2.5;
/// Olho na cabeça (canto superior esquerdo) e tamanho.
const EYE_AT: (u16, u16) = (23, 21);
const EYE_SIZE: (u16, u16) = (6, 8);

/// Cores.
const WHITE: u32 = 0xff_ff_ff;
const SHADE: u32 = 0xe2_d8_f2;
const ARM_BACK: u32 = 0xe6_de_f4;
const MANE: u32 = 0xff_7e_c0;
const MANE_DARK: u32 = 0xe0_55_9f;
const MANE_LIGHT: u32 = 0xff_c4_e2;
const HOOF: u32 = 0xf5_8c_c2;
const HOOF_BACK: u32 = 0xd9_60_9e;
const EYE_DARK: u32 = 0x3a_24_62;
const BLUSH: u32 = 0xff_a3_c8;
const MOUTH: u32 = 0xe0_55_9f;

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
    /// Pixels que o corpo desce.
    pub bob: u16,
    /// Pixels que a cabeça desce (pode atrasar em relação ao corpo, como um balanço).
    pub head: u16,
    /// Peito: variação da altura do tronco (respiração).
    pub chest: f32,
    /// No ar (pulo/queda): as pernas não precisam alcançar o chão.
    pub airborne: bool,
}

impl FrameSpec {
    const REST: Self = Self {
        legs: (0.0, 0.0),
        lift: (0.0, 0.0),
        arms: (0.3, 0.3),
        sway: 0.0,
        bob: 0,
        head: 0,
        chest: 0.0,
        airborne: false,
    };
}

/// Camadas de um quadro do corpo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BodyFrame {
    /// Pernas, cauda e tronco.
    pub body: Pixmap,
    pub back_arm: Pixmap,
    pub front_arm: Pixmap,
    /// Quanto a cabeça desce neste quadro.
    pub head_drop: u16,
}

/// Receitas na ordem de `Pose::index`: respiração, aceno, caminhada, pulo, queda e aterrissagem.
#[must_use]
pub fn specs() -> Vec<FrameSpec> {
    let mut out = Vec::with_capacity(Pose::COUNT);
    out.extend((0..IDLE_FRAMES).map(breathe));
    out.extend((0..WAVE_FRAMES).map(wave));
    out.extend((0..WALK_FRAMES).map(walk));
    out.push(FrameSpec {
        legs: (-0.9, 0.6),
        arms: (2.5, 2.2),
        sway: 1.0,
        airborne: true,
        ..FrameSpec::REST
    });
    out.push(FrameSpec {
        legs: (0.25, -0.2),
        arms: (1.7, 1.4),
        sway: -1.0,
        airborne: true,
        ..FrameSpec::REST
    });
    out.push(FrameSpec {
        legs: (0.45, -0.45),
        arms: (1.1, 0.9),
        sway: -0.5,
        bob: 2,
        head: 2,
        chest: -0.6,
        ..FrameSpec::REST
    });
    out
}

/// Respiração: o peito sobe e desce, a cabeça acompanha, braços e cauda balançam devagar.
fn breathe(i: u8) -> FrameSpec {
    let phase = std::f32::consts::TAU * f32::from(i) / f32::from(IDLE_FRAMES);
    let breath = phase.sin();
    FrameSpec {
        arms: (
            0.12f32.mul_add(breath, 0.3),
            0.12f32.mul_add((phase + 1.3).sin(), 0.3),
        ),
        sway: 0.8 * (phase + 0.6).sin(),
        head: u16::from(breath < -0.4),
        chest: 0.7 * breath,
        ..FrameSpec::REST
    }
}

/// Aceno: braço da frente para o alto, indo e voltando, e a cauda abanando.
fn wave(i: u8) -> FrameSpec {
    const ARM: [f32; 4] = [2.45, 2.95, 2.6, 3.1];
    const TAIL: [f32; 4] = [-1.0, 1.0, -0.6, 0.6];
    let k = usize::from(i % WAVE_FRAMES);
    FrameSpec {
        arms: (ARM.get(k).copied().unwrap_or(2.5), 0.45),
        sway: TAIL.get(k).copied().unwrap_or(0.0),
        head: u16::from(k == 2),
        ..FrameSpec::REST
    }
}

/// Caminhada: pernas e braços alternados, a perna que avança levanta o casco, o corpo desce
/// nas passadas largas e a cabeça balança um quadro atrás do corpo.
fn walk(i: u8) -> FrameSpec {
    let at = |k: u8| {
        let phase = std::f32::consts::TAU * f32::from(k % WALK_FRAMES) / f32::from(WALK_FRAMES);
        phase.sin_cos()
    };
    let (swing, step) = at(i);
    let (previous, _) = at(i + WALK_FRAMES - 1);
    FrameSpec {
        legs: (0.55 * swing, -0.55 * swing),
        lift: (2.5 * step.max(0.0), 2.5 * (-step).max(0.0)),
        arms: (0.45f32.mul_add(-swing, 0.4), 0.45f32.mul_add(swing, 0.4)),
        sway: swing,
        bob: u16::from(swing.abs() > 0.5),
        head: u16::from(previous.abs() > 0.5),
        ..FrameSpec::REST
    }
}

/// Monta as camadas de todos os quadros do corpo.
#[must_use]
pub fn body_frames() -> Vec<BodyFrame> {
    specs().iter().map(render).collect()
}

/// Uma cabeça por expressão, na ordem de `Expression::index`.
///
/// # Errors
/// Cabeça ASCII inválida (coberta pelos testes).
pub fn heads() -> Result<Vec<Pixmap>, ArtError> {
    let base = Pixmap::from_ascii(HEAD, PALETTE)?;
    let all = [
        Expression::Calm,
        Expression::Blink,
        Expression::Smile,
        Expression::Excited,
        Expression::Joy,
    ];
    Ok(all
        .iter()
        .map(|e| face(&base, *e).outlined(rgb(OUTLINE)))
        .collect())
}

/// Monta um quadro completo (para ícone, testes e capturas).
#[must_use]
pub fn compose(frame: &BodyFrame, head: &Pixmap) -> Pixmap {
    let mut out = frame.body.clone();
    out.overlay(&frame.back_arm, 0, 0);
    out.overlay(head, HEAD_AT.0, HEAD_AT.1 + frame.head_drop);
    out.overlay(&frame.front_arm, 0, 0);
    out
}

/// Desenha as camadas de um quadro, cada uma com seu contorno.
fn render(spec: &FrameSpec) -> BodyFrame {
    let outline = rgb(OUTLINE);
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
    let mut body = Pixmap::new(FRAME_W, FRAME_H);
    let back_leg = reach(back, spec.lift.1);
    limb(
        &mut body,
        Limb::leg((20.0, hip), back, back_leg, (SHADE, HOOF_BACK)),
    );
    tail(&mut body, (15.0, 47.0 + bob), spec.sway);
    torso(&mut body, bob, spec.chest);
    let front_leg = reach(front, spec.lift.0);
    limb(
        &mut body,
        Limb::leg((25.0, hip), front, front_leg, (WHITE, HOOF)),
    );
    let mut back_arm = Pixmap::new(FRAME_W, FRAME_H);
    limb(
        &mut back_arm,
        Limb::arm((22.0, 41.0 + bob), spec.arms.1, (ARM_BACK, HOOF_BACK)),
    );
    let mut front_arm = Pixmap::new(FRAME_W, FRAME_H);
    limb(
        &mut front_arm,
        Limb::arm((28.0, 41.0 + bob), spec.arms.0, (WHITE, HOOF)),
    );
    BodyFrame {
        body: body.outlined(outline),
        back_arm: back_arm.outlined(outline),
        front_arm: front_arm.outlined(outline),
        head_drop: spec.head,
    }
}

/// Aplica a expressão (olhos, boca e bochechas) à cabeça.
fn face(base: &Pixmap, expression: Expression) -> Pixmap {
    let mut head = base.clone();
    match expression {
        Expression::Calm => {}
        Expression::Blink => close_eye(&mut head, &BLINK_LINE),
        Expression::Smile => open_mouth(&mut head),
        Expression::Excited => {
            open_mouth(&mut head);
            head.set(24, 26, rgb(WHITE));
            head.set(25, 25, rgb(WHITE));
        }
        Expression::Joy => {
            close_eye(&mut head, &HAPPY_ARC);
            open_mouth(&mut head);
            for x in 24..29 {
                head.set(x, 29, rgb(BLUSH));
            }
        }
    }
    head
}

/// Olho fechado relaxado (‿), para piscar.
const BLINK_LINE: [(u16, u16); 6] = [(23, 25), (24, 26), (25, 26), (26, 26), (27, 26), (28, 25)];
/// Olho fechado de alegria (^).
const HAPPY_ARC: [(u16, u16); 6] = [(23, 26), (24, 25), (25, 24), (26, 24), (27, 25), (28, 26)];

fn close_eye(head: &mut Pixmap, line: &[(u16, u16)]) {
    for y in EYE_AT.1..EYE_AT.1 + EYE_SIZE.1 {
        for x in EYE_AT.0..EYE_AT.0 + EYE_SIZE.0 {
            head.set(x, y, rgb(WHITE));
        }
    }
    for &(x, y) in line {
        head.set(x, y, rgb(EYE_DARK));
    }
}

/// Boca aberta sorrindo (formato de "D" deitado).
fn open_mouth(head: &mut Pixmap) {
    let ink = rgb(OUTLINE);
    for x in 30..34 {
        head.set(x, 31, ink);
    }
    head.set(30, 32, ink);
    head.set(31, 32, rgb(MOUTH));
    head.set(32, 32, rgb(MOUTH));
    head.set(33, 32, ink);
    head.set(31, 33, ink);
    head.set(32, 33, ink);
}

/// Tronco: elipse branca (o peito varia com a respiração) com sombra atrás, embaixo.
fn torso(p: &mut Pixmap, bob: f32, chest: f32) {
    let (white, shade) = (rgb(WHITE), rgb(SHADE));
    let radius = (8.5, 9.0 + chest);
    // A base do tronco fica parada: só o peito sobe e desce.
    let center = (22.0, 52.0 + bob - radius.1);
    for (x, y) in p.coords() {
        let (fx, fy) = (f32::from(x) + 0.5, f32::from(y) + 0.5);
        if ((fx - center.0) / radius.0).hypot((fy - center.1) / radius.1) <= 1.0 {
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
            length: 8.5,
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
            let x = (sway * 1.6).mul_add(-t, (2.5 * t * t).mul_add(curl.sin(), along));
            let arc = 3.0f32.mul_add(-(t * 3.2).sin(), base.1);
            let y = (sway * 2.0).mul_add(t, (4.0 * t * t).mul_add(curl.cos(), arc));
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

    fn composed() -> Vec<Pixmap> {
        let heads = heads().unwrap();
        body_frames()
            .iter()
            .map(|f| compose(f, &heads[0]))
            .collect()
    }

    fn visible(p: &Pixmap) -> usize {
        p.coords().filter(|(x, y)| p.is_opaque(*x, *y)).count()
    }

    #[test]
    fn um_quadro_por_pose_e_uma_cabeca_por_expressao() {
        let frames = body_frames();
        assert_eq!(frames.len(), Pose::COUNT);
        assert!(
            frames
                .iter()
                .all(|f| (f.body.width(), f.body.height()) == (FRAME_W, FRAME_H))
        );
        assert_eq!(heads().unwrap().len(), Expression::COUNT);
    }

    #[test]
    fn cabeca_cabe_no_quadro_mesmo_balancando() {
        let head = Pixmap::from_ascii(HEAD, PALETTE).unwrap();
        let max_drop = body_frames().iter().map(|f| f.head_drop).max().unwrap();
        assert!(HEAD_AT.0 + head.width() <= FRAME_W);
        assert!(HEAD_AT.1 + head.height() + max_drop <= FRAME_H);
    }

    #[test]
    fn os_dois_bracos_aparecem_em_todo_quadro() {
        for (i, f) in body_frames().iter().enumerate() {
            assert!(
                visible(&f.back_arm) > 15,
                "braço de trás sumiu no quadro {i}"
            );
            assert!(
                visible(&f.front_arm) > 15,
                "braço da frente sumiu no quadro {i}"
            );
        }
    }

    /// Primeiro par de itens iguais, se houver.
    fn repeated<T: PartialEq>(items: &[T]) -> Option<(usize, usize)> {
        (0..items.len())
            .flat_map(|i| (i + 1..items.len()).map(move |j| (i, j)))
            .find(|&(i, j)| items[i] == items[j])
    }

    #[test]
    fn quadros_de_cada_animacao_sao_diferentes() {
        let f = composed();
        for (start, end) in [(0, 8), (8, 12), (12, 18)] {
            assert_eq!(repeated(&f[start..end]), None, "animação {start}..{end}");
        }
    }

    #[test]
    fn expressoes_sao_todas_diferentes() {
        assert_eq!(repeated(&heads().unwrap()), None);
    }

    #[test]
    fn pes_tocam_a_linha_do_chao_em_todo_quadro_no_chao() {
        let ground = 62_u16;
        let solid = |c: [u8; 4]| c[3] > 0 && c != rgb(OUTLINE);
        let grounded = (0..18).chain([20]);
        let frames = composed();
        for i in grounded {
            let f = &frames[i];
            let touching = (0..FRAME_W).any(|x| f.get(x, ground - 1).is_some_and(solid));
            assert!(touching, "quadro {i} flutuando");
            let sinking = (0..FRAME_W).any(|x| f.get(x, ground).is_some_and(solid));
            assert!(!sinking, "quadro {i} afundando no chão");
        }
    }
}

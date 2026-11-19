//! O unicórnio, um pônei de quatro patas, desenhado em camadas montadas na hora: corpo
//! (patas, cauda, barriga e pescoço) e cabeça (uma por expressão).
//!
//! A cabeça é desenhada à mão (ASCII); o resto é gerado por ângulos, o que dá uma animação
//! fluida com poucos dados. Como num desenho de perfil, as patas do lado de lá ficam atrás do
//! corpo, num tom mais escuro; no trote, as patas andam em pares diagonais.

use super::pixmap::{ArtError, Pixmap, Rgba, rgb};
use super::sprites::{OUTLINE, PALETTE};
use crate::animation::{Expression, IDLE_FRAMES, PAW_FRAMES, Pose, WALK_FRAMES};

/// Largura de um quadro.
pub const FRAME_W: u16 = 64;
/// Altura de um quadro.
pub const FRAME_H: u16 = 62;
/// Ponto do quadro que fica sobre a posição dos pés (centro das patas, linha do chão).
pub const FEET: (f32, f32) = (31.0, 60.0);
/// Onde a cabeça é colada no quadro (antes do balanço).
pub const HEAD_AT: (u16, u16) = (23, 0);
/// Comprimento das patas no ar (no chão, elas se esticam até a linha do chão).
const LEG: f32 = 9.0;
/// Raio dos cascos (meia largura das patas).
const LEG_RADIUS: f32 = 2.5;
/// Olho na cabeça (canto superior esquerdo) e tamanho.
const EYE_AT: (u16, u16) = (23, 23);
const EYE_SIZE: (u16, u16) = (6, 8);

/// Onde cada pata se prende ao corpo: dianteira de cá, dianteira de lá, traseira de cá,
/// traseira de lá.
const HIPS: [(f32, f32); 4] = [(39.0, 48.0), (42.0, 47.0), (21.0, 48.0), (24.0, 47.0)];
/// Patas de cá (desenhadas na frente do corpo).
const NEAR: [bool; 4] = [true, false, true, false];

/// Cores.
const WHITE: u32 = 0xff_ff_ff;
const SHADE: u32 = 0xe2_d8_f2;
const FAR: u32 = 0xd6_ca_ec;
const MANE: u32 = 0xff_7e_c0;
const MANE_DARK: u32 = 0xe0_55_9f;
const MANE_LIGHT: u32 = 0xff_c4_e2;
const HOOF: u32 = 0xf5_8c_c2;
const HOOF_FAR: u32 = 0xd9_60_9e;
const EYE_DARK: u32 = 0x3a_24_62;
const BLUSH: u32 = 0xff_a3_c8;
const MOUTH: u32 = 0xe0_55_9f;

/// Receita de um quadro: ângulos das patas (radianos; 0 = para baixo, positivo = para a
/// frente) na ordem de [`HIPS`], e balanços.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FrameSpec {
    pub legs: [f32; 4],
    /// Quanto cada casco sobe do chão (as patas que avançam no passo).
    pub lift: [f32; 4],
    /// Balanço da cauda (-1 a 1).
    pub sway: f32,
    /// Pixels que o corpo desce.
    pub bob: u16,
    /// Pixels que a cabeça desce (pode atrasar em relação ao corpo, como um balanço).
    pub head: u16,
    /// Variação da altura da barriga (respiração).
    pub chest: f32,
    /// No ar (pulo/queda): as patas não precisam alcançar o chão.
    pub airborne: bool,
}

impl FrameSpec {
    const REST: Self = Self {
        legs: [0.0; 4],
        lift: [0.0; 4],
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
    /// Patas, cauda, barriga e pescoço.
    pub body: Pixmap,
    /// Quanto a cabeça desce neste quadro.
    pub head_drop: u16,
}

/// Receitas na ordem de `Pose::index`: respiração, patinha, trote, pulo, queda e aterrissagem.
#[must_use]
pub fn specs() -> Vec<FrameSpec> {
    let mut out = Vec::with_capacity(Pose::COUNT);
    out.extend((0..IDLE_FRAMES).map(breathe));
    out.extend((0..PAW_FRAMES).map(paw));
    out.extend((0..WALK_FRAMES).map(trot));
    // Pulo: dianteiras dobradas para a frente, traseiras esticadas para trás.
    out.push(FrameSpec {
        legs: [1.3, 1.1, -1.0, -0.8],
        sway: 1.0,
        airborne: true,
        ..FrameSpec::REST
    });
    // Queda: dianteiras procurando o chão.
    out.push(FrameSpec {
        legs: [0.5, 0.35, -0.4, -0.25],
        sway: -1.0,
        airborne: true,
        ..FrameSpec::REST
    });
    out.push(FrameSpec {
        legs: [0.35, 0.25, -0.35, -0.25],
        sway: -0.5,
        bob: 2,
        head: 2,
        chest: -0.6,
        ..FrameSpec::REST
    });
    out
}

/// Respiração: a barriga sobe e desce, a cabeça acompanha, o peso muda de leve entre as
/// patas e a cauda balança devagar.
fn breathe(i: u8) -> FrameSpec {
    let phase = std::f32::consts::TAU * f32::from(i) / f32::from(IDLE_FRAMES);
    let breath = phase.sin();
    let shift = 0.06 * (phase + 0.8).sin();
    FrameSpec {
        legs: [shift, shift, -shift, -shift],
        sway: 0.8 * (phase + 0.6).sin(),
        head: u16::from(breath < -0.4),
        chest: 0.7 * breath,
        ..FrameSpec::REST
    }
}

/// Patinha: a dianteira de cá sobe, sobe mais, desce e bate no chão; a cauda abana.
fn paw(i: u8) -> FrameSpec {
    const ANGLE: [f32; 4] = [0.7, 1.05, 0.7, 0.15];
    const LIFT: [f32; 4] = [4.0, 6.0, 4.0, 0.0];
    const TAIL: [f32; 4] = [-1.0, 1.0, -0.6, 0.6];
    let k = usize::from(i % PAW_FRAMES);
    let at = |table: [f32; 4]| table.get(k).copied().unwrap_or(0.0);
    FrameSpec {
        legs: [at(ANGLE), 0.0, 0.0, 0.0],
        lift: [at(LIFT), 0.0, 0.0, 0.0],
        sway: at(TAIL),
        head: u16::from(k == 3),
        ..FrameSpec::REST
    }
}

/// Trote: patas em pares diagonais (dianteira de cá com traseira de lá), as que avançam
/// levantam o casco, o corpo desce nas passadas largas e a cabeça balança um quadro depois.
fn trot(i: u8) -> FrameSpec {
    const PAIR: [f32; 4] = [1.0, -1.0, -1.0, 1.0];
    let at = |k: u8| {
        let phase = std::f32::consts::TAU * f32::from(k % WALK_FRAMES) / f32::from(WALK_FRAMES);
        phase.sin_cos()
    };
    let (swing, step) = at(i);
    let (previous, _) = at(i + WALK_FRAMES - 1);
    FrameSpec {
        legs: PAIR.map(|side| 0.5 * side * swing),
        lift: PAIR.map(|side| 3.0 * (side * step).max(0.0)),
        sway: swing,
        bob: u16::from(swing.abs() > 0.5),
        head: u16::from(previous.abs() > 0.5),
        ..FrameSpec::REST
    }
}

/// Monta todos os quadros do corpo.
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
    out.overlay(head, HEAD_AT.0, HEAD_AT.1 + frame.head_drop);
    out
}

/// Desenha o corpo: patas de lá, cauda, barriga, pescoço e patas de cá; contorno por fim.
fn render(spec: &FrameSpec) -> BodyFrame {
    let bob = f32::from(spec.bob);
    let mut body = Pixmap::new(FRAME_W, FRAME_H);
    let leg = |i: usize| {
        let hip = HIPS.get(i).copied().unwrap_or(HIPS[0]);
        let hip = (hip.0, hip.1 + bob);
        let angle = spec.legs.get(i).copied().unwrap_or(0.0);
        let lift = spec.lift.get(i).copied().unwrap_or(0.0);
        // No chão, cada pata se estica até a linha do chão (menos o quanto o casco subiu).
        let length = if spec.airborne {
            LEG
        } else {
            (FEET.1 - 0.1 - LEG_RADIUS - hip.1) / angle.cos() - lift
        };
        let near = NEAR.get(i).copied().unwrap_or(true);
        let colors = if near { (WHITE, HOOF) } else { (FAR, HOOF_FAR) };
        Limb::leg(hip, angle, length, colors)
    };
    for i in [1, 3] {
        limb(&mut body, leg(i));
    }
    tail(&mut body, (19.0, 37.0 + bob), spec.sway);
    barrel(&mut body, bob, spec.chest);
    for i in [0, 2] {
        limb(&mut body, leg(i));
    }
    BodyFrame {
        body: body.outlined(rgb(OUTLINE)),
        head_drop: spec.head,
    }
}

/// Barriga (elipse deitada, a base fica parada e o dorso respira) e pescoço até a cabeça.
fn barrel(p: &mut Pixmap, bob: f32, chest: f32) {
    let (white, shade) = (rgb(WHITE), rgb(SHADE));
    let radius = (14.0, 8.5 + chest);
    let center = (30.0, 51.0 + bob - radius.1);
    let neck = ((40.0, 37.0 + bob), (6.5, 8.0));
    for (x, y) in p.coords() {
        let (fx, fy) = (f32::from(x) + 0.5, f32::from(y) + 0.5);
        let in_barrel = ((fx - center.0) / radius.0).hypot((fy - center.1) / radius.1) <= 1.0;
        let in_neck = ((fx - neck.0.0) / neck.1.0).hypot((fy - neck.0.1) / neck.1.1) <= 1.0;
        if in_barrel || in_neck {
            let belly = fy > radius.1.mul_add(0.55, center.1);
            p.set(x, y, if belly { shade } else { white });
        }
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
            head.set(24, 28, rgb(WHITE));
            head.set(25, 27, rgb(WHITE));
        }
        Expression::Joy => {
            close_eye(&mut head, &HAPPY_ARC);
            open_mouth(&mut head);
            for x in 24..29 {
                head.set(x, 31, rgb(BLUSH));
            }
        }
    }
    head
}

/// Olho fechado relaxado (‿), para piscar.
const BLINK_LINE: [(u16, u16); 6] = [(23, 27), (24, 28), (25, 28), (26, 28), (27, 28), (28, 27)];
/// Olho fechado de alegria (^).
const HAPPY_ARC: [(u16, u16); 6] = [(23, 28), (24, 27), (25, 26), (26, 26), (27, 27), (28, 28)];

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
        head.set(x, 33, ink);
    }
    head.set(30, 34, ink);
    head.set(31, 34, rgb(MOUTH));
    head.set(32, 34, rgb(MOUTH));
    head.set(33, 34, ink);
    head.set(31, 35, ink);
    head.set(32, 35, ink);
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

/// Uma mecha: curva de Bézier cúbica com largura que afina, e um cacho na ponta.
#[derive(Debug, Clone, Copy)]
struct Lock {
    curve: [(f32, f32); 4],
    /// Largura no começo e na ponta.
    width: (f32, f32),
    /// Raio do cacho na ponta; o sinal escolhe o sentido do giro.
    curl: f32,
}

/// Pontos (x, y, raio) ao longo da mecha, incluindo o cacho.
fn lock_points(l: &Lock) -> Vec<(f32, f32, f32)> {
    let mut points: Vec<(f32, f32, f32)> = (0..=40_u8)
        .map(|i| {
            let t = f32::from(i) / 40.0;
            let (x, y) = bezier(&l.curve, t);
            (x, y, (l.width.1 - l.width.0).mul_add(t, l.width.0) / 2.0)
        })
        .collect();
    let (Some(&(end_x, end_y, end_r)), Some(&(prev_x, prev_y, _))) =
        (points.last(), points.iter().rev().nth(1))
    else {
        return points;
    };
    let turn = l.curl.signum();
    let heading = turn.mul_add(
        std::f32::consts::FRAC_PI_2,
        (end_y - prev_y).atan2(end_x - prev_x),
    );
    let reach = l.curl.abs();
    let center = (
        reach.mul_add(heading.cos(), end_x),
        reach.mul_add(heading.sin(), end_y),
    );
    let start = (end_y - center.1).atan2(end_x - center.0);
    points.extend((1..=20_u8).map(|k| {
        let k = f32::from(k);
        let angle = (turn * k / 20.0).mul_add(std::f32::consts::PI * 1.4, start);
        let radius = reach * (1.0 - k / 40.0);
        let spiral = (
            radius.mul_add(angle.cos(), center.0),
            radius.mul_add(angle.sin(), center.1),
        );
        (spiral.0, spiral.1, (end_r * (1.0 - k / 30.0)).max(0.9))
    }));
    points
}

fn bezier(c: &[(f32, f32); 4], t: f32) -> (f32, f32) {
    let u = 1.0 - t;
    let w = [u * u * u, 3.0 * u * u * t, 3.0 * u * t * t, t * t * t];
    c.iter().zip(w).fold((0.0, 0.0), |(x, y), (p, k)| {
        (p.0.mul_add(k, x), p.1.mul_add(k, y))
    })
}

/// Desenha uma mecha com fio escuro, brilho e a espiral do cacho.
fn draw_lock(p: &mut Pixmap, l: &Lock) {
    let points = lock_points(l);
    for &(x, y, r) in &points {
        disc(p, (x, y), r, rgb(MANE));
    }
    let body = points.get(..41).unwrap_or_default();
    for pair in body.windows(2).skip(4) {
        if let [(px, py, _), (x, y, r)] = pair {
            let normal = (y - py).atan2(x - px) + std::f32::consts::FRAC_PI_2;
            p.plot(
                (r * 0.35).mul_add(normal.cos(), *x),
                (r * 0.35).mul_add(normal.sin(), *y),
                rgb(MANE_DARK),
            );
        }
    }
    for pair in body.windows(2).skip(2).take(16) {
        if let [(px, py, _), (x, y, r)] = pair {
            let normal = (y - py).atan2(x - px) + std::f32::consts::FRAC_PI_2;
            p.plot(
                (r * -0.5).mul_add(normal.cos(), *x),
                (r * -0.5).mul_add(normal.sin(), *y),
                rgb(MANE_LIGHT),
            );
        }
    }
    for &(x, y, _) in points.iter().skip(45) {
        p.plot(x, y, rgb(MANE_DARK));
    }
}

/// Cauda de pônei: nasce no alto da garupa, arqueia para trás e cai em duas mechas
/// cacheadas até perto do chão. `sway` (-1 a 1) balança as pontas.
fn tail(p: &mut Pixmap, root: (f32, f32), sway: f32) {
    let (x, y) = root;
    let swing = |k: f32| sway * k;
    let back = Lock {
        curve: [
            (x + 1.0, y + 2.0),
            (x - 7.0 + swing(1.0), y + 1.0),
            (x - 9.0 + swing(2.0), y + 10.0),
            (x - 4.0 + swing(2.5), y + 16.0),
        ],
        width: (5.0, 3.0),
        curl: -2.2,
    };
    let main = Lock {
        curve: [
            (x + 1.0, y),
            (x - 9.0 + swing(1.0), y - 5.0),
            (x - 14.0 + swing(2.0), y + 6.0),
            (x - 9.0 + swing(3.0), y + 15.0),
        ],
        width: (7.0, 4.0),
        curl: 2.6,
    };
    draw_lock(p, &back);
    draw_lock(p, &main);
}

fn disc(p: &mut Pixmap, (cx, cy): (f32, f32), r: f32, color: Rgba) {
    for (x, y) in p.coords() {
        if (f32::from(x) + 0.5 - cx).hypot(f32::from(y) + 0.5 - cy) <= r {
            p.set(x, y, color);
        }
    }
}

/// Cabeça olhando para a direita: crina farta e cacheada, com franja curva até a testa e o
/// chifre saindo do meio do cabelo. 40x40.
#[rustfmt::skip]
pub const HEAD: &[&str] = &[
    "........................................",
    "........................Y...............",
    ".......................Y................",
    ".......................yD...............",
    "......................YD................",
    "......................YyD...............",
    ".............WW......yYD................",
    ".............WW......YYyD...............",
    ".............WWMMMMMYyYD................",
    "............WLLWMMLLyYYyD...............",
    "..........MMWLLWMMMYYyYDM...............",
    ".........MMWWLLWWMMYyYYDLLM.............",
    "......MMMMMWWLLWWMMMMmmmMLLM............",
    "....MMMMMMWWWLLWWLMMMMMmmmMMM...........",
    "...MmmmmMMMMMMMMMLLMMMMMMmmMM...........",
    "...mmMmmMMMMMMMmmmLLMMMMMMmMMM..........",
    "..MmMLmMMLLMMMMMMmmLMMMMMMmmMM..........",
    "..mmLmMMLLMmMMMMMMmMLMMMMMMmMMM.........",
    ".MmMmmMLLMmmMLLMMMMmMMMMMMMmMMmm........",
    ".MmMmMMLMMmMMLMMMMMmMMMMMMMmMMMm........",
    ".MmMmMLMMMmMLLMMMMMmMMMMMMMMmmmm........",
    ".MmMmMLMMmmMLMMMMmMmMMMMMMMMMmmW........",
    ".MmmMMLMMmMLLMMMmmMmMMMMMMWWWWWW........",
    "..mmMLMMMmMLMMMMmmmmMMWWEEEEWWWW........",
    "..mmMMMMMmMLMWWWWmmMWWWEEEEEEWWW........",
    ".MMmmMMMMmMLMWWWWWWWWWWEIIWWEWWWW.......",
    ".mMmmMMMMmMLMWWWWWWWWWWEIIWIEWWWWWW.....",
    ".mmmmMMMMmMMMWWWWWWWWWWEIIIIEWWWWWWW....",
    ".mmmmMMMMmMMMWWWWWWWWWWEiiiIEWWWWWWW....",
    "....MmMMMmmMMWWWWWWWWWWEiiiiEWWWWWooW...",
    "....MmMMMMmMMMmmmWWWWWWWEEEEWWWWWWWWW...",
    "...mmMmMMMmmMMMMmMWWWWWWWWWWWWWWWWWWW...",
    "...mMMmMM.MmmMMMmMWWWWWWWCCCWWWWWWWWW...",
    "...mmmmMM..MmMmmmWWWWWWWWWWWWWoWWoWW....",
    "...MmmmM....MmmmMWWWWWWWWWWWWWWooWWW....",
    ".....M.......WWWWWWWWWWWWWWWWWWWWWW.....",
    "...............WWWWWWWWWW..WWWWWW.......",
    "........................................",
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

    /// Primeiro par de itens iguais, se houver.
    fn repeated<T: PartialEq>(items: &[T]) -> Option<(usize, usize)> {
        (0..items.len())
            .flat_map(|i| (i + 1..items.len()).map(move |j| (i, j)))
            .find(|&(i, j)| items[i] == items[j])
    }

    fn count(p: &Pixmap, color: u32) -> usize {
        p.coords()
            .filter(|(x, y)| p.get(*x, *y) == Some(rgb(color)))
            .count()
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
    fn as_quatro_patas_aparecem_parado() {
        for (i, f) in body_frames().iter().enumerate().take(8) {
            assert!(count(&f.body, HOOF) >= 10, "patas de cá no quadro {i}");
            assert!(count(&f.body, HOOF_FAR) >= 6, "patas de lá no quadro {i}");
        }
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
    fn cascos_tocam_a_linha_do_chao_em_todo_quadro_no_chao() {
        let ground = 60_u16;
        let solid = |c: [u8; 4]| c[3] > 0 && c != rgb(OUTLINE);
        let frames = composed();
        for i in (0..18).chain([20]) {
            let f = &frames[i];
            let touching = (0..FRAME_W).any(|x| f.get(x, ground - 1).is_some_and(solid));
            assert!(touching, "quadro {i} flutuando");
            let sinking = (0..FRAME_W).any(|x| f.get(x, ground).is_some_and(solid));
            assert!(!sinking, "quadro {i} afundando no chão");
        }
    }

    #[test]
    fn trote_move_patas_em_pares_diagonais() {
        let specs = specs();
        for s in &specs[12..18] {
            assert!(
                (s.legs[0] - s.legs[3]).abs() < 1e-6,
                "dianteira de cá com traseira de lá"
            );
            assert!(
                (s.legs[1] - s.legs[2]).abs() < 1e-6,
                "dianteira de lá com traseira de cá"
            );
        }
    }
}

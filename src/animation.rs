//! Animação do pônei-unicórnio: escolhe a pose do corpo e a expressão do rosto a partir do
//! movimento (puro: testável sem janela).
//!
//! Corpo e rosto são independentes — qualquer expressão combina com qualquer pose — e o
//! desenho monta as camadas na hora.

/// Quadros da caminhada e duração de cada um na velocidade máxima.
pub const WALK_FRAMES: u8 = 6;
const WALK_FRAME_TIME: f32 = 0.085;
/// Quadros da respiração (parado) e duração de cada um.
pub const IDLE_FRAMES: u8 = 8;
const IDLE_FRAME_TIME: f32 = 0.2;
/// Quadros da patinha e duração de cada um.
pub const PAW_FRAMES: u8 = 4;
const PAW_FRAME_TIME: f32 = 0.18;
/// Parado há este tempo, o pônei bate a patinha no chão de vez em quando.
const PAW_AFTER: f32 = 3.0;
const PAW_EVERY: f32 = 6.0;
const PAW_FOR: f32 = 1.44;
/// Intervalo entre piscadas e duração da piscada.
const BLINK_EVERY: f32 = 3.2;
const BLINK_FOR: f32 = 0.16;
/// Duração da carinha de alegria ao pegar um coração.
const JOY_TIME: f32 = 0.7;
/// Agachadinho ao aterrissar: duração e velocidade mínima de queda para acontecer.
const LAND_TIME: f32 = 0.12;
const LAND_IMPACT: f32 = 250.0;
/// Abaixo desta velocidade o unicórnio é considerado parado.
pub const WALK_THRESHOLD: f32 = 13.0;

/// Pose do corpo.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pose {
    /// Parado, respirando (quadro 0 a 7).
    Idle(u8),
    /// Batendo a patinha no chão (quadro 0 a 3).
    Paw(u8),
    Walk(u8),
    Jump,
    Fall,
    /// Agachadinho logo após aterrissar.
    Land,
}

impl Pose {
    /// Quantidade de quadros de corpo.
    pub const COUNT: usize = 21;

    /// Índice do quadro na folha de sprites.
    #[must_use]
    pub fn index(self) -> usize {
        match self {
            Self::Idle(f) => usize::from(f % IDLE_FRAMES),
            Self::Paw(f) => 8 + usize::from(f % PAW_FRAMES),
            Self::Walk(f) => 12 + usize::from(f % WALK_FRAMES),
            Self::Jump => 18,
            Self::Fall => 19,
            Self::Land => 20,
        }
    }
}

/// Expressão do rosto.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Expression {
    /// Sorriso tranquilo.
    Calm,
    Blink,
    /// Boca aberta, sorrindo (andando, batendo a patinha).
    Smile,
    /// Olhos brilhando (no ar).
    Excited,
    /// Olhinhos "^^" e bochechas coradas (pegou um coração).
    Joy,
}

impl Expression {
    /// Quantidade de expressões.
    pub const COUNT: usize = 5;

    /// Índice da cabeça na folha de sprites.
    #[must_use]
    pub const fn index(self) -> usize {
        match self {
            Self::Calm => 0,
            Self::Blink => 1,
            Self::Smile => 2,
            Self::Excited => 3,
            Self::Joy => 4,
        }
    }
}

/// O que a animação precisa saber do movimento.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Motion {
    pub airborne: bool,
    /// Subindo (no ar).
    pub rising: bool,
    /// Velocidade horizontal, em módulo.
    pub speed: f32,
}

impl Motion {
    fn walking(self) -> bool {
        !self.airborne && self.speed > WALK_THRESHOLD
    }
}

/// Relógios da animação.
#[derive(Debug, Clone, Default)]
pub struct Animator {
    walk_frame: u8,
    walk_clock: f32,
    idle_clock: f32,
    joy: f32,
    landing: f32,
}

impl Animator {
    /// Avança os relógios (a caminhada acompanha a velocidade).
    pub fn tick(&mut self, motion: Motion, dt: f32) {
        self.joy = (self.joy - dt).max(0.0);
        self.landing = (self.landing - dt).max(0.0);
        if !motion.walking() {
            self.walk_clock = 0.0;
            self.idle_clock = if motion.airborne {
                0.0
            } else {
                self.idle_clock + dt
            };
            return;
        }
        self.idle_clock = 0.0;
        self.walk_clock = dt.mul_add(motion.speed / crate::player::MAX_SPEED, self.walk_clock);
        if self.walk_clock >= WALK_FRAME_TIME {
            self.walk_clock -= WALK_FRAME_TIME;
            self.walk_frame = (self.walk_frame + 1) % WALK_FRAMES;
        }
    }

    /// Avisa que aterrissou caindo a `impact` px/s: quedas fortes dão um agachadinho.
    pub const fn landed(&mut self, impact: f32) {
        if impact >= LAND_IMPACT {
            self.landing = LAND_TIME;
        }
    }

    /// Avisa que pegou um coração: carinha de alegria por um instante.
    pub const fn cheer(&mut self) {
        self.joy = JOY_TIME;
    }

    /// Pose do corpo para o movimento atual.
    #[must_use]
    pub fn pose(&self, motion: Motion) -> Pose {
        if motion.airborne {
            return if motion.rising {
                Pose::Jump
            } else {
                Pose::Fall
            };
        }
        if self.landing > 0.0 {
            return Pose::Land;
        }
        if motion.walking() {
            return Pose::Walk(self.walk_frame);
        }
        self.pawing().map_or_else(
            || Pose::Idle(frame_at(self.idle_clock, IDLE_FRAME_TIME, IDLE_FRAMES)),
            |t| Pose::Paw(frame_at(t, PAW_FRAME_TIME, PAW_FRAMES)),
        )
    }

    /// Expressão do rosto para o movimento atual.
    #[must_use]
    pub fn expression(&self, motion: Motion) -> Expression {
        if self.joy > 0.0 {
            Expression::Joy
        } else if motion.airborne {
            Expression::Excited
        } else if motion.walking() || self.pawing().is_some() {
            Expression::Smile
        } else if self.idle_clock % BLINK_EVERY > BLINK_EVERY - BLINK_FOR {
            Expression::Blink
        } else {
            Expression::Calm
        }
    }

    /// Tempo dentro da batida de patinha atual, se estiver batendo.
    fn pawing(&self) -> Option<f32> {
        let since = self.idle_clock - PAW_AFTER;
        let t = since.rem_euclid(PAW_EVERY);
        (since >= 0.0 && t < PAW_FOR).then_some(t)
    }
}

/// Quadro de uma animação em laço de `frames` quadros de `frame_time` segundos.
fn frame_at(clock: f32, frame_time: f32, frames: u8) -> u8 {
    let phase = clock.rem_euclid(frame_time * f32::from(frames));
    (1..frames)
        .take_while(|k| phase >= frame_time * f32::from(*k))
        .last()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const STILL: Motion = Motion {
        airborne: false,
        rising: false,
        speed: 0.0,
    };
    const WALKING: Motion = Motion {
        speed: 185.0,
        ..STILL
    };
    const RISING: Motion = Motion {
        airborne: true,
        rising: true,
        speed: 0.0,
    };

    fn run(a: &mut Animator, motion: Motion, seconds: f32) -> Vec<(Pose, Expression)> {
        let dt = 1.0 / 120.0;
        std::iter::successors(Some(0.0_f32), |t| Some(t + dt))
            .take_while(|t| *t < seconds)
            .map(|_| {
                a.tick(motion, dt);
                (a.pose(motion), a.expression(motion))
            })
            .collect()
    }

    #[test]
    fn indices_de_pose_sao_unicos_e_cabem_na_folha() {
        let mut poses: Vec<Pose> = (0..IDLE_FRAMES).map(Pose::Idle).collect();
        poses.extend((0..PAW_FRAMES).map(Pose::Paw));
        poses.extend((0..WALK_FRAMES).map(Pose::Walk));
        poses.extend([Pose::Jump, Pose::Fall, Pose::Land]);
        let mut seen: Vec<usize> = poses.iter().map(|p| p.index()).collect();
        seen.sort_unstable();
        seen.dedup();
        assert_eq!(seen, (0..Pose::COUNT).collect::<Vec<_>>());
    }

    #[test]
    fn indices_de_expressao_sao_unicos() {
        let all = [
            Expression::Calm,
            Expression::Blink,
            Expression::Smile,
            Expression::Excited,
            Expression::Joy,
        ];
        let mut seen: Vec<usize> = all.iter().map(|e| e.index()).collect();
        seen.sort_unstable();
        seen.dedup();
        assert_eq!(seen, (0..Expression::COUNT).collect::<Vec<_>>());
    }

    #[test]
    fn parado_respira_em_ciclo_suave() {
        let mut a = Animator::default();
        let mut frames: Vec<u8> = run(&mut a, STILL, IDLE_FRAME_TIME.mul_add(8.0, -0.05))
            .iter()
            .filter_map(|(p, _)| {
                if let Pose::Idle(f) = p {
                    Some(*f)
                } else {
                    None
                }
            })
            .collect();
        frames.dedup();
        assert_eq!(frames, (0..IDLE_FRAMES).collect::<Vec<_>>());
    }

    #[test]
    fn parado_por_um_tempo_bate_a_patinha_sorrindo_e_volta_a_respirar() {
        let mut a = Animator::default();
        let states = run(&mut a, STILL, PAW_AFTER + PAW_EVERY - 0.5);
        let pawing: Vec<&(Pose, Expression)> = states
            .iter()
            .filter(|(p, _)| matches!(p, Pose::Paw(_)))
            .collect();
        assert!(!pawing.is_empty(), "deveria bater a patinha");
        assert!(pawing.iter().all(|(_, e)| *e == Expression::Smile));
        let mut frames: Vec<Pose> = pawing.iter().map(|(p, _)| *p).collect();
        frames.dedup();
        assert!(frames.len() >= usize::from(PAW_FRAMES));
        assert!(matches!(states.last(), Some((Pose::Idle(_), _))));
    }

    #[test]
    fn pisca_de_vez_em_quando_quando_parado() {
        let mut a = Animator::default();
        // A primeira piscada cai durante a patinha (o sorriso vence); a segunda aparece.
        let states = run(&mut a, STILL, BLINK_EVERY.mul_add(2.0, 0.1));
        assert!(states.iter().any(|(_, e)| *e == Expression::Blink));
        assert!(
            states
                .iter()
                .filter(|(_, e)| *e == Expression::Calm)
                .count()
                > 300
        );
    }

    #[test]
    fn caminhada_passa_por_todos_os_quadros_sorrindo() {
        let mut a = Animator::default();
        let states = run(&mut a, WALKING, 1.0);
        let mut frames: Vec<u8> = states
            .iter()
            .filter_map(|(p, _)| {
                if let Pose::Walk(f) = p {
                    Some(*f)
                } else {
                    None
                }
            })
            .collect();
        frames.sort_unstable();
        frames.dedup();
        assert_eq!(frames, (0..WALK_FRAMES).collect::<Vec<_>>());
        assert!(states.iter().all(|(_, e)| *e == Expression::Smile));
    }

    #[test]
    fn no_ar_fica_empolgado_e_alterna_pulo_e_queda() {
        let a = Animator::default();
        assert_eq!(a.pose(RISING), Pose::Jump);
        assert_eq!(a.expression(RISING), Expression::Excited);
        let falling = Motion {
            rising: false,
            ..RISING
        };
        assert_eq!(a.pose(falling), Pose::Fall);
    }

    #[test]
    fn queda_forte_da_agachadinho_rapido() {
        let mut a = Animator::default();
        a.landed(LAND_IMPACT - 1.0);
        assert_ne!(a.pose(STILL), Pose::Land, "queda fraca não agacha");
        a.landed(LAND_IMPACT + 100.0);
        assert_eq!(a.pose(STILL), Pose::Land);
        run(&mut a, STILL, LAND_TIME + 0.01);
        assert_ne!(a.pose(STILL), Pose::Land);
    }

    #[test]
    fn coracao_da_carinha_de_alegria_que_passa() {
        let mut a = Animator::default();
        a.cheer();
        assert_eq!(a.expression(WALKING), Expression::Joy);
        assert_eq!(a.expression(RISING), Expression::Joy);
        run(&mut a, WALKING, JOY_TIME + 0.01);
        assert_eq!(a.expression(WALKING), Expression::Smile);
    }

    #[test]
    fn quadro_em_laco() {
        assert_eq!(frame_at(0.0, 0.2, 8), 0);
        assert_eq!(frame_at(0.25, 0.2, 8), 1);
        assert_eq!(frame_at(1.59, 0.2, 8), 7);
        assert_eq!(frame_at(1.61, 0.2, 8), 0);
    }
}

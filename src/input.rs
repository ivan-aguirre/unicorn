//! Estado dos controles em um quadro, independente da biblioteca de janela (testável).

/// Estado de uma tecla de ação.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Button {
    /// Foi apertada neste quadro (borda de subida).
    pub pressed: bool,
    /// Está sendo segurada.
    pub held: bool,
}

impl Button {
    /// Tecla apertada agora (e, portanto, segurada).
    pub const TAPPED: Self = Self {
        pressed: true,
        held: true,
    };
}

/// Controles do jogo: setas para andar, espaço para pular e Enter para pausar.
///
/// Segurar espaço continua pulando ao tocar o chão (mais fácil para crianças pequenas).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Input {
    pub left: bool,
    pub right: bool,
    pub jump: Button,
    pub pause: Button,
}

impl Input {
    /// Só a seta para a direita.
    pub const RIGHT: Self = Self {
        left: false,
        right: true,
        jump: Button {
            pressed: false,
            held: false,
        },
        pause: Button {
            pressed: false,
            held: false,
        },
    };
    /// Só a seta para a esquerda.
    pub const LEFT: Self = Self {
        left: true,
        right: false,
        ..Self::RIGHT
    };

    /// Direção pedida: -1 (esquerda), 0 (parado ou as duas setas) ou 1 (direita).
    #[must_use]
    pub const fn axis(self) -> f32 {
        match (self.left, self.right) {
            (true, false) => -1.0,
            (false, true) => 1.0,
            _ => 0.0,
        }
    }

    /// O mesmo estado, sem as bordas de subida (que só valem uma vez).
    #[must_use]
    pub const fn without_edges(self) -> Self {
        Self {
            jump: Button {
                pressed: false,
                held: self.jump.held,
            },
            pause: Button {
                pressed: false,
                held: self.pause.held,
            },
            ..self
        }
    }
}

/// Guarda bordas de subida até que um passo de simulação as consuma.
///
/// Em monitores rápidos pode haver quadros sem nenhum passo fixo; sem isso,
/// um toque rápido em Enter ou espaço seria perdido.
#[derive(Debug, Default)]
pub struct InputLatch {
    jump: bool,
    pause: bool,
}

impl InputLatch {
    /// Registra o estado lido no quadro e devolve o que o próximo passo deve ver.
    pub const fn feed(&mut self, input: Input) -> Input {
        self.jump |= input.jump.pressed;
        self.pause |= input.pause.pressed;
        Input {
            jump: Button {
                pressed: self.jump,
                held: input.jump.held,
            },
            pause: Button {
                pressed: self.pause,
                held: input.pause.held,
            },
            ..input
        }
    }

    /// Marca as bordas como consumidas por um passo de simulação.
    pub const fn consume(&mut self) {
        self.jump = false;
        self.pause = false;
    }
}

/// Quanto tempo segurar Esc para sair: um toque acidental (de criança) não fecha o jogo.
pub const QUIT_HOLD: f32 = 1.5;

/// Conta quanto tempo Esc ficou segurado.
#[derive(Debug, Default)]
pub struct QuitHold {
    held_for: f32,
}

impl QuitHold {
    /// Atualiza com o estado da tecla. Devolve `true` quando é hora de sair.
    pub fn update(&mut self, held: bool, dt: f32) -> bool {
        self.held_for = if held {
            self.held_for + dt.max(0.0)
        } else {
            0.0
        };
        self.held_for >= QUIT_HOLD
    }

    /// Progresso até sair, de 0 a 1 (para desenhar a barra).
    #[must_use]
    pub fn progress(&self) -> f32 {
        (self.held_for / QUIT_HOLD).min(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eixo_segue_as_setas_e_se_anula_com_as_duas() {
        assert!(Input::default().axis().abs() < f32::EPSILON);
        assert!((Input::LEFT.axis() + 1.0).abs() < f32::EPSILON);
        assert!((Input::RIGHT.axis() - 1.0).abs() < f32::EPSILON);
        let both = Input {
            left: true,
            ..Input::RIGHT
        };
        assert!(both.axis().abs() < f32::EPSILON);
    }

    #[test]
    fn sem_bordas_mantem_teclas_seguradas() {
        let input = Input {
            jump: Button::TAPPED,
            pause: Button::TAPPED,
            ..Input::RIGHT
        };
        let held = input.without_edges();
        assert!(held.right && held.jump.held && held.pause.held);
        assert!(!held.jump.pressed && !held.pause.pressed);
    }

    #[test]
    fn trava_preserva_toque_ate_ser_consumido() {
        let mut latch = InputLatch::default();
        let tap = Input {
            pause: Button::TAPPED,
            ..Input::default()
        };
        // Quadro sem passo: o toque fica guardado.
        assert!(latch.feed(tap).pause.pressed);
        assert!(latch.feed(Input::default()).pause.pressed);
        latch.consume();
        assert!(!latch.feed(Input::default()).pause.pressed);
    }

    #[test]
    fn trava_nao_inventa_tecla_segurada() {
        let mut latch = InputLatch::default();
        let tap = Input {
            jump: Button::TAPPED,
            ..Input::default()
        };
        latch.feed(tap);
        let next = latch.feed(Input::default());
        assert!(next.jump.pressed && !next.jump.held);
    }

    #[test]
    fn sair_exige_segurar_esc() {
        let mut quit = QuitHold::default();
        for _ in 0..10 {
            assert!(!quit.update(true, 0.1));
        }
        assert!(quit.progress() > 0.6 && quit.progress() < 0.7);
        // Soltar zera: toques soltos nunca fecham o jogo.
        assert!(!quit.update(false, 0.1));
        assert!(quit.progress().abs() < f32::EPSILON);
        let mut closed = false;
        for _ in 0..16 {
            closed |= quit.update(true, 0.1);
        }
        assert!(closed);
        assert!((quit.progress() - 1.0).abs() < f32::EPSILON);
    }
}

//! Tela inicial: escolher entre passear (o jogo dos corações) e letras (digitar palavras).
//! Sem texto para ler: dois desenhos grandes, setas para escolher e Enter (ou espaço) para
//! começar.

/// Os modos do jogo.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice {
    Walk,
    Letters,
}

/// Teclas do menu neste quadro (bordas de subida).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MenuInput {
    pub left: bool,
    pub right: bool,
    pub confirm: bool,
}

/// Estado da tela inicial.
#[derive(Debug, Clone)]
pub struct Menu {
    pub choice: Choice,
    /// Tempo na tela (anima o cenário e o destaque).
    pub time: f32,
}

impl Menu {
    /// Menu com `choice` destacado.
    #[must_use]
    pub const fn new(choice: Choice) -> Self {
        Self { choice, time: 0.0 }
    }

    /// Avança `dt` segundos. Devolve o modo escolhido quando Enter ou espaço é apertado.
    pub fn update(&mut self, input: MenuInput, dt: f32) -> Option<Choice> {
        self.time += dt;
        if input.left {
            self.choice = Choice::Walk;
        }
        if input.right {
            self.choice = Choice::Letters;
        }
        input.confirm.then_some(self.choice)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setas_trocam_e_enter_confirma() {
        let mut m = Menu::new(Choice::Walk);
        let right = MenuInput {
            right: true,
            ..MenuInput::default()
        };
        assert_eq!(m.update(right, 0.1), None);
        assert_eq!(m.choice, Choice::Letters);
        let left = MenuInput {
            left: true,
            ..MenuInput::default()
        };
        m.update(left, 0.1);
        assert_eq!(m.choice, Choice::Walk);
        let confirm = MenuInput {
            confirm: true,
            ..MenuInput::default()
        };
        assert_eq!(m.update(confirm, 0.1), Some(Choice::Walk));
        assert!((m.time - 0.3).abs() < 1e-5);
    }

    #[test]
    fn sem_tecla_nada_muda() {
        let mut m = Menu::new(Choice::Letters);
        assert_eq!(m.update(MenuInput::default(), 0.1), None);
        assert_eq!(m.choice, Choice::Letters);
    }
}

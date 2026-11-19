//! Passo fixo de simulação: a física fica igual em qualquer taxa de quadros.

/// Duração de um passo de simulação (120 Hz).
pub const STEP: f32 = 1.0 / 120.0;
/// Máximo de passos por quadro: evita a "espiral da morte" após um travamento longo.
pub const MAX_STEPS: u8 = 8;

/// Acumula o tempo real e informa quantos passos fixos executar.
#[derive(Debug, Default)]
pub struct FixedStep {
    accumulator: f32,
}

impl FixedStep {
    /// Soma o tempo do quadro e devolve quantos passos de [`STEP`] rodar agora.
    pub fn advance(&mut self, frame_dt: f32) -> u8 {
        let budget = STEP * f32::from(MAX_STEPS);
        self.accumulator = (self.accumulator + frame_dt.max(0.0)).min(budget);
        let mut steps = 0;
        while self.accumulator >= STEP && steps < MAX_STEPS {
            self.accumulator -= STEP;
            steps += 1;
        }
        steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sessenta_quadros_por_segundo_dao_dois_passos_por_quadro() {
        let mut clock = FixedStep::default();
        let total: u32 = (0..60).map(|_| u32::from(clock.advance(1.0 / 60.0))).sum();
        assert!((119..=120).contains(&total), "total = {total}");
    }

    #[test]
    fn monitor_rapido_tem_quadros_sem_passo() {
        let mut clock = FixedStep::default();
        let steps: Vec<u8> = (0..4).map(|_| clock.advance(1.0 / 240.0)).collect();
        assert!(steps.contains(&0));
        assert_eq!(steps.iter().map(|s| u32::from(*s)).sum::<u32>(), 2);
    }

    #[test]
    fn travamento_longo_e_limitado() {
        let mut clock = FixedStep::default();
        assert_eq!(clock.advance(5.0), MAX_STEPS);
        assert_eq!(clock.advance(0.0), 0);
    }

    #[test]
    fn tempo_negativo_e_ignorado() {
        let mut clock = FixedStep::default();
        assert_eq!(clock.advance(-1.0), 0);
    }
}

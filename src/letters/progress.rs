//! Progressão das letras: vogais, consoantes frequentes, o resto do alfabeto e, por fim,
//! palavras.
//!
//! Cada letra precisa de alguns acertos para liberar a próxima leva; dentro da leva, as letras
//! com menos acertos aparecem mais.

use fastrand::Rng;

/// Levas de letras, na ordem em que são liberadas.
pub const SETS: [&str; 3] = ["AEIOU", "BCDLMNPSTV", "FGHJKQRXZWY"];
/// Acertos de cada letra da leva para liberar a próxima.
pub const HITS_TO_ADVANCE: u8 = 3;
/// A partir de quantos acertos uma letra passa a ter o peso mínimo no sorteio.
const WEIGHT_CAP: u8 = 6;

/// Em que fase a criança está.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    /// Letras soltas da leva `0..SETS.len()` (e das anteriores).
    Letters(usize),
    /// Palavras inteiras, letra por letra.
    Words,
}

/// Acertos por letra e a fase atual.
#[derive(Debug, Clone, Default)]
pub struct Progress {
    hits: [u8; 26],
    level: usize,
}

impl Progress {
    /// Fase atual.
    #[must_use]
    pub const fn stage(&self) -> Stage {
        if self.level < SETS.len() {
            Stage::Letters(self.level)
        } else {
            Stage::Words
        }
    }

    /// Acertos da letra `c` (0 para o que não é letra de A a Z).
    #[must_use]
    pub fn hits(&self, c: char) -> u8 {
        slot(c).and_then(|i| self.hits.get(i)).copied().unwrap_or(0)
    }

    /// Letras já liberadas.
    pub fn unlocked(&self) -> impl Iterator<Item = char> + '_ {
        SETS.iter()
            .take(self.level.min(SETS.len() - 1) + 1)
            .flat_map(|s| s.chars())
    }

    /// Conta um acerto e libera a próxima fase quando a leva atual estiver completa.
    pub fn hit(&mut self, c: char) {
        if let Some(h) = slot(c).and_then(|i| self.hits.get_mut(i)) {
            *h = h.saturating_add(1);
        }
        let done = SETS
            .get(self.level)
            .is_some_and(|set| set.chars().all(|l| self.hits(l) >= HITS_TO_ADVANCE));
        if done {
            self.level += 1;
        }
    }

    /// Sorteia a próxima letra entre as liberadas, preferindo as com menos acertos e evitando
    /// repetir `previous`.
    #[must_use]
    pub fn pick(&self, rng: &mut Rng, previous: Option<char>) -> char {
        let candidates: Vec<(char, u32)> = self
            .unlocked()
            .filter(|c| Some(*c) != previous)
            .map(|c| (c, weight(self.hits(c))))
            .collect();
        let total: u32 = candidates.iter().map(|(_, w)| w).sum();
        let mut roll = rng.u32(..total.max(1));
        for (c, w) in &candidates {
            if roll < *w {
                return *c;
            }
            roll -= w;
        }
        previous.unwrap_or('A')
    }
}

/// Peso no sorteio: quanto menos acertos, mais chance.
fn weight(hits: u8) -> u32 {
    u32::from(WEIGHT_CAP - hits.min(WEIGHT_CAP)) + 1
}

/// Posição da letra de A a Z.
fn slot(c: char) -> Option<usize> {
    c.is_ascii_uppercase()
        .then(|| usize::from(u8::try_from(c).unwrap_or(b'A') - b'A'))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn complete(p: &mut Progress, set: &str) {
        for c in set.chars() {
            for _ in 0..HITS_TO_ADVANCE {
                p.hit(c);
            }
        }
    }

    #[test]
    fn levas_cobrem_o_alfabeto_sem_repetir() {
        let mut all: Vec<char> = SETS.iter().flat_map(|s| s.chars()).collect();
        all.sort_unstable();
        assert_eq!(all, ('A'..='Z').collect::<Vec<_>>());
    }

    #[test]
    fn comeca_pelas_vogais() {
        let p = Progress::default();
        assert_eq!(p.stage(), Stage::Letters(0));
        assert_eq!(p.unlocked().collect::<String>(), "AEIOU");
    }

    #[test]
    fn libera_as_fases_em_ordem() {
        let mut p = Progress::default();
        for (i, set) in SETS.iter().enumerate() {
            assert_eq!(p.stage(), Stage::Letters(i));
            complete(&mut p, set);
        }
        assert_eq!(p.stage(), Stage::Words);
        assert_eq!(p.unlocked().count(), 26);
    }

    #[test]
    fn falta_um_acerto_nao_avanca() {
        let mut p = Progress::default();
        complete(&mut p, "AEIO");
        p.hit('U');
        p.hit('U');
        assert_eq!(p.stage(), Stage::Letters(0));
        p.hit('U');
        assert_eq!(p.stage(), Stage::Letters(1));
    }

    #[test]
    fn letras_de_outras_levas_e_simbolos_nao_contam() {
        let mut p = Progress::default();
        for _ in 0..10 {
            p.hit('Z');
            p.hit('Ç');
            p.hit('1');
        }
        assert_eq!(p.stage(), Stage::Letters(0));
        assert_eq!(p.hits('Ç'), 0);
        assert_eq!(p.hits('Z'), 10);
    }

    #[test]
    fn sorteio_so_usa_liberadas_e_nao_repete_a_anterior() {
        let p = Progress::default();
        let mut rng = Rng::with_seed(1);
        for _ in 0..200 {
            let c = p.pick(&mut rng, Some('A'));
            assert!("EIOU".contains(c), "{c}");
        }
    }

    #[test]
    fn sorteio_prefere_letras_com_menos_acertos() {
        // A, E, I e O já bem treinadas (sem liberar a próxima leva: falta o U).
        let mut p = Progress::default();
        for c in "AEIO".chars() {
            for _ in 0..WEIGHT_CAP {
                p.hit(c);
            }
        }
        let mut rng = Rng::with_seed(2);
        let u = (0..1000).filter(|_| p.pick(&mut rng, None) == 'U').count();
        assert!(u > 400, "U saiu só {u} vezes em 1000");
    }
}

//! Partidas simuladas do modo Letras, sem janela: uma criança que às vezes erra a tecla, às
//! vezes demora, e mesmo assim avança das vogais até as palavras.

use fastrand::Rng;
use unicorn::clock::STEP;
use unicorn::effects::MAX_PARTICLES;
use unicorn::letters::progress::Stage;
use unicorn::letters::{LetterEvents, Letters, Press, STARS_PER_PARTY, keyboard};
use unicorn::level::{Level, SEED};

/// Quantos passos uma criança leva para achar a tecla e o que ela aperta antes.
struct Child {
    rng: Rng,
    /// Passos desde que a bolha atual apareceu (ou desde a última tecla).
    waited: u32,
    /// Quando vai apertar (sorteado a cada tentativa).
    after: u32,
}

impl Child {
    const fn new(seed: u64) -> Self {
        Self {
            rng: Rng::with_seed(seed),
            waited: 0,
            after: 60,
        }
    }

    /// O que a criança digita neste passo, vendo a letra da bolha (se houver).
    fn typing(&mut self, bubble: Option<char>) -> Vec<char> {
        let Some(letter) = bubble else {
            self.waited = 0;
            return Vec::new();
        };
        self.waited += 1;
        if self.waited < self.after {
            return Vec::new();
        }
        self.waited = 0;
        self.after = self.rng.u32(30..600);
        // Um terço das vezes aperta uma tecla errada; depois tenta de novo.
        if self.rng.u8(..3) == 0 {
            return vec![if letter == 'Q' { 'W' } else { 'Q' }];
        }
        vec![letter]
    }
}

/// Joga `seconds` segundos, checando invariantes, e devolve todos os eventos.
fn play(letters: &mut Letters, child: &mut Child, seconds: u32) -> Vec<LetterEvents> {
    let mut all = Vec::new();
    for _ in 0..seconds * 120 {
        let typed = child.typing(letters.bubble().map(|b| b.letter));
        let events = letters.step(STEP, &typed);
        assert!(letters.effects.particles().len() <= MAX_PARTICLES);
        if let Some(b) = letters.bubble() {
            assert!(
                b.center().x > 150.0 && b.center().x < 600.0,
                "bolha fora do lugar"
            );
        }
        all.push(events);
    }
    all
}

#[test]
fn das_vogais_ate_as_palavras_mesmo_errando() {
    let mut letters = Letters::new(Level::generate(SEED), Rng::with_seed(1));
    let mut child = Child::new(2);
    let events = play(&mut letters, &mut child, 60 * 25);
    let pops = events.iter().filter(|e| e.popped()).count();
    let misses = events
        .iter()
        .filter(|e| e.press == Some(Press::Missed))
        .count();
    let words = events.iter().filter(|e| e.word_done).count();
    let parties = events.iter().filter(|e| e.celebrated).count();
    assert_eq!(letters.progress.stage(), Stage::Words, "{pops} estouros");
    assert!(words >= 3, "completou só {words} palavras");
    assert!(misses > 10, "a criança simulada deveria errar às vezes");
    assert_eq!(u32::try_from(pops).unwrap(), letters.stars());
    assert_eq!(
        u32::try_from(parties).unwrap(),
        letters.stars() / STARS_PER_PARTY
    );
}

#[test]
fn sem_ninguem_jogar_as_bolhas_continuam_voltando() {
    let mut letters = Letters::new(Level::generate(SEED), Rng::with_seed(3));
    let mut seen = 0;
    let mut had_bubble = false;
    for _ in 0..120 * 120 {
        letters.step(STEP, &[]);
        let now = letters.bubble().is_some();
        seen += usize::from(now && !had_bubble);
        had_bubble = now;
    }
    assert!(seen >= 4, "a bolha apareceu só {seen} vezes em 2 minutos");
    assert_eq!(letters.stars(), 0);
    assert_eq!(letters.progress.stage(), Stage::Letters(0));
}

#[test]
fn letras_digitadas_sem_bolha_nao_contam_nem_quebram() {
    let mut letters = Letters::new(Level::generate(SEED), Rng::with_seed(4));
    let all: Vec<char> = keyboard::keys().map(|(c, _)| c).collect();
    for _ in 0..30 {
        let events = letters.step(STEP, &all);
        if letters.bubble().is_none() {
            assert_eq!(events.press, None);
        }
    }
    // Com tudo apertado ao mesmo tempo, a bolha que aparecer estoura: nada além disso.
    assert!(letters.stars() <= 1);
}

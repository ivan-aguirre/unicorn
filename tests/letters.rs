//! Partidas simuladas do modo Letras, sem janela: uma criança que às vezes erra a tecla, às
//! vezes demora, e mesmo assim completa palavras e ganha festas.

use fastrand::Rng;
use unicorn::clock::STEP;
use unicorn::effects::MAX_PARTICLES;
use unicorn::letters::{LetterEvents, Letters, Press, keyboard};
use unicorn::level::{Level, SEED};

/// Quantos passos uma criança leva para achar a tecla e o que ela aperta antes.
struct Child {
    rng: Rng,
    /// Passos desde que a letra atual apareceu (ou desde a última tecla).
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

    /// O que a criança digita neste passo, vendo a letra a digitar (se houver).
    fn typing(&mut self, target: Option<char>) -> Vec<char> {
        let Some(letter) = target else {
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
        let typed = child.typing(letters.target());
        let events = letters.step(STEP, &typed);
        assert!(letters.effects.particles().len() <= MAX_PARTICLES);
        assert!(letters.words.typed() <= letters.words.word().text.len());
        all.push(events);
    }
    all
}

#[test]
fn crianca_que_erra_e_demora_completa_varias_palavras() {
    let mut letters = Letters::new(Level::generate(SEED), Rng::with_seed(1));
    let mut child = Child::new(2);
    let events = play(&mut letters, &mut child, 60 * 5);
    let hits = events.iter().filter(|e| e.hit()).count();
    let misses = events
        .iter()
        .filter(|e| e.press == Some(Press::Missed))
        .count();
    let parties = events.iter().filter(|e| e.celebrated).count();
    assert!(parties >= 5, "completou só {parties} palavras");
    assert!(misses > 10, "a criança simulada deveria errar às vezes");
    assert_eq!(u32::try_from(parties).unwrap(), letters.stars());
    assert!(hits >= parties * 3, "cada palavra tem ao menos 3 letras");
}

#[test]
fn sem_ninguem_digitar_a_palavra_fica_esperando() {
    let mut letters = Letters::new(Level::generate(SEED), Rng::with_seed(5));
    let first = letters.words.word().text;
    let target = letters.target();
    for _ in 0..120 * 60 {
        let events = letters.step(STEP, &[]);
        assert_eq!(events, LetterEvents::default());
    }
    assert_eq!(letters.words.word().text, first);
    assert_eq!(letters.target(), target, "a mesma letra continua esperando");
    assert_eq!(letters.stars(), 0);
}

#[test]
fn todas_as_teclas_ao_mesmo_tempo_nao_quebram_o_jogo() {
    let mut letters = Letters::new(Level::generate(SEED), Rng::with_seed(4));
    let all: Vec<char> = keyboard::keys().map(|(c, _)| c).collect();
    for _ in 0..30 {
        letters.step(STEP, &all);
    }
    // Cada passo acerta no máximo uma letra (a da vez), então nada passa do tamanho da palavra.
    assert!(letters.stars() <= 30);
    assert!(letters.words.typed() <= letters.words.word().text.len());
}

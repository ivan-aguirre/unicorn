//! Palavras da última fase: cada uma com uma figura, digitadas letra por letra em ordem.

use fastrand::Rng;

/// Figura de cada palavra (desenhada em `art::sprites`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Picture {
    Ball,
    Duck,
    Cat,
    House,
    Frog,
    Moon,
    Grapes,
    Kite,
    Cake,
    Die,
    Banana,
    Tomato,
}

impl Picture {
    /// Todas as figuras, na ordem de [`Picture::index`].
    pub const ALL: [Self; 12] = [
        Self::Ball,
        Self::Duck,
        Self::Cat,
        Self::House,
        Self::Frog,
        Self::Moon,
        Self::Grapes,
        Self::Kite,
        Self::Cake,
        Self::Die,
        Self::Banana,
        Self::Tomato,
    ];

    /// Posição na lista de figuras da arte.
    #[must_use]
    pub const fn index(self) -> usize {
        match self {
            Self::Ball => 0,
            Self::Duck => 1,
            Self::Cat => 2,
            Self::House => 3,
            Self::Frog => 4,
            Self::Moon => 5,
            Self::Grapes => 6,
            Self::Kite => 7,
            Self::Cake => 8,
            Self::Die => 9,
            Self::Banana => 10,
            Self::Tomato => 11,
        }
    }
}

/// Uma palavra (maiúsculas de A a Z) e a figura dela.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Word {
    pub text: &'static str,
    pub picture: Picture,
}

/// Palavras curtas, sem acento e fáceis de desenhar.
pub const WORDS: [Word; 12] = [
    Word {
        text: "BOLA",
        picture: Picture::Ball,
    },
    Word {
        text: "PATO",
        picture: Picture::Duck,
    },
    Word {
        text: "GATO",
        picture: Picture::Cat,
    },
    Word {
        text: "CASA",
        picture: Picture::House,
    },
    Word {
        text: "SAPO",
        picture: Picture::Frog,
    },
    Word {
        text: "LUA",
        picture: Picture::Moon,
    },
    Word {
        text: "UVA",
        picture: Picture::Grapes,
    },
    Word {
        text: "PIPA",
        picture: Picture::Kite,
    },
    Word {
        text: "BOLO",
        picture: Picture::Cake,
    },
    Word {
        text: "DADO",
        picture: Picture::Die,
    },
    Word {
        text: "BANANA",
        picture: Picture::Banana,
    },
    Word {
        text: "TOMATE",
        picture: Picture::Tomato,
    },
];

/// Tempo em que a palavra completa fica na tela antes da próxima.
pub const SHOW_TIME: f32 = 2.5;

/// A palavra da vez e quantas letras dela já foram digitadas.
#[derive(Debug, Clone)]
pub struct WordTask {
    order: Vec<usize>,
    cursor: usize,
    typed: usize,
    /// Tempo restante mostrando a palavra completa (0 enquanto ela está sendo digitada).
    showing: f32,
}

impl WordTask {
    /// Começa por uma ordem embaralhada das palavras.
    #[must_use]
    pub fn new(rng: &mut Rng) -> Self {
        let mut order: Vec<usize> = (0..WORDS.len()).collect();
        rng.shuffle(&mut order);
        Self {
            order,
            cursor: 0,
            typed: 0,
            showing: 0.0,
        }
    }

    /// A palavra da vez.
    #[must_use]
    pub fn word(&self) -> &'static Word {
        let index = self.order.get(self.cursor).copied().unwrap_or(0);
        WORDS.get(index).unwrap_or(&WORDS[0])
    }

    /// Quantas letras já foram digitadas.
    #[must_use]
    pub const fn typed(&self) -> usize {
        self.typed
    }

    /// Próxima letra a digitar (nenhuma enquanto mostra a palavra completa).
    #[must_use]
    pub fn next_letter(&self) -> Option<char> {
        if self.showing > 0.0 {
            return None;
        }
        self.word().text.chars().nth(self.typed)
    }

    /// Tempo restante mostrando a palavra completa (0 enquanto é digitada).
    #[must_use]
    pub const fn showing(&self) -> f32 {
        self.showing
    }

    /// Conta a próxima letra como digitada. Devolve `true` se completou a palavra.
    pub fn advance_letter(&mut self) -> bool {
        self.typed += 1;
        let done = self.typed >= self.word().text.len();
        if done {
            self.showing = SHOW_TIME;
        }
        done
    }

    /// Passa o tempo; ao fim da exibição, vai para a próxima palavra.
    pub fn update(&mut self, dt: f32, rng: &mut Rng) {
        if self.showing <= 0.0 {
            return;
        }
        self.showing -= dt;
        if self.showing <= 0.0 {
            self.showing = 0.0;
            self.next_word(rng);
        }
    }

    /// Próxima palavra; ao fim da lista, embaralha de novo sem repetir a última.
    fn next_word(&mut self, rng: &mut Rng) {
        let last = self.order.get(self.cursor).copied();
        self.typed = 0;
        self.cursor += 1;
        if self.cursor >= self.order.len() {
            self.cursor = 0;
            rng.shuffle(&mut self.order);
            if self.order.first().copied() == last {
                self.order.rotate_left(1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn palavras_so_com_letras_de_a_a_z_e_figuras_unicas() {
        let mut pictures: Vec<usize> = WORDS.iter().map(|w| w.picture.index()).collect();
        pictures.sort_unstable();
        assert_eq!(pictures, (0..Picture::ALL.len()).collect::<Vec<_>>());
        for (i, p) in Picture::ALL.iter().enumerate() {
            assert_eq!(p.index(), i);
        }
        for w in WORDS {
            assert!((3..=6).contains(&w.text.len()), "{}", w.text);
            assert!(w.text.chars().all(|c| c.is_ascii_uppercase()), "{}", w.text);
        }
    }

    /// Digita a palavra da vez inteira e espera a próxima.
    fn type_word(task: &mut WordTask, rng: &mut Rng) -> &'static str {
        let word = task.word().text;
        for (i, c) in word.chars().enumerate() {
            assert_eq!(task.typed(), i);
            assert_eq!(task.next_letter(), Some(c));
            assert_eq!(task.advance_letter(), i + 1 == word.len());
        }
        assert_eq!(
            task.next_letter(),
            None,
            "sem letra enquanto mostra a palavra"
        );
        for _ in 0..30 {
            task.update(0.1, rng);
        }
        assert_eq!(task.typed(), 0);
        word
    }

    #[test]
    fn letras_em_ordem_e_depois_a_proxima_palavra() {
        let mut rng = Rng::with_seed(1);
        let mut task = WordTask::new(&mut rng);
        let first = type_word(&mut task, &mut rng);
        assert_ne!(task.word().text, first);
    }

    #[test]
    fn passa_por_todas_antes_de_repetir() {
        let mut rng = Rng::with_seed(3);
        let mut task = WordTask::new(&mut rng);
        let seen: Vec<&str> = (0..WORDS.len() * 2)
            .map(|_| type_word(&mut task, &mut rng))
            .collect();
        for round in seen.chunks(WORDS.len()) {
            let mut sorted = round.to_vec();
            sorted.sort_unstable();
            sorted.dedup();
            assert_eq!(sorted.len(), WORDS.len());
        }
        assert!(seen.windows(2).all(|w| w[0] != w[1]), "repetiu seguida");
    }
}

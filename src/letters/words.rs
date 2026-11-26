//! Palavras do modo Letras: a figura e a palavra aparecem, e a criança digita a mesma palavra,
//! letra por letra, em ordem. Com a palavra completa, tem festa.

use fastrand::Rng;

use crate::effects::CELEBRATION_TIME;

/// Figura de cada palavra (desenhada em `art::pictures`).
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
    Cup,
    Pear,
    Bell,
    Seal,
    Mouse,
    Window,
    Shoe,
    Popcorn,
    Cow,
    Monkey,
    Rooster,
    Giraffe,
    Toucan,
    Whale,
    Coconut,
    Lollipop,
    Soup,
    Suitcase,
    Table,
    Bed,
    Candle,
    Boot,
    Ship,
    Magnifier,
    Teapot,
    Doll,
    Fire,
    Mug,
}

impl Picture {
    /// Todas as figuras, na ordem de [`Picture::index`].
    pub const ALL: [Self; 40] = [
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
        Self::Cup,
        Self::Pear,
        Self::Bell,
        Self::Seal,
        Self::Mouse,
        Self::Window,
        Self::Shoe,
        Self::Popcorn,
        Self::Cow,
        Self::Monkey,
        Self::Rooster,
        Self::Giraffe,
        Self::Toucan,
        Self::Whale,
        Self::Coconut,
        Self::Lollipop,
        Self::Soup,
        Self::Suitcase,
        Self::Table,
        Self::Bed,
        Self::Candle,
        Self::Boot,
        Self::Ship,
        Self::Magnifier,
        Self::Teapot,
        Self::Doll,
        Self::Fire,
        Self::Mug,
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
            Self::Cup => 12,
            Self::Pear => 13,
            Self::Bell => 14,
            Self::Seal => 15,
            Self::Mouse => 16,
            Self::Window => 17,
            Self::Shoe => 18,
            Self::Popcorn => 19,
            Self::Cow => 20,
            Self::Monkey => 21,
            Self::Rooster => 22,
            Self::Giraffe => 23,
            Self::Toucan => 24,
            Self::Whale => 25,
            Self::Coconut => 26,
            Self::Lollipop => 27,
            Self::Soup => 28,
            Self::Suitcase => 29,
            Self::Table => 30,
            Self::Bed => 31,
            Self::Candle => 32,
            Self::Boot => 33,
            Self::Ship => 34,
            Self::Magnifier => 35,
            Self::Teapot => 36,
            Self::Doll => 37,
            Self::Fire => 38,
            Self::Mug => 39,
        }
    }
}

/// Uma palavra (maiúsculas de A a Z) e a figura dela.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Word {
    pub text: &'static str,
    pub picture: Picture,
}

/// Palavras para alfabetização, de figuras que uma criança pequena reconhece.
///
/// Sem acento e só com sílabas simples (consoante + vogal, ou vogal sozinha), sem dígrafos
/// (CH, LH, NH) nem encontros consonantais. Juntas, passam por todas as vogais e pelas consoantes mais usadas
/// no começo da leitura (B, C, D, F, G, J, L, M, N, P, R, S, T, V).
pub const WORDS: [Word; 40] = [
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
    Word {
        text: "COPO",
        picture: Picture::Cup,
    },
    Word {
        text: "PERA",
        picture: Picture::Pear,
    },
    Word {
        text: "SINO",
        picture: Picture::Bell,
    },
    Word {
        text: "FOCA",
        picture: Picture::Seal,
    },
    Word {
        text: "RATO",
        picture: Picture::Mouse,
    },
    Word {
        text: "JANELA",
        picture: Picture::Window,
    },
    Word {
        text: "SAPATO",
        picture: Picture::Shoe,
    },
    Word {
        text: "PIPOCA",
        picture: Picture::Popcorn,
    },
    Word {
        text: "VACA",
        picture: Picture::Cow,
    },
    Word {
        text: "MACACO",
        picture: Picture::Monkey,
    },
    Word {
        text: "GALO",
        picture: Picture::Rooster,
    },
    Word {
        text: "GIRAFA",
        picture: Picture::Giraffe,
    },
    Word {
        text: "TUCANO",
        picture: Picture::Toucan,
    },
    Word {
        text: "BALEIA",
        picture: Picture::Whale,
    },
    Word {
        text: "COCO",
        picture: Picture::Coconut,
    },
    Word {
        text: "PIRULITO",
        picture: Picture::Lollipop,
    },
    Word {
        text: "SOPA",
        picture: Picture::Soup,
    },
    Word {
        text: "MALA",
        picture: Picture::Suitcase,
    },
    Word {
        text: "MESA",
        picture: Picture::Table,
    },
    Word {
        text: "CAMA",
        picture: Picture::Bed,
    },
    Word {
        text: "VELA",
        picture: Picture::Candle,
    },
    Word {
        text: "BOTA",
        picture: Picture::Boot,
    },
    Word {
        text: "NAVIO",
        picture: Picture::Ship,
    },
    Word {
        text: "LUPA",
        picture: Picture::Magnifier,
    },
    Word {
        text: "BULE",
        picture: Picture::Teapot,
    },
    Word {
        text: "BONECA",
        picture: Picture::Doll,
    },
    Word {
        text: "FOGO",
        picture: Picture::Fire,
    },
    Word {
        text: "CANECA",
        picture: Picture::Mug,
    },
];

/// Tempo em que a palavra completa fica na tela, com a festa, antes da próxima.
pub const SHOW_TIME: f32 = CELEBRATION_TIME + 0.5;

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
            assert!((3..=8).contains(&w.text.len()), "{}", w.text);
            assert!(w.text.chars().all(|c| c.is_ascii_uppercase()), "{}", w.text);
        }
    }

    #[test]
    fn palavras_so_com_silabas_simples_e_sem_repetir() {
        let vowel = |c: char| "AEIOU".contains(c);
        for w in WORDS {
            let letters: Vec<char> = w.text.chars().collect();
            // Duas consoantes seguidas seriam dígrafo (CH, LH, NH) ou encontro (BR, PL).
            let pair = letters.windows(2).any(|p| !vowel(p[0]) && !vowel(p[1]));
            assert!(!pair, "{} tem consoantes seguidas", w.text);
            assert!(
                vowel(letters[letters.len() - 1]),
                "{} termina em consoante",
                w.text
            );
        }
        let mut texts: Vec<&str> = WORDS.iter().map(|w| w.text).collect();
        texts.sort_unstable();
        texts.dedup();
        assert_eq!(texts.len(), WORDS.len());
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
        for _ in 0..50 {
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

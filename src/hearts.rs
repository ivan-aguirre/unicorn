//! Corações para coletar: espalhados pelo nível, alguns no alto (só com pulo), e novos
//! corações surgem aleatoriamente fora da tela enquanto se joga.

use fastrand::Rng;
use macroquad::math::{Rect, Vec2};

use crate::level::scenery::range;
use crate::level::{self, Level};
use crate::view::VIEW_W;

/// Tamanho da caixa de colisão de um coração.
pub const SIZE: f32 = 18.0;
/// Altura do centro de um coração "no chão" (alcançável andando).
pub const LOW: f32 = 20.0;
/// Faixa de altura de um coração "no alto" (exige pulo), acima do chão.
pub const HIGH: (f32, f32) = (74.0, 96.0);
/// Distância entre corações na distribuição inicial.
const SPACING: (f32, f32) = (110.0, 250.0);
/// Chance (em %) de um coração ficar no alto.
const HIGH_CHANCE: u8 = 35;
/// Espera até um coração coletado reaparecer em outro lugar.
const RESPAWN_DELAY: (f32, f32) = (0.6, 2.5);
/// Distância além da borda da tela onde novos corações surgem.
const SPAWN_AHEAD: (f32, f32) = (40.0, 600.0);
/// Limite de corações vivos (memória e custo constantes).
pub const MAX_HEARTS: usize = 600;

/// Um coração no nível.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Heart {
    /// Centro.
    pub pos: Vec2,
    /// Fase da flutuação, para que não balancem todos juntos.
    pub phase: f32,
    /// Tempo desde que surgiu (para o brilho de aparecimento).
    pub age: f32,
}

impl Heart {
    /// Caixa de colisão.
    #[must_use]
    pub fn hitbox(&self) -> Rect {
        let half = SIZE / 2.0;
        Rect::new(self.pos.x - half, self.pos.y - half, SIZE, SIZE)
    }
}

/// Todos os corações, ordenados por `x` para consultas rápidas por faixa.
#[derive(Debug)]
pub struct Hearts {
    items: Vec<Heart>,
    respawn_timers: Vec<f32>,
    rng: Rng,
}

impl Hearts {
    /// Distribui corações por todo o nível.
    #[must_use]
    pub fn new(level: &Level, mut rng: Rng) -> Self {
        let mut items = Vec::new();
        let mut x = range(&mut rng, 270.0, 370.0);
        let end = level.length() - 32.0;
        for _ in 0..MAX_HEARTS {
            if x >= end {
                break;
            }
            let pos = place(&mut rng, level, x);
            items.push(Heart {
                pos,
                phase: range(&mut rng, 0.0, std::f32::consts::TAU),
                age: 10.0,
            });
            x += range(&mut rng, SPACING.0, SPACING.1);
        }
        Self {
            items,
            respawn_timers: Vec::new(),
            rng,
        }
    }

    /// Remove os corações tocados pela caixa e devolve as posições deles.
    pub fn collect(&mut self, hitbox: Rect) -> Vec<Vec2> {
        let half = SIZE / 2.0;
        let lo = self.items.partition_point(|h| h.pos.x < hitbox.x - half);
        let hi = self
            .items
            .partition_point(|h| h.pos.x <= hitbox.right() + half);
        let mut taken = Vec::new();
        for i in (lo..hi).rev() {
            if self
                .items
                .get(i)
                .is_some_and(|h| h.hitbox().overlaps(&hitbox))
            {
                taken.push(self.items.remove(i).pos);
                let delay = range(&mut self.rng, RESPAWN_DELAY.0, RESPAWN_DELAY.1);
                self.respawn_timers.push(delay);
            }
        }
        taken
    }

    /// Envelhece os corações e faz surgir os pendentes fora da tela (preferindo à frente).
    pub fn update(&mut self, dt: f32, level: &Level, view_x: f32, ahead: f32) {
        for heart in &mut self.items {
            heart.age += dt;
        }
        let mut due = 0;
        self.respawn_timers.retain_mut(|t| {
            *t -= dt;
            let ready = *t <= 0.0;
            due += usize::from(ready);
            !ready
        });
        for _ in 0..due {
            self.spawn(level, view_x, ahead);
        }
    }

    /// Corações com centro em `[from, to]`.
    #[must_use]
    pub fn in_span(&self, from: f32, to: f32) -> &[Heart] {
        level::span(&self.items, |h| h.pos.x, from, to)
    }

    /// Quantidade de corações no nível.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.items.len()
    }

    /// Se não há corações no nível.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Quantidade de corações esperando para reaparecer.
    #[must_use]
    pub const fn pending(&self) -> usize {
        self.respawn_timers.len()
    }

    fn spawn(&mut self, level: &Level, view_x: f32, ahead: f32) {
        if self.items.len() >= MAX_HEARTS {
            return;
        }
        let x = spawn_x(&mut self.rng, level.length(), view_x, ahead);
        let heart = Heart {
            pos: place(&mut self.rng, level, x),
            phase: range(&mut self.rng, 0.0, std::f32::consts::TAU),
            age: 0.0,
        };
        let at = self.items.partition_point(|h| h.pos.x < x);
        self.items.insert(at, heart);
    }
}

/// Escolhe onde um coração novo surge: fora da tela, de preferência no sentido do movimento.
fn spawn_x(rng: &mut Rng, length: f32, view_x: f32, ahead: f32) -> f32 {
    let distance = range(rng, SPAWN_AHEAD.0, SPAWN_AHEAD.1);
    let forward = if rng.u8(..100) < 75 { ahead } else { -ahead };
    let right = view_x + VIEW_W + distance;
    let left = view_x - distance;
    let (first, second) = if forward >= 0.0 {
        (right, left)
    } else {
        (left, right)
    };
    let valid = |x: f32| (32.0..=length - 32.0).contains(&x);
    if valid(first) {
        first
    } else if valid(second) {
        second
    } else {
        range(rng, 32.0, length - 32.0)
    }
}

/// Altura de um coração em `x`: sobre a plataforma, no alto (pulo) ou junto ao chão.
fn place(rng: &mut Rng, level: &Level, x: f32) -> Vec2 {
    if let Some(p) = level.platform_at(x) {
        let base = if rng.bool() {
            p.y
        } else {
            level.terrain.height_at(x)
        };
        return Vec2::new(x, base - LOW);
    }
    let ground = level.terrain.height_at(x);
    let lift = if rng.u8(..100) < HIGH_CHANCE {
        range(rng, HIGH.0, HIGH.1).round()
    } else {
        LOW
    };
    Vec2::new(x, ground - lift)
}

/// Contador de corações: a cada 50 volta a zero (com festa!).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct HeartCounter {
    count: u8,
}

impl HeartCounter {
    /// Quantos corações valem uma festa e zeram a contagem.
    pub const GOAL: u8 = 50;

    /// Soma um coração. Devolve `true` quando completou a meta e voltou a zero.
    pub const fn add(&mut self) -> bool {
        self.count += 1;
        if self.count >= Self::GOAL {
            self.count = 0;
            return true;
        }
        false
    }

    /// Valor exibido (0 a `GOAL - 1`).
    #[must_use]
    pub const fn value(self) -> u8 {
        self.count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::player::{HEIGHT, JUMP_HEIGHT};

    fn level() -> Level {
        Level::generate(level::SEED)
    }

    fn hearts(level: &Level) -> Hearts {
        Hearts::new(level, Rng::with_seed(99))
    }

    #[test]
    fn contador_volta_a_zero_a_cada_meta() {
        let mut c = HeartCounter::default();
        for i in 1..HeartCounter::GOAL {
            assert!(!c.add());
            assert_eq!(c.value(), i);
        }
        assert!(c.add());
        assert_eq!(c.value(), 0);
        assert!(!c.add());
        assert_eq!(c.value(), 1);
    }

    #[test]
    fn coracoes_cobrem_o_nivel_inteiro_e_ordenados() {
        let level = level();
        let h = hearts(&level);
        assert!(h.len() > 250, "apenas {}", h.len());
        assert!(h.items.windows(2).all(|w| w[0].pos.x <= w[1].pos.x));
        let last = h.items.last().map_or(0.0, |h| h.pos.x);
        assert!(last > level.length() - 400.0);
    }

    #[test]
    fn alguns_exigem_pulo_e_todos_sao_alcancaveis() {
        let level = level();
        let h = hearts(&level);
        let mut high = 0;
        for heart in &h.items {
            let ground = level.terrain.height_at(heart.pos.x);
            let support = level
                .platform_at(heart.pos.x)
                .map_or(ground, |p| p.y.min(ground));
            let lift = support - heart.pos.y;
            // Fora do alcance parado, mas dentro do alcance do pulo.
            if lift - SIZE / 2.0 > HEIGHT {
                high += 1;
            }
            assert!(
                lift + SIZE / 2.0 <= JUMP_HEIGHT + HEIGHT,
                "inalcançável em x={}",
                heart.pos.x
            );
        }
        assert!(high > h.len() / 5, "poucos corações no alto: {high}");
    }

    #[test]
    fn coletar_remove_e_agenda_reaparecimento() {
        let level = level();
        let mut h = hearts(&level);
        let target = h.items[10];
        let before = h.len();
        let got = h.collect(target.hitbox());
        assert_eq!(got, vec![target.pos]);
        assert_eq!(h.len(), before - 1);
        assert_eq!(h.pending(), 1);
        assert_eq!(h.collect(target.hitbox()), []);
    }

    #[test]
    fn reaparece_fora_da_tela_a_frente() {
        let level = level();
        let mut h = hearts(&level);
        let target = h.items[20];
        h.collect(target.hitbox());
        let before = h.len();
        let view_x = 5000.0;
        for _ in 0..400 {
            h.update(0.01, &level, view_x, 1.0);
        }
        assert_eq!(h.len(), before + 1);
        assert_eq!(h.pending(), 0);
        let fresh: Vec<&Heart> = h.items.iter().filter(|x| x.age < 4.0).collect();
        assert_eq!(fresh.len(), 1);
        let x = fresh[0].pos.x;
        assert!(
            x < view_x || x > view_x + VIEW_W,
            "surgiu dentro da tela em x={x}"
        );
        assert!(h.items.windows(2).all(|w| w[0].pos.x <= w[1].pos.x));
    }

    #[test]
    fn surgimento_respeita_os_limites_do_nivel() {
        let mut rng = Rng::with_seed(5);
        for _ in 0..500 {
            let near_end = spawn_x(&mut rng, 10_000.0, 10_000.0 - VIEW_W, 1.0);
            assert!((32.0..=10_000.0 - 32.0).contains(&near_end));
            assert!(
                near_end < 10_000.0 - VIEW_W,
                "deveria surgir atrás, fora da tela"
            );
            let near_start = spawn_x(&mut rng, 10_000.0, 0.0, -1.0);
            assert!(near_start > VIEW_W);
        }
    }

    #[test]
    fn surgimento_e_aleatorio() {
        let mut rng = Rng::with_seed(8);
        let xs: Vec<f32> = (0..20)
            .map(|_| spawn_x(&mut rng, 30_000.0, 9000.0, 1.0))
            .collect();
        let distinct = xs.windows(2).filter(|w| (w[0] - w[1]).abs() > 1.0).count();
        assert!(distinct > 15);
    }

    #[test]
    fn nunca_passa_do_limite_de_coracoes() {
        let level = level();
        let mut h = hearts(&level);
        h.respawn_timers = vec![0.0; MAX_HEARTS];
        h.update(0.1, &level, 3000.0, 1.0);
        assert!(h.len() <= MAX_HEARTS);
    }

    #[test]
    fn consulta_por_faixa() {
        let level = level();
        let h = hearts(&level);
        let visible = h.in_span(1000.0, 1000.0 + VIEW_W);
        assert_ne!(visible, []);
        assert!(
            visible
                .iter()
                .all(|x| (1000.0..=1000.0 + VIEW_W).contains(&x.pos.x))
        );
    }
}

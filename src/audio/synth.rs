//! Efeitos sonoros sintetizados em código e empacotados como WAV (pura: testável).
//!
//! Sons curtos e suaves, pensados para ouvidos pequenos. Gerar em vez de carregar arquivos
//! mantém o jogo num único executável, sem pasta de assets.

/// Taxa de amostragem nativa do mixer do macroquad (evita reamostragem).
pub const SAMPLE_RATE: u32 = 44_100;
const SAMPLE_RATE_F: f32 = 44_100.0;
/// Volume geral dos efeitos (0 a 1).
const MASTER: f32 = 0.32;

/// Forma de onda.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Wave {
    Sine,
    Triangle,
    Square,
}

/// Um tom com glissando de `from` a `to` Hz.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tone {
    pub from: f32,
    pub to: f32,
    pub ms: u32,
    pub wave: Wave,
}

const fn tone(from: f32, to: f32, ms: u32, wave: Wave) -> Tone {
    Tone { from, to, ms, wave }
}

/// "Boing" ascendente do pulo.
pub const JUMP: &[Tone] = &[tone(330.0, 660.0, 130, Wave::Triangle)];
/// Plim-plim de coração.
pub const HEART: &[Tone] = &[
    tone(1318.5, 1318.5, 70, Wave::Sine),
    tone(1975.5, 1975.5, 170, Wave::Sine),
];
/// Fanfarra da meta de corações (dó-mi-sol-dó).
pub const CELEBRATE: &[Tone] = &[
    tone(523.3, 523.3, 120, Wave::Square),
    tone(659.3, 659.3, 120, Wave::Square),
    tone(784.0, 784.0, 120, Wave::Square),
    tone(1046.5, 1046.5, 420, Wave::Square),
];
/// Bip de pausa.
pub const PAUSE: &[Tone] = &[tone(880.0, 587.3, 110, Wave::Sine)];
/// Bip de volta ao jogo.
pub const RESUME: &[Tone] = &[tone(587.3, 880.0, 110, Wave::Sine)];
/// "Plop" da bolha estourando: subida rápida e um brilho agudo.
pub const POP: &[Tone] = &[
    tone(420.0, 980.0, 60, Wave::Sine),
    tone(1568.0, 1760.0, 90, Wave::Sine),
];
/// "Bup" suave da tecla errada: grave e curto, sem soar como erro.
pub const MISS: &[Tone] = &[tone(233.1, 196.0, 120, Wave::Triangle)];
/// Arpejo da palavra completa (sol-si-ré-sol).
pub const WORD: &[Tone] = &[
    tone(784.0, 784.0, 90, Wave::Sine),
    tone(987.8, 987.8, 90, Wave::Sine),
    tone(1174.7, 1174.7, 90, Wave::Sine),
    tone(1568.0, 1568.0, 260, Wave::Sine),
];

/// Amostras (mono, -1 a 1) da sequência de tons.
#[must_use]
pub fn render(tones: &[Tone]) -> Vec<f32> {
    let mut out = Vec::new();
    for t in tones {
        render_tone(*t, &mut out);
    }
    out
}

fn render_tone(t: Tone, out: &mut Vec<f32>) {
    let n = t.ms * SAMPLE_RATE / 1000;
    let dt = 1.0 / SAMPLE_RATE_F;
    let duration = f32::from(u16::try_from(t.ms).unwrap_or(u16::MAX)) / 1000.0;
    let (mut time, mut phase) = (0.0_f32, 0.0_f32);
    for _ in 0..n {
        let progress = (time / duration).min(1.0);
        let freq = (t.to - t.from).mul_add(progress, t.from);
        phase = freq.mul_add(dt, phase).fract();
        out.push(oscillator(t.wave, phase) * envelope(time, duration) * MASTER);
        time += dt;
    }
}

/// Valor da onda numa fase de 0 a 1.
fn oscillator(wave: Wave, phase: f32) -> f32 {
    match wave {
        Wave::Sine => (phase * std::f32::consts::TAU).sin(),
        Wave::Triangle => 4.0f32.mul_add(-(phase - 0.5).abs(), 1.0),
        // Quadrada mais suave (menos estridente): metade da amplitude.
        Wave::Square => {
            if phase < 0.5 {
                0.5
            } else {
                -0.5
            }
        }
    }
}

/// Ataque rápido (sem estalos) e decaimento até zero no fim.
fn envelope(time: f32, duration: f32) -> f32 {
    let attack = (time / 0.005).min(1.0);
    let release = ((duration - time) / 0.03).clamp(0.0, 1.0);
    let decay = 0.5f32.mul_add(-(time / duration.max(f32::EPSILON)).min(1.0), 1.0);
    attack * release * decay
}

/// Empacota amostras como WAV mono em ponto flutuante de 32 bits.
#[must_use]
pub fn wav(samples: &[f32]) -> Vec<u8> {
    let data_len = u32::try_from(samples.len() * 4).unwrap_or(u32::MAX);
    let mut out = Vec::with_capacity(samples.len() * 4 + 44);
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&(36 + data_len).to_le_bytes());
    out.extend_from_slice(b"WAVEfmt ");
    out.extend_from_slice(&16_u32.to_le_bytes());
    out.extend_from_slice(&3_u16.to_le_bytes()); // IEEE float
    out.extend_from_slice(&1_u16.to_le_bytes()); // mono
    out.extend_from_slice(&SAMPLE_RATE.to_le_bytes());
    out.extend_from_slice(&(SAMPLE_RATE * 4).to_le_bytes()); // bytes por segundo
    out.extend_from_slice(&4_u16.to_le_bytes()); // bytes por quadro
    out.extend_from_slice(&32_u16.to_le_bytes()); // bits por amostra
    out.extend_from_slice(b"data");
    out.extend_from_slice(&data_len.to_le_bytes());
    for s in samples {
        out.extend_from_slice(&s.to_le_bytes());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL: [&[Tone]; 8] = [JUMP, HEART, CELEBRATE, PAUSE, RESUME, POP, MISS, WORD];

    #[test]
    fn taxas_de_amostragem_coincidem() {
        assert!((f32::from(u16::try_from(SAMPLE_RATE).unwrap()) - SAMPLE_RATE_F).abs() < 0.5);
    }

    #[test]
    fn duracao_corresponde_aos_tons() {
        let samples = render(HEART);
        assert_eq!(samples.len(), (70 + 170) * 44_100 / 1000);
    }

    #[test]
    fn amostras_ficam_no_intervalo_e_sem_estalo_nas_pontas() {
        for tones in ALL {
            let s = render(tones);
            assert!(s.iter().all(|v| v.abs() <= MASTER + 1e-4));
            assert!(s.first().is_some_and(|v| v.abs() < 0.01));
            assert!(s.last().is_some_and(|v| v.abs() < 0.02));
            assert!(s.iter().any(|v| v.abs() > MASTER * 0.3), "som mudo");
        }
    }

    #[test]
    fn wav_e_lido_pelo_mesmo_decodificador_do_jogo() {
        for tones in ALL {
            let samples = render(tones);
            let bytes = wav(&samples);
            let mut reader = audrey::Reader::new(std::io::Cursor::new(bytes)).unwrap();
            let desc = reader.description();
            assert_eq!(desc.sample_rate(), SAMPLE_RATE);
            assert_eq!(desc.channel_count(), 1);
            let decoded: Vec<f32> = reader.samples::<f32>().map(Result::unwrap).collect();
            assert_eq!(decoded.len(), samples.len());
            assert!(
                decoded
                    .iter()
                    .zip(&samples)
                    .all(|(a, b)| (a - b).abs() < 1e-6)
            );
        }
    }

    #[test]
    fn cabecalho_wav_tem_tamanhos_corretos() {
        let bytes = wav(&[0.0; 10]);
        assert_eq!(bytes.len(), 44 + 40);
        assert_eq!(&bytes[0..4], b"RIFF");
        assert_eq!(u32::from_le_bytes(bytes[4..8].try_into().unwrap()), 36 + 40);
        assert_eq!(u32::from_le_bytes(bytes[40..44].try_into().unwrap()), 40);
    }
}

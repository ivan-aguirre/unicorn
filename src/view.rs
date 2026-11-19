//! Resolução virtual em que o jogo é desenhado (pixel art) antes de ser ampliado na janela.
//!
//! A câmera só se move na horizontal: as coordenadas `y` do mundo são as mesmas da tela,
//! o que deixa o cenário estável e fácil de acompanhar para crianças pequenas.

/// Largura da tela virtual, em pixels.
pub const VIEW_W_PX: u16 = 384;
/// Altura da tela virtual, em pixels.
pub const VIEW_H_PX: u16 = 216;
/// Largura da tela virtual, em unidades do mundo.
pub const VIEW_W: f32 = 384.0;
/// Altura da tela virtual, em unidades do mundo.
pub const VIEW_H: f32 = 216.0;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dimensoes_em_pixels_e_em_mundo_coincidem() {
        assert!((f32::from(VIEW_W_PX) - VIEW_W).abs() < f32::EPSILON);
        assert!((f32::from(VIEW_H_PX) - VIEW_H).abs() < f32::EPSILON);
    }
}

//! Imagem RGBA em memória, montada antes de virar textura (pura: testável sem janela).

/// Cor RGBA.
pub type Rgba = [u8; 4];

/// Transparente.
pub const CLEAR: Rgba = [0, 0, 0, 0];

/// Converte `0xRRGGBB` em RGBA opaco.
#[must_use]
pub const fn rgb(hex: u32) -> Rgba {
    let [_, r, g, b] = hex.to_be_bytes();
    [r, g, b, 255]
}

/// Problemas ao montar uma imagem a partir de ASCII.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArtError {
    /// Linha com largura diferente das demais.
    RaggedRow { row: usize },
    /// Caractere sem cor na paleta.
    UnknownColor { symbol: char },
    /// Imagem grande demais (ou vazia).
    BadSize,
}

impl std::fmt::Display for ArtError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RaggedRow { row } => write!(f, "linha {row} com largura diferente"),
            Self::UnknownColor { symbol } => write!(f, "cor desconhecida '{symbol}'"),
            Self::BadSize => write!(f, "tamanho de imagem inválido"),
        }
    }
}

impl std::error::Error for ArtError {}

/// Paleta: cada caractere ASCII vira uma cor. `.` é sempre transparente.
pub type Palette = [(u8, u32)];

/// Imagem RGBA com dimensões em `u16` (limite das texturas).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pixmap {
    width: u16,
    height: u16,
    rgba: Vec<u8>,
}

impl Pixmap {
    /// Imagem transparente.
    #[must_use]
    pub fn new(width: u16, height: u16) -> Self {
        let len = usize::from(width) * usize::from(height) * 4;
        Self {
            width,
            height,
            rgba: vec![0; len],
        }
    }

    /// Monta a imagem a partir de linhas de texto, uma letra por pixel.
    ///
    /// # Errors
    /// Linhas com larguras diferentes, letras fora da paleta ou tamanho inválido.
    pub fn from_ascii(rows: &[&str], palette: &Palette) -> Result<Self, ArtError> {
        let width = rows.first().map_or(0, |r| r.len());
        let w = u16::try_from(width).map_err(|_| ArtError::BadSize)?;
        let h = u16::try_from(rows.len()).map_err(|_| ArtError::BadSize)?;
        if w == 0 || h == 0 {
            return Err(ArtError::BadSize);
        }
        let mut out = Self::new(w, h);
        for ((row, text), y) in rows.iter().enumerate().zip(0..h) {
            if text.len() != width {
                return Err(ArtError::RaggedRow { row });
            }
            for (symbol, x) in text.bytes().zip(0..w) {
                out.set(x, y, lookup(palette, symbol)?);
            }
        }
        Ok(out)
    }

    /// Largura.
    #[must_use]
    pub const fn width(&self) -> u16 {
        self.width
    }

    /// Altura.
    #[must_use]
    pub const fn height(&self) -> u16 {
        self.height
    }

    /// Bytes RGBA, linha a linha.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.rgba
    }

    /// Cor do pixel (ou `None` fora da imagem).
    #[must_use]
    pub fn get(&self, x: u16, y: u16) -> Option<Rgba> {
        let i = self.offset(x, y)?;
        let px = self.rgba.get(i..i + 4)?;
        <Rgba>::try_from(px).ok()
    }

    /// Pinta um pixel (ignora coordenadas fora da imagem).
    pub fn set(&mut self, x: u16, y: u16, color: Rgba) {
        if let Some(i) = self.offset(x, y)
            && let Some(px) = self.rgba.get_mut(i..i + 4)
        {
            px.copy_from_slice(&color);
        }
    }

    /// Pinta um pixel em coordenadas com sinal (fora da imagem é ignorado).
    pub fn set_i(&mut self, x: i32, y: i32, color: Rgba) {
        if let (Ok(x), Ok(y)) = (u16::try_from(x), u16::try_from(y)) {
            self.set(x, y, color);
        }
    }

    /// Pinta o pixel que contém o ponto `(x, y)` (fora da imagem é ignorado).
    pub fn plot(&mut self, x: f32, y: f32, color: Rgba) {
        if let (Some(px), Some(py)) = (floor_px(x, self.width), floor_px(y, self.height)) {
            self.set(px, py, color);
        }
    }

    /// Se o pixel é visível (fora da imagem conta como transparente).
    #[must_use]
    pub fn is_opaque(&self, x: u16, y: u16) -> bool {
        self.get(x, y).is_some_and(|c| c[3] > 0)
    }

    /// Copia os pixels visíveis de `other` com o canto em `(dx, dy)`.
    pub fn overlay(&mut self, other: &Self, dx: u16, dy: u16) {
        for (x, y) in other.coords() {
            if let Some(c) = other.get(x, y).filter(|c| c[3] > 0) {
                self.set(x.saturating_add(dx), y.saturating_add(dy), c);
            }
        }
    }

    /// Contorno de 1 pixel em volta das partes visíveis (vizinhança de 4).
    ///
    /// Desenhar o contorno automaticamente deixa os sprites ASCII mais simples de editar.
    #[must_use]
    pub fn outlined(&self, color: Rgba) -> Self {
        let mut out = self.clone();
        for (x, y) in self.coords() {
            if !self.is_opaque(x, y) && self.touches_opaque(x, y) {
                out.set(x, y, color);
            }
        }
        out
    }

    /// Sombra projetada 1 pixel abaixo e à direita (a imagem cresce 1 pixel).
    ///
    /// Melhor que o contorno para traços finos, como letras: não fecha os vãos internos.
    #[must_use]
    pub fn shadowed(&self, color: Rgba) -> Self {
        let mut out = Self::new(self.width.saturating_add(1), self.height.saturating_add(1));
        for (x, y) in self.coords() {
            if self.is_opaque(x, y) {
                out.set(x + 1, y + 1, color);
            }
        }
        out.overlay(self, 0, 0);
        out
    }

    /// Troca cores exatas (para variações de paleta, como flores de outra cor).
    #[must_use]
    pub fn recolored(&self, swaps: &[(Rgba, Rgba)]) -> Self {
        let mut out = self.clone();
        for px in out.rgba.as_chunks_mut::<4>().0 {
            if let Some((_, to)) = swaps.iter().find(|(from, _)| px == from) {
                px.copy_from_slice(to);
            }
        }
        out
    }

    /// Todas as coordenadas, linha a linha.
    pub fn coords(&self) -> impl Iterator<Item = (u16, u16)> + use<> {
        let (w, h) = (self.width, self.height);
        (0..h).flat_map(move |y| (0..w).map(move |x| (x, y)))
    }

    fn touches_opaque(&self, x: u16, y: u16) -> bool {
        let left = x.checked_sub(1).is_some_and(|l| self.is_opaque(l, y));
        let up = y.checked_sub(1).is_some_and(|u| self.is_opaque(x, u));
        left || up
            || self.is_opaque(x.saturating_add(1), y)
            || self.is_opaque(x, y.saturating_add(1))
    }

    fn offset(&self, x: u16, y: u16) -> Option<usize> {
        (x < self.width && y < self.height)
            .then(|| (usize::from(y) * usize::from(self.width) + usize::from(x)) * 4)
    }
}

/// Coordenada inteira do pixel que contém `v`, sem conversão com perda (`as`).
fn floor_px(v: f32, limit: u16) -> Option<u16> {
    (0..limit)
        .rev()
        .find(|i| f32::from(*i) <= v)
        .filter(|i| v < f32::from(*i) + 1.0)
}

fn lookup(palette: &Palette, symbol: u8) -> Result<Rgba, ArtError> {
    if symbol == b'.' {
        return Ok(CLEAR);
    }
    palette
        .iter()
        .find(|(s, _)| *s == symbol)
        .map(|(_, hex)| rgb(*hex))
        .ok_or_else(|| ArtError::UnknownColor {
            symbol: char::from(symbol),
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const PAL: &Palette = &[(b'r', 0xff_00_00), (b'g', 0x00_ff_00)];

    #[test]
    fn converte_hex_em_rgba() {
        assert_eq!(rgb(0x12_34_56), [0x12, 0x34, 0x56, 255]);
    }

    #[test]
    fn monta_a_partir_de_ascii() {
        let p = Pixmap::from_ascii(&["r.", ".g"], PAL).unwrap();
        assert_eq!((p.width(), p.height()), (2, 2));
        assert_eq!(p.get(0, 0), Some([255, 0, 0, 255]));
        assert_eq!(p.get(1, 0), Some(CLEAR));
        assert_eq!(p.get(1, 1), Some([0, 255, 0, 255]));
        assert_eq!(p.get(2, 0), None);
        assert_eq!(p.bytes().len(), 16);
    }

    #[test]
    fn rejeita_linha_torta_cor_desconhecida_e_vazio() {
        assert_eq!(
            Pixmap::from_ascii(&["rr", "r"], PAL),
            Err(ArtError::RaggedRow { row: 1 })
        );
        assert_eq!(
            Pixmap::from_ascii(&["x"], PAL),
            Err(ArtError::UnknownColor { symbol: 'x' })
        );
        assert_eq!(Pixmap::from_ascii(&[], PAL), Err(ArtError::BadSize));
        assert_eq!(ArtError::BadSize.to_string(), "tamanho de imagem inválido");
    }

    #[test]
    fn contorno_envolve_so_o_que_e_visivel() {
        let p = Pixmap::from_ascii(&["...", ".r.", "..."], PAL).unwrap();
        let o = p.outlined([1, 2, 3, 255]);
        assert_eq!(o.get(1, 0), Some([1, 2, 3, 255]));
        assert_eq!(o.get(0, 1), Some([1, 2, 3, 255]));
        assert_eq!(o.get(1, 1), Some([255, 0, 0, 255]));
        // Diagonais ficam transparentes (contorno arredondado).
        assert_eq!(o.get(0, 0), Some(CLEAR));
    }

    #[test]
    fn sobrepoe_ignorando_transparencia() {
        let mut base = Pixmap::from_ascii(&["gg", "gg"], PAL).unwrap();
        let top = Pixmap::from_ascii(&["r."], PAL).unwrap();
        base.overlay(&top, 0, 1);
        assert_eq!(base.get(0, 1), Some([255, 0, 0, 255]));
        assert_eq!(base.get(1, 1), Some([0, 255, 0, 255]));
        assert_eq!(base.get(0, 0), Some([0, 255, 0, 255]));
    }

    #[test]
    fn sombra_fica_abaixo_a_direita_sem_fechar_vaos() {
        let p = Pixmap::from_ascii(&["r.r", "r.r"], PAL).unwrap();
        let s = p.shadowed([1, 1, 1, 255]);
        assert_eq!((s.width(), s.height()), (4, 3));
        assert_eq!(s.get(0, 0), Some([255, 0, 0, 255]));
        assert_eq!(s.get(1, 1), Some([1, 1, 1, 255]));
        assert_eq!(s.get(1, 0), Some(CLEAR), "o vão continua aberto");
        assert_eq!(s.get(3, 2), Some([1, 1, 1, 255]));
    }

    #[test]
    fn plota_ponto_no_pixel_que_o_contem() {
        let mut p = Pixmap::new(4, 4);
        p.plot(2.7, 1.2, [9, 9, 9, 255]);
        assert!(p.is_opaque(2, 1));
        p.plot(-0.5, 1.0, [9, 9, 9, 255]);
        p.plot(4.0, 1.0, [9, 9, 9, 255]);
        assert_eq!(p.coords().filter(|(x, y)| p.is_opaque(*x, *y)).count(), 1);
    }

    #[test]
    fn troca_cores() {
        let p = Pixmap::from_ascii(&["rg"], PAL).unwrap();
        let q = p.recolored(&[(rgb(0xff_00_00), rgb(0x00_00_ff))]);
        assert_eq!(q.get(0, 0), Some([0, 0, 255, 255]));
        assert_eq!(q.get(1, 0), Some([0, 255, 0, 255]));
    }

    #[test]
    fn pintar_fora_da_imagem_e_ignorado() {
        let mut p = Pixmap::new(2, 2);
        p.set(5, 5, [9, 9, 9, 9]);
        p.set_i(-1, 0, [9, 9, 9, 9]);
        assert!(
            p.bytes().iter().all(|b| *b == 0),
            "nada deveria ser pintado"
        );
        p.set_i(1, 1, [9, 9, 9, 9]);
        assert!(p.is_opaque(1, 1));
    }
}

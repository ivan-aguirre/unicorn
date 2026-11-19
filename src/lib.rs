//! Unicórnio: jogo de plataforma para crianças pequenas.
//!
//! Um unicórnio fofo anda por um nível longo de grama, cachoeiras, nuvens e arco-íris
//! coletando corações. A lógica (nível, física, corações, efeitos) é pura e testada sem
//! janela; `render`, `audio` e `app` fazem a ligação com o macroquad.

pub mod animation;
pub mod app;
pub mod art;
pub mod audio;
pub mod camera;
pub mod clock;
pub mod effects;
pub mod game;
pub mod hearts;
pub mod input;
pub mod level;
pub mod player;
pub mod render;
pub mod view;

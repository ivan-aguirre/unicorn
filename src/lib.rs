//! Unicórnio: jogo de plataforma para crianças pequenas.
//!
//! Dois modos, escolhidos na tela inicial (`menu`): no passeio, um unicórnio fofo anda por um
//! nível longo de grama, cachoeiras, nuvens e arco-íris coletando corações; em `letters`,
//! aparecem um desenho e a palavra dele, e a criança digita a mesma palavra, treinando o
//! teclado e a alfabetização. A lógica é pura e testada sem janela; `render`,
//! `audio` e `app` fazem a ligação com o macroquad.

pub mod ambient;
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
pub mod letters;
pub mod level;
pub mod menu;
pub mod player;
pub mod render;
pub mod view;

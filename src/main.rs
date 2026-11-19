//! Ponto de entrada: abre a janela e roda o jogo.

// Sem console extra ao abrir o jogo no Windows.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use unicorn::app::window_conf;

#[macroquad::main(window_conf)]
async fn main() {
    unicorn::app::run().await;
}

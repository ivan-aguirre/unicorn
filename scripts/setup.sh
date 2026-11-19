#!/bin/sh
# Prepara o ambiente de desenvolvimento (Linux, macOS ou Git Bash no Windows).
set -eu

cd "$(dirname "$0")/.."

if ! command -v cargo >/dev/null 2>&1; then
    echo "Rust não encontrado. Instale via https://rustup.rs e rode novamente." >&2
    exit 1
fi

# Linux: bibliotecas de sistema para janela (X11/OpenGL) e som (ALSA).
if [ "$(uname -s)" = "Linux" ]; then
    if command -v apt-get >/dev/null 2>&1; then
        echo "Debian/Ubuntu: sudo apt-get install libasound2-dev libx11-dev libxi-dev libgl1-mesa-dev"
    elif command -v pacman >/dev/null 2>&1; then
        echo "Arch: sudo pacman -S --needed alsa-lib libx11 libxi mesa"
    elif command -v dnf >/dev/null 2>&1; then
        echo "Fedora: sudo dnf install alsa-lib-devel libX11-devel libXi-devel mesa-libGL-devel"
    fi
fi

rustup component add rustfmt clippy
command -v cargo-deny >/dev/null 2>&1 || cargo install cargo-deny --locked

git config core.hooksPath .githooks
chmod +x .githooks/pre-commit scripts/*.sh 2>/dev/null || true

echo "Ambiente pronto. O hook de pre-commit está ativo."

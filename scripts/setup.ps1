# Prepara o ambiente de desenvolvimento no Windows (PowerShell).
# O hook de pre-commit roda via Git for Windows (sh embutido).
$ErrorActionPreference = 'Stop'
Set-Location (Join-Path $PSScriptRoot '..')

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    throw 'Rust não encontrado. Instale via https://rustup.rs e rode novamente.'
}

rustup component add rustfmt clippy
if (-not (Get-Command cargo-deny -ErrorAction SilentlyContinue)) {
    cargo install cargo-deny --locked
}
git config core.hooksPath .githooks

Write-Host 'Ambiente pronto. O hook de pre-commit está ativo.'

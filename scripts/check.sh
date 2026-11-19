#!/bin/sh
# Verificações de qualidade. Usado pelo hook de pre-commit.
# Uso: scripts/check.sh            (formatação + linter + testes)
#      scripts/check.sh --security (inclui auditoria de dependências)
set -eu

cd "$(dirname "$0")/.."

step() { printf '\n==> %s\n' "$1"; }

step "Formatação (rustfmt)"
cargo fmt --all -- --check

step "Linter (clippy pedantic + nursery, limites de complexidade, zero avisos)"
cargo clippy --all-targets --locked -- -D warnings

step "Testes (unitários + partidas simuladas)"
cargo test --locked --quiet

if [ "${1:-}" = "--security" ]; then
    step "Segurança: dependências (cargo-deny: RustSec, licenças, origens)"
    cargo deny check
fi

printf '\nTudo certo.\n'

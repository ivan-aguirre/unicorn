#!/bin/sh
# Gera assets/fontes/Baloo2-ExtraBold.ttf: a fonte Baloo 2 (SIL Open Font License, ver
# assets/fontes/OFL.txt) fixada no peso ExtraBold e reduzida às letras A-Z, Ç e dígitos
# (~6 KB, embutida no executável). Só é preciso rodar ao mudar os caracteres.
#
# Requisitos: uv (https://docs.astral.sh/uv) e curl. Nada é instalado no sistema.
# Uso: scripts/gerar-fonte.sh
set -eu

cd "$(dirname "$0")/.."

URL='https://raw.githubusercontent.com/google/fonts/main/ofl/baloo2'
TMP=target/fonte
CHARS='ABCDEFGHIJKLMNOPQRSTUVWXYZÇ0123456789'

mkdir -p "$TMP"
[ -s "$TMP/Baloo2.ttf" ] || curl -sfL "$URL/Baloo2%5Bwght%5D.ttf" -o "$TMP/Baloo2.ttf"
curl -sfL "$URL/OFL.txt" -o assets/fontes/OFL.txt

# O macroquad (fontdue) não lê fontes variáveis: fixa o peso 800 (ExtraBold).
uvx --quiet --from fonttools fonttools varLib.instancer "$TMP/Baloo2.ttf" wght=800 \
    -o "$TMP/Baloo2-ExtraBold.ttf"
uvx --quiet --from fonttools pyftsubset "$TMP/Baloo2-ExtraBold.ttf" --text="$CHARS" \
    --layout-features='' --output-file=assets/fontes/Baloo2-ExtraBold.ttf

echo "Fonte gerada em assets/fontes/Baloo2-ExtraBold.ttf."

# Desenvolvimento

Detalhes técnicos do jogo Unicórnio. Para jogar e compilar, veja o [README](README.md).

## Regras do jogo

- **Um único nível, bem longo**: 54.400 px (85 telas, alguns minutos de caminhada), com aclives,
  declives suaves e plataformas flutuantes. O cenário é sempre o mesmo (semente fixa).
- **Corações** espalhados por todo o nível: uns junto ao chão, outros no alto (só pulando) ou
  sobre plataformas. A posição deles muda a cada partida.
- Cada coração pego **reaparece aleatoriamente** em outro lugar, fora da tela e de preferência
  à frente do unicórnio — nunca faltam corações.
- **A cada 50 corações a contagem volta a zero**, com festa: arco-íris gigante, chuva de
  corações e estrelas, uma nuvem sorridente em cada base do arco-íris e uma fanfarra. A barra
  sob o contador mostra o quanto falta.
- **Unicórnio expressivo** (`src/animation.rs`): respira parado (8 quadros), bate a patinha
  no chão depois de alguns segundos parado, trota em 6 quadros (patas em pares diagonais,
  cauda balançando), salta com as dianteiras dobradas, agacha ao aterrissar e muda de
  expressão — sorriso calmo e piscadas parado, boca aberta andando, olhos brilhando no ar e
  carinha "^^" ao pegar um coração.
- **Cenário vivo** (`src/ambient.rs` + `src/render/ambient.rs`): passarinhos azuis voando
  sozinhos ou em grupinhos (batendo as asas e planando), borboletas voando em "oito" sobre as
  flores, peixinhos pulando no lago (com respingo e ondinhas), brilhos piscando na água e no
  arco-íris, cachoeiras que curvam na borda e abrem ao cair (três faixas de água, brilhos,
  pedras, espuma, gotas e névoa), cordilheira ao fundo com picos nevados, colinas verdes e névoa no horizonte, lago com gradiente de profundidade, cristas de onda e reflexo
  das colinas, colinas de pedra com topo ondulado, camadas e grama com tufos, árvores balançando
  com o vento, pétalas caindo das cerejeiras, sol com raios girando e nuvens flutuando. Tudo é
  função do tempo, sem estado: leve.
- **Tela inicial** (`src/menu.rs`): dois cartões grandes, sem texto: o passeio (cabeça do
  unicórnio e corações) e as letras (o desenho da bola e as teclas de BOLA). Setas escolhem, Enter ou espaço
  começa. Segurar Esc volta à tela inicial; na tela inicial, sai. O Esc precisa ser solto
  entre uma coisa e outra, então segurar demais não fecha o jogo sem querer.
- **Modo Letras** (`src/letters/`), para treinar o teclado enquanto aprende a ler:
  - um cartão mostra um desenho, a palavra escrita e um espaço vazio por letra. São 40
    palavras pensadas para a alfabetização (`src/letters/words.rs`): sem acento, só com
    sílabas simples (consoante + vogal, ou vogal sozinha), sem dígrafos nem encontros
    consonantais, de coisas que a criança reconhece: BOLA, PATO, GATO, CASA, SAPO, LUA, UVA,
    PIPA, BOLO, DADO, BANANA, TOMATE, COPO, PERA, SINO, FOCA, RATO, JANELA, SAPATO, PIPOCA,
    VACA, MACACO, GALO, GIRAFA, TUCANO, BALEIA, COCO, PIRULITO, SOPA, MALA, MESA, CAMA, VELA,
    BOTA, NAVIO, LUPA, BULE, BONECA, FOGO, CANECA. Os desenhos ficam em `src/art/pictures/`,
    separados em bichos, comidas e coisas.
    Um teste garante a regra das sílabas;
  - a criança digita a mesma palavra, em ordem: a letra da vez fica marcada na palavra e a
    tecla dela acende num teclado ABNT2 desenhado embaixo;
  - a letra certa entra no espaço dela com um "plop"; a tecla errada só pisca em cinza com um
    "bup" suave, sem contar como erro;
  - com a palavra completa vêm uma estrela e a festa do arco-íris (o fundo do cartão fica
    transparente para ela aparecer); o desenho pula, o unicórnio comemora e, terminada a festa,
    vem a próxima palavra (todas passam antes de repetir);
  - os desenhos não são pixel art: são formas vetoriais (`src/art/vector.rs` e
    `src/art/pictures/`) rasterizadas na inicialização em 384x384, com bordas suaves e
    contorno, e desenhadas com filtro linear (nítidas até em tela cheia);
  - não há voz: só efeitos sonoros.
- **Tolerância para mãos pequenas**: dá para pular logo após sair de uma beira, o toque em
  espaço pouco antes de tocar o chão é lembrado e sair exige segurar Esc (um toque acidental
  não fecha o jogo).

## Ferramentas

```sh
scripts/setup.sh              # ou scripts/setup.ps1 no Windows: componentes, cargo-deny e hook
scripts/check.sh              # rustfmt + clippy (zero avisos) + testes — o mesmo do pre-commit
scripts/check.sh --security   # inclui cargo-deny (RustSec, licenças, origens)
scripts/gerar-fonte.sh        # regera a fonte embutida (assets/fontes) a partir da Baloo 2
cargo run --example snapshot -- target/snapshots   # capturas de cenas e folha de sprites
```

### Teclas de teste

Só nos builds de desenvolvimento (`cargo run`, sem `--release`); no executável de release elas
não fazem nada:

| Tecla | Ação |
|---|---|
| F9 | Começa a festa dos 50 corações agora (não mexe no contador) |

## Qualidade

- **Linter agressivo** (mais rígido que o padrão): clippy `pedantic` + `nursery` + `cargo` como
  erro, complexidade cognitiva ≤ 10 (SonarQube usa 15; clippy, 25), funções ≤ 50 linhas, ≤ 5 argumentos, aninhamento ≤ 4,
  proibição de `unwrap`/`expect`/`panic`/indexação direta e `unsafe` fora de testes. Nenhum
  `#[allow]` no código.
- **Testes**: unitários para toda a lógica e a arte (física, terreno, plataformas, cenário,
  corações, contador, animação, vida no cenário, efeitos, câmera, sprites, figuras, som, menu,
  teclado e palavras) e partidas simuladas de ponta a ponta,
  tudo sem janela:
  - `tests/gameplay.rs`: atravessar o nível inteiro, festa dos 50, pausa, pulo alcançando
    corações no alto;
  - `tests/letters.rs`: uma criança que erra e demora e mesmo assim completa várias
    palavras; a palavra que fica esperando quando ninguém digita; todas as teclas apertadas
    ao mesmo tempo.
- **Conferência visual**: o exemplo `snapshot` gera capturas da tela virtual em vários pontos
  do nível, uma sequência de quadros para ver as animações, a festa, a pausa e a folha de
  sprites. Gera também capturas da janela inteira, com o desenho suave: o menu e, no modo
  Letras, o cartão da palavra vazio, pela metade com uma tecla errada, completo (com a festa)
  e a palavra seguinte; e `palavras_1.png` e `palavras_2.png`, uma prévia de todas as palavras com os desenhos.
- **Hook de pre-commit** roda `scripts/check.sh` e bloqueia o commit se algo falhar.

## Arquitetura

```
src/
  app.rs        janela, teclado e laço principal (passo fixo de 120 Hz)
  menu.rs       tela inicial: escolha entre passear e letras
  letters/      modo Letras: estado e passo (mod), palavras e teclado ABNT2
  game.rs       estado do passeio e um passo de simulação (pausa, corações, festa)
  player.rs     física do unicórnio (andar, pular, rampas, plataformas)
  animation.rs  pose do corpo e expressão do rosto a partir do movimento
  ambient.rs    trajetórias da vida no cenário (pássaros, peixes, borboletas, pétalas)
  hearts.rs     corações: distribuição, coleta, reaparecimento e o contador até 50
  level/        nível determinístico: terreno, plataformas e cenário com paralaxe
  effects.rs    partículas (brilhos, chuva de estrelas e festa)
  camera.rs     câmera horizontal suave, olhando à frente
  input.rs      controles independentes da janela (e o "segurar Esc para sair")
  clock.rs      passo fixo de simulação
  art/          pixel art em memória: sprites ASCII, imagens geradas, contorno automático;
                desenhos vetoriais em alta definição das palavras (vector, pictures)
  audio/        efeitos sonoros sintetizados (WAV gerado em código)
  render/       desenho em tela virtual 640x360 ampliada, HUD, pausa, festa e saída; por cima,
                desenho suave na resolução da janela (smooth): letras, cartões, teclado, menu
assets/fontes/  fonte Baloo 2 reduzida (embutida) e a licença dela
```

A lógica não depende da janela: tudo em `level/`, `player`, `animation`, `ambient`, `hearts`,
`game`, `menu`, `letters/`, `art` e `audio/synth` é testado sem abrir janela nem placa de som.

## Decisões técnicas

- **[macroquad](https://github.com/not-fl3/macroquad)**: biblioteca 2D madura e leve em Rust,
  sem dependências nativas no Windows/macOS; compila rápido e gera um executável pequeno.
  (Bevy seria pesado demais; SDL2/raylib exigiriam bibliotecas C.)
- **Arte e som gerados no código**: cabeça do unicórnio desenhada em ASCII e corpo, patas e
  cauda gerados por ângulos e curvas de Bézier (`src/art/unicorn.rs`), em duas camadas com
  contorno próprio (corpo e cabeça, uma por expressão); as patas do lado de lá ficam atrás do
  corpo, num tom mais escuro. Coração pela curva clássica do coração; céu, sol, nuvens,
  arco-íris, árvores, passarinhos e texturas do chão gerados por funções; sons sintetizados.
  Resultado: um único executável, sem pasta de assets nem licenças de terceiros.
- **Letras suaves por cima da pixel art**: as letras, o cartão e o teclado do modo Letras
  ficavam serrilhados demais na tela virtual pequena e difíceis de ler. Por isso são desenhados
  depois da ampliação, direto na resolução da janela (`render/smooth.rs`), com formas
  antisserrilhadas (MSAA 4x) e uma fonte vetorial. As coordenadas continuam as da tela virtual,
  convertidas pela mesma escala da imagem. O cenário e o unicórnio continuam em pixel art.
- **Fonte Baloo 2** (OFL, sem nome reservado): redonda e gorda, legível para crianças,
  escolhida entre Andika, Nunito, Fredoka e Baloo 2. O macroquad (fontdue) não lê fontes
  variáveis, então `scripts/gerar-fonte.sh` fixa o peso ExtraBold e reduz a fonte às letras
  A–Z, Ç e dígitos: ~6 KB embutidos com `include_bytes!`.
- **Sem voz**: as vozes sintéticas pt-BR testadas (Piper) soaram ruins, e não há voz infantil
  livre e offline. O modo Letras usa só efeitos sonoros.
- **Pixel art nítida**: tudo é desenhado numa tela virtual de 640x360 e ampliado com filtro
  "nearest" (inclusive o atlas de texturas, que no macroquad é linear por padrão e borrava o
  que era ampliado) (escala inteira sempre que possível: 2x em 720p, 3x em 1080p, 4x em 1440p);
  posições arredondadas ao pixel.
- **Leveza**: só o visível é desenhado (busca binária nas listas ordenadas), toda a arte fica
  num atlas (poucas trocas de textura), limites fixos de corações e partículas, animações do
  cenário sem estado. Medido no build de release: ~3% de um núcleo de CPU.
- **Exceções no `cargo deny`** (sem correção disponível, revisar a cada atualização do
  macroquad): RUSTSEC-2025-0035 (soundness da API global do macroquad — o jogo só a usa na
  thread principal e não processa entrada externa) e RUSTSEC-2026-0192 (`ttf-parser` sem
  manutenção, usado pelo renderizador de fontes do macroquad — o jogo usa fonte própria).
- **Inspiração**: cenário com terra em xadrez, colinas com cachoeiras e mar ao fundo lembra a
  fase inicial de Sonic (Mega Drive); o unicórnio segue o estilo chibi de desenhos japoneses
  infantis. Toda a arte é original.

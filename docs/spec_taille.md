# Taille de la spécification du noyau — Shrimp

Décision du point 1.3 de la feuille de route, 2026-10-21. Le cahier
(§2, §8) exige que la spécification du noyau tienne dans la fenêtre de
contexte d'une IA et laisse le chiffre à fixer ; le budget
(`docs/budget.md`, §3) fait de ce chiffre le plafond qui refuse un trait
admis par la grille. Ici : ce qu'on compte, avec quoi, ce que pèsent les
langages de référence, et le plafond. Tout ce qui est chiffré a été
mesuré le jour de la décision, par un outil rejouable qui est dans le
dépôt.

**Lu et approuvé, Maxime MARCONATO (aka MaaaX^^, aka EPOCH84),
2026-10-21.** Toute modification ultérieure se date et se signe.

Trois arbitrages pris par MaaaX avant écriture, le 2026-10-21 :
plusieurs tokenizers ouverts et un seul moteur réécrit, score le moins
favorable retenu ; un chiffre fixe, pas une fraction d'une fenêtre qui
grandit ; le livre I entier compté, tel que l'IA le charge.

## 1. Ce qu'on compte

**Le livre I de la bible, entier, tel que l'IA le charge** : texte
normatif, grammaire, et les exemples du noyau, puisque c'est avec eux que
l'IA relit le langage. En tokens, parce que c'est l'unité qui remplit
une fenêtre de contexte : un mot ou un octet n'en mesurent qu'un reflet,
et un texte dense en symboles coûte plus de tokens par mot qu'une prose.

Hors plafond :

- les annexes d'édition, ce qu'une édition masque et que l'IA qui écrit
  dans l'édition courante n'a pas à charger (`docs/budget.md`, §3) ;
- l'index de `std`, qui a sa propre mesure, fixée à l'étape 4 ;
- les livres suivants de la bible, qui décrivent la bibliothèque et
  l'outillage, pas le langage.

## 2. Avec quoi : quatre vocabulaires, un moteur, le pire score

Un token est l'unité d'un modèle précis, et le découpeur des modèles
d'Anthropic n'est pas public. Deux issues étaient mauvaises : compter
avec un seul vocabulaire ouvert et deviner une marge, ou se fabriquer un
vocabulaire, qui ne mesurerait rien. L'issue retenue :

- **quatre vocabulaires ouverts, de trois éditeurs**, figés dans le
  dépôt avec leur empreinte et leur licence (`tools/tokcount/vocab/`) :
  `o200k_base` et `cl100k_base` d'OpenAI, celui de Qwen3 d'Alibaba, celui
  de DeepSeek-V3. Ils ne bougent plus : le compte de 2026 se refait à
  l'identique en 2031, quoi qu'il arrive à leurs éditeurs ;
- **un seul moteur BPE, écrit dans le dépôt** (`tools/tokcount/`, Rust
  aujourd'hui, Shrimp quand Shrimp existera), qui ne dépend d'aucune
  bibliothèque des éditeurs. Seules les données changent d'un vocabulaire
  à l'autre : la table de fusion, le prédécoupage, la normalisation ;
- **le score retenu est le plus défavorable des quatre.** Il remplace la
  marge devinée par une marge mesurée : si quatre familles de modèles
  tiennent, la cinquième, inconnue, a toutes les chances de tenir. Un
  vocabulaire nouveau s'ajoute, le plafond ne bouge pas.

**Vérifié, pas supposé.** Chaque vocabulaire a été comparé à son
implémentation d'origine, tiktoken 0.14.0 et tokenizers 0.23.3, sur les
cinq spécifications de référence du §3 : vingt comptes, zéro token
d'écart (`tools/tokcount/check/compare.py`). Un écart d'un token aurait
été un échec.

**Ce qui n'est pas mesuré.** Le tokenizer d'Anthropic, faute d'accès
hors ligne ; le pire des quatre en tient lieu. Les quelques chaînes que
les vocabulaires Qwen et DeepSeek reconnaissent d'un bloc (`<think>`,
`<tool_call>`) ne sont pas reconnues par le moteur ; une spécification
ne les contient pas.

## 3. Ce que pèsent les langages de référence

Mesuré le 2026-10-21 par `tools/tokcount/measure.sh`, sources épinglées
par version et par empreinte, pdftotext 24.02.0 pour les deux PDF, balises
retirées pour les deux pages HTML. Le script rejoue tout.

| Spécification | Source | o200k_base | cl100k_base | qwen3 | deepseek_v3 | **pire** |
|---|---|---:|---:|---:|---:|---:|
| Go 1.25 | `doc/go_spec.html` du dépôt Go, étiquette `go1.25.0` | 55 389 | 55 279 | 56 240 | 56 519 | **56 519** |
| Zig 0.15.1 | référence du langage, page unique, ziglang.org | 113 236 | 112 758 | 115 770 | 117 907 | **117 907** |
| Rust | *The Rust Reference*, dépôt `rust-lang/reference`, commit `3989fe9` (2026-10-20), `src/*.md` | 289 930 | 289 229 | 293 748 | 303 578 | **303 578** |
| C23 | brouillon de travail N3220, open-std.org, texte du PDF | 546 585 | 543 582 | 601 915 | 558 918 | **601 915** |
| C++23 | brouillon N4950 (2023-05-10), open-std.org, texte du PDF | 1 539 006 | 1 534 808 | 1 617 308 | 1 607 002 | **1 617 308** |

Les deux normes décrivent le langage et la bibliothèque d'un seul tenant.
Coupées au début de la clause « Library », tout ce qui précède compris
(avant-propos, table des matières) :

| Langage seul | Clauses | o200k_base | cl100k_base | qwen3 | deepseek_v3 | **pire** |
|---|---|---:|---:|---:|---:|---:|
| C23 | 1 à 6 | 139 322 | 138 944 | 144 182 | 141 255 | **144 182** |
| C++23 | 1 à 15 | 373 149 | 373 314 | 380 613 | 380 718 | **380 718** |

Ce que chaque texte contient, pour lire les chiffres :

- **Go** : le langage seul, sans la bibliothèque. C'est l'étalon de
  concision : un langage complet avec ramasse-miettes, interfaces et
  génériques, en 56 000 tokens.
- **Zig** : le langage avec ses exemples compilés, nombreux, la table
  des matières et la liste des builtins. Une large part du texte est du
  code.
- **Rust** : la référence seule, sans le *Book* ni la documentation de
  `std`. Elle n'est pas une spécification complète, elle le dit
  elle-même.
- **C et C++** : la norme entière d'abord, puis les seules clauses du
  langage. Les PDF incluent index, annexes et pieds de page, que
  pdftotext rend tels quels.

Les quatre vocabulaires s'écartent de 2 % à 11 % entre eux sur le même
texte ; le pire est Qwen sur les deux PDF, DeepSeek sur les trois autres.
C'est l'écart que la marge devinée aurait dû couvrir.

**Ce que ça précise au cahier.** Le §2 disait, de mémoire : « C tient,
Go tient, Zig tient en une page ; Rust non, C++ encore moins. » Mesuré
contre une fenêtre de 200 000 tokens : Go, Zig et les clauses du langage
de C tiennent ; la norme C entière, la référence de Rust et C++ même sans
sa bibliothèque ne tiennent pas. L'intuition était juste, le cahier
reçoit les chiffres (§2, daté).

## 4. Le plafond

Les mesures ci-dessus posent la fourchette : la spécification complète
d'un langage sans ramasse-miettes, avec grammaire et exemples, pèse entre
Go et Rust. Shrimp retire à Rust la moitié de ce qui fait sa longueur,
par décision déjà prise : pas de macros, pas d'`async`, pas d'héritage,
pas de destructeur, pas de surcharge d'opérateurs, pas d'inférence, une
seule façon de faire chaque chose. Il y ajoute ce que Rust n'a pas à
spécifier : les ressources linéaires, et des exemples à chaque entrée.

**Le plafond est un nombre fixe, pas une fraction d'une fenêtre.** Les
fenêtres grandissent vite ; un chiffre qui passe en 2026 passera à l'aise
dans quelques années, sauf catastrophe industrielle (MaaaX, 2026-10-21).
Le chiffre borne la spécification, pas le modèle qui la lit, et il doit
laisser, dans une fenêtre d'aujourd'hui, la place du programme, de
l'index de `std` et de la conversation.

**Plafond : 100 000 tokens tout compris, au pire des quatre
vocabulaires.** Tranché par MaaaX le 2026-10-21, après mesure de la
référence de Rust amputée de ce que Shrimp refuse (assembleur en ligne,
macros, attributs de génération et de diagnostic, destructeurs,
coercions, élision des durées de vie, glossaire et index : 88 000
retirés, 216 000 restants) : « 100 000 tout compris, ou le plus près
possible ». Si le texte réel de l'étape 3 ne tient pas, le plafond se
révise au plus près par décision datée, jamais en passant.

- Le double de Go, sous Zig, sous les clauses du langage de C, le tiers
  de la référence de Rust. C'est ambitieux pour un langage qui porte les
  garanties G1 à G5, et c'est voulu : le plafond sert à refuser.
- Il laisse la moitié d'une fenêtre de 200 000 tokens au reste. Les
  fenêtres courantes vont, de mémoire et sans vérification, de 128 000 à
  un million ; le chiffre est pensé pour la plus petite des sérieuses,
  pas pour la plus grande.
- Il se mesure sur le texte réel à l'étape 3, et il se révise par
  décision de MaaaX, datée et motivée, jamais en passant
  (`docs/budget.md`, §3). Les sept documents de conception de `docs/`,
  celui-ci compris, pèsent 20 000 tokens au pire vocabulaire, pour situer
  l'échelle.

Écarté : 60 000, le niveau de Go, qui ne laisserait pas la place des
exemples que la bible exige à chaque entrée ; 150 000, qui passe dans un
million mais laisse 50 000 dans 200 000, trop peu pour écrire ; la paire
plafond 150 000 et objectif 100 000, qu'un plafond révisable par décision
datée rend inutile.

## 5. Un tronc commun et des sections : le livre I comme un jeu de skills

On n'a pas besoin de tout, tout le temps. Un programme qui n'emploie ni
`unsafe`, ni l'interop C, ni les fils d'exécution se lit sans leurs
chapitres. Le livre I prend donc la forme d'un **tronc commun**, toujours
chargé, et de **sections spécialisées**, chargées quand le programme en
a besoin (MaaaX, 2026-10-21 : « ça ressemble à des skills », et c'est
exactement ça — un corps court toujours présent, des références ouvertes
à la demande, un déclencheur par référence).

Trois règles, sans lesquelles la forme triche :

1. **Chaque section est étanche.** Elle ne change pas le sens de ce que
   le tronc dit : un programme qui ne l'emploie pas se lit et se vérifie
   avec le tronc seul. C'est G4, rien de caché, appliqué à la
   spécification elle-même.
2. **Le tronc porte l'index des sections**, une ligne par section qui dit
   quand la charger — « le programme contient `unsafe` → section N ».
   Sans cette ligne, l'IA ne sait pas ce qu'elle ne charge pas.
3. **Le plafond reste sur le tout**, tronc et sections. Sinon il fuit :
   chaque trait coûteux irait en section pour y échapper. Le tronc reçoit
   son propre chiffre, plus petit, quand le découpage existe.

Candidates évidentes à une section : `unsafe`, l'interop C, la
concurrence. Le découpage, la liste des sections et le chiffre du tronc
se fixent au point 1.7, la forme de la bible ; `tokcount` compte déjà par
fichier et en total, il n'a rien à apprendre. La taille d'une session
courante devient celle du tronc plus une ou deux sections, bien sous le
plafond.

## 6. Rejouer

```
tools/tokcount/measure.sh                     # les cinq références, sources épinglées
cargo run --release --manifest-path tools/tokcount/Cargo.toml -- count docs/*.md
```

Le premier a besoin de `curl`, `pdftotext` et `cargo` ; le second, de
`cargo` seul, et il ne touche pas au réseau : les vocabulaires sont dans
le dépôt. À l'étape 3, c'est le second qui tourne sur le livre I à chaque
version, et son résultat entre dans la bible avec le plafond en regard.

## Sources

Relues le 2026-10-21, empreintes SHA-256 dans `tools/tokcount/measure.sh`
et `tools/tokcount/vocab/README.md`.

- Go : https://raw.githubusercontent.com/golang/go/go1.25.0/doc/go_spec.html
- Zig : https://ziglang.org/documentation/0.15.1/
- Rust : https://github.com/rust-lang/reference, commit
  `3989fe91f1a601b51684a582ad17b2503f221787`
- C23 : https://www.open-std.org/jtc1/sc22/wg14/www/docs/n3220.pdf
- C++23 : https://www.open-std.org/jtc1/sc22/wg21/docs/papers/2023/n4950.pdf
- tiktoken 0.14.0, `tiktoken_ext/openai_public.py`, pour les motifs de
  prédécoupage et les empreintes des deux vocabulaires OpenAI
- `tokenizer.json` de Qwen/Qwen3-8B et de deepseek-ai/DeepSeek-V3, pour
  les tables de fusion, les motifs et la normalisation

# Cahier des charges — Shrimp

Ce que Shrimp doit être, et comment on saura qu'il l'est. Posé le
2026-10-20, avant toute ligne de spécification ou de code. Les décisions
sont dans `AGENTS.md` ; ici, la matière.

**Lu et approuvé, Maxime MARCONATO (aka MaaaX^^, aka EPOCH84), 2026-10-21.** Toute modification ultérieure
se date et se signe de la même façon.

## 1. Thèse

L'avenir de l'interface de programmation est le langage naturel, peu
importe la langue : à l'humain les idées, à l'IA la programmation.
L'artefact, lui, reste formel. On a déjà essayé de faire courir du
langage naturel — COBOL en 1959, SQL ensuite — et la précision a tout
fait, l'anglais rien. Il faut donc toujours une couche formelle sous la
couche d'intention : c'est elle qui tourne, c'est elle qu'on vérifie,
c'est le contrat entre l'idée et la machine, et entre l'IA d'aujourd'hui
et celle de demain.

Conséquence : quand l'IA écrit, le rôle de l'humain passe d'écrire à
relire. **Shrimp est optimisé pour la lecture et la vérification, pas
pour la vitesse d'écriture.** C'est l'inverse de quarante ans de
conception de langages, qui ont couru après la concision pour celui qui
tape.

## 2. Les deux lecteurs

### L'humain

- Lisible par quelqu'un qui n'a pas écrit le code, et qui le reprend au
  clavier quand l'IA est en panne — « quand l'ordinateur est en panne, il
  reste le stylo bille » : le code doit se lire imprimé.
- **Local** : une fonction se comprend en lisant la fonction. Rien ne
  vient d'ailleurs sans être nommé — ni flux de contrôle caché, ni
  allocation cachée, ni conversion implicite, ni macro qui réécrit la
  syntaxe, ni surcharge d'opérateur surprenante.
- **Petit** : le langage tient dans une tête. Une façon de faire chaque
  chose.
- **Explicite plutôt qu'astucieux** : la propriété se lit dans la
  signature ; l'erreur est une valeur ; le cas oublié ne compile pas.
- Mots-clés et conventions lisibles sans dictionnaire — la frontière
  avec le verbeux (Ada) est à trouver par l'usage.

### L'IA

Ce que l'IA demande, formulé par l'IA elle-même :

- **Un gros corpus stable.** Elle programme bien là où elle a beaucoup lu
  et où rien n'a bougé. Un langage neuf n'a pas de corpus : deux parades,
  une syntaxe proche de ce qu'elle connaît (Rust) pour que l'acquis se
  transfère, et une spécification assez courte pour être relue à chaque
  session.
- **Un compilateur qui la contredit.** Ses erreurs sont confiantes ; le
  vérificateur statique est son premier relecteur, avant l'humain. Les
  messages d'erreur sont conçus pour qu'une IA agisse dessus : précis,
  localisés, avec la correction possible et son coût.
- **Rien d'implicite** — mêmes raisons que pour l'humain : elle se perd
  dans ce qu'elle ne peut pas lire localement.
- **Zéro comportement indéfini** : elle ne débogue pas mieux que l'humain
  ce qui n'est pas reproductible. Construction déterministe.
- **La spécification du noyau tient dans sa fenêtre de contexte.**
  Critère neuf, propre à cette époque : si elle relit toute la référence
  du langage à chaque session, elle ne se trompe plus sur le langage,
  seulement sur le problème. C tient, Go tient, Zig tient en une page ;
  Rust non, C++ encore moins. Mesure en tokens, cible à fixer. *Mesuré
  le 2026-10-21, point 1.3, `docs/spec_taille.md` :* contre 200 000
  tokens, Go (56 519), Zig (117 907) et les clauses du langage de C
  (144 182) tiennent ; la référence de Rust (303 578) et C++ sans sa
  bibliothèque (380 718) non. Plafond de Shrimp : 100 000 tout compris.
- **La référence de `std` lisible par machine** : une signature, une
  ligne, un thème par entrée, pour vérifier au lieu de deviner.

Ce que l'IA n'est pas : infaillible. Plus rapide que la plupart des
humains pour écrire du code courant ; pas meilleure que les meilleurs
pour l'architecture, pour savoir quoi ne pas construire, ni pour une
course dans cinq cent mille lignes. Le garde-fou, c'est précisément que
l'humain puisse la relire. D'où tout ce qui précède.

## 3. Ambition

**Aussi fiable que Rust, beaucoup plus simple à lire.**

« Infiniment plus simple » n'existe pas : le vérificateur d'emprunt de
Rust est le prix de « sûreté mémoire, sans ramasse-miettes, sans
contrôle à l'exécution ». Retirer un des trois termes simplifie tout —
Go paie un ramasse-miettes, Zig paie des contrôles à l'exécution et de
la discipline, Ada/SPARK paie de la verbosité. Garder les trois et jeter
la complexité est un problème ouvert (Hylo le tente par la sémantique de
valeur mutable).

Le renversement qui rend l'ambition tenable : **la difficulté de Rust est
à l'écriture, pas à la lecture.** Se battre avec l'emprunteur revient à
l'IA. Ce qui reste sur le bureau de l'humain — génériques lourds,
asynchrone, macros procédurales — est un sous-ensemble évitable. Shrimp
part donc de la sémantique de Rust et attaque ses trois endroits
douloureux : l'incomplétude du vérificateur, les temps de compilation,
le budget de complexité. Cible, formulée comme un pari : les garanties
de Rust, la simplicité de Zig, les temps de compilation d'OCaml.

## 4. Plus rapide que le C

Possible, et déjà fait par d'autres, parce que la vitesse vient de ce
que le compilateur a le droit de supposer : Fortran bat C sur le calcul
parce que ses tableaux ne s'aliasent pas ; `&mut` exclusif en Rust vaut
`restrict` partout ; un `sort` générique bat `qsort` parce que le
comparateur est inliné ; Halide et ISPC battent le C écrit à la main
parce qu'ils expriment le parallélisme. **La rigueur qui rend un langage
fiable est la même qui le rend rapide ; le comportement indéfini est
l'ennemi des deux.** Plafond : rien ne bat le meilleur C avec
intrinsèques ; tout peut battre le C qu'on écrit réellement.

## 5. Garanties candidates

Tranchées le 2026-10-21 dans `docs/garanties.md` : cinq retenues sur
un critère, pas sur un nombre. Tout trait qui en contredit une est
refusé, même s'il est « le meilleur » ailleurs. La table ci-dessous est
la matière de départ, conservée telle quelle.

| Candidate | Qui l'a | Ce qu'elle coûte |
|---|---|---|
| Zéro comportement indéfini dans le code sûr | Rust | Un sous-langage `unsafe` à encadrer |
| Sûreté mémoire sans ramasse-miettes | Rust, Austral, Hylo | Un modèle de propriété, donc des programmes corrects refusés |
| Zéro course de données | Rust (`Send`/`Sync`), Erlang (par construction) | Un modèle de partage dans le langage |
| Rien de caché : ni flux de contrôle, ni allocation, ni conversion | Zig | Pas d'exceptions, pas de macros, pas de surcharge libre |
| Erreurs comme valeurs, cas exhaustifs | Rust, Zig | Pas d'exceptions |
| Compatibilité : ce qui compile aujourd'hui compile demain | Go (depuis 2012), C | Porte d'entrée étroite, éditions |
| Spécification bornée en taille | Scheme, Go | Refuser des traits utiles |
| Abstraction à coût nul | Rust, C++ | Monomorphisation, donc temps de compilation |

Toutes ne se combinent pas : le choix est fait, voir `docs/garanties.md`.

## 6. Non-objectifs

- Interface graphique dans `std` (Swing, `tkinter` : on sait ce que ça
  devient). *Amendé le 2026-10-21, point 1.2, MaaaX :* les IHM vivent
  dans `x/`, en enveloppes C sûres par système, sans abstraction
  commune ; détail dans `docs/budget.md`, §5.
- Rivaliser avec l'écosystème de Rust : Shrimp parle C à la frontière et
  peut appeler ce qui existe.
- Adoption par le monde comme critère de réussite : un langage pour
  deux d'abord. Ce qui tue les langages personnels — pas d'écosystème,
  personne ne le connaît — ne tient plus quand l'IA régénère les
  bibliothèques depuis la spec et que la spec tient dans son contexte.
  La documentation (§9) prépare l'adoption sans en dépendre.
- Nouveauté syntaxique : la syntaxe est la partie la moins importante et
  la plus disputée ; on garde celle des autres.
- « Le meilleur de chaque langage » : les traits ne s'additionnent pas,
  ils interagissent (exceptions et propriété, ramasse-miettes et
  destructeurs, macros et lisibilité, héritage et sémantique de valeur).

## 7. Contraintes

- **Interopérabilité C** obligatoire, dans les deux sens : tout langage
  doit parler C.
- **Cibles**, dans l'ordre : Linux, puis Windows, puis macOS, puis les
  mobiles, puis les autres — Raspberry Pi, ESP32, STM32 et consorts, où
  `core` tourne sans OS ni allocation. Multi-système et multi-processeur
  sont des objectifs ; multi-génération (Z80, 6502, 68000) est un
  souhait, tenu s'il ne coûte aucune contrainte au langage, lâché sinon.
  Le détail — niveaux de prise en charge, compilation croisée,
  WebAssembly, BSD — se fixe au point 1.6 de la feuille de route.
- **Un seul outil**, une seule chaîne, pas de choix d'exécuteur ou de
  formateur laissé à l'utilisateur.

## 8. Mesures

Rien à souder, mais tout à mesurer. Un chiffre par exigence, sinon
l'exigence est une opinion.

| Exigence | Mesure | Référence |
|---|---|---|
| Spec courte | Tokens de la spec du noyau, au pire de quatre vocabulaires figés (`tools/tokcount`) | 100 000 tout compris, fixé le 2026-10-21 (`docs/spec_taille.md`) ; Go 56 519, Zig 117 907, Rust 303 578 |
| Vite compilé | Secondes pour un programme de référence, à froid et incrémental | Rust, Go, OCaml sur le même programme |
| Code rapide | Temps d'exécution des mêmes programmes | C (`-O2`), Rust (`--release`) |
| Vérificateur pas trop strict | Programmes corrects refusés sur un corpus donné | Rust sur le même corpus |
| Lisible | Un humain qui n'a pas écrit le code en explique une fonction sans aide | MaaaX, sur du code écrit par l'IA |
| Stable | Programmes du corpus qui compilent encore après chaque version | 100 % après la 1.0 |
| Rien de caché | Allocations et appels non visibles dans le source | Zéro |

## 9. Documentation

Ajouté le 2026-10-21. Un langage se documente comme il se spécifie :
d'un bloc, et dans une seule langue source.

- **Une bible**, référence complète du langage — noyau, `core`, `std`,
  outillage —, écrite en anglais, comme tout ce qui appartient au
  langage. Livrée en ligne et en PDF, les deux engendrés depuis la même
  source.
- **Traduite dans les principales langues**, depuis l'anglais, pour
  faciliter l'adoption ; la version anglaise fait foi.
- **Chaque entrée de `core` et de `std` porte un exemple pratique
  réel**, commenté de manière didactique : pas un appel isolé, un
  programme qu'on aurait pu écrire pour de vrai, qui montre l'entrée
  dans son usage. Les exemples compilent et sont testés à chaque
  version : un exemple qui ne compile plus est un défaut de la
  bibliothèque.
- **L'index lisible par machine** (§2) est engendré depuis les mêmes
  sources que la bible ; rien n'est écrit deux fois.

## 10. Outillage éditeur

Ajouté le 2026-10-21 (MaaaX). Lire du code, c'est aussi le lire dans un
éditeur.

- **Une extension pour VS Code et VSCodium**, la même, publiée sur les
  deux places (Marketplace et Open VSX), livrée avec le langage et non
  laissée à la communauté.
- **Une coloration syntaxique conçue pour la lecture**, pas pour la
  décoration : ce qui demande l'attention du relecteur doit sauter aux
  yeux. En premier, **les blocs `unsafe`**, qui passent inaperçus dans
  la masse du code et sont précisément l'endroit où les garanties
  s'arrêtent : fond distinct, bordure, ou tout autre signal visible sur
  une page entière. Ensuite les types à consommer et leur consommation,
  les allocations nommées, les appels à la frontière C.
- **La coloration vient du compilateur**, par un serveur de langage,
  pour colorer ce que le code *est* et pas ce à quoi il ressemble :
  une grammaire de surface ne sait pas qu'une valeur est une ressource.
- Même exigence pour la version imprimée et le PDF de la bible : ce qui
  est signalé à l'écran l'est sur le papier.

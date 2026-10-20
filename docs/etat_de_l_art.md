# État de l'art — ce qu'on lit avant d'écrire

À remplir à l'étape 2 avec une grille fixe : pour chaque langage, ce
qu'il fait pour chacune des garanties retenues, et ce que ça lui coûte.
Ci-dessous, la liste de départ et la raison de chaque entrée. Les
mécanismes sont cités de mémoire : à relire à la source avant d'entrer
dans une décision.

## Langages

| Langage | Pourquoi le lire | Ce qu'on en prend, a priori | Ce qu'on évite |
|---|---|---|---|
| **Rust** | La base. Propriété, emprunt, `Send`/`Sync`, types somme, `Result`/`?`, traits, éditions, cargo | Les garanties et leur implémentation éprouvée | Les temps de compilation, l'asynchrone, les macros procédurales, les signatures illisibles |
| **Zig** | « Rien de caché » : ni flux de contrôle ni allocation cachés, allocateurs explicites, `comptime` à la place des génériques et des macros, erreurs et optionnels comme valeurs ; référence en une page | La doctrine de l'explicite, `comptime` | Pas sûr par construction ; pas en 1.0, casse à chaque version |
| **Go** | Spec lisible en une après-midi, promesse de compatibilité tenue depuis 2012, `std` complète, incubateur `x/` | La gouvernance de la bibliothèque, la stabilité | Le ramasse-miettes, `nil`, les courses non interdites |
| **Ada / SPARK** | Conçu pour être lu et maintenu trente ans par d'autres ; SPARK prouve l'absence d'erreur à l'exécution, et a emprunté à Rust un modèle de propriété pour ses pointeurs | La lisibilité, les contrats, la preuve | La verbosité, l'écosystème |
| **Hylo** | Sémantique de valeur mutable, zéro référence dans le langage : la tentative de garder les garanties de Rust en jetant la complexité | Le modèle, s'il tient | Recherche, pas stable |
| **Austral** | Types linéaires, spécification minuscule | La taille de la spec comme discipline | Niche |
| **Vale** | Références générationnelles : sûreté mémoire par contrôle à l'exécution bon marché | Un point de la grille sûreté/coût | Recherche |
| **Mojo** | Propriété sur une syntaxe Python, par l'auteur de LLVM et Swift | Ce qu'il a simplifié de Rust | Propriétaire, en mouvement |
| **Swift** | A commencé simple, est devenu énorme ; concurrence stricte à la Rust dans Swift 6 | Le contre-exemple du budget de complexité | — |
| **OCaml** | Types algébriques, inférence, temps de compilation très courts ; le premier compilateur de Rust a été écrit avec | La vitesse de compilation comme objectif | Le ramasse-miettes |
| **Erlang** | Pas de mémoire partagée : acteurs et messages, quarante ans en télécom | Le modèle de concurrence le plus simple qui soit fiable | Pas un langage système |
| **C** | L'ABI de l'industrie : tout langage doit lui parler | La frontière | Tout le reste |
| **Fortran** | Plus rapide que C sur le calcul parce que ses tableaux ne s'aliasent pas | La preuve que la rigueur fait la vitesse | — |
| **Halide, ISPC** | Battent le C écrit à la main en exprimant le parallélisme | Ce qu'un langage peut dire que C ne dit pas | Domaine restreint |
| **WLangage** | Bibliothèque énorme et cohérente, un préfixe par famille | La convention unique | Propriétaire, plateforme plutôt que langage |
| **PHP** | Bibliothèque énorme et incohérente | — | L'incohérence |

## Mécanismes et projets à suivre

- **Polonius**, nouvelle formulation du vérificateur d'emprunt de Rust ;
  **Tree Borrows**, modèle d'aliasing pour `unsafe` ; **Verus**,
  vérification de programmes Rust ; **Dafny**, **Lean**, pour ce que la
  preuve coûte et rapporte.
- **Backends** : LLVM (Rust, Swift, Zig, Clang), Cranelift (simple,
  compile vite), QBE (minuscule, utilisé par Hare), C émis (Nim ; cfront à
  l'origine de C++).
- **Incubation** : `golang.org/x/`, Boost → C++, `hashbrown` et
  `once_cell` → `std` de Rust, PEP 594 (retrait des piles mortes de
  Python).

## Livres et textes

- Pierce, *Types and Programming Languages* — le noyau mathématiquement
  compris sur lequel tout ce qui a réussi depuis vingt ans est bâti.
- Nystrom, *Crafting Interpreters* — la chaîne complète en pratique.
- Cooper et Torczon, *Engineering a Compiler* ; Aho, Lam, Sethi, Ullman,
  *Compilers* — le dragon, pour le backend.
- Dijkstra, *On the foolishness of « natural language programming »*
  (EWD667, 1978) — pourquoi l'artefact reste formel.

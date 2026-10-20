# Architecture — Shrimp

Proposée le 2026-10-20, à valider étape par étape. Deux objets : un
noyau gelé, une bibliothèque en trois étages.

## 1. Le noyau

Le noyau, c'est ce qu'on ne pourra plus changer. Petit, gelé tôt,
spécifié dans un texte qui tient dans la fenêtre de contexte d'une IA,
avec une implémentation de référence et une suite de tests qui fait foi.

Il contient, et rien d'autre :
- le **modèle de propriété et d'aliasing** — la seule chose qu'on ne
  greffe pas après coup (C++ essaie depuis quinze ans) ;
- le **modèle d'erreur** : valeurs, exhaustivité, propagation ;
- le **modèle mémoire** : ce qui est garanti entre fils ;
- le **système de modules** et la visibilité ;
- le **système de types** : somme, produit, génériques ou `comptime`,
  traits ou interfaces — à trancher ;
- la **syntaxe minimale**, empruntée, pas inventée ;
- la **frontière** : `unsafe` s'il existe, et l'ABI C.

La bibliothèque standard, même énorme, n'en fait pas partie.

## 2. Les trois étages

| Étage | Contenu | Règle |
|---|---|---|
| `core` | Sans OS, sans allocation : types de base, tranches, options et résultats, formatage minimal | Ce qui tient dans un microcontrôleur ; ce que Z80 pourrait viser |
| `std` | Tout ce dont 90 % des programmes ont besoin : texte et Unicode, collections, maths, temps, fichiers, réseau et HTTP, JSON, TOML, CSV, XML, hachage et TLS, expressions régulières, compression, processus, arguments, tests | Une seule convention partout. Écrite en Shrimp. Précompilée, liée à la demande : un gros `std` ne coûte rien à un petit binaire |
| `x/` | Incubateur officiel : même outillage, même gouvernance, autorisé à casser | La file d'attente vers `std` ; ce qui n'est jamais promu s'archive |

Modèles : `std` à la Go (un serveur de production sans dépendance),
cohérence à la WLangage (un préfixe par famille, même ordre de
paramètres partout — ici des modules par thème, forme moderne du même
geste), incubateur à la `golang.org/x/` (`context`, `slices`, `maps`,
`log/slog` y sont nés), antichambre à la Boost → C++.

Contre-modèles : PHP (énorme et incohérent : `strpos(botte, aiguille)`
mais `array_search(aiguille, botte)`), Python (`urllib` si pénible que
tout le monde installe `requests` ; dix-neuf modules morts retirés en
3.13), Java (`java.util.Date` cassée depuis le premier jour et toujours
là).

## 3. La convention unique de `std`

À écrire à l'étape 4, avant la première fonction. Au minimum :
- nommage : un mot par concept, le même partout ;
- ordre des paramètres : le sujet d'abord, toujours ;
- un seul type d'erreur par famille, propagé de la même façon ;
- documentation : une ligne par entrée, qui tient dans l'index.

## 4. Promotion de `x/` vers `std`

Critères, tous requis :
1. API inchangée depuis plusieurs années ;
2. une seule conception évidente — pas de promotion quand deux écoles
   s'affrontent (les exécuteurs asynchrones de Rust) ;
3. des mainteneurs qui remettent les clés ;
4. aucune dépendance hors `std` ;
5. tests et documentation au niveau de `std` ;
6. **réécriture aux conventions de `std` à l'entrée** : c'est là que
   « bien faite » devient « cohérente ».

Porte de sortie : **éditions**. Rien n'est jamais supprimé ; une édition
masque le déprécié. Go n'enlève jamais rien (d'où sa sévérité à
l'entrée), Python enlève dans la douleur, les éditions sont le compromis.

## 5. L'index

`std` est livrée avec sa référence lisible par machine : une entrée par
fonction, type ou constante — module, nom, signature, une ligne, thème,
édition d'apparition. Un fichier que l'IA charge pour vérifier une
signature au lieu de l'inventer, et que l'outillage engendre depuis les
sources, jamais à la main.

## 6. Outillage

Un seul exécutable, `shrimp` : `build`, `run`, `test`, `fmt`, `doc`,
`index`. Pas de `shrimpc` séparé, pas de choix de formateur. Extension
des sources : `.shr` ou `.shrimp` (`.sh` est pris).

Premier compilateur : en Rust, avec un backend choisi pour arriver vite
à un programme qui tourne — C émis, Cranelift ou LLVM. Auto-hébergement
plus tard, quand le langage le mérite.

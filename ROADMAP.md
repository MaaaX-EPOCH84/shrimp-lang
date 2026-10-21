# Feuille de route — Shrimp

Statut : ⬜ à faire · 🔶 en cours · ✅ fait · ❌ annulé

Squelette validé par MaaaX le 2026-10-21, étape 1 détaillée le même jour.

| # | id | Étape | Contenu | État |
|---|---|---|---|---|
| 1 | E1 | Décisions | Garanties, budget de complexité, non-objectifs, spec en tokens, licence, outil, cibles, forme de la bible | 🔶 |
| 2 | E2 | État de l'art | Grille fixe par langage et par garantie retenue, avec le coût | ⬜ |
| 3 | E3 | Noyau | Spec gelée et mesurée en tokens ; livrable : Bible, livre I | ⬜ |
| 4 | E4 | Bibliothèque I | Trois étages, éditions, convention unique, format de l'index | ⬜ |
| 5 | E5 | Bibliothèque II | Carte des thèmes de `core` et `std`, inventaire, exclusions ; livrable : Bible, livre II | ⬜ |
| 6 | E6 | Audit I | Relecture point par point des étapes 1-5, cohérence noyau / bibliothèque / bible | ⬜ |
| 7 | E7 | Prototype | Compilateur en Rust, backend, premier programme, mesures | ⬜ |
| 8 | E8 | Bibliothèque III | `core` en Shrimp, sans OS ni allocation | ⬜ |
| 9 | E9 | Bibliothèque IV | `std` premier palier, de quoi écrire de vrais programmes | ⬜ |
| 10 | E10 | Outillage | `shrimp build`, `test`, `fmt`, `doc`, `index` | ⬜ |
| 11 | E11 | Audit II | Relecture des étapes 7-10 : le prototype contre la spec, écarts corrigés | ⬜ |
| 12 | E12 | Épreuve | Programmes réels, IA au clavier, MaaaX relit ; refus comptés | ⬜ |
| 13 | E13 | Bibliothèque V | `std` paliers suivants par thème, incubateur `x/`, promotion, archivage | ⬜ |
| 14 | E14 | Bible | Livre complet assemblé, index engendré, puis traductions | ⬜ |
| 15 | E15 | Audit III | Relecture globale avant le gel : noyau, bible, mesures | ⬜ |

## Étape 1 — Décisions  [E1]
1.1 ✅ [P1] Les garanties non négociables — `docs/garanties.md`, lu et approuvé par MaaaX le 2026-10-21. Cinq retenues sur critère (vérifiable mécaniquement, interdit quelque chose de nommable, non déductible d'une autre) : G1 zéro UB hors `unsafe`, G2 durée de vie déterminée à la compilation, G3 zéro course de données, G4 rien de caché, G5 pas de cas oublié. Écartées : sûreté mémoire (déduite de G1, reformulée en G2), erreurs comme valeurs (absorbée par G4 et G5), compatibilité et spec bornée (gouvernance, 1.2 et 1.3), abstraction à coût nul (n'interdit rien). Arbitrage majeur : pas de destructeur, les ressources sont des types à consommer explicitement (choix de MaaaX contre le `Drop` à la Rust proposé d'abord). Dix paires arbitrées, 23 verdicts, sources relues sauf Go 2012
1.2 ✅ [P2] Le budget de complexité et les non-objectifs — `docs/budget.md`, lu et approuvé par MaaaX le 2026-10-21. Budget = grille de sept questions dans l'ordre (Q0 veto des garanties, Q1 bibliothèque, Q2 mesure servie, Q3 programme témoin, Q4 une seule façon, Q5 chaque interaction réglée, Q6 coût en tokens), le premier non arrête ; plafond en tokens de spec à chiffrer en 1.3, tout admis d'ici là sous réserve ; admettre irréversible, refuser réversible, dans le doute refuser. Verdicts : `defer` refusé Q4 (double la consommation imposée ; contrainte pour l'étape 3 : consommation sur tous les chemins écrite une fois, sans destructeur), `async`/`await` refusé Q5 (future annulée = ressource non consommée sans destructeur ; `std` doublée), héritage refusé Q5 (hiérarchie ouverte contre G5, slicing contre la sémantique de valeur), inférence locale refusée Q3 (MaaaX : strict, tout type écrit, un type inféré est du code implicite à la lecture ; l'assistant proposait admis borné). Non-objectifs : quatre confirmés, IHM amendé (enveloppes C sûres par système dans `x/`, sans abstraction commune), amendement reporté au cahier §6 daté et signé ; pas de sixième. Écarté : un plafond seul sans grille, ou une grille sans plafond. Sources relues : Go spec et FAQ, Zig reference, Rust std, async book, book, edition guide
1.3 ⬜ [P3] La taille de la spec du noyau — chiffre en tokens, tokenizer de référence, script de comptage rejouable
1.4 ⬜ [P4] La licence — GPLv3 ou MIT/Apache-2.0 ; fichier LICENSE posé
1.5 ⬜ [P5] L'outil, l'extension et l'identité — `shrimp` seul, `.shr` ou `.shrimp`, mascotte
1.6 ⬜ [P6] Les cibles et la frontière — ordre des cibles, interop C dans les deux sens confirmée, ce que `core` doit tenir sur ESP32-C3
1.7 ⬜ [P7] La forme de la bible — découpage en livres, format source, lieu dans le dépôt, règle de traduction
1.8 ⬜ [P8] Le protocole de mesure — pour chaque ligne du cahier §8 : programmes de référence, langages comparés, machine, script rejouable

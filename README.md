# Shrimp

> **Projet en cours de conception — rien n'est implémenté, rien n'est
> utilisable.** Ce dépôt est public pour être lu, pas pour être installé.
> *Work in progress: the language is being designed, nothing runs yet.*

Un langage de programmation pour deux lecteurs : l'humain, qui a les
idées et relit, et l'IA, qui écrit. Le compilateur arbitre.

**État : conception. Rien n'est implémenté.** Première pierre posée le
20 octobre 2026, sans engagement de délai.

Ambition en une ligne : les garanties de Rust, beaucoup plus simple à
lire. *Jumbo shrimp* : un noyau minuscule, une bibliothèque énorme.

- `docs/cahier_des_charges.md` — ce que Shrimp doit être, et comment on le mesurera
- `docs/architecture.md` — noyau gelé, trois étages de bibliothèque, incubateur
- `docs/garanties.md` — les cinq garanties non négociables, et les candidates écartées
- `docs/budget.md` — la grille « chaque trait paie sa place », le plafond, les non-objectifs
- `docs/spec_taille.md` — le plafond en tokens de la spec du noyau, et ce que pèsent C, C++, Go, Zig et Rust
- `docs/etat_de_l_art.md` — ce qu'on lit avant d'écrire une ligne
- `docs/nom.md` — le nom, et ceux qu'on a écartés
- `docs/licence.md` — pourquoi MIT ou Apache-2.0, et pas GPLv3
- `tools/tokcount/` — le compteur de tokens, quatre vocabulaires figés, un moteur, le pire score

## Licence

MIT ou Apache-2.0, au choix du receveur : `LICENSE-MIT`,
`LICENSE-APACHE`. Une seule licence pour tout, code et documents.
Fichiers tiers redistribués : `THIRD_PARTY.md`.

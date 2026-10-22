# Licence — Shrimp

Décision du point 1.4 de la feuille de route, 2026-10-22. Le dépôt
était privé jusqu'ici : sans licence, un dépôt public est « tous droits
réservés ». Ici : la licence retenue, ce qu'elle couvre, pourquoi, et ce
qui a été écarté.

**Lu et approuvé, Maxime MARCONATO (aka MaaaX^^, aka EPOCH84),
2026-10-22.** Toute modification ultérieure se date et se signe.

## 1. Décision

**Double licence MIT ou Apache-2.0, au choix du receveur**, comme Rust.
Tranché par MaaaX le 2026-10-22 entre les deux options ouvertes depuis
le 2026-10-20, GPLv3 et MIT/Apache-2.0.

- `LICENSE-MIT` : texte MIT de référence (SPDX), titulaire « 2026
  MaaaX-EPOCH84 and the Shrimp contributors ».
- `LICENSE-APACHE` : texte Apache-2.0 de référence, copie octet pour
  octet de `apache.org/licenses/LICENSE-2.0.txt` (SHA-256
  `cfc7749b…523d30`, identique à `/usr/share/common-licenses/Apache-2.0`
  de Debian).
- `THIRD_PARTY.md` : les quatre vocabulaires figés de `tools/tokcount/`,
  redistribués sous leur propre licence (MIT pour OpenAI et DeepSeek,
  Apache-2.0 pour Qwen).
- `Cargo.toml` de `tokcount` : `license = "MIT OR Apache-2.0"`.
- Chaque fichier source porte en tête la mention de licence et la ligne
  `SPDX-License-Identifier: MIT OR Apache-2.0`, dans l'en-tête commun
  du dépôt (modèle : `tools/tokcount/src/main.rs`).

## 2. Périmètre

**Une seule licence pour tout** : compilateur, `core`, `std`, outillage,
documents de conception de `docs/`, et la bible à venir. Tranché par
MaaaX le 2026-10-22. Rust fait de même : `rust-lang/book` et
`rust-lang/reference` portent les deux mêmes fichiers que le compilateur
(vérifié le 2026-10-22 sur les dépôts). Une licence de documentation à
part (CC BY 4.0) aurait été une règle de plus à expliquer, pour un gain
nul : les deux textes couvrent déjà « software and associated
documentation files ».

Le titulaire nomme l'auteur sans fermer la porte : « MaaaX-EPOCH84 and
the Shrimp contributors ». Pas de cession de droits demandée aux
contributeurs, chacun garde les siens, comme Rust (`COPYRIGHT`).

## 3. Pourquoi pas GPLv3

L'argument décisif tient au langage, pas au compilateur. `core` et
`std` sont liées à chaque programme Shrimp. Sous GPLv3, tout programme
compilé serait une œuvre dérivée de la bibliothèque, donc GPL, sauf
exception écrite. GCC vit avec cette exception depuis le début : la
*GCC Runtime Library Exception*, version 3.1 du 31 mars 2009, qui
autorise la propagation du code cible lié à `libgcc` et consorts sous
d'autres termes (vérifié le 2026-10-22 sur gnu.org). C'est un texte
juridique de plus à écrire, à maintenir et à faire comprendre, à
l'opposé de « rien de caché ».

Pour un compilateur seul, GPLv3 aurait été défendable, c'est le choix de
`foregrampm`. Mais Shrimp n'est pas un outil, c'est un langage : ce qui
compte est ce que les programmes écrits avec lui peuvent devenir, et la
réponse doit être « n'importe quoi ».

## 4. Pourquoi les deux et pas une seule

- **Apache-2.0** apporte la concession explicite de brevets (§3) et la
  clause de résiliation en cas d'attaque en contrefaçon de brevet. MIT
  n'en dit rien.
- **MIT** reste compatible avec le code sous GPLv2 seule, ce que
  Apache-2.0 n'est pas selon la FSF. Le noyau Linux est sous GPLv2.
- Le choix laissé au receveur coûte un fichier de plus et aucune
  phrase de plus dans le langage. Rust l'a éprouvé, et l'écosystème
  Cargo le note en une expression SPDX,
  `MIT OR Apache-2.0`, que `tokcount` porte désormais.

## 5. Écarté

- **GPLv3 seule** : §3.
- **MIT seule ou Apache-2.0 seule** : §4.
- **Licence de documentation à part** : §2.
- **MPL-2.0** : copyleft par fichier, intermédiaire défendable pour une
  bibliothèque, mais l'intérêt de Shrimp n'est pas de retenir des
  corrections, c'est d'être lu et vérifié ; et c'est une troisième
  famille à expliquer.
- **Fichier `COPYRIGHT` à la Rust** : inutile tant que le dépôt n'a
  qu'un auteur et quatre fichiers tiers ; `THIRD_PARTY.md` suffit, à
  revoir si l'outillage REUSE devient nécessaire.

## 6. Conséquence immédiate

Le dépôt GitHub passe en public dès que ce point est commité et
poussé, avec en tête du `README.md` l'avertissement que le projet est
en cours de conception et que rien n'est implémenté (MaaaX,
2026-10-22).

## Sources

- https://github.com/rust-lang/rust — `README.md` §License,
  `LICENSE-MIT`, `LICENSE-APACHE`, `COPYRIGHT`, lus le 2026-10-22.
- https://github.com/rust-lang/book et
  https://github.com/rust-lang/reference — racines listées le
  2026-10-22, `LICENSE-MIT` et `LICENSE-APACHE` présents.
- https://www.apache.org/licenses/LICENSE-2.0.txt — téléchargé le
  2026-10-22, empreinte ci-dessus.
- https://spdx.org/licenses/MIT.txt — téléchargé le 2026-10-22.
- https://www.gnu.org/licenses/gcc-exception-3.1.en.html — lu le
  2026-10-22.
- https://www.gnu.org/licenses/license-list.en.html#apache2 — lu le
  2026-10-22 : « not compatible with GPL version 2 », à cause des
  clauses de résiliation de brevet et d'indemnisation.

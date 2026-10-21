# Budget de complexité et non-objectifs — Shrimp

Décision du point 1.2 de la feuille de route, 2026-10-21. Les garanties
(`docs/garanties.md`) disent ce qui est refusé d'office ; ici, comment
se décide tout le reste — ce qu'aucune garantie ne tranche —, et ce que
le projet déclare ne pas poursuivre. Les faits sur les autres langages
ont été relus à la source le jour de la décision ; les sources sont en
fin de document.

**Lu et approuvé, Maxime MARCONATO (aka MaaaX^^, aka EPOCH84),
2026-10-21.** Toute modification ultérieure se date et se signe.

Quatre arbitrages pris par MaaaX avant écriture, le 2026-10-21 : le
budget est une grille pour admettre et un plafond pour refuser ; les
quatre traits renvoyés par 1.1 se tranchent ici ; les non-objectifs
restent cinq ; l'IHM reçoit un amendement.

## 1. Ce qu'est le budget

Un trait qu'aucune garantie ne refuse n'est pas admis pour autant. Il
coûte, et il coûte dans une monnaie bornée : **la spécification du
noyau doit tenir dans la fenêtre de contexte d'une IA** (cahier §8,
chiffre au point 1.3). Chaque trait y dépense ses règles, ses cas
limites et ses interactions avec les traits déjà admis.

Deux propriétés font du budget une porte à sens unique :

- **Admettre est irréversible.** Ce qui compile aujourd'hui compile
  demain ; une édition masque, elle ne supprime pas
  (`docs/architecture.md`, §4). Un trait admis reste dans le langage
  pour toujours, même déprécié.
- **Refuser est réversible.** Rien n'empêche d'admettre plus tard un
  trait refusé aujourd'hui, si un programme témoin montre qu'il manque.

**Dans le doute, refuser.** L'asymétrie commande la règle.

Le budget tranche en deux temps : une **grille**, qui dit si un trait
paie sa place ; un **plafond**, qui dit s'il reste de la place. La
grille s'applique dès aujourd'hui ; le plafond attend son chiffre.

## 2. La grille : « chaque trait paie sa place »

Sept questions, dans cet ordre. **Le premier « non » arrête** : le
trait est refusé, et la question qui l'a refusé est archivée avec lui.
Un trait admis a répondu aux sept, et ses réponses sont archivées
aussi, parce qu'elles sont la spec en germe.

| # | Question | Réponse qui fait passer |
|---|---|---|
| Q0 | Une garantie le refuse-t-elle ? | Non. Déjà fait en 1.1 pour les traits connus ; refait pour tout trait nouveau |
| Q1 | Peut-il vivre en bibliothèque ? | Non. S'il le peut, il y va — `std` ou `x/` — et ne coûte rien au noyau |
| Q2 | Quelle mesure du cahier §8 sert-il, ou quelle garantie l'exige ? | Au moins une, nommée. Un trait qui n'avance aucune mesure ne paie rien |
| Q3 | Le programme témoin est-il plus lisible avec que sans ? | Oui, côte à côte : le même programme écrit avec les seuls traits déjà admis, et avec le candidat. Si le « sans » se lit aussi bien, le candidat est redondant |
| Q4 | Double-t-il une façon de faire déjà admise ? | Non, ou il la remplace entièrement. Une seule façon de faire chaque chose |
| Q5 | Avec quels traits admis interagit-il, et chaque interaction a-t-elle sa règle ? | Toutes listées, toutes réglées. Une interaction sans règle est un refus : c'est l'endroit exact où les langages « meilleur de chaque » se cassent (cahier §6) |
| Q6 | Que coûte-t-il en tokens de spec ? | Une estimation à l'entrée, la mesure sur le texte réel à l'étape 3 ; sous le plafond, ou le trait attend |

Trois verdicts possibles : **Admis**, **Refusé** (avec la question),
**Reporté** (grille passée, plafond non encore chiffré ou texte de spec
non encore écrit ; le trait n'entre pas tant que la mesure manque).

**Qui l'applique.** L'IA instruit : elle écrit les sept réponses et le
programme témoin. MaaaX tranche, date et signe. Un trait n'entre pas
dans le noyau sans sa grille archivée.

**Ce que la grille ne juge pas.** La syntaxe, qui se copie (cahier §6),
et la vitesse du code produit, qui se mesure (point 1.8). La grille
juge ce qu'un trait coûte à lire et à spécifier.

## 3. Le plafond

Le plafond est un nombre de tokens, celui de la spec du noyau, fixé au
point 1.3 avec le tokenizer et le script qui le mesurent. Jusque-là :

- la grille seule décide, et tout « Admis » rendu avant 1.3 l'est
  **sous réserve du plafond** ;
- un trait admis par la grille mais qui ferait dépasser le plafond est
  **Refusé**, ou le plafond est relevé par une décision de MaaaX, datée
  et motivée. Il ne se relève jamais en passant ;
- ce qu'une édition masque sort du texte courant de la spec et va en
  annexe, hors plafond : l'IA qui écrit dans l'édition courante n'a pas
  à le charger. L'annexe reste normative pour les éditions antérieures.

Avant la version 1.0, rien n'est gelé : la grille s'applique, le
plafond se mesure, les verdicts se révisent à l'audit (étapes 6, 11,
15). À partir de 1.0, la porte est à sens unique.

## 4. Les quatre traits renvoyés par le point 1.1

### `defer` — Refusé (Q4)

**Ce que c'est.** Go : « A "defer" statement invokes a function whose
execution is deferred to the moment the surrounding function returns ».
Zig : « Executes an expression unconditionally at scope exit », en
ordre inverse.

**La grille.** Q0 : visible, aucune garantie ne tranche (1.1). Q1 : non,
c'est du flux de contrôle. Q2 : il servirait « lisible » en rapprochant
la libération de l'acquisition. Q3 : le programme témoin est une
fonction qui ouvre un fichier, fait trois opérations qui peuvent
échouer, et le ferme. Avec `defer`, la fermeture est écrite en haut et
exécutée en bas : le lecteur doit tenir en tête une action différée,
ce que G4 tolère mais n'aime pas. Sans `defer`, la fermeture est une
consommation explicite que le compilateur impose sur chaque chemin de
sortie (arbitrage G2 × G4 du point 1.1). **Q4 : il double la
consommation imposée.** Deux façons de libérer une ressource, l'une
vérifiée par le typage, l'autre par une convention de placement. Refus.

**Ce que le refus laisse à l'étape 3.** Le besoin derrière `defer` est
réel : avec un opérateur de propagation à la `?`, une ressource vivante
doit être consommée avant chaque propagation, et l'écrire à chaque
ligne est illisible. Contrainte pour la conception des ressources :
**la consommation sur tous les chemins de sortie doit pouvoir s'écrire
une seule fois, visiblement, sans destructeur.** Le mécanisme — bloc à
ressource, ou autre — est du ressort de la spec, sous G4.

### `async`/`await` — Refusé (Q5)

**Ce que c'est.** Rust : « Use `async` in front of `fn`, `closure`, or
a `block` to turn the marked code into a `Future`. As such the code
will not be run immediately, but will only be evaluated when the
returned future is `.await`ed » ; « Futures alone are *inert*; they
must be *actively* `poll`ed for the underlying computation to make
progress ». L'exécuteur n'est pas fourni : « Rust lets you choose one
depending on your requirements, rather than providing one ».

**La grille.** Q0 : visible, pas tranché (1.1). Q1 : l'exécuteur, oui ;
la transformation d'une fonction en machine d'états, non — elle est
dans le compilateur. Q2 : aucune mesure du §8 ; ce qu'il sert, c'est
l'écriture de serveurs, que `std` doit permettre (réseau, HTTP) mais
que des fils et G3 permettent aussi. Q3 : le programme témoin — un
serveur qui sert dix connexions — se lit mieux en fils synchrones :
chaque connexion est une fonction ordinaire. Q4 : il double le modèle
de concurrence par fils, et **il double `std`** : chaque fonction
bloquante demande sa jumelle asynchrone (Rust a `std::fs` et
`tokio::fs`), ce qui contredit « une seule convention partout ». Ce
serait déjà un refus. **Q5 : une interaction sans règle.** Une future
abandonnée avant sa fin tient des ressources ; en Rust, l'annulation
est un `Drop`. Shrimp n'a pas de destructeur (1.1) : une future
annulée est une ressource non consommée, et aucune règle ne dit qui la
consomme. Refus.

**Ce que le refus laisse.** La concurrence de Shrimp est celle de G3 :
fils, et un modèle de partage typé à fixer à l'étape 3. Si un programme
témoin montre un jour qu'un serveur en fils échoue à une mesure du §8,
le trait repasse par la grille avec une règle pour l'annulation — ou
vit en bibliothèque dans `x/`, sans syntaxe.

### Héritage — Refusé (Q5)

**Ce que c'est.** Rust n'en a pas : « There is no way to define a
struct that inherits the parent struct's fields and method
implementations without using a macro. » Ses deux usages, d'après le
même texte : réutiliser du code, et utiliser un type fils là où le
type parent est attendu. Go non plus : « Object-oriented programming,
at least in the best-known languages, involves too much discussion of
the relationships between types ».

**La grille.** Q0 : pas tranché (1.1). Q1 : non. Q2 : « lisible »,
pour la réutilisation. Q3 : le programme témoin — trois formes avec
une aire — se lit aussi bien en type somme fermé, et mieux quand il
faut savoir ce qu'une valeur peut être. Q4 : la réutilisation est déjà
servie par la composition et les génériques (admis en 1.1) ; la
substitution le sera par le mécanisme de polymorphisme de l'étape 3.
**Q5 : deux interactions sans règle.** Avec G5 : une hiérarchie est
ouverte, un fils peut être ajouté ailleurs, donc un filtrage sur la
hiérarchie n'est jamais exhaustif — G5 ne peut pas s'appliquer. Avec
la sémantique de valeur et G2 : un fils copié dans un parent perd ses
champs (le *slicing* de C++), ou bien tout passe par référence, ce qui
impose l'allocation là où G4 veut la voir écrite. Refus.

**Ce que le refus laisse.** Types somme fermés, composition, et un
polymorphisme par interface dont la forme — statique par génériques,
dynamique par type déclaré — se fixe à l'étape 3 sous G4.

### Inférence de types locale — Refusé (Q3)

**Ce que c'est.** Rust : « The compiler can usually infer what type we
want to use based on the value and how we use it. In cases when many
types are possible … we must add a type annotation ». Avec une borne :
« In function signatures, you *must* declare the type of each
parameter. … Requiring type annotations in function definitions means
the compiler almost never needs you to use them elsewhere ».

**La grille.** Q0 : rien ne s'exécute, G4 ne tranche pas (1.1) — le
verdict de `garanties.md` tient, ce n'est pas G4 qui refuse. Q1 : non.
Q2 : « lisible », en retirant les annotations des variables locales.
**Q3 : le programme témoin se lit mieux avec les types écrits.** Le
lecteur de référence de la mesure « lisible » (cahier §8) est MaaaX,
et son verdict, le 2026-10-21 : un type inféré est du code implicite
à la lecture ; un type écrit ne laisse aucun doute. Sans outil, sans
IDE, au stylo bille, `let n = f(x)` oblige à aller lire `f` ;
`let n: u32 = f(x)` se lit sur place. Refus.

**Ce que le refus coûte, et à qui.** Écrire chaque type est long et
parfois lourd — un type générique imbriqué sur un `let`. Ce coût est
à l'écriture, donc à l'IA : c'est exactement le renversement du cahier
§3, la difficulté se déplace vers celui qui écrit, la lecture reste
simple. Le compilateur, lui, vérifie au lieu de deviner : une
annotation fausse est une erreur, pas une inférence.

**Ce que le refus laisse à l'étape 3.** Shrimp est strict : toute
liaison — variable locale, paramètre, retour, champ, constante,
paramètre de fermeture — porte son type écrit. Reste à la spec de
fixer comment un littéral prend le sien (annotation de la liaison,
suffixe, ou les deux) sans promotion silencieuse, et de s'assurer que
les types restent courts à écrire : des alias nommés, pas de
l'inférence.

## 5. Les non-objectifs

Les cinq du cahier §6, relus un par un. Pas de sixième : les verdicts
de garanties (pas de macros, pas de surcharge d'opérateurs, pas
d'exceptions) sont des refus motivés, archivés dans `garanties.md`, pas
des non-objectifs.

| Non-objectif | Sort | Motif |
|---|---|---|
| Interface graphique dans `std` | **Amendé** | « Pas dans `std` » ne disait pas où. Tranché : les IHM vivent dans `x/`, sous la forme d'enveloppes C sûres **par système** — une pour chaque boîte à outils native —, **sans abstraction commune** au-dessus. Shrimp n'unifie rien : chaque système expose la sienne. C'est précisément l'abstraction commune (Swing, `tkinter`) que le non-objectif visait, et elle reste refusée |
| Rivaliser avec l'écosystème de Rust | Confirmé | La frontière C est un chapitre du noyau (1.1, §2 bis) ; c'est elle qui rend l'existant appelable |
| Adoption par le monde comme critère de réussite | Confirmé | Un langage pour deux, d'abord ; la mesure « lisible » se fait avec MaaaX, pas avec un sondage |
| Nouveauté syntaxique | Confirmé | La grille ne juge pas la syntaxe (§2) ; elle se copie |
| « Le meilleur de chaque langage » | Confirmé, et outillé | Q5 est sa traduction mécanique : une interaction sans règle refuse le trait |

L'amendement est reporté dans le cahier §6, daté et signé.

## 6. Sources relues le 2026-10-21

- Go, *The Go Programming Language Specification*, « Defer
  statements » : « A "defer" statement invokes a function whose
  execution is deferred to the moment the surrounding function returns,
  either because the surrounding function executed a return statement,
  reached the end of its function body, or because the corresponding
  goroutine is panicking. »
- Zig, *Language Reference* (master), « defer » : « Executes an
  expression unconditionally at scope exit. » ; « Defer expressions are
  evaluated in reverse order. »
- Rust, documentation de `std`, mot-clé `async` : « Use `async` in
  front of `fn`, `closure`, or a `block` to turn the marked code into a
  `Future`. As such the code will not be run immediately, but will only
  be evaluated when the returned future is `.await`ed. »
- Rust, `std::future::Future` : « Futures alone are *inert*; they must
  be *actively* `poll`ed for the underlying computation to make
  progress ».
- Rust, *Asynchronous Programming in Rust*, « async and await » :
  « Rust lets you choose one depending on your requirements, rather
  than providing one. » ; « You'll need to include the Tokio crate as
  a dependency in your Cargo.toml ».
- Rust, *The Rust Programming Language*, « Characteristics of
  Object-Oriented Languages » : « If a language must have inheritance
  to be object oriented, then Rust is not such a language. There is no
  way to define a struct that inherits the parent struct's fields and
  method implementations without using a macro. » ; « Inheritance is
  often at risk of sharing more code than necessary. »
- Go, *FAQ*, « Why is there no type inheritance? » : « Object-oriented
  programming, at least in the best-known languages, involves too much
  discussion of the relationships between types, relationships that
  often could be derived automatically. Go takes a different approach. »
- Rust, *The Rust Programming Language*, « Data Types » : « The
  compiler can usually infer what type we want to use based on the
  value and how we use it. In cases when many types are possible … we
  must add a type annotation ». « Functions » : « In function
  signatures, you *must* declare the type of each parameter. This is a
  deliberate decision in Rust's design: Requiring type annotations in
  function definitions means the compiler almost never needs you to use
  them elsewhere in the code to figure out what type you mean. »
- Rust, *The Rust Edition Guide*, « What are editions? » : « crates in
  one edition **must** seamlessly interoperate with those compiled with
  other editions » ; « Editions do not split the ecosystem ». C'est le
  modèle de la porte de sortie du §3.

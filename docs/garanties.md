# Garanties non négociables — Shrimp

Décision du point 1.1 de la feuille de route, 2026-10-21. Les
candidates sont dans `docs/cahier_des_charges.md`, §5 ; ici, ce qui est
retenu, ce qui est écarté, et pourquoi. Les faits sur les autres
langages ont été relus à la source le jour de la décision ; les sources
sont en fin de document.

**Lu et approuvé, Maxime MARCONATO (aka MaaaX^^, aka EPOCH84),
2026-10-21.** Toute modification ultérieure se date et se signe.

## 1. Le critère

Le nombre de garanties n'est pas fixé d'avance. Une propriété mérite le
statut de garantie si, et seulement si, elle remplit les trois
conditions :

1. **Vérifiable mécaniquement** — par le compilateur, ou par un test
   rejouable. Sinon c'est un vœu.
2. **Elle interdit quelque chose de précis et nommable** — un trait, une
   construction, un comportement. Une garantie qui n'interdit rien ne
   tranche rien.
3. **Elle ne se déduit pas d'une autre** — sans quoi elle n'est qu'un
   corollaire, et le compter double les paires à arbitrer.

Pourquoi en avoir peu, quand même : chaque paire de garanties se
dispute, « non négociable » s'use quand la liste s'allonge, et chaque
garantie est un chapitre de spec dans une fenêtre de contexte bornée.

Une garantie est un **droit de veto** : tout trait qui en contredit une
est refusé, même excellent ailleurs. Ce qu'aucune garantie ne refuse
n'est pas accepté pour autant : il passe au budget de complexité (point
1.2).

## 2. Les garanties retenues

Cinq. Chacune est écrite comme une phrase testable sur un programme.

### G1 — Zéro comportement indéfini hors d'un bloc `unsafe` nommé

**Énoncé.** Tout programme accepté par le compilateur a un comportement
défini par la spécification, pour toute entrée, sauf à l'intérieur de
blocs marqués `unsafe`, dont le programmeur répond. Le code sûr ne peut
pas déclencher de comportement indéfini, même en appelant du code
`unsafe` correct.

**Vérification.** Le compilateur ; et un interpréteur de référence qui
détecte toute violation sur la suite de tests (ce que Miri fait pour
Rust).

**Interdit.** Lecture de mémoire non initialisée, déréférencement de
pointeur invalide, dépassement d'entier à sémantique laissée ouverte,
accès hors bornes non contrôlé, toute « optimisation » fondée sur un
comportement non spécifié. Chaque opération a un résultat défini :
valeur, erreur, ou arrêt du programme — jamais « n'importe quoi ».

**Coût.** Un sous-langage `unsafe` à spécifier (modèle d'aliasing
compris) ; des contrôles à l'exécution là où le type ne prouve rien
(bornes, division), donc du code en moins vite que du C qui ne contrôle
rien.

**Qui l'a.** Rust : « If all you do is write Safe Rust, you will never
have to worry about type-safety or memory-safety. You will never endure
a dangling pointer, a use-after-free, or any other kind of Undefined
Behavior » (Nomicon). Austral : « well-defined semantics for all
arithmetic operations ». Zig ne l'a pas : il utilise « Illegal
Behavior » comme outil, contrôlé ou non selon le mode de compilation.

### G2 — Durée de vie déterminée à la compilation

**Énoncé.** Pour toute ressource — mémoire, fichier, verrou — l'endroit
où elle est libérée se lit dans le source et se décide à la compilation.
Pas de ramasse-miettes, pas de comptage de références implicite, pas de
finaliseur à un instant inconnu.

**Vérification.** Le compilateur, par le modèle de propriété (étape 3).
Test rejouable : `core` tourne sans allocateur ni exécutif ; aucun
symbole de ramasse-miettes dans un binaire.

**Interdit.** Le ramasse-miettes (Go, Java, OCaml, Swift pour le
comptage automatique), les finaliseurs, et tout partage de propriété
qui ne soit pas écrit dans un type nommé. Un partage explicite, à la
`Rc`, reste possible ; comme il n'y a pas de destructeur (G4), la façon
dont le dernier propriétaire rend la mémoire est un point de l'étape 3.

**Pourquoi elle ne se déduit pas de G1.** La sûreté mémoire, elle, se
déduit de G1 : un usage après libération est un comportement indéfini,
donc exclu. Mais G1 se satisfait aussi avec un ramasse-miettes ; Java
n'a presque pas de comportement indéfini. G2 fixe le *moyen* : sans
exécutif. C'est elle qui rend `core` possible sur un microcontrôleur.

**Coût.** Un modèle de propriété et d'emprunt, donc des programmes
corrects refusés (mesure du cahier, §8) ; c'est l'endroit où vit la
complexité de Rust.

**Qui l'a.** Rust (propriété et emprunt), Austral (types linéaires :
« Memory can be managed safely and without runtime overhead »), Hylo
(sémantique de valeur ; projet déclaré « still in their early stages »).

### G3 — Zéro course de données

**Énoncé.** Deux fils ne peuvent pas accéder à un même emplacement sans
synchronisation si l'un écrit. Un programme qui le tenterait ne compile
pas.

**Vérification.** Le compilateur, par le typage du partage entre fils
(ce que `Send`/`Sync` font en Rust) ; test rejouable sur un corpus de
programmes concurrents fautifs qui doivent être refusés.

**Interdit.** La variable globale mutable sans synchronisation, le fil
qui capture une référence mutable partagée, tout modèle de fils « à la
C » où le partage est libre.

**Pourquoi elle ne se déduit pas de G1.** Chez Rust, une course de
données est un comportement indéfini, donc G1 l'exclut. Mais ce n'est
pas une nécessité : Java *définit* le comportement des programmes avec
courses (modèle mémoire, JLS §17 : « When code contains a data race,
counterintuitive results are often possible »), et Go aussi. G1 seul
autoriserait donc un Shrimp où la course est légale et spécifiée. G3
dit qu'elle ne compile pas.

**Coût.** Un modèle de partage dans le langage — types marqueurs ou
acteurs sans mémoire partagée (Erlang : « Threads of execution in
Erlang share no data ») —, à trancher à l'étape 3. Pas de garantie
contre les interblocages ni les courses « générales » : Rust non plus
(« Rust does not prevent general race conditions »).

### G4 — Rien de caché : une fonction se lit dans la fonction

**Énoncé.** Tout ce qui s'exécute, s'alloue ou se convertit dans une
fonction est nommé sur la ligne où cela se produit, ou se déduit des
seules signatures des fonctions et types qu'elle nomme. Une seule règle
implicite, écrite dans la spec : la mémoire d'une valeur est rendue à
l'accolade fermante. Cette règle n'exécute jamais de code écrit par un
utilisateur : il n'y a pas de destructeur.

**Vérification.** Syntaxique, donc par le compilateur ; mesure du
cahier §8 « allocations et appels non visibles dans le source : zéro ».

**Interdit.** Les exceptions et toute sortie non locale rattrapable ;
les conversions implicites ; les macros qui réécrivent la syntaxe ; la
surcharge d'opérateurs sur les types utilisateur (`+` qui alloue) ; la
construction ou copie implicite coûteuse ; l'allocation sans allocateur
ou type alloueur nommé. Un arrêt du programme sur contrôle échoué
(bornes, division par zéro) n'est pas une sortie rattrapable : il est
admis par G4, et exigé par G1.

**Pas de destructeur : les ressources se consomment.** Deux sortes de
choses meurent à l'accolade, et elles ne sont pas traitées pareil.

- **La mémoire** — chaîne, tableau, structure — se rend par une
  opération toujours identique, qui ne peut pas échouer utilement et où
  il n'y a rien à lire. Le compilateur l'émet : c'est la règle implicite
  de G4, et la seule.
- **Une ressource** — fichier, verrou, connexion — se rend par du code
  que quelqu'un a écrit, qui peut échouer et qui agit hors du programme.
  Ce code ne s'exécute jamais sans être appelé.

Un type qui tient une ressource est déclaré **à consommer**. Une valeur
de ce type doit être passée exactement une fois, sur chaque chemin qui
sort de la portée, à une fonction qui la consomme ; sinon le programme
ne compile pas, et l'erreur nomme la valeur et la sortie fautive. Une
structure qui contient une ressource est à consommer à son tour. Le
`?` n'est permis que si aucune valeur à consommer n'est vivante. Sur
un arrêt du programme, rien n'est consommé : le système d'exploitation
récupère.

Ce que ça donne : G4 sans exception ; la fermeture qui échoue est un
résultat ordinaire, que G5 oblige à traiter ; la double libération et
l'oubli ne compilent pas, ce qu'un destructeur ne détecte pas. Prix :
les chemins d'erreur s'écrivent à la main quand une ressource est
vivante, et c'est celui qui écrit qui paie. C'est le modèle linéaire
d'Austral, limité aux ressources. Syntaxe de la déclaration et partage
d'une ressource entre propriétaires : étape 3.

**Coût.** Du code plus long qu'en Rust ou en Python ; pas d'arithmétique
sur types utilisateur par opérateurs ; une syntaxe de propagation
d'erreur visible (le `?` de Rust est visible : il passe, sauf quand une
ressource à consommer est vivante).

**Qui l'a.** Zig : « There is no hidden control flow, no hidden memory
allocations, no preprocessor, and no macros » ; Austral : « No
exceptions (and no surprise control flow) », « No implicit function
calls », « No implicit type conversions ». Rust ne l'a pas : surcharge,
`Deref`, conversions par `From`, macros.

### G5 — Pas de cas oublié

**Énoncé.** Toute valeur qui peut être une erreur ou une absence a un
type qui le dit, et toute alternative d'un type somme — erreur, absence,
variante — est traitée explicitement, ou le programme ne compile pas.

**Vérification.** Le compilateur : exhaustivité des filtrages, résultat
non consommé refusé (une erreur, pas un avertissement).

**Interdit.** `null` comme valeur d'un type ordinaire ; le code de retour
qu'on peut ignorer (C) ; le `switch` sans cas par défaut qui tombe en
silence ; le `Result` jeté.

**Pourquoi elle ne se déduit pas de G4.** G4 exclut les exceptions,
donc impose que l'erreur soit une valeur ; mais une valeur peut être
ignorée. G5 dit qu'elle ne peut pas l'être. C et Go ont des erreurs
comme valeurs et pas G5.

**`null`.** Aucun type ordinaire n'a de valeur « rien » : l'absence est
un type à part, que le compilateur oblige à traiter avant d'en sortir
la valeur. Ce n'est pas une garantie de plus — elle se déduit de G5 —,
mais c'est son cas le plus fréquent. Le zéro survit en deux endroits :
dans les pointeurs bruts, sous `unsafe` et à la frontière C, où le
passage vers le code sûr impose le test ; et dans la représentation,
où une option de référence peut tenir sur un mot, ce que seul le
compilateur voit.

**Coût.** Du code qui traite des cas que le programmeur « sait »
impossibles ; une pression sur la conception des types somme (étape 3).

**Qui l'a.** Rust (en partie : résultat ignoré n'est qu'un
avertissement), Austral (« algebraic data types … with exhaustiveness
checking »), Zig (unions d'erreurs, `switch` exhaustif).

## 2 bis. La frontière C

Le cahier impose l'interopérabilité C dans les deux sens (§7). Les
garanties ne traversent pas la frontière : elles disent ce qui se passe
quand on la franchit.

| Garantie | À la frontière |
|---|---|
| G1 | Tout appel vers C, et tout appel de C vers Shrimp, est `unsafe` : le compilateur ne voit pas de l'autre côté. Celui qui écrit l'enveloppe sûre répond de sa correction — la règle de *soundness* de Rust. Pas de déroulement de pile à travers C : il n'existe pas (G4), un arrêt est un arrêt |
| G2 | La propriété se déclare en franchissant : un pointeur reçu de C est emprunté ou possédé, et le type le dit ; une ressource que C libère n'est pas un type à consommer côté Shrimp |
| G3 | Une fonction C n'est pas partageable entre fils par défaut ; elle le devient par déclaration, sous `unsafe` |
| G4 | Un appel externe est nommé, donc visible ; un rappel de C vers Shrimp passe par un type de fonction déclaré. Rien ne change |
| G5 | Un code de retour C arrive comme un entier ; l'enveloppe le convertit en résultat ou en absence, et c'est à elle que G5 s'applique. La frontière brute, elle, reste du C |

La frontière est donc un lieu, pas une exception : tout ce qui est
`unsafe` y est écrit comme tel, et tout ce qui en sort est sûr parce
que quelqu'un l'a garanti par écrit. Sa spécification — ABI, types
représentables, chaînes, rappels — est un chapitre du noyau (étape 3).

## 3. Les candidates écartées

| Candidate du cahier | Sort | Motif |
|---|---|---|
| Sûreté mémoire sans ramasse-miettes | Reformulée en G2 | La sûreté se déduit de G1 (échoue au critère 3) ; ce qui ne s'en déduit pas, c'est « sans exécutif », et c'est G2 |
| Erreurs comme valeurs | Absorbée par G4 et G5 | « Comme valeurs » se déduit de G4 (pas d'exception) ; ce qui reste, l'exhaustivité, est G5 |
| Compatibilité dans le temps | Hors liste | Engagement de gouvernance, pas propriété d'un programme ; se garantit par corpus et éditions, point 1.2 |
| Spécification bornée | Hors liste | Même nature : contrainte sur le projet, mesurée en tokens au point 1.3 |
| Abstraction à coût nul | Refusée comme garantie | N'interdit rien de nommable (critère 2) : on peut toujours écrire lent. C'est un principe de conception, mesuré au point 1.8 contre C et Rust |

## 4. Les paires

Dix paires. Celles qui se disputent, et l'arbitrage :

| Paire | Tension | Arbitrage |
|---|---|---|
| G1 × G2 | Sans exécutif, la sûreté exige un modèle de propriété, qui refuse des programmes corrects | Accepté ; c'est la mesure « programmes corrects refusés », à tenir sous Rust |
| G1 × G4 | Les contrôles à l'exécution sont du code non écrit | Un contrôle qui arrête n'est pas rattrapable : admis. Un contrôle qui coûte se voit à la mesure, pas dans le source |
| G2 × G4 | Libérer sans exécutif demande du code à chaque fin de vie | La mémoire est rendue par le compilateur, règle unique sans code utilisateur ; les ressources se consomment explicitement, le compilateur l'impose |
| G3 × G4 | Le typage du partage est inféré | Rien ne s'exécute : l'inférence statique n'est pas cachée au sens de G4 |
| G4 × G5 | Traiter chaque cas allonge le code ; la propagation doit rester visible | Un opérateur de propagation visible, à la `?`, satisfait les deux |
| les cinq autres | — | Pas de tension trouvée |

## 5. Verdicts

Les garanties passées sur des traits connus. Un verdict qui hésite
serait une garantie mal écrite ; « au budget » n'est pas une
hésitation : c'est qu'aucune garantie ne tranche, et que le point 1.2
tranchera.

| Trait | Verdict | Par |
|---|---|---|
| Exceptions | Non | G4 |
| Ramasse-miettes | Non | G2 |
| Comptage de références automatique (Swift) | Non | G2 |
| Partage explicite à la `Rc` | Oui dans le principe, mécanisme à l'étape 3 | G2 l'admet, G4 contraint sa libération |
| Destructeur écrit par l'utilisateur | Non | G4 |
| Libération de la mémoire à l'accolade | Oui, règle unique du langage | G4 l'admet, G2 l'exige |
| Type à consommer (ressource linéaire) | Oui, exactement une fois par chemin | G2, G4, G5 |
| `defer` | Au budget | Visible, mais redondant avec la consommation imposée |
| `null` | Non | G5 |
| Code de retour ignorable | Non | G5 |
| Conversion implicite | Non | G4 |
| Surcharge d'opérateurs sur types utilisateur | Non | G4 |
| Macros syntaxiques | Non | G4 |
| Bloc `unsafe` nommé | Oui | G1 l'exige pour exister |
| Appel d'une fonction C | Oui, sous `unsafe`, enveloppe sûre à charge de l'auteur | G1, G5 |
| Déroulement de pile à travers C | Non | G4 : rien n'est rattrapable |
| Variable globale mutable | Non sans synchronisation typée | G3 |
| Dépassement d'entier | Comportement défini, lequel à l'étape 3 | G1 |
| Arrêt du programme sur contrôle échoué | Oui | G1, G4 |
| Génériques monomorphisés | Oui | Aucune ne s'y oppose |
| Asynchrone (`async`/`await`) | Au budget | Visible, donc G4 ne tranche pas ; coût à l'étape 1.2 |
| Héritage | Au budget | Aucune garantie ne tranche |
| Inférence de types locale | Au budget | Rien ne s'exécute ; G4 ne tranche pas |

## 6. Sources relues le 2026-10-21

- Rust, *The Reference*, « Behavior considered undefined » : la liste
  inclut « Data races », l'accès à une place « dangling », la lecture
  d'entiers et pointeurs non initialisés.
- Rust, *The Rustonomicon*, « Meet Safe and Unsafe » et « Data Races and
  Race Conditions » : citations ci-dessus.
- Zig, *Overview* (ziglang.org/learn/overview) : « There is no hidden
  control flow, no hidden memory allocations, no preprocessor, and no
  macros » ; « Zig uses Illegal Behavior as a razor sharp tool » ; pas de
  revendication de sûreté mémoire.
- Austral, *Features* (austral-lang.org/features) : liste des
  anti-features citées.
- Hylo (hylo-lang.org) : « A Safe Systems Programming Language »,
  « compiler and standard library are still in their early stages ».
- Java, *JLS* SE 21, §17.4 : définition de la course de données,
  sémantique des programmes mal synchronisés.
- Erlang, *Concurrent Programming* : « Threads of execution in Erlang
  share no data ».
- Go, *Go 1 and the Future of Go Programs* : la promesse de
  compatibilité, sans date sur la page (la « depuis 2012 » du cahier
  reste citée de mémoire).

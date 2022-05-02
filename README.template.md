# ENS Paris-Saclay, cours de programmation avancée (niveau L3 informatique)

# Projet de programmation : diagrames de décision binaires en Rust

## Travail à effectuer

Le but de ce projet est d'implémenter une bibliothèque de BDD en
Rust. Les BDDs sont une structure de donnée permettant de représenter
de manière compacte et efficacemenet une fonction booléenne. Si vous
ne connaissez pas cette structure de données, vous êtes invités à en
prendre connaissance en lisant, par exemple, la page Wikipedia
française à son sujet :
[https://fr.wikipedia.org/wiki/Diagramme_de_d%C3%A9cision_binaire](https://fr.wikipedia.org/wiki/Diagramme_de_d%C3%A9cision_binaire). Tous
les BDDs considérés dans ce projet seront en fait des ROBDD: on les
spposera réduits et ordonnés.

La bibliothèque fournira des opérations booléenne efficaces sur les
BDD, mais n'implémentera pas l'algorithme efficace de construction de
BDD à partir d'une formule booléenne utilisant l'expansion de Shannon.

Le projet est en deux parties :

  - La première partie consiste en l'implémentation d'une bibliothèque
    de BDD bas niveau. Afin de comprendre ce qui est demandé, vous
    devez lire le contenu du fichier [raw.rs](src/raw.rs), puis le
    compléter.
  - La deuxième partie concerne une surcouche à la version de bas
    niveau de la bibliothèque, qui permet une plus grande facilité
    d'utilisation et une plus grande sûreté pour le programme
    client. Pour comprendre ce qui est demande, vous devez lire le
    contenu du fichier [lib.rs](src/lib.rs), puis le compléter.

La deuxième partie, qui dépend de la première, demande d'écrire moins
de code, mais utilise des concepts Rusts plus avancés. Elle nécessite
l'utilisation des types à mutabilité intérieure, qui feront l'objet
d'un cours pendant le semestre. Il sera possible d'obtenir la moyenne
de ce projet en ne traitant parfaitement que la première partie.

Il est, par ailleurs, demandé d'écrire un fichier README.md portant
sur le travail effectué, les difficultés rencontrées et les choix
effectués, afin de guider les le travail du correcteur.

Enfin, tout travail supplémentaire sera récompensé. Il peut s'agir de
fonctionnalités supplémentaire à la bibliothèque, de l'amélioration de
ses performances en utilisant des optimisations à choisir, de nouveaux
benchmarks ou d'utilisations de la bibliothèque dans le cadre d'un
programme. Bien sûr, dans ce cas, il est nécessaire de bien documenter
le travail effectué dans le fichier README.md.

## Code fourni

L'archive contient un squelette de projet à compléter, avec tout ce
qu'il faut pour compiler le code simplement avec l'outil Cargo. Il
n'est pas nécessaire de télécharger manuellement la bibliothèque
Bumpalo, dépendance du projet: Cargo s'en occupera automatiquement.

En particulier:

  - Les fichiers [raw.rs](src/raw.rs) et [lib.rs](src/lib.rs) sont à
    compléter.
  - Le dossier [bin](src/bin) contient plusieurs programmes permettant
    d'évaluer votre implémentation sur des problèmes complets:
     - [nqueens.rs](src/bin/nqueens.rs) est un encodage du problème
       des N reines;
     - [pigeonhole.rs](src/bin/pigeonhole.rs) encode dans un BDD le
       principe des tiroirs.
     - Les fichiers xxxx_raw.rs contiennent les mêmes tests, mais
       programmés en utilisant l'interface bas-niveau. Il peuvent donc
       s'exécuter sans avoir fait la deuxième partie du projet.

Afin de compiler le projet, on peut exécuter la commande `cargo build`
à la racine. Cela compile la bibliothèque de BDD, ainsi que tous les
tests l'utilisant. Les exécutables peuvent alors être trouvés dans le
dossier [target/debug/](target/debug/). Pour activer les optimisations
et obtenir de bonnes performances, il faut utiliser l'option
`--release`. Les exécutables sont alors dans
[target/release/](target/release/).

Cargo permet aussi de compiler et de lancer un exécutable directement,
à l'aide de la commande `cargo run --bin nqueens`. On peut rajouter
l'option `--release` afin d'activer les optimisations, et remplacer
`nqueens` par le nom de l'exécutable souhaité.

Par ailleurs, les fichiers lib.rs et raw.rs contiennent des tests, qui
peuvent vous aider à chercher les bugs dans vore implémentation. Vous
pouvez les exécuter en lançant la commande `cargo test`.

Il est attendu que tous les exécutables et tous les tests s'exécutent
sans erreur en quelques secondes.

## Rendu du projet.

La date de rendu est fixée le vendredi 8 mai, à 23:59.
Il vous est demandé d'envoyer votre projet sous la forme d'une archive
.tar.gz à l'adresses email suivante:

   Jacques-Henri Jourdan <jacques-henri.jourdan@normalesup.org>

L'archive doit suivre la même arborescence que celle qui vous est
fournie.
Elle doit contenir :
  - Les fichiers lib.rs et raw.rs complétés.
  - Un fichier README.md décrivant le travail réalisé, les problèmes
    rencontrés, et éventuellement les ajouts effectués.

// Ces deux annotations permettent de désactiver des avertissements produits par
// compilateur Rust, dus au fait que le code est incomplet dans cette version.
// Elles devront être retirées lorsque les trous seront complétés.
#![allow(unused_variables)]
#![allow(dead_code)]

// Le module `raw`, offrant une interface de bas niveau à la bibliothèque
// de BDD, est un sous-module du module principal de la bibliothèque.
// Il est public, ce qui signifie que les utilisateur de la bibliothèque auront
// accès non seulement à l'interface de haut niveau définie dans le module
// principal (ce fichier), mais aussi à l'interface de bas niveau définie
// dans `raw`.
pub mod raw;

// L'interface de haut niveau permet de palier à plusieurs défauts de
// l'interface bas-niveau :
//    - Il faut spécifier explicitement le contexte lors de chaque opération
//      sur les BDDs, alors qu'un BDD donné ne peut correspondre qu'à un
//      contexte. Il parraît évident qu'il faut se servir du contexte qui a
//      permis la création d'un BDD pour les opérations qui le concernent.
//    - Alors que la propriété exclusive du contexte est nécessaire pour la
//      plupart des opérations, cette structure de donnée n'est pas `Copy`.
//      Cela rend la bibliothèque de BDD difficile à l'utilisation, car il faut
//      alors gérer la propriété de cette structure de donné qui sert partout.
//    - Dans l'interface de la bibliothèque, rien n'interdit de créer plusieurs
//      contextes, et d'utiliser un BDD avec un contexte qui ne lui correspond
//      pas. Les propriétés fournies par le hash-consing (unicité, qui permet
//      une comparaison et un hachage rapide) sont alors fausses, ce qui rend
//      beaucoup de fonctions incorrectes.
//
// Afin de corriger ces défauts, on va créer un nouveau type BDD et un nouveau
// type de contexte. Le nouveau type de BDD contiendra un pointeur vers le
// contexte, ce qui permettra d'effectuer des opérations sur les BDD sans
// fournir explicitement le contexte. De plus, le nouveau type des contextes,
// qui sera toujours nécessaire pour créer des formules booléennes atomiques,
// ne contiendra en fait qu'une référence partagée vers le contexte, ce qui lui
// permettra d'être `Copy`. On utilisera la mutabilité intérieure afin de
// rendre ces partages possibles.
//
// Enfin, pour rendre l'interface sûre et interdire l'utilisation des
// BDDs avec un mauvais contexte, on va se servir de la variable de
// lifetime `'arena` comme identifiant "fantôme" du contexte. À chaque contexte
// sera associée une lifetime `'arena` différente, et, puisque les types `Bdd`
// et `Context` sont paramétrés par `'arena`, il sera impossible de mélanger
// les contextes. Pour garantir que la lifetime `'arena` est bien unique, c'est
// la bibliothèque elle-même qui va créer l'arène et l'emprunt correspondant.

use std::ops::*;
use std::hash::Hash;

#[derive(Copy, Clone)]
pub struct Context<'arena, V> (&'arena V /* À CHANGER */);

#[derive(Copy, Clone)]
pub struct Bdd<'arena, V> {
    raw: raw::Bdd<'arena, V>,
    // À COMPLÉTER
}

// La fonction `use_bdd` crée une nouvelle arène d'allocation et un nouveau
// contexte, et exécute la clôture passée en paramètre, avec le contexte créé.
// Puisque la lifetime `'arena` est créée dans cette fonction, on a la garantie
// qu'il n'y aura pas d'autre contexte qui l'utiliseront, et, par conséquent,
// elle identifie uniquement le contexte.
pub fn use_bdd<V: Hash + Copy + Ord, T, F>(f: F) -> T
  where F: for<'arena> FnOnce(Context<'arena, V>) -> T {
    // À COMPLÉTER
    panic!()
}


// Il y a moins d'opérations sur les contextes de haut niveau que sur les
// contextes de bas niveau. Elles sont réduites aux opérations qui n'ont pas
// d'autres BDD en paramètre: les formules constantes VRAI et FAUX et les
// atomes.
impl<'arena, V: Hash + Copy + Ord> Context<'arena, V> {
    pub fn true_(self) -> Bdd<'arena, V> {
        // À COMPLÉTER
        panic!()
    }

    pub fn false_(self) -> Bdd<'arena, V> {
        // À COMPLÉTER
        panic!()
    }

    pub fn var(self, x: V) -> Bdd<'arena, V> {
        // À COMPLÉTER
        panic!()
    }
}

// Les autres opérations sur les BDDs sont implémentées en surchargeant les
// opérateurs Rust correspondants.

impl<'arena, V> PartialEq<Bdd<'arena, V>> for Bdd<'arena, V> {
    fn eq(&self, x: &Bdd<'arena, V>) -> bool {
        // À COMPLÉTER
        panic!()
    }
}
impl<'arena, V> Eq for Bdd<'arena, V> { }

impl<'arena, V: Hash + Copy + Ord> Not for Bdd<'arena, V> {
    type Output = Bdd<'arena, V>;
    fn not(self) -> Bdd<'arena, V> {
        // À COMPLÉTER
        panic!()
    }
}

impl<'arena, V: Hash + Copy + Ord> BitAnd for Bdd<'arena, V> {
    type Output = Bdd<'arena, V>;
    fn bitand(self, rhs: Bdd<'arena, V>) -> Bdd<'arena, V> {
        // À COMPLÉTER
        panic!()
    }
}

impl<'arena, V: Hash + Copy + Ord> BitAndAssign for Bdd<'arena, V> {
    fn bitand_assign(&mut self, rhs: Bdd<'arena, V>) {
        // À COMPLÉTER
        panic!()
    }
}

impl<'arena, V: Hash + Copy + Ord> BitOr for Bdd<'arena, V> {
    type Output = Bdd<'arena, V>;
    fn bitor(self, rhs: Bdd<'arena, V>) -> Bdd<'arena, V> {
        // À COMPLÉTER
        panic!()
    }
}

impl<'arena, V: Hash + Copy + Ord> BitOrAssign for Bdd<'arena, V> {
    fn bitor_assign(&mut self, rhs: Bdd<'arena, V>) {
        // À COMPLÉTER
        panic!()
    }
}

impl<'arena, V: Hash + Copy + Ord> BitXor for Bdd<'arena, V> {
    type Output = Bdd<'arena, V>;
    fn bitxor(self, rhs: Bdd<'arena, V>) -> Bdd<'arena, V> {
        // À COMPLÉTER
        panic!()
    }
}

impl<'arena, V: Hash + Copy + Ord> BitXorAssign for Bdd<'arena, V> {
    fn bitxor_assign(&mut self, rhs: Bdd<'arena, V>) {
        // À COMPLÉTER
        panic!()
    }
}

// La méthode `nsat` est une version de haut niveau de la méthode `nsat` de
// bas niveau.
impl<'arena, V: Hash + Copy + Ord> Bdd<'arena, V> {
    pub fn nsat(self, vars: &[V]) -> u64 {
        // À COMPLÉTER
        panic!()
    }
}


#[test]
fn test() {
    use_bdd(|ctx| {
        let x = ctx.var(0);
        let y = ctx.var(1);
        let z = ctx.var(2);
        let f = ctx.false_();
        let t = ctx.true_();
        let vars = [0, 1];

        assert!(x.nsat(&vars) == 2);
        assert!(y.nsat(&vars) == 2);
        assert!((x & y).nsat(&vars) == 1);
        assert!((x | y).nsat(&vars) == 3);
        assert!((x ^ y).nsat(&vars) == 2);

        assert!(x & x == x);
        assert!(y & y == y);
        assert!(x & y == y & x);
        assert!(x & f == f);
        assert!(x & t == x);
        assert!(x & !x == f);

        assert!(x | x == x);
        assert!(x | y == y | x);
        assert!(x | f == x);
        assert!(x | t == t);
        assert!(x | !x == t);

        assert!(x & (y | z) == (x & y) | (x & z));
        assert!(x | (y & z) == (x | y) & (x | z));
        assert!(!(y & z) == !y | !z);
        assert!(!(y | z) == !y & !z);

        assert!(x ^ y == y ^ x);
        assert!(x ^ x == f);
        assert!(x ^ !x == t);
        assert!(x ^ t == !x);
        assert!(x ^ f == x);
        assert!(x ^ y == (x & !y) | (!x & y));
    });
}

// Ces deux annotations permettent de désactiver des avertissements produits par
// compilateur Rust, dus au fait que le code est incomplet dans cette version.
// Elles devront être retirées lorsque les trous seront complétés.
#![allow(unused_variables)]
#![allow(dead_code)]

use std::collections::hash_set::HashSet;
use std::hash::Hash;
use bumpalo::Bump;

// Un BDD est une structure de donnée comportant potentiellement beaucoup
// d'alias. C'est ce qui permet un partage optimal et donc la compacité de la
// structure de donnée.
// La méthode habituelle en Rust pour représenter des structures de données avec
// du partage (comme les BDDs) est d'utiliser des pointeurs avec comptage de
// références. Cependant, cette technique comporte des inconvénients :
// de nombreuses opérations requièrent d'incrémenter ou de décrémenter le compte
// de référence, ce qui a un certain coût en temps d'exécution. Par ailleurs, le
// compte lui-même nécessite de la mémoire, ce qui représente un autre coût en
// soi.
//
// Afin d'éviter ces problèmes, nous allons ici utiliser une autre technique :
// nous allons utiliser un allocateur à arène, fourni par la
// bibliothèque "bumpalo", dont la documentation peut être trouvée ici :
//     https://docs.rs/bumpalo/latest/bumpalo/
// bumpalo permet d'alouer de la mémoire de manière très efficace dans une zone
// de la mémoire gérée par la bibliothèque elle-même. La méthode permettant une
// telle opération est `bumpalo::Bump::alloc`, dont la signature est la
// suivante:
//    pub fn alloc<'arena, T>(&'arena self, val: T) -> &'arena mut T
// Elle prend en paramètre la valeur initiale de l'objet à alouer, et renvoie un
// emprunt vers un nouvel emplacement de l'arène le contenant. Il se trouve que
// cet emprunt est mutable, mais on peut facilement le transformer en un emprunt
// partagé si besoin.
// Crucialement, cette méthode prend un emprunt *partagé* vers l'arène, de telle
// sorte qu'on puisse faire plusieurs allocations dans l'arène (en copiant
// l'emprunt partagé vers l'arène), tout en gardant valides les pointeurs vers
// les allocations précédentes.
// Enfin, lorsque la lifetime `'arena` se termine, tous les pointeurs vers
// l'intérieur de l'arène deviennent invalides, puisque ce sont des emprunts à
// la lifetime 'arena. D'un autre côté, cela signifie qu'on réobtient la
// propriété exclusive de l'arène, et on peut donc désalouer son contenu (soit
// explicitement en appelant la méthode `bumpalo::Bump::reset`, soit
// implicitment en appelant son destructeur). Ainsi, la désalocation de l'arène
// se produit d'"un seul coup", lorsque l'on sait statiquement que les pointeurs
// aloués ne seront plus utilisés. En particulier, on n'a pas besoin de comptage
// de références pour savoir lorsqu'un objet peut être désaloué, et on peut
// utiliser directement un emprunt partagé à la lifetime `'arena` comme pointeur
// pour représenter les BDDs.


// Un nœud d'un BDD est soit terminal (True, False), soit interne.
// Un nœud est `Copy` (donc `Clone`), et on utilise l'implémentation par défaut
// pour les traits `Eq`, `PartialEq` et `Hash` pour pouvoir l'utiliser dans des
// tables de hachage. On dérive aussi une implémentation de `Debug` pour avoir
// des messages d'erreurs plus agréables lorsqu'un test échoue.
//
// Comme toutes les structures de données de BDDs, ce type est paramétré par la
// durée de vie 'arena, correspondant à la durée de vie des pointeurs utilisés
// dans l'arène d'allocation.
// De même, ce type est paramètré par le type `V` des variables booléennes du
// BDD.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Node<'arena, V> {
    False, True,
    If { var: V, children: [Bdd<'arena, V>; 2] }
}

// Un BDD n'est autre qu'un emprunt vers un nœud dans l'arène d'allocation des
// BDDs. Ce type a les invariants suivants:
//   - L'emprunt pointe vers l'arène d'allocation des BDDs.
//   - Aucun autre BDD dans l'arène lui est isomorphe.
#[derive(Copy, Clone, Debug)]
pub struct Bdd<'arena, V>(&'arena Node<'arena, V>);

// On redéfinit l'égalité et la fonction de hachage du type des BDDs, afin
// d'utiliser l'invariant d'unicité: puisque que chaque BDD ne peut être
// représenté qu'une seule fois dans l'arène d'allocation, il suffit de comparer
// et de hacher l'*adresse*, plutôt que de parcourir le BDD récrusivement.
impl<'arena, V> PartialEq<Bdd<'arena, V>> for Bdd<'arena, V> {
    fn eq(&self, x: &Bdd<'arena, V>) -> bool {
        std::ptr::eq(self.0, x.0)
    }
}
impl<'arena, V> Eq for Bdd<'arena, V> { }
impl<'arena, V> Hash for Bdd<'arena, V> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::ptr::hash(self.0, state)
    }
}

// Les opérations sur les BDDs demandent d'accéder à des structures de données
// globales. Celles-ci sont stockées dans un objet appelé "contexte" , de type
// `Context`.
pub struct Context<'arena, V> {
    // L'allocateur d'arène utilisé pour les BDDs
    alloc: &'arena Bump,

    // La table de hash-consing: elle contient tous les nœuds de BDDs qui ont
    // été aloués dans l'arène, et elle permet de s'assurer que l'arène ne
    // contient jamais deux BDDs isomorphes.
    hashcons: HashSet<&'arena Node<'arena, V>>,

    // À COMPLÉTER AVEC D'AUTRES STRUCTURES DE DONNÉES GLOBALES (COMME LES
    // TABLES DE MÉMOÏSATION).
}


// Toutes les opérations sur les BDDs demandent à ce que le type `V`
// des variables implémente `Hash + Copy + Ord`:
//   - `Hash` est nécessaire pour le hash-consing.
//   - `Copy` permet de manipuler facilement les variables sans se soucier des
//      questions de propriété.
//   - `Ord` établit une relation d'ordre sur les variables, qui est utilisé
//     pour ordonner les variables dans le BDD. En particulier, on s'assurera
//     que les variables apparaissent dans l'ordre croissant, avec les plus
//     petites variables proches de la racine du BDD.
// En pratique, on utilisera typiquement un type d'entier.
impl<'arena, V: Hash + Copy + Ord> Context<'arena, V> {
    // `new` crée un nouveau contexte de BDD, à partir d'une arène d'allocation
    // déjà créée par l'appelant.
    pub fn new(alloc: &'arena Bump) -> Self {
        return Context { alloc: alloc, hashcons: HashSet::new() }
    }

    // La méthode privée `hashcons` prend un nœud en paramètre, et
    // renvoie un BDD dont le contenu est ce nœud. Cette méthode doit
    // soit alouer un nouveau nœud dans l'arène, ou utiliser un nœud
    // déjà existant.
    fn hashcons(&mut self, n: Node<'arena, V>) -> Bdd<'arena, V> {
        if !self.hashcons.contains(&n) {
            let node = self.alloc.alloc(n);
            self.hashcons.insert(node);
        }

        return Bdd::<'arena, V>(self.hashcons.get(&n).unwrap())
    }

    // La méthode privée `node` permet de créer un nouveau nœud interne.
    // Elle suppose que `var` est strictement plus petite que toutes les
    // variables apparaissant dans `children`.
    // Cette fonction doit traiter de manière appropriée le cas où les deux fils
    // du nouveau nœud envisagé sont en fait égaux.
    fn node(&mut self, var: V, children: [Bdd<'arena, V>; 2]) -> Bdd<'arena, V> {
        if children[0] == children[1] {
            return children[0];
        }
        let node : Node<'arena, V> = Node::If { var: var, children: children };
        return self.hashcons(node)
    }

    // La méthode `true_` renvoie le BDD correspondant à la formule booléenne
    // VRAI.
    pub fn true_(&mut self) -> Bdd<'arena, V> {
        return Bdd(&Node::True)
    }

    // La méthode `false_` renvoie le BDD correspondant à la formule booléenne
    // VRAI.
    pub fn false_(&mut self) -> Bdd<'arena, V> {
        return Bdd(&Node::False)
    }

    // La méthode `var` renvoie le BDD correspondant à la formule booléenne
    // réduite à une variable simple.
    pub fn var(&mut self, x: V) -> Bdd<'arena, V> {
        let children = [self.false_(), self.true_()];
        return self.node(x, children)
    }

    // La méthode `not` renvoie la négation du BDD donné en paramètre.
    // On pourra l'implémenter en parcourant récursivement le BDD passé en
    // paramètre, et en mémoïsant le résultat pour éviter d'effectuer les mêmes
    // calculs plusieurs fois. La table de mémoïsation pourra être stockée dans
    // le contexte, pour être réutilisée lors de plusieurs appels à `not` sur
    // des BDDs proches.
    pub fn not(&mut self, x: Bdd<'arena, V>) -> Bdd<'arena, V> {
        match x {
            Bdd(&Node::True) => Bdd(&Node::False),
            Bdd(&Node::False) => Bdd(&Node::True),
            Bdd(&Node::If {var, children}) => {
                let children2 = [self.not(children[0]), self.not(children[1])];
                return self.node(var, children2)
            }
        }
    }

    // La méthode `and` renvoie la conjonction des BDDs donnés en paramètres.
    // On pourra l'implémenter avec un algorithme proche de celui pour `not`,
    // en parcourant récrusivement et simultanément les deux BDDs passés en
    // paramètres.
    pub fn and(&mut self, a: Bdd<'arena, V>, b: Bdd<'arena, V>) -> Bdd<'arena, V> {
        match (a, b) {
            (Bdd(&Node::True), _) => return b,
            (_, Bdd(&Node::True)) => return a,
            (Bdd(&Node::False), _) => return Bdd(&Node::False),
            (_, Bdd(&Node::False)) => return Bdd(&Node::False),
            (Bdd(Node::If{var : var_a, children : children_a}), Bdd(Node::If{var : var_b, children : children_b})) => {
                if var_a == var_b {
                    let children2 = [self.and(children_a[0], children_b[0]), self.and(children_a[1], children_b[1])];
                    return self.node(*var_a, children2)
                } else if var_a < var_b {
                    let children2 = [self.and(children_a[0], b), self.and(children_a[1], b)];
                    return self.node(*var_a, children2)
                } else {
                    let children2 = [self.and(a, children_b[0]), self.and(a, children_b[1])];
                    return self.node(*var_b, children2)
                }
            }
        }
    }

    // La méthode `or` renvoie la disjonction des BDDs donnés en paramètres.
    pub fn or(&mut self, a: Bdd<'arena, V>, b: Bdd<'arena, V>) -> Bdd<'arena, V> {
        let na = self.not(a);
        let nb = self.not(b);
        let nand = self.and(na, nb);
        return self.not(nand)
    }

    // La méthode `xor` renvoie la disjonction exclusive des BDDs donnés en paramètres.
    pub fn xor(&mut self, a: Bdd<'arena, V>, b: Bdd<'arena, V>) -> Bdd<'arena, V> {
        let aob = self.or(a, b);
        let aab = self.and(a, b);
        let naab = self.not(aab);
        return self.and(aob, naab);
    }
}

impl<'arena, V: Hash + Copy + Ord> Bdd<'arena, V> {
    // La méthode `nsat` sur les BDDs permet de calculer le nombre
    // d'affectations possibles des variables permettant de satisfaire la
    // formule booléenne correspondante.

    // Elle prend en paramètre un tableau de variable `vars`, que l'on
    // supposera trié dans l'ordre croissant et sans doublons, et qui contient
    // les variables à considérer pour l'énumération. En particulier, toutes les
    // variables apparaissant dans le BDD doivent apparaître dans ce tableau.
    // On pourra aussi utiliser un algorithme de parcours récursif du BDD, avec
    // de la mémoïsation.
    // Cette fonction peut supposer que le résultat du calcul est suffisamment
    // petit pour la capacité du type `u64`.
    pub fn nsat(self, vars: &[V]) -> u64 {
        match self {
            Bdd(Node::True) => return u64::pow(2, vars.len() as u32),
            Bdd(Node::False) => return 0,
            Bdd(Node::If{var, children}) => {
                if vars[0] == *var {
                    let nsat_left = children[0].nsat(&vars[1..]);
                    let nsat_right = children[1].nsat(&vars[1..]);
                    return nsat_left + nsat_right
                } else {
                    return 2*self.nsat(&vars[1..])
                }
            }
        }
    }
}

#[test]
fn test_new_ctx() {
    let allo = bumpalo::Bump::new();
    let mut _ctx : Context<u16> = Context::new(&allo);
}

#[test]
fn test_var_true_false() {
    let allo = bumpalo::Bump::new();
    let mut ctx = Context::new(&allo);
    let x = ctx.var(0);
    let y = ctx.var(1);
    let z = ctx.var(2);
    assert_ne!(x, y);
    assert_ne!(y, z);
    assert_ne!(x, z);
    assert_ne!(x, ctx.true_());
    assert_ne!(x, ctx.false_());
    assert_ne!(ctx.false_(), ctx.true_());
}

#[test]
fn test_not() {
    let allo = bumpalo::Bump::new();
    let mut ctx = Context::new(&allo);
    let x = ctx.var(0);
    let y = ctx.var(1);
    let f = ctx.false_();
    let t = ctx.true_();

    assert_ne!(x, ctx.not(x));
    assert_ne!(t, ctx.not(x));
    assert_ne!(f, ctx.not(x));
    assert_ne!(ctx.not(y), ctx.not(x));
    let nx = ctx.not(x);
    assert_eq!(ctx.not(nx), x);
}

#[test]
fn test_and() {
    let allo = bumpalo::Bump::new();
    let mut ctx = Context::new(&allo);
    let x = ctx.var(0);
    let y = ctx.var(1);
    let z = ctx.var(2);
    let t = ctx.true_();
    let f = ctx.false_();

    assert_eq!(ctx.and(t, t), t);
    assert_eq!(ctx.and(t, f), f);
    assert_eq!(ctx.and(f, t), f);
    assert_eq!(ctx.and(f, f), f);

    assert_eq!(ctx.and(x, x), ctx.and(x, x));
    assert_eq!(ctx.and(x, x), x);
    assert_eq!(ctx.and(y, y), y);
    assert_eq!(ctx.and(x, y), ctx.and(y, x));
    assert_eq!(ctx.and(x, f), f);
    assert_eq!(ctx.and(f, x), f);
    assert_eq!(ctx.and(t, y), y);
    assert_eq!(ctx.and(y, t), y);

    let nx = ctx.not(x);
    assert_eq!(ctx.and(x, nx), f);

    let xy = ctx.and(x, y);
    let yz = ctx.and(y, z);
    let xz = ctx.and(x, z);
    assert_eq!(ctx.and(xy, x), xy);
    assert_eq!(ctx.and(xy, y), xy);
    assert_eq!(ctx.and(xy, xy), xy);
    assert_eq!(ctx.and(xy, z), ctx.and(z, xy));
    assert_eq!(ctx.and(xz, y), ctx.and(z, xy));
    assert_eq!(ctx.and(yz, x), ctx.and(x, yz));
}

#[test]
fn test_or() {
    let allo = bumpalo::Bump::new();
    let mut ctx = Context::new(&allo);
    let x = ctx.var(0);
    let y = ctx.var(1);
    let z = ctx.var(2);
    let t = ctx.true_();
    let f = ctx.false_();

    assert_eq!(ctx.or(t, t), t);
    assert_eq!(ctx.or(t, f), t);
    assert_eq!(ctx.or(f, t), t);
    assert_eq!(ctx.or(f, f), f);

    assert_eq!(ctx.or(x, x), ctx.or(x, x));
    assert_eq!(ctx.or(x, x), x);
    assert_eq!(ctx.or(y, y), y);
    assert_eq!(ctx.or(x, y), ctx.or(y, x));
    assert_eq!(ctx.or(x, f), x);
    assert_eq!(ctx.or(f, x), x);
    assert_eq!(ctx.or(t, y), t);
    assert_eq!(ctx.or(y, t), t);

    let nx = ctx.not(x);
    assert_eq!(ctx.or(x, nx), t);

    let xy = ctx.or(x, y);
    let yz = ctx.or(y, z);
    let xz = ctx.or(x, z);
    assert_eq!(ctx.or(xy, x), xy);
    assert_eq!(ctx.or(xy, y), xy);
    assert_eq!(ctx.or(xy, xy), xy);
    assert_eq!(ctx.or(xy, z), ctx.or(z, xy));
    assert_eq!(ctx.or(xz, y), ctx.or(z, xy));
    assert_eq!(ctx.or(yz, x), ctx.or(x, yz));
}

#[test]
fn test_andor() {
    let allo = bumpalo::Bump::new();
    let mut ctx = Context::new(&allo);
    let x = ctx.var(0);
    let y = ctx.var(1);
    let z = ctx.var(2);

    let xay = ctx.and(x, y);
    let yoz = ctx.or(y, z);
    let xaz = ctx.and(x, z);
    assert_eq!(ctx.and(x, yoz), ctx.or(xay, xaz));

    let yox = ctx.or(y, x);
    let zax = ctx.and(z, x);
    let yoz = ctx.or(y, z);
    assert_eq!(ctx.or(y, zax), ctx.and(yoz, yox));

    let yaz = ctx.and(y, z);
    let ny = ctx.not(y);
    let nz = ctx.not(z);
    assert_eq!(ctx.not(yaz), ctx.or(ny, nz));

    let xoy = ctx.or(x, y);
    let nx = ctx.not(x);
    let ny = ctx.not(y);
    assert_eq!(ctx.not(xoy), ctx.and(nx, ny));
}

#[test]
fn test_xor() {
    let allo = bumpalo::Bump::new();
    let mut ctx = Context::new(&allo);
    let x = ctx.var(0);
    let y = ctx.var(1);
    let z = ctx.var(2);
    let t = ctx.true_();
    let f = ctx.false_();

    assert_eq!(ctx.xor(t, t), f);
    assert_eq!(ctx.xor(t, f), t);
    assert_eq!(ctx.xor(f, t), t);
    assert_eq!(ctx.xor(f, f), f);

    assert_eq!(ctx.xor(x, x), f);
    assert_eq!(ctx.xor(x, y), ctx.xor(y, x));
    assert_eq!(ctx.xor(x, f), x);
    assert_eq!(ctx.xor(f, x), x);
    assert_eq!(ctx.xor(t, y), ctx.not(y));
    assert_eq!(ctx.xor(y, t), ctx.not(y));

    let nx = ctx.not(x);
    assert_eq!(ctx.xor(x, nx), t);

    let xy = ctx.xor(x, y);
    let yz = ctx.xor(y, z);
    let xz = ctx.xor(x, z);
    assert_eq!(ctx.xor(xy, x), y);
    assert_eq!(ctx.xor(xy, y), x);
    assert_eq!(ctx.xor(xy, xy), f);
    assert_eq!(ctx.xor(xy, z), ctx.xor(z, xy));
    assert_eq!(ctx.xor(xz, y), ctx.xor(z, xy));
    assert_eq!(ctx.xor(yz, x), ctx.xor(x, yz));
}

#[test]
fn test_xor_and_or() {
    let allo = bumpalo::Bump::new();
    let mut ctx = Context::new(&allo);
    let x = ctx.var(0);
    let y = ctx.var(1);
    let z = ctx.var(2);

    let xay = ctx.and(x, y);
    let xaz = ctx.and(x, z);
    let yxz = ctx.xor(y, z);
    assert_eq!(ctx.xor(xay, xaz), ctx.and(x, yxz));

    let nx = ctx.not(x);
    let ny = ctx.not(y);
    let xany = ctx.and(x, ny);
    let nxay = ctx.and(nx, y);
    assert_eq!(ctx.xor(x, y), ctx.or(xany, nxay));
}

#[test]
fn test_nsat() {
    let allo = bumpalo::Bump::new();
    let mut ctx = Context::new(&allo);
    let x = ctx.var(0);
    let y = ctx.var(1);
    let z = ctx.var(2);
    let f = ctx.false_();
    let t = ctx.true_();
    let vars = [0, 1, 2];

    assert_eq!(t.nsat(&vars), 8);
    assert_eq!(f.nsat(&vars), 0);
    assert_eq!(x.nsat(&vars), 4);
    assert_eq!(ctx.not(x).nsat(&vars), 4);
    assert_eq!(y.nsat(&vars), 4);
    assert_eq!(z.nsat(&vars), 4);
    assert_eq!(ctx.and(x, y).nsat(&vars), 2);
    assert_eq!(ctx.or(x, y).nsat(&vars), 6);
    assert_eq!(ctx.xor(x, y).nsat(&vars), 4);
    assert_eq!(t.nsat(&[]), 1);
    assert_eq!(f.nsat(&[]), 0);
}

use bdd::use_bdd;

fn main() {
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

use bdd::{Bdd, Context, use_bdd};
use std::time::Instant;

struct PigeonHolePb<'a> {
    bddctx: Context<'a, u16>,
    pigeons: u8,
    holes: u8
}

impl<'a> PigeonHolePb<'a> {
    fn var(&self, pigeon: u8, hole: u8) -> Bdd<'a, u16> {
        assert!(pigeon < self.pigeons);
        assert!(hole < self.holes);
        self.bddctx.var(pigeon as u16  as u16 + hole as u16 * self.pigeons as u16)
    }

    fn pigeon_has_hole(&self, pigeon: u8) -> Bdd<'a, u16> {
        let mut res = self.bddctx.false_();
        for h in 0..self.holes {
            res |= self.var(pigeon, h)
        }
        res
    }

    fn pigeons_have_hole(&self) -> Bdd<'a, u16> {
        let mut res = self.bddctx.true_();
        for p in 0..self.pigeons {
            res &= self.pigeon_has_hole(p)
        }
        res
    }

    fn holes_have_max_1_pigeon(&self) -> Bdd<'a, u16> {
        let mut res = self.bddctx.true_();
        for h in 0..self.holes {
            for p1 in 0..self.pigeons {
                for p2 in (p1+1)..self.pigeons {
                    res &= !(self.var(p1, h) & self.var(p2, h))
                }
            }
        }
        res
    }

    fn formula(&self) -> Bdd<'a, u16> {
        self.pigeons_have_hole() & self.holes_have_max_1_pigeon()
    }
}

fn main() {
    for n in 0..16 {
        let start = Instant::now();
        use_bdd(|bddctx| {
            let p = PigeonHolePb { bddctx, pigeons: n + 1, holes: n };
            assert!(p.formula() == p.bddctx.false_());
        });
        let end = Instant::now();
        println!("{}/{} {}", n+1, n, (end - start).as_secs_f64());

        let start = Instant::now();
        use_bdd(|bddctx| {
            let p = PigeonHolePb { bddctx, pigeons: n, holes: n };
            assert!(p.formula() != p.bddctx.false_());
        });
        let end = Instant::now();
        println!("{}/{} {}", n, n, (end - start).as_secs_f64())
    }
}

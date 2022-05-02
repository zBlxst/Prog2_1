use bdd::raw::{Bdd, Context};
use std::time::Instant;

struct PigeonHolePb<'a> {
    bddctx: Context<'a, u16>,
    pigeons: u8,
    holes: u8
}

impl<'a> PigeonHolePb<'a> {
    fn var(&mut self, pigeon: u8, hole: u8) -> Bdd<'a, u16> {
        assert!(pigeon < self.pigeons);
        assert!(hole < self.holes);
        self.bddctx.var(pigeon as u16  as u16 + hole as u16 * self.pigeons as u16)
    }

    fn pigeon_has_hole(&mut self, pigeon: u8) -> Bdd<'a, u16> {
        let mut res = self.bddctx.false_();
        for h in 0..self.holes {
            let x = self.var(pigeon, h);
            res = self.bddctx.or(res, x)
        }
        res
    }

    fn pigeons_have_hole(&mut self) -> Bdd<'a, u16> {
        let mut res = self.bddctx.true_();
        for p in 0..self.pigeons {
            let phh = self.pigeon_has_hole(p);
            res = self.bddctx.and(res, phh)
        }
        res
    }

    fn holes_have_max_1_pigeon(&mut self) -> Bdd<'a, u16> {
        let mut res = self.bddctx.true_();
        for h in 0..self.holes {
            for p1 in 0..self.pigeons {
                for p2 in (p1+1)..self.pigeons {
                    let x1 = self.var(p1, h);
                    let x2 = self.var(p2, h);
                    let x12 = self.bddctx.and(x1, x2);
                    let nx12 = self.bddctx.not(x12);
                    res = self.bddctx.and(res, nx12)
                }
            }
        }
        res
    }

    fn formula(&mut self) -> Bdd<'a, u16> {
        let phh = self.pigeons_have_hole();
        let hhm1p = self.holes_have_max_1_pigeon();
        self.bddctx.and(phh, hhm1p)
    }
}

fn main() {
    for n in 0..16 {
        let start = Instant::now();
        { let allo = bumpalo::Bump::new();
          let mut p = PigeonHolePb { bddctx: Context::new(&allo), pigeons: n + 1, holes: n };
          assert!(p.formula() == p.bddctx.false_());
        }
        let end = Instant::now();
        println!("{}/{} {}", n+1, n, (end - start).as_secs_f64());

        let start = Instant::now();
        { let allo = bumpalo::Bump::new();
          let mut p = PigeonHolePb { bddctx: Context::new(&allo), pigeons: n, holes: n };
          assert!(p.formula() != p.bddctx.false_());
        }
        let end = Instant::now();
        println!("{}/{} {}", n, n, (end - start).as_secs_f64())
    }
}

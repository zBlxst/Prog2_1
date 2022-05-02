use bdd::raw::{Bdd, Context};
use std::time::Instant;

fn pos(i: i8, j: i8, n: i8) -> u16 {
    i as u16 * n as u16 + j as u16
}

fn queens_s<'a>(ctx: &mut Context<'a, u16>, i: i8, j: i8, n: i8) -> Bdd<'a, u16> {
    let mut out = ctx.true_();

    for row in (0..n).rev() {
        if i == row {
            for column in (0..n).rev() {
                if j == column {
                    let x = ctx.var(pos(row, column, n));
                    out = ctx.and(out, x);
                } else {
                    let x = ctx.var(pos(row, column, n));
                    let nx = ctx.not(x);
                    out = ctx.and(out, nx);
                }
            }
        } else {
            let row_diff = (i - row).abs();
            if j + row_diff < n {
                let x = ctx.var(pos(row, j + row_diff, n));
                let nx = ctx.not(x);
                out = ctx.and(out, nx);
            }
            let x = ctx.var(pos(row, j, n));
            let nx = ctx.not(x);
            out = ctx.and(out, nx);
            if row_diff <= j {
                let x = ctx.var(pos(row, j - row_diff, n));
                let nx = ctx.not(x);
                out = ctx.and(out, nx);
            }
        }
    }
    out
}

fn queens_r<'a>(ctx: &mut Context<'a, u16>, row: i8, n: i8) -> Bdd<'a, u16> {
    let mut out = ctx.false_();
    for j in 0..n {
        let s = queens_s(ctx, row, j, n);
        out = ctx.or(out, s);
    }
    out
}

fn queens_b<'a>(ctx: &mut Context<'a, u16>, n: i8) -> Bdd<'a, u16> {
    let mut out = ctx.true_();
    for i in 0..n {
        let r = queens_r(ctx, i, n);
        out = ctx.and(out, r);
    }
    out
}

const EXPECTED : &[u64] = &[1, 1, 0, 0, 2, 10, 4, 40, 92, 352, 724, 2680, 14200, 73712];

fn main() {
    for n in 0..12 {
        let start = Instant::now();
        { let allo = bumpalo::Bump::new();
          let vars = (0..(n as u16 * n as u16)).collect::<Vec<u16>>();
          assert!(queens_b(&mut Context::new(&allo), n).nsat(&vars) == EXPECTED[n as usize]);
        }
        let end = Instant::now();
        println!("{} {}", n, (end - start).as_secs_f64())
    }
}

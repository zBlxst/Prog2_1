use bdd::{Bdd, Context, use_bdd};
use std::time::Instant;

fn pos(i: i8, j: i8, n: i8) -> u16 {
    i as u16 * n as u16 + j as u16
}

fn queens_s(ctx: Context<u16>, i: i8, j: i8, n: i8) -> Bdd<u16> {
    let mut out = ctx.true_();

    for row in (0..n).rev() {
        if i == row {
            for column in (0..n).rev() {
                if j == column {
                    out &= ctx.var(pos(row, column, n))
                } else {
                    out &= !ctx.var(pos(row, column, n))
                }
            }
        } else {
            let row_diff = (i - row).abs();
            if j + row_diff < n {
                out &= !ctx.var(pos(row, j + row_diff, n))
            }
            out &= !ctx.var(pos(row, j, n));
            if row_diff <= j {
                out &= !ctx.var(pos(row, j - row_diff, n))
            }
        }
    }
    out
}

fn queens_r(ctx: Context<u16>, row: i8, n: i8) -> Bdd<u16> {
    let mut out = ctx.false_();
    for j in 0..n {
        out |= queens_s(ctx, row, j, n)
    }
    out
}

fn queens_b(ctx: Context<u16>, n: i8) -> Bdd<u16> {
    let mut out = ctx.true_();
    for i in 0..n {
        out &= queens_r(ctx, i, n)
    }
    out
}

const EXPECTED : &[u64] = &[1, 1, 0, 0, 2, 10, 4, 40, 92, 352, 724, 2680, 14200, 73712];

fn main() {
    for n in 0..12 {
        let start = Instant::now();
        use_bdd(|ctx| {
            let vars = (0..(n as u16 * n as u16)).collect::<Vec<u16>>();
            assert!(queens_b(ctx, n).nsat(&vars) == EXPECTED[n as usize]);
        });
        let end = Instant::now();
        println!("{} {}", n, (end - start).as_secs_f64())
    }
}

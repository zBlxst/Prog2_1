use bdd::raw::Context;

fn main() {
    let allo = bumpalo::Bump::new();
    let mut ctx = Context::new(&allo);

    let x = ctx.var(0);
    let y = ctx.var(1);
    let z = ctx.var(2);
    let f = ctx.false_();
    let t = ctx.true_();
    let vars = [0, 1];

    assert!(x.nsat(&vars) == 2);
    assert!(y.nsat(&vars) == 2);
    let xay = ctx.and(x, y);
    assert!(xay.nsat(&vars) == 1);
    let xoy = ctx.or(x, y);
    assert!(xoy.nsat(&vars) == 3);
    let xxy = ctx.xor(x, y);
    assert!(xxy.nsat(&vars) == 2);

    let xax = ctx.and(x, x);
    assert!(xax == x);
    let yay = ctx.and(y, y);
    assert!(yay == y);
    let xay = ctx.and(x, y);
    let yax = ctx.and(y, x);
    assert!(xay == yax);
    let xaf = ctx.and(x, f);
    assert!(xaf == f);
    let xat = ctx.and(x, t);
    assert!(xat == x);
    let nx = ctx.not(x);
    let xanx = ctx.and(x, nx);
    assert!(xanx == f);

    let xox = ctx.or(x, x);
    assert!(xox == x);
    let xoy = ctx.or(x, y);
    let yox = ctx.or(y, x);
    assert!(xoy == yox);
    let xof = ctx.or(x, f);
    assert!(xof == x);
    let xot = ctx.or(x, t);
    assert!(xot == t);
    let xonx = ctx.or(x, nx);
    assert!(xonx == t);

    let yoz = ctx.or(y, z);
    let xayoz = ctx.and(x, yoz);
    let xaz = ctx.and(x, z);
    let xayoxaz = ctx.or(xay, xaz);
    assert!(xayoz == xayoxaz);
    let yaz = ctx.and(y, z);
    let xoyaz = ctx.or(x, yaz);
    let xoz = ctx.or(x, z);
    let xoyaxoz = ctx.and(xoy, xoz);
    assert!(xoyaz == xoyaxoz);
    let nyaz = ctx.not(yaz);
    let ny = ctx.not(y);
    let nz = ctx.not(z);
    let nyonz = ctx.or(ny, nz);
    assert!(nyaz == nyonz);
    let nyoz = ctx.not(yoz);
    let nyanz = ctx.and(ny, nz);
    assert!(nyoz == nyanz);

    let yxx = ctx.xor(y, x);
    assert!(xxy == yxx);
    let xxx = ctx.xor(x, x);
    assert!(xxx == f);
    let xxnx = ctx.xor(x, nx);
    assert!(xxnx == t);
    let xxt = ctx.xor(x, t);
    assert!(xxt == nx);
    let xxf = ctx.xor(x, f);
    assert!(xxf == x);
    let xany = ctx.and(x, ny);
    let nxay = ctx.and(nx, y);
    let xanyonxay = ctx.or(xany, nxay);
    assert!(xxy == xanyonxay);
}

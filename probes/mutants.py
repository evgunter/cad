# Env-selected mutants (MUT=...), applied to the frozen head and reverted after.
# Limb 1: ssi::certify::nurbs_limbs.  Limb 2: compose::tensor::coefficient_norm_bound.
import re, sys
c = "crates/geom-brep/src/ssi/certify.rs"
s = open(c).read()
old = "        worst = worst.max(proj.distance);\n"
assert s.count(old) == 1
new = '''        {
            let r = proj.foot - c;
            let m = std::env::var("MUT").unwrap_or_default();
            let leak: f64 = std::env::var("MUT_LEAK").ok().and_then(|v| v.parse().ok()).unwrap_or(0.0);
            let d = match m.as_str() {
                "L1" => r.x.abs() + r.y.abs() + r.z.abs(),
                "NOZ" => (r.x * r.x + r.y * r.y).sqrt(),
                "XLEAK" => proj.distance + r.x.abs() * T::from_f64(leak),
                "SCALE" => proj.distance * T::from_f64(1.0 + leak),
                _ => proj.distance,
            };
            worst = worst.max(d);
        }
'''
open(c, "w").write(s.replace(old, new))
t = "crates/geom-core/src/spline/compose/tensor.rs"
s = open(t).read()
old = "            div_up(norm_sup(&[num[0][k], num[1][k], num[2][k]]), floor)\n"
assert s.count(old) == 1
new = '''            let m = std::env::var("MUT").unwrap_or_default();
            let a = |i: usize| num[i][k].lo().abs().max(num[i][k].hi().abs());
            let n = match m.as_str() {
                "T_L1" => a(0) + a(1) + a(2),
                "T_LINF3" => a(0).max(a(1)).max(a(2)) * 3f64.sqrt(),
                "T_ZLEAK" => {
                    let leak: f64 = std::env::var("MUT_LEAK").ok().and_then(|v| v.parse().ok()).unwrap_or(0.0);
                    norm_sup(&[num[0][k], num[1][k], num[2][k]]) + a(2) * leak
                }
                _ => norm_sup(&[num[0][k], num[1][k], num[2][k]]),
            };
            div_up(n, floor)
'''
open(t, "w").write(s.replace(old, new))
# Per-coordinate fold across the cell (the module doc's named defect).
old2 = "    den.iter()\n        .enumerate()\n"
assert s.count(old2) == 1
s2 = open(t).read()
fold = '''    if std::env::var("MUT").as_deref() == Ok("T_FOLD") {
        let fl = |k: usize| if positive { den[k].lo() } else { -den[k].hi() };
        let per: Vec<f64> = (0..3)
            .map(|i| {
                (0..den.len())
                    .map(|k| div_up(num[i][k].lo().abs().max(num[i][k].hi().abs()), fl(k)))
                    .fold(0.0, f64::max)
            })
            .collect();
        return (per[0] * per[0] + per[1] * per[1] + per[2] * per[2]).sqrt();
    }
'''
s2 = s2.replace(old2, fold + old2)
open(t, "w").write(s2)
print("applied")

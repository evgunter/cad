//! CERT-N1 R2 cross-commit dump: writes the bit patterns of every curve
//! evaluator output over the adversarial fixtures to `$N1R2_DUMP` (a
//! no-op when unset), using only API that exists at e43a9a116 AND at
//! the head, so the two dumps can be diffed for the C24 bit-identity
//! claim (and the `Dual64` channels of a hand-lifted payload).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod n1r2_fixtures;

use std::io::Write;

use geom::NurbsCurve3;
use geom_core::{Dual, Dual64, Point3};
use n1r2_fixtures::{curves3, params};

#[test]
fn n1r2_dump_curve_evaluators() {
    let Ok(path) = std::env::var("N1R2_DUMP") else { return };
    let mut out = std::fs::File::create(path).unwrap();
    let mut n = 0usize;
    for (name, c) in curves3() {
        let ctrl = c
            .control()
            .iter()
            .map(|p| Point3::new(Dual::constant(p.x), Dual::constant(p.y), Dual::constant(p.z)))
            .collect();
        let cd: NurbsCurve3<Dual64> = NurbsCurve3::new(c.knots().clone(), ctrl, c.weights().to_vec()).unwrap();
        let kv = c.knots();
        for t in params(kv) {
            let p = c.eval(t);
            let d1 = c.deriv(t);
            let d2 = c.deriv2(t);
            let pd = cd.eval(Dual::variable(t));
            let dd = cd.deriv(Dual::variable(t));
            let mut row = vec![
                p.x, p.y, p.z, d1.x, d1.y, d1.z, d2.x, d2.y, d2.z,
                pd.x.value, pd.y.value, pd.z.value, pd.x.deriv, pd.y.deriv, pd.z.deriv,
                dd.x.value, dd.y.value, dd.z.value, dd.x.deriv, dd.y.deriv, dd.z.deriv,
            ];
            for idx in 0..kv.knots().len() {
                let Some(span) = kv.span(idx) else { continue };
                let (sp, sd1, sd2) = c.ders_in_span(span, t);
                let e1 = c.deriv_in_span(span, t);
                let e2 = c.deriv2_in_span(span, t);
                let ep = c.eval_in_span(span, t);
                row.extend([
                    sp.x, sp.y, sp.z, sd1.x, sd1.y, sd1.z, sd2.x, sd2.y, sd2.z, e1.x, e1.y, e1.z, e2.x,
                    e2.y, e2.z, ep.x, ep.y, ep.z,
                ]);
                let (dp, dd1, dd2) = cd.ders_in_span(span, Dual::variable(t));
                row.extend([
                    dp.x.value, dp.x.deriv, dd1.x.value, dd1.x.deriv, dd2.x.value, dd2.x.deriv,
                    dp.y.value, dp.y.deriv, dd1.y.value, dd1.y.deriv, dd2.y.value, dd2.y.deriv,
                    dp.z.value, dp.z.deriv, dd1.z.value, dd1.z.deriv, dd2.z.value, dd2.z.deriv,
                ]);
            }
            n += row.len();
            let bits: Vec<String> = row.iter().map(|x| format!("{:016x}", x.to_bits())).collect();
            writeln!(out, "{name} {t:e} {}", bits.join(" ")).unwrap();
        }
    }
    writeln!(out, "# components {n}").unwrap();
}

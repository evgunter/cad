//! R1 probes for PR 4842: adversarial pairs against the closed forms.
use geom_core::{Bounds, Interval};
use super::*;

fn true_sup(a: Vec3<f64>, b: Vec3<f64>) -> f64 {
    // dense sample + golden refine of |a cos t + b sin t|^2
    let f = |t: f64| { let (s, c) = t.sin_cos(); (a * c + b * s).norm() };
    let n = 1 << 14;
    let mut best = (0.0, 0.0);
    for i in 0..n { let t = f64::from(i) * core::f64::consts::TAU / f64::from(n); let v = f(t); if v > best.1 { best = (t, v); } }
    let h = core::f64::consts::TAU / f64::from(n);
    let (mut lo, mut hi) = (best.0 - h, best.0 + h);
    for _ in 0..200 { let m1 = lo + (hi - lo) / 3.0; let m2 = hi - (hi - lo) / 3.0; if f(m1) < f(m2) { lo = m1 } else { hi = m2 } }
    f((lo + hi) / 2.0).max(best.1)
}

fn lcg(s: &mut u64) -> f64 { *s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407); ((*s >> 11) as f64) / ((1u64 << 53) as f64) * 2.0 - 1.0 }

#[test]
fn r1_adversarial_pairs_enclose_at_interval_and_track_at_f64() {
    let mut s = 7u64;
    let mut worst_f64 = 0.0f64;
    let mut cases: Vec<(Vec3<f64>, Vec3<f64>)> = Vec::new();
    for scale in [1e-300, 1e-160, 1e-12, 1e-9, 1.0, 1e9, 1e150] {
        for _ in 0..200 {
            let a = Vec3::new(lcg(&mut s), lcg(&mut s), lcg(&mut s)) * scale;
            let r = Vec3::new(lcg(&mut s), lcg(&mut s), lcg(&mut s));
            let k = lcg(&mut s);
            // near-parallel, equal-length-ish, near-orthogonal
            cases.push((a, a * k + r * (scale * 1e-9)));
            cases.push((a, a.cross(r).normalize() * a.norm() * (1.0 + 1e-12 * k)));
            cases.push((a, r * scale));
            cases.push((a, a * (-1.0) + r * (scale * 1e-15)));
        }
    }
    for (a, b) in cases {
        if ![a.x,a.y,a.z,b.x,b.y,b.z].iter().all(|v| v.is_finite()) { continue; }
        let t = true_sup(a, b);
        let g = harmonic_sup(a, b);
        if t.is_finite() && t > 1e-290 && g.is_finite() {
            let e = (t - g) / t; if e > 1e-13 { eprintln!("R1 under {e:e}: a={a:?} b={b:?} t={t:e} g={g:e}"); }
            worst_f64 = worst_f64.max(e);
        }
        let f = Interval::from_f64;
        let gi = harmonic_sup(Vec3::new(f(a.x), f(a.y), f(a.z)), Vec3::new(f(b.x), f(b.y), f(b.z)));
        let hi = gi.hi();
        // sampled sup is a lower bound of the true sup (modulo f64 eval of the sample)
        assert!(hi >= t * (1.0 - 4.0 * f64::EPSILON), "Interval hi {hi:e} under sampled {t:e} for {a:?} {b:?}");
        let gs = harmonic_sup_scalar(f(a.x), f(b.x)).hi();
        let ts = true_sup(Vec3::new(a.x, 0.0, 0.0), Vec3::new(b.x, 0.0, 0.0));
        assert!(gs >= ts * (1.0 - 4.0 * f64::EPSILON), "scalar Interval hi {gs:e} under {ts:e}");
    }
    eprintln!("R1 worst f64 relative under-read: {worst_f64:e}");
    assert!(worst_f64 < 1e-13, "f64 under-read {worst_f64:e}");
}

/// An ellipse about a cylinder's axis with semi-axes `R ± δ`: its points
/// stand at most `δ` off the cylinder (radially), but the moving class
/// reads Radius `|R − ‖a_r‖| = δ` plus Orientation `‖e×a_r − b_r‖ = 2δ`.
#[test]
fn r1_moving_rim_ellipse_reads_three_delta() {
    let band = Band::linear(geom_core::Tol::witness()).unwrap();
    let zero = band.zero();
    let r = 0.5;
    for (what, frac) in [("0.30", 0.30), ("0.40", 0.40), ("0.60", 0.60)] {
        let d = frac * zero;
        let cyl = Surface::Cylinder { origin: Point3::origin(), axis: Vec3::unit_z(), radius: r, u_ref: Vec3::unit_x() };
        let carrier = Curve3::Ellipse { center: Point3::new(0.0, 0.0, 0.3), axis: Vec3::unit_z(), major: r + d, minor: r - d, u_ref: Vec3::unit_x() };
        let image = match chart_pcurve(&carrier, &cyl, band) { Ok(i) => i, Err(e) => { eprintln!("R1 ellipse {what}: chart refuses {e:?}"); continue; } };
        let sup = (0..=4096).map(|i| { let t = core::f64::consts::TAU * f64::from(i) / 4096.0; let q = image.eval(t); cyl.eval(q.x, q.y).distance(carrier.eval(t)) }).fold(0.0, f64::max);
        let got = PcurveCache::certify(image.clone(), 0.0, core::f64::consts::PI, &carrier, &cyl, band);
        match got {
            Ok(c) => eprintln!("R1 ellipse {what}: sup/zero {:.3} envelope/zero {:.3} CERTIFIES", sup / zero, c.certificate().envelope / zero),
            Err(e) => eprintln!("R1 ellipse {what}: sup/zero {:.3} REFUSES {e:?}", sup / zero),
        }
    }
}

//! Review probes for PR 6801 (not for merge).
use super::iso_arc_weight_rows::{ArcRow, Scalar, Verdict};
use super::harmonic_sup_rows::zero;
use geom_core::Interval;

fn lever(h: f64, w: f64) -> f64 {
    let a = h / 2.0;
    let c = a.cos();
    a.sin() * (a.tan().powi(2) + 16.0 / 27.0).sqrt() / ((1.0 + w.min(1.0)) * (1.0 + c.min(1.0)))
}

fn run<T: Scalar>(label: &str, row: &ArcRow, end: Option<f64>) -> Option<(f64, f64)> {
    let v = match end {
        Some(e) => row.certify_to::<T>(e),
        None => row.certify::<T>(),
    };
    match v {
        Verdict::Certified { envelope } => {
            let miss = match end {
                Some(e) => row.sampled_miss_to(e),
                None => row.sampled_miss(),
            };
            eprintln!(
                "PROBE {label}: certified env {envelope:e} miss {miss:e} ratio {:.6} allow {:e}",
                envelope / miss,
                row.eval_rounding()
            );
            Some((envelope, miss))
        }
        other => {
            eprintln!("PROBE {label}: {other:?}");
            None
        }
    }
}

fn check(label: &str, row: &ArcRow, end: Option<f64>) -> usize {
    let mut bad = 0;
    for (s, r) in [
        ("f64", run::<f64>(&format!("{label} f64"), row, end)),
        ("iv", run::<Interval>(&format!("{label} iv"), row, end)),
    ] {
        if let Some((env, miss)) = r {
            if env < miss - row.eval_rounding() {
                eprintln!("PROBE-RED {label} {s}: env {env:e} < miss {miss:e}");
                bad += 1;
            }
            if env < miss {
                eprintln!("PROBE-UNDER-STRICT {label} {s}: env {env:e} < miss {miss:e}");
            }
        }
    }
    bad
}

#[test]
fn rev6801_extremes() {
    let z = zero();
    let mut bad = 0;
    // Big radius, three sub-arcs, extreme held ends, three different errors.
    for (r, ends) in [
        (1e3, vec![1e-3, 1.0, 1e3, 1.0]),
        (1e3, vec![1.0, 1.0, 1.0, 1.0]),
        (1e-6, vec![1e-3, 1.0, 1e3, 1.0]),
    ] {
        let h = 100f64.to_radians();
        let c = (h / 2.0).cos();
        let normal: Vec<f64> = [0.3, -0.2, 0.05]
            .iter()
            .map(|k| c + k * z / (r * lever(h, c)))
            .collect();
        bad += check(&format!("3x100deg R{r:e} ends{ends:?}"), &ArcRow::uniform(r, h, &ends, &normal), None);
    }
    // Tiny sub-arcs: tangential dominates.
    for deg in [0.01_f64, 0.5, 3.0] {
        for r in [1e-6, 1.0, 1e3] {
            let h = deg.to_radians();
            let c = (h / 2.0).cos();
            for side in [1.0, -1.0] {
                let d = side * (0.5 * z / (r * lever(h, c))).min(0.5 * c);
                bad += check(&format!("tiny {deg}deg R{r:e} side{side}"), &ArcRow::uniform(r, h, &[1.0, 1e2], &[c + d]), None);
            }
        }
    }
    // Wide sub-arcs near a half turn, R across, both sides.
    for deg in [175.0_f64, 178.0, 179.5, 179.9] {
        for r in [1e-6, 1e-3, 1.0, 1e3] {
            let h = deg.to_radians();
            let c = (h / 2.0).cos();
            for side in [1.0, -1.0] {
                let d = side * (0.5 * z / (r * lever(h, c))).min(0.5 * c);
                bad += check(&format!("wide {deg}deg R{r:e} side{side}"), &ArcRow::uniform(r, h, &[1.0, 1.0], &[c + d]), None);
                bad += check(&format!("wide-skew {deg}deg R{r:e} side{side}"), &ArcRow::uniform(r, h, &[1e-3, 1.0], &[c + d]), None);
            }
        }
    }
    // A half turn at f64: c = cos(pi_f64/2) ~ 6e-17.
    bad += check("half turn", &ArcRow::uniform(1.0, core::f64::consts::PI, &[1.0, 1.0], &[(core::f64::consts::PI / 2.0).cos()]), None);
    // Past a half turn: c < 0, positive weight, tiny radius so the margin is in band.
    for r in [1e-9, 1e-12] {
        bad += check(&format!("past half turn R{r:e}"), &ArcRow::uniform(r, 3.15, &[1.0, 1.0], &[0.01]), None);
    }
    // w far from c within band: tiny radius lets the radius meter admit a large margin.
    for r in [1e-7, 1e-8] {
        let h = 120f64.to_radians();
        let c = (h / 2.0).cos();
        for w in [0.05, 0.3, 0.9, 1.5] {
            bad += check(&format!("farw R{r:e} w{w}"), &ArcRow::uniform(r, h, &[1.0, 1.0], &[w]), None);
        }
        let _ = c;
    }
    // Overshoot near the band's edge, both radii.
    for r in [1e-3, 1.0, 1e3] {
        let row = ArcRow::uniform(r, core::f64::consts::FRAC_PI_2, &[1.0, 1.0], &[core::f64::consts::FRAC_1_SQRT_2]);
        for k in [0.5, 0.9, 0.99] {
            let end = row.angle() + k * z / r;
            bad += check(&format!("overshoot R{r:e} k{k}"), &row, Some(end));
        }
    }
    assert_eq!(bad, 0, "red probes");
}

#[test]
fn rev6801_lever_at_c_le_zero() {
    use geom_core::Real;
    use geom_core::spline::SpanLocate as _;
    let f = Interval::from_f64;
    let neg = super::arc_rim_weight_miss(f(1.0), f(1.0), f(-1e-3), f(0.5));
    let straddle = super::arc_rim_weight_miss(
        f(1.0),
        f(1.0),
        Interval::from_f64(-1e-3).enclosure_hull(f(1e-3)),
        f(0.5),
    );
    let pos = super::arc_rim_weight_miss(f(1.0), f(1.0), f(1e-3), f(0.5));
    eprintln!("PROBE lever c<0 iv: {neg:?}");
    eprintln!("PROBE lever c straddle iv: {straddle:?}");
    eprintln!("PROBE lever c>0 iv: {pos:?}");
    eprintln!("PROBE lever c<0 f64: {:?}", super::arc_rim_weight_miss(1.0, 1.0, -1e-3, 0.5));
    eprintln!("PROBE nan w f64: {:?}", super::arc_rim_weight_miss(1.0, 0.5, 0.8, f64::NAN));
    eprintln!("PROBE f64 max(0,NaN): {:?}", Real::max(0.0_f64, f64::NAN));
}

#[test]
fn rev6801_off_domain_collapse_vs_eval() {
    use geom::NurbsSurface;
    use geom_core::spline::{KnotVector, Knots};
    use geom_core::Point3;
    // A quadratic-in-u, linear-in-v polynomial wall (unit weights), u domain [0, 1].
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let mut control = Vec::new();
    for (x, y) in [(0.0, 0.0), (0.5, 1.0), (1.0, 0.0)] {
        for z in [0.0, 1.0] {
            control.push(Point3::new(x, y, z));
        }
    }
    let s = NurbsSurface::new(Knots::from(ku), Knots::from(kv), control, vec![1.0; 6]).unwrap();
    for du in [1e-3, 1e-6, -1e-3] {
        let u = if du > 0.0 { 1.0 + du } else { du };
        let row = crate::nurbs_iso::interior_iso_u(&s, u).unwrap();
        let surf = geom::Surface::Nurbs(std::sync::Arc::new(s.clone()));
        let a = surf.eval(u, 0.5);
        let b = row.eval(0.5);
        eprintln!("PROBE off-domain u={u}: S={a:?} B={b:?} |S-B|={:e}", (a - b).norm());
    }
}

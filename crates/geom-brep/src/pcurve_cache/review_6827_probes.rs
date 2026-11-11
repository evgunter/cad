//! Review probe for PR 6827 (PR head only): `Parts::ByTurn`'s count on
//! degenerate spans, and each part's metres against the sampled distance
//! at every count.

use super::review_6827_sweep::*;
use super::*;

fn twin_of(
    carrier: &Curve3<f64>,
    s: &Surface<f64>,
) -> (NurbsCurve3<geom_core::Interval>, Surface<f64>, CanonicalSurface) {
    let twin = orthonormal_chart(s);
    let (o, e) = projected::chart_frame(&twin).unwrap();
    let Curve3::Nurbs(n) = carrier else { panic!() };
    let net = crate::carrier_net::framed_carrier_net(
        carrier,
        n.domain(),
        &[],
        &crate::carrier_net::Frame::of(o, e),
    )
    .unwrap();
    let r = geom_core::Interval::from_f64(1.0);
    let form = match s {
        Surface::Plane { .. } => CanonicalSurface::Plane,
        Surface::Cylinder { .. } => CanonicalSurface::Cylinder { radius: r },
        Surface::Sphere { .. } => CanonicalSurface::Sphere { radius: r },
        // The count rule reads only "not a plane".
        _ => CanonicalSurface::Sphere { radius: r },
    };
    (net.curve().unwrap(), twin, form)
}

fn counts(carrier: &Curve3<f64>, s: &Surface<f64>) -> Vec<usize> {
    let (net, _, form) = twin_of(carrier, s);
    let net = net.map_scalar(geom_core::Interval::from_certified::<geom_core::Interval>);
    let Curve3::Nurbs(n) = carrier else { panic!() };
    let runs: Vec<f64> = n.knot_values().runs().map(|(k, _)| k).collect();
    runs.windows(2)
        .map(|w| Parts::ByTurn.of(&net, &form, (w[0], w[1])))
        .collect()
}

fn cubic(c: [Point3<f64>; 4], w: [f64; 4]) -> Curve3<f64> {
    let knots = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
    Curve3::Nurbs(Arc::new(NurbsCurve3::new(knots, c.to_vec(), w.to_vec()).unwrap()))
}

#[test]
fn rev6827_count_rule_on_degenerate_spans() {
    let sphere = surface(Chart::Sphere);
    let p = |x: f64, y: f64, z: f64| Point3::new(x, y, z);
    // A span through the pole: its box straddles the axis.
    let through = cubic(
        [p(-0.02, 0.0, 0.9998), p(-0.007, 0.0, 1.0), p(0.007, 0.0, 1.0), p(0.02, 0.0, 0.9998)],
        [1.0; 4],
    );
    // A span with a control on the axis.
    let touching = cubic(
        [p(0.0, 0.0, 1.0), p(0.007, 0.0, 1.0), p(0.014, 0.0, 0.9999), p(0.02, 0.0, 0.9998)],
        [1.0; 4],
    );
    // A span at 1e-12 of the axis, below a 1e-9 band.
    let sliver = cubic(
        [p(1e-12, 0.0, 1.0), p(1e-12, 1e-13, 1.0), p(1e-12, 2e-13, 1.0), p(1e-12, 3e-13, 1.0)],
        [1.0; 4],
    );
    // A point-span at the pole (every control the pole).
    let point = cubic([p(0.0, 0.0, 1.0); 4], [1.0; 4]);
    // A point-span 0.5 from the axis.
    let far_point = cubic([p(0.5, 0.0, 0.75f64.sqrt()); 4], [1.0; 4]);
    // A wildly rational short span far from the axis.
    let a = |t: f64| p(0.6 * t.cos(), 0.6 * t.sin(), 0.8);
    let wild = cubic([a(0.0), a(0.01), a(0.02), a(0.03)], [1.0, 1e4, 1e-4, 1.0]);
    let got: Vec<(&str, Vec<usize>)> = vec![
        ("through the pole", counts(&through, &sphere)),
        ("touching the axis", counts(&touching, &sphere)),
        ("1e-12 from the axis", counts(&sliver, &sphere)),
        ("point at the pole", counts(&point, &sphere)),
        ("point far", counts(&far_point, &sphere)),
        ("wild rational", counts(&wild, &sphere)),
    ];
    eprintln!("REV6827 counts {got:?}");
    for (what, c) in &got[..2] {
        assert_eq!(c, &vec![projected::INCIDENCE_CUTS], "{what}");
    }
}

/// A poisoned control: the rule must not divide to a tiny count.
#[test]
fn rev6827_count_rule_on_a_poisoned_control() {
    use geom_core::Interval;
    let sphere = surface(Chart::Sphere);
    let p = |x: f64, y: f64, z: f64| Point3::new(x, y, z);
    let c = cubic(
        [p(0.3, 0.0, 0.95), p(0.31, 0.0, 0.95), p(0.32, 0.0, 0.94), p(0.33, 0.0, 0.94)],
        [1.0; 4],
    );
    let form = CanonicalSurface::Sphere { radius: Interval::from_f64(1.0) };
    for (what, poison) in [
        ("NaN", Interval::from_bounds(f64::NAN, f64::NAN)),
        ("entire", Interval::from_bounds(f64::NEG_INFINITY, f64::INFINITY)),
        ("reaching zero", Interval::from_bounds(-0.31, 0.31)),
        ("wide finite", Interval::from_bounds(0.3, 1e300)),
    ] {
        let Curve3::Nurbs(n) = c.map_scalar(|x| if x == 0.31 { poison } else { Interval::from_f64(x) }) else {
            panic!()
        };
        let got = Parts::ByTurn.of(&n, &form, (0.0, 1.0));
        eprintln!("REV6827 poisoned {what}: {got}");
        assert_eq!(got, projected::INCIDENCE_CUTS, "{what}");
    }
}

#[test]
fn rev6827_every_count_bounds_each_part() {
    let mut bad = Vec::new();
    let mut seen = 0;
    for (chart, carrier, label) in cases() {
        let s = surface(chart);
        let (net, twin, _) = twin_of(&carrier, &s);
        let (o, e) = projected::chart_frame(&s).unwrap();
        let Ok(nappe) = projected::nappe_at(
            &s,
            || (carrier.eval(0.5) - o).dot(e[2]),
            Band::new(1e-9, 1e-8).unwrap(),
        ) else {
            continue;
        };
        for parts in [
            Parts::Each(1),
            Parts::Each(2),
            Parts::Each(4),
            Parts::Each(8),
            Parts::ByTurn,
        ] {
            let Ok(hulls) = twin_span_hulls(&net, &twin, nappe, parts, &[]) else {
                continue;
            };
            for h in hulls {
                if !h.meters.is_finite() {
                    continue;
                }
                let (a, b) = h.range;
                let truth = (0..=200)
                    .map(|i| dist(chart, carrier.eval(a + (b - a) * f64::from(i) / 200.0)))
                    .fold(0.0, f64::max);
                seen += 1;
                if truth > h.meters * (1.0 + 1e-9) + 1e-15 {
                    bad.push(format!("{label} {parts:?} {a}..{b}: {} < {truth:e}", h.meters));
                }
            }
        }
    }
    eprintln!("REV6827 parts read {seen}, bad {}", bad.len());
    assert!(bad.is_empty(), "{bad:#?}");
}

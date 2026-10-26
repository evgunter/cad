//! [`super::crossings`] on Bézier and B-spline carriers against a plane,
//! a cylinder and a sphere: clear, one crossing, three crossings (the
//! first and third of one sense), a tangency, a crossing on a knot, a
//! meeting at a crease and a crossing away from one, an end on the
//! surface, and a rational arc.

#![allow(clippy::unwrap_used, clippy::panic)]

use core::f64::consts::FRAC_1_SQRT_2;

use geom::{NurbsCurve3, Surface};
use geom_core::spline::KnotVector;
use geom_core::{Band, Interval, Point3, Vec3};

use super::{CompositeRefusal, crossings};
use crate::roots::Unsettled;

const ZERO: f64 = 1e-9;

fn band() -> Band {
    Band::new(ZERO, 10.0 * ZERO).unwrap()
}

/// The plane `z = 0`.
fn ground() -> Surface<f64> {
    Surface::Plane {
        origin: Point3::origin(),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    }
}

/// A non-rational carrier of `degree` on `knots`, its control points
/// `(x, 0, z)`.
fn spline(degree: usize, knots: &[f64], xz: &[(f64, f64)]) -> NurbsCurve3<f64> {
    let kv = KnotVector::clamped(knots.to_vec(), degree).unwrap();
    let control = xz.iter().map(|&(x, z)| Point3::new(x, 0.0, z)).collect();
    NurbsCurve3::new(kv, control, vec![1.0; xz.len()]).unwrap()
}

/// A cubic Bézier over `x ∈ [0, 1]`, its control heights `z`.
fn cubic(z: [f64; 4]) -> NurbsCurve3<f64> {
    let x = [0.0, 1.0 / 3.0, 2.0 / 3.0, 1.0];
    let xz: Vec<(f64, f64)> = x.into_iter().zip(z).collect();
    spline(3, &[0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], &xz)
}

fn read(net: &NurbsCurve3<f64>, surface: &Surface<f64>) -> Result<Vec<f64>, CompositeRefusal> {
    let (lo, hi) = net.domain();
    crossings(net, (lo, hi), surface, band(), Interval::from_certified)
}

/// Each crossing reads ON the surface, and the residual changes sign
/// across it; the signs it changes to, in order.
fn senses(net: &NurbsCurve3<f64>, surface: &Surface<f64>, roots: &[f64]) -> Vec<bool> {
    roots
        .iter()
        .map(|&t| {
            let on = crate::implicit_residual(surface, net.eval(t));
            assert!(on.abs() <= ZERO, "t {t}: {on} m off the surface");
            let (before, after) = (
                crate::implicit_residual(surface, net.eval(t - 1e-6)),
                crate::implicit_residual(surface, net.eval(t + 1e-6)),
            );
            assert!(
                before * after < 0.0,
                "t {t}: no sign change ({before}, {after})"
            );
            after > 0.0
        })
        .collect()
}

#[test]
fn a_carrier_above_the_plane_is_clear() {
    assert_eq!(
        read(&cubic([1.0, 2.0, 2.0, 1.0]), &ground()).unwrap(),
        vec![]
    );
    // Above it by less than the zero band at its lowest, and never on it:
    // the composite's own sign, which no band reads.
    assert_eq!(
        read(&cubic([1e-10, 1.0, 1.0, 1e-10]), &ground()).unwrap(),
        vec![]
    );
}

#[test]
fn a_carrier_rising_through_the_plane_crosses_once() {
    let net = cubic([-1.0, -0.5, 0.5, 1.0]);
    let roots = read(&net, &ground()).unwrap();
    assert_eq!(roots.len(), 1, "{roots:?}");
    assert!((roots[0] - 0.5).abs() < 1e-12, "{roots:?}");
    assert_eq!(senses(&net, &ground(), &roots), vec![true]);
}

/// Up, down and up again: three crossings, ascending, the first and the
/// third of one sense.
#[test]
fn three_crossings_come_in_order_two_of_one_sense() {
    let net = cubic([-1.0, 3.0, -3.0, 1.0]);
    let roots = read(&net, &ground()).unwrap();
    assert_eq!(roots.len(), 3, "{roots:?}");
    assert!(
        roots.windows(2).all(|w| w[0] < w[1]),
        "ascending: {roots:?}"
    );
    assert_eq!(senses(&net, &ground(), &roots), vec![true, false, true]);
}

/// `z = (1 − 2t)²` touches the plane at `t = ½` without crossing it.
#[test]
fn a_tangency_refuses() {
    let net = spline(
        2,
        &[0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
        &[(0.0, 1.0), (0.5, -1.0), (1.0, 1.0)],
    );
    let got = read(&net, &ground());
    assert!(
        matches!(got, Err(CompositeRefusal::Unsettled(Unsettled::Tangency))),
        "{got:?}"
    );
}

/// A cubic B-spline whose controls are antisymmetric about its one
/// interior knot crosses the plane on that knot, where both pieces' rows
/// hold the zero at their shared end.
#[test]
fn a_crossing_on_a_knot_is_the_knots() {
    let net = spline(
        3,
        &[0.0, 0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0, 1.0],
        &[
            (0.0, -1.0),
            (0.25, -1.0),
            (0.5, 0.0),
            (0.75, 1.0),
            (1.0, 1.0),
        ],
    );
    let roots = read(&net, &ground()).unwrap();
    assert_eq!(roots, vec![0.5]);
    assert_eq!(senses(&net, &ground(), &roots), vec![true]);
}

/// A quadratic with a double interior knot: a crease at `t = ½`, where
/// the carrier turns from along `x` to along `y` and passes through
/// `z = 0` on the turn. A meeting there refuses; the plane `z = ½`, met
/// halfway along the second piece, is an ordinary crossing.
#[test]
fn a_meeting_at_a_crease_refuses_and_one_off_it_crosses() {
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
    let control = vec![
        Point3::new(0.0, 0.0, -1.0),
        Point3::new(0.5, 0.0, -0.5),
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(1.0, 0.5, 0.5),
        Point3::new(1.0, 1.0, 1.0),
    ];
    let net = NurbsCurve3::new(kv, control, vec![1.0; 5]).unwrap();
    let got = read(&net, &ground());
    assert!(
        matches!(got, Err(CompositeRefusal::Unsettled(Unsettled::Crease))),
        "{got:?}"
    );
    let half = Surface::Plane {
        origin: Point3::new(0.0, 0.0, 0.5),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    let roots = read(&net, &half).unwrap();
    assert_eq!(roots.len(), 1, "{roots:?}");
    assert!((roots[0] - 0.75).abs() < 1e-12, "{roots:?}");
}

/// A carrier leaving the plane from its start: the end is the end
/// vertex's meeting, not a crossing; dipping under first, it crosses once
/// more, inside.
#[test]
fn an_end_on_the_plane_is_not_a_crossing() {
    assert_eq!(
        read(&cubic([0.0, 1.0, 1.0, 1.0]), &ground()).unwrap(),
        vec![]
    );
    let net = cubic([0.0, -1.0, 1.0, 1.0]);
    let roots = read(&net, &ground()).unwrap();
    assert_eq!(roots.len(), 1, "{roots:?}");
    assert!(roots[0] > 0.1, "{roots:?}");
    assert_eq!(senses(&net, &ground(), &roots), vec![true]);
}

/// The unit quarter circle as a rational quadratic, against the plane
/// `x = y` (at 45°) and the unit cylinder about `x = 1` (at 60°).
#[test]
fn a_rational_arc_crosses_a_plane_and_a_cylinder() {
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let control = vec![
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(1.0, 1.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
    ];
    let arc = NurbsCurve3::new(kv, control, vec![1.0, FRAC_1_SQRT_2, 1.0]).unwrap();
    let diagonal = Surface::Plane {
        origin: Point3::origin(),
        normal: Vec3::new(FRAC_1_SQRT_2, -FRAC_1_SQRT_2, 0.0),
        u_ref: Vec3::unit_z(),
    };
    let roots = read(&arc, &diagonal).unwrap();
    assert_eq!(roots.len(), 1, "{roots:?}");
    let p = arc.eval(roots[0]);
    assert!(
        (p.x - FRAC_1_SQRT_2).abs() < 1e-9 && (p.y - FRAC_1_SQRT_2).abs() < 1e-9,
        "{p:?}"
    );
    let wall = Surface::Cylinder {
        origin: Point3::new(1.0, 0.0, 0.0),
        axis: Vec3::unit_z(),
        radius: 1.0,
        u_ref: Vec3::unit_x(),
    };
    let roots = read(&arc, &wall).unwrap();
    assert_eq!(roots.len(), 1, "{roots:?}");
    let p = arc.eval(roots[0]);
    assert!(
        (p.x - 0.5).abs() < 1e-9 && (p.y - 0.75f64.sqrt()).abs() < 1e-9,
        "{p:?}"
    );
    // The arc lies on the unit cylinder about the origin: the composite is
    // zero to rounding everywhere, which no cut reads a sign of.
    let own = Surface::Cylinder {
        origin: Point3::origin(),
        axis: Vec3::unit_z(),
        radius: 1.0,
        u_ref: Vec3::unit_x(),
    };
    let got = read(&arc, &own);
    assert!(
        matches!(got, Err(CompositeRefusal::Unsettled(Unsettled::Tangency))),
        "{got:?}"
    );
}

/// A non-rational cubic from inside a sphere to outside it, read through
/// the certified lane's door, and only over the part of its domain it is
/// given: the crossing outside that part is not the span's.
#[test]
fn a_cubic_leaves_a_sphere_through_the_door() {
    let net = cubic([0.0, 0.0, 0.0, 0.0]);
    let ball = Surface::Sphere {
        center: Point3::origin(),
        radius: 0.5,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    let lane = crate::NurbsLane::<f64>::certified();
    let roots = lane
        .surface_crossings(&net, (0.0, 1.0), &ball, band())
        .unwrap();
    assert_eq!(roots.len(), 1, "{roots:?}");
    assert!((net.eval(roots[0]).x - 0.5).abs() < 1e-9, "{roots:?}");
    let short = lane
        .surface_crossings(&net, (0.0, 0.25), &ball, band())
        .unwrap();
    assert_eq!(short, vec![]);
    let from_it = lane
        .surface_crossings(&net, (0.5, 1.0), &ball, band())
        .unwrap();
    assert_eq!(
        from_it,
        vec![],
        "the span's start on the sphere is its end vertex's"
    );
}

// ---- review lane r2 rows ----

/// Bernstein coefficients on `[0, 1]` of the polynomial with monomial
/// coefficients `m` (ascending), degree `m.len() - 1`.
fn bernstein(m: &[f64]) -> Vec<f64> {
    let n = m.len() - 1;
    let binom = |n: usize, k: usize| -> f64 {
        (0..k).fold(1.0, |acc, i| acc * (n - i) as f64 / (i + 1) as f64)
    };
    (0..=n)
        .map(|i| (0..=i).map(|j| binom(i, j) / binom(n, j) * m[j]).sum())
        .collect()
}

/// A Bézier of `x = t` and `z` the polynomial with monomial coefficients `m`.
fn bezier_z(m: &[f64]) -> NurbsCurve3<f64> {
    let n = m.len() - 1;
    let z = bernstein(m);
    let mut knots = vec![0.0; n + 1];
    knots.extend(vec![1.0; n + 1]);
    let xz: Vec<(f64, f64)> = (0..=n).map(|i| (i as f64 / n as f64, z[i])).collect();
    spline(n, &knots, &xz)
}

fn poly_from_roots(roots: &[f64], scale: f64) -> Vec<f64> {
    let mut m = vec![scale];
    for &r in roots {
        let mut next = vec![0.0; m.len() + 1];
        for (i, &c) in m.iter().enumerate() {
            next[i] -= r * c;
            next[i + 1] += c;
        }
        m = next;
    }
    m
}

/// A V at a crease whose apex is above the plane by less than the zero
/// band: C5 says an in-band decision escalates and never prunes, so this
/// row expects a refusal.
#[test]
fn r2_in_band_apex_at_a_crease_is_not_read_clear() {
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
    let control = vec![
        Point3::new(0.0, 0.0, 1.0),
        Point3::new(0.5, 0.0, 0.5),
        Point3::new(1.0, 0.0, 1e-12),
        Point3::new(1.5, 0.0, 0.5),
        Point3::new(2.0, 0.0, 1.0),
    ];
    let net = NurbsCurve3::new(kv, control, vec![1.0; 5]).unwrap();
    let got = read(&net, &ground());
    assert!(got.is_err(), "an in-band touch at a crease read {got:?}");
}

/// Same V, apex just below: two crossings each within the band of the
/// crease, so Crease.
#[test]
fn r2_in_band_dip_at_a_crease_refuses_crease() {
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
    let control = vec![
        Point3::new(0.0, 0.0, 1.0),
        Point3::new(0.5, 0.0, 0.5),
        Point3::new(1.0, 0.0, -1e-12),
        Point3::new(1.5, 0.0, 0.5),
        Point3::new(2.0, 0.0, 1.0),
    ];
    let net = NurbsCurve3::new(kv, control, vec![1.0; 5]).unwrap();
    let got = read(&net, &ground());
    assert!(
        matches!(got, Err(CompositeRefusal::Unsettled(Unsettled::Crease))),
        "{got:?}"
    );
}

#[test]
fn r2_double_and_triple_roots_refuse() {
    for roots in [&[0.37, 0.37][..], &[0.37, 0.37, 0.37], &[0.1, 0.37, 0.37, 0.9]] {
        let net = bezier_z(&poly_from_roots(roots, 1.0));
        let got = read(&net, &ground());
        assert!(got.is_err(), "{roots:?}: {got:?}");
    }
}

#[test]
fn r2_close_roots_never_read_one() {
    for gap in [1e-3, 1e-5, 1e-7, 1e-9, 1e-11, 1e-13] {
        let net = bezier_z(&poly_from_roots(&[0.4, 0.4 + gap, 0.9], 1.0));
        let got = read(&net, &ground());
        match &got {
            Ok(r) => assert!(r.len() == 3 || gap < 1e-12, "gap {gap}: {r:?}"),
            Err(_) => {}
        }
        eprintln!("gap {gap}: {got:?}");
    }
}

#[test]
fn r2_high_degree_roots_counted() {
    let roots = [0.05, 0.15, 0.3, 0.45, 0.55, 0.7, 0.8, 0.95];
    let net = bezier_z(&poly_from_roots(&roots, 100.0));
    let got = read(&net, &ground()).unwrap();
    assert_eq!(got.len(), roots.len(), "{got:?}");
    for (g, r) in got.iter().zip(roots) {
        assert!((g - r).abs() < 1e-9, "{got:?}");
    }
}

#[test]
fn r2_near_zero_weights() {
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    for w in [1e-3, 1e-8, 1e-15, 1e-30, 1e3, 1e12] {
        let control = vec![
            Point3::new(0.0, 0.0, -1.0),
            Point3::new(0.5, 0.0, 5.0),
            Point3::new(1.0, 0.0, -1.0),
        ];
        let net = NurbsCurve3::new(kv.clone(), control, vec![1.0, w, 1.0]).unwrap();
        let got = read(&net, &ground());
        // Dense truth.
        let mut changes = 0;
        let mut prev = net.eval(0.0).z;
        for i in 1..=200_000 {
            let z = net.eval(i as f64 / 200_000.0).z;
            if (z > 0.0) != (prev > 0.0) {
                changes += 1;
            }
            prev = z;
        }
        eprintln!("w {w}: {got:?} dense {changes}");
        if let Ok(r) = &got {
            assert_eq!(r.len(), changes, "w {w}");
            for &t in r {
                assert!(net.eval(t).z.abs() <= 1e-9, "w {w} t {t}");
            }
        }
    }
}

/// Randomised: every Ok answer has as many roots as a dense scan has
/// sign changes, and each on the plane.
#[test]
fn r2_random_splines_against_dense_truth() {
    let mut seed: u64 = 0x5eed_1234_abcd;
    let mut rnd = || {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((seed >> 11) as f64) / ((1u64 << 53) as f64)
    };
    let (mut ok, mut refused) = (0, 0);
    for case in 0..400 {
        let degree = 1 + (rnd() * 5.0) as usize;
        let ncp = degree + 1 + (rnd() * 6.0) as usize;
        let interior = ncp - degree - 1;
        let mut inner: Vec<f64> = (0..interior).map(|_| (rnd() * 8.0).floor() / 8.0 + 0.0625).collect();
        // sometimes repeat knots up to degree (creases)
        if interior >= 2 && rnd() < 0.3 {
            inner[1] = inner[0];
        }
        inner.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mut knots = vec![0.0; degree + 1];
        knots.extend(inner);
        knots.extend(vec![1.0; degree + 1]);
        let Ok(kv) = KnotVector::clamped(knots.clone(), degree) else { continue };
        let control: Vec<Point3<f64>> = (0..ncp)
            .map(|_| Point3::new(rnd(), rnd(), rnd() * 2.0 - 1.0))
            .collect();
        let weights: Vec<f64> = (0..ncp).map(|_| if case % 2 == 0 { 1.0 } else { 0.2 + rnd() * 3.0 }).collect();
        let Ok(net) = NurbsCurve3::new(kv, control, weights) else { continue };
        let got = read(&net, &ground());
        let n = 400_000;
        let mut changes = 0;
        let mut prev = net.eval(0.0).z;
        let mut minabs = f64::INFINITY;
        for i in 1..=n {
            let z = net.eval(i as f64 / n as f64).z;
            minabs = minabs.min(z.abs());
            if (z > 0.0) != (prev > 0.0) && z != 0.0 && prev != 0.0 {
                changes += 1;
            }
            prev = z;
        }
        match got {
            Ok(r) => {
                ok += 1;
                for &t in &r {
                    assert!(net.eval(t).z.abs() <= 1e-9, "case {case} t {t}");
                }
                assert_eq!(r.len(), changes, "case {case}: {r:?} vs dense {changes}, knots {knots:?}, min|z| {minabs}");
            }
            Err(_) => refused += 1,
        }
    }
    eprintln!("ok {ok} refused {refused}");
    assert!(ok > 100);
}

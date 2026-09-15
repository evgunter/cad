//! **What `NurbsSurface::reversed_v` / `reversed_u` actually accept.**
//!
//! The door is defined on a knot vector that is its own reflection as a
//! set of REAL numbers, and the doc says so. What that leaves in and
//! out is not obvious from the sentence, because "symmetric" means
//! symmetric AFTER decimal-to-binary rounding — so these rows are the
//! acceptance set written down, in the two forms a caller meets it:
//! the interior pair a user types, and the knot vector the kernel's own
//! loft producer mints for `k` equally spaced sections.
//!
//! The loft row is also the evidence behind
//! `work/blend/interpolate-columns-averaged-knots-could-be-mirror-symmetric.md`: the
//! averaged knots `skin::interpolate_columns` computes could be minted
//! mirror-symmetric by construction when the parameters are, and the
//! `k = 7` and `k = 8` refusals below are what it costs that they are
//! not.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{NurbsSurface, Surface};
use geom_core::spline::KnotVector;
use geom_core::{Affine3, Point3, Tol, Vec3};

/// A `3 × nv` net with no symmetry of its own, on a given `knots_v`.
fn surface_on(knots_v: Vec<f64>, pv: usize) -> NurbsSurface<f64> {
    let kv = KnotVector::clamped(knots_v, pv).unwrap();
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1).unwrap();
    let nv = kv.control_count();
    let mut control = Vec::new();
    let mut weights = Vec::new();
    for iu in 0..3usize {
        for iv in 0..nv {
            control.push(Point3::new(
                iu as f64 + 0.37 * iv as f64,
                (iv * iv) as f64 * 0.11 - iu as f64,
                ((iu * 7 + iv * 3) % 5) as f64 * 0.5,
            ));
            weights.push(1.0 + ((iu * 3 + iv * 5) % 7) as f64 * 0.25);
        }
    }
    NurbsSurface::new(ku, kv, control, weights).unwrap()
}

/// An interior pair a user WRITES as mirrored is usually not mirrored
/// once it is `f64`: its two halves' real sum misses `lo + hi` by one
/// rounding residual. Every pair here passes the rounded test
/// (`a + b == 1.0` exactly), so this row is also what separates the
/// shipped exact test from the rounded one the spec first prescribed.
#[test]
fn decimal_symmetric_interior_pairs_mostly_refuse() {
    // (name, lo knot, hi knot, accepted?)
    let pairs: [(&str, f64, f64, bool); 8] = [
        ("thirds", 1.0 / 3.0, 2.0 / 3.0, false),
        ("0.1/0.9", 0.1, 0.9, false),
        ("0.2/0.8", 0.2, 0.8, false),
        ("0.3/0.7", 0.3, 0.7, false),
        ("0.45/0.55", 0.45, 0.55, false),
        ("0.4/0.6", 0.4, 0.6, true),
        ("0.125/0.875", 0.125, 0.875, true),
        ("0.25/0.75", 0.25, 0.75, true),
    ];
    for (name, a, b, accepted) in pairs {
        assert_eq!(a + b, 1.0, "{name}: the ROUNDED sum IS lo + hi");
        let s = surface_on(vec![0.0, 0.0, 0.0, a, b, 1.0, 1.0, 1.0], 2);
        let out = s.reversed_v();
        assert_eq!(
            out.is_ok(),
            accepted,
            "{name}: ({a:?}, {b:?}) — the door decides the REAL sum, not the rounded one; \
             got {:?}",
            out.err()
        );
        if !accepted {
            // The whole gap is one residual of the rounded sum — below
            // half an ulp of 1.0, which is why the rounded test above
            // cannot see it.
            let residual = geom_core::exact::two_sum(a, b).1;
            assert!(
                residual != 0.0 && residual.abs() <= f64::EPSILON / 2.0,
                "{name}: the pair misses the midline by one 2Sum residual \
                 ({residual:e}), invisible to the rounded sum"
            );
        }
    }
}

/// The unit square, through the crate's own section helper.
fn square() -> sweep::Section {
    crate::common::quad([(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)])
}

/// The `knots_v` a `k`-section loft of equally spaced squares mints,
/// and whether the door reverses the resulting wall.
fn lofted_wall_verdict(k: usize) -> (Vec<f64>, bool) {
    let sections: Vec<_> = (0..k).map(|_| square()).collect();
    let places: Vec<_> = (0..k)
        .map(|j| Affine3::translation(Vec3::new(0.0, 0.0, j as f64)))
        .collect();
    let lofted =
        sweep::loft_body::<f64>(&sections, &places, 2, Tol::witness()).expect("the loft builds");
    let fk = lofted.side_faces[0][0];
    let sk = lofted.body.get_face(fk).unwrap().surface;
    let n = match lofted.body.get_surface(sk) {
        Some(Surface::Nurbs(n)) => (**n).clone(),
        other => panic!("a lofted wall is a NURBS chart: {other:?}"),
    };
    let knots = n.knots_v().knots().to_vec();
    (knots, n.reversed_v().is_ok())
}

/// On the kernel's OWN producer the acceptance set falls out as a
/// section count: equally spaced sections give a mirror-symmetric
/// `knots_v` up to `k = 6` and a refusing one at `k = 7` and `k = 8`,
/// because `skin::interpolate_columns` averages chord parameters that
/// are only symmetric to rounding. A user who lofts seven sections and
/// asks for a reversed wall gets a typed refusal, not a surface.
#[test]
fn a_lofted_walls_v_knots_are_symmetric_up_to_six_sections() {
    for k in 3..=8usize {
        let (knots, reversible) = lofted_wall_verdict(k);
        assert_eq!(
            reversible,
            k <= 6,
            "k = {k}: knots_v = {knots:?} — the loft's averaged knots are \
             mirror-symmetric exactly up to six sections"
        );
    }
}

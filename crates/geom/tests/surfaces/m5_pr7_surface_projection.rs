//! M5 PR 7, substrate row: **surface** point inversion (NURBS Book
//! §6.1's surface half) — the certified foot points C2.1 requires for
//! the limb-1 residual of a rung-3 cache whose operand is a NURBS
//! surface (no implicit form, so "distance to the surface" has to be
//! *exhibited*, with the orthogonality residuals that stop a bad
//! projection laundering a bad cache).
//!
//! Rows: convergence + residual honesty on an interior foot; a planted
//! **wrong-sheet** seed whose orthogonality residuals are tiny and
//! whose distance is large (the laundering attempt that must be
//! visible); a **domain-edge** foot whose distance is small and whose
//! orthogonality residual is honestly large; poison totality.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{NurbsSurface, PROJECT_EPS_POINT};
use geom_core::Point3;
use geom_core::spline::KnotVector;

/// A gently curved bicubic patch over `[0,1]²` in `(x, y)`, bulging in
/// `+z`: one sheet, no self-intersections, interior feet well posed.
fn bump_patch() -> NurbsSurface<f64> {
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
    let mut control = Vec::with_capacity(16);
    for iu in 0..4 {
        for iv in 0..4 {
            let x = iu as f64 / 3.0;
            let y = iv as f64 / 3.0;
            // A single interior bump: zero at the boundary ring.
            let z = 0.9 * (x * (1.0 - x)) * (y * (1.0 - y)) * 4.0;
            control.push(Point3::new(x, y, z));
        }
    }
    NurbsSurface::new(ku, kv, control, vec![1.0; 16]).unwrap()
}

/// A unit-radius **cylinder** patch (the exact §7.3 nine-point
/// rational-quadratic circle × a degree-1 height): every point off the
/// axis has exactly **two** stationary distance sheets — the near wall
/// and the far wall — both with vanishing orthogonality residuals and
/// distances differing by the diameter. The cleanest possible planted
/// wrong-sheet fixture.
fn cylinder_patch() -> NurbsSurface<f64> {
    let sqrt2_2 = core::f64::consts::FRAC_1_SQRT_2;
    let ku = KnotVector::clamped(
        vec![
            0.0, 0.0, 0.0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1.0, 1.0, 1.0,
        ],
        2,
    )
    .unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let ring = [
        (1.0, 0.0),
        (1.0, 1.0),
        (0.0, 1.0),
        (-1.0, 1.0),
        (-1.0, 0.0),
        (-1.0, -1.0),
        (0.0, -1.0),
        (1.0, -1.0),
        (1.0, 0.0),
    ];
    let ws = [1.0, sqrt2_2, 1.0, sqrt2_2, 1.0, sqrt2_2, 1.0, sqrt2_2, 1.0];
    let mut control = Vec::with_capacity(18);
    let mut weights = Vec::with_capacity(18);
    for (i, (cx, cy)) in ring.iter().enumerate() {
        control.push(Point3::new(*cx, *cy, 0.0));
        control.push(Point3::new(*cx, *cy, 1.0));
        weights.push(ws[i]);
        weights.push(ws[i]);
    }
    NurbsSurface::new(ku, kv, control, weights).unwrap()
}

#[test]
fn interior_foot_converges_with_honest_residuals() {
    let s = bump_patch();
    // A point straight above the crown of the bump.
    let p = Point3::new(0.5, 0.5, 3.0);
    let pr = s.project(p).unwrap();
    // The nearest point is the crown itself.
    assert!((pr.u - 0.5).abs() < 1e-6, "u = {}", pr.u);
    assert!((pr.v - 0.5).abs() < 1e-6, "v = {}", pr.v);
    // Both orthogonality residuals converged to (near) zero …
    assert!(pr.orthogonality_u < 1e-9, "{pr:?}");
    assert!(pr.orthogonality_v < 1e-9, "{pr:?}");
    // … and the distance is the honest one, not zero.
    let expect = (p - pr.foot).norm();
    assert!((pr.distance - expect).abs() < 1e-12, "{pr:?}");
    assert!(pr.distance > 2.0, "{pr:?}");
}

#[test]
fn projection_residual_matches_a_recomputation_from_the_reported_foot() {
    // The foot, the distance, and the orthogonality residuals must be
    // mutually consistent — a consumer bands the carried numbers, so a
    // struct whose fields disagreed with its own `(u, v)` would be a
    // laundering channel all by itself.
    let s = bump_patch();
    for &(px, py, pz) in &[(0.2, 0.7, 1.0), (0.9, 0.1, -0.5), (0.45, 0.55, 0.2)] {
        let p = Point3::new(px, py, pz);
        let pr = s.project(p).unwrap();
        let j = s.ders(pr.u, pr.v);
        let r = j.point - p;
        assert!((pr.foot - j.point).norm() < 1e-15, "{pr:?}");
        assert!((pr.distance - r.norm()).abs() < 1e-15, "{pr:?}");
        assert!(
            (pr.orthogonality_u - j.du.dot(r).abs()).abs() < 1e-15,
            "{pr:?}"
        );
        assert!(
            (pr.orthogonality_v - j.dv.dot(r).abs()).abs() < 1e-15,
            "{pr:?}"
        );
    }
}

#[test]
fn a_wrong_sheet_seed_is_visible_in_the_distance_not_the_orthogonality() {
    // THE C2.1 row: "a bad projection cannot launder a bad cache".
    let s = cylinder_patch();
    // Off-axis inside the cylinder: near wall at 0.8, far wall at 1.2.
    let p = Point3::new(0.2, 0.0, 0.5);
    let good = s.project(p).unwrap();
    assert!((good.distance - 0.8).abs() < 1e-9, "near wall: {good:?}");
    // Seed on the FAR wall (u = 0.5 is the θ = π quarter-point).
    let bad = s.project_from_seed(p, 0.5, 0.5).unwrap();
    assert!(
        (bad.distance - 1.2).abs() < 1e-9,
        "the planted seed did not stay on the far wall: {bad:?}"
    );
    // The far foot is a genuine stationary point: both orthogonality
    // residuals vanish, so orthogonality ALONE would accept it as a
    // certified foot — and a cache lying 0.4 m off the surface would
    // then certify. Only the distance exposes it.
    assert!(
        bad.orthogonality_u < 1e-9 && bad.orthogonality_v < 1e-9,
        "expected a converged far stationary point, got {bad:?}"
    );
    assert!(bad.distance > good.distance + 0.39, "{bad:?} vs {good:?}");
}

#[test]
fn a_domain_edge_foot_reports_an_honestly_large_orthogonality_residual() {
    let s = bump_patch();
    // Far off the u = 0 edge: the true nearest point is on the
    // boundary, where the gradient does not vanish.
    let p = Point3::new(-4.0, 0.5, 0.0);
    let pr = s.project(p).unwrap();
    assert!(pr.u <= PROJECT_EPS_POINT, "clamped to u = 0: {pr:?}");
    // Distance is the honest boundary distance …
    assert!(pr.distance > 3.9, "{pr:?}");
    // … and the u-orthogonality residual is large, which is exactly
    // what tells a consumer "this is a clamped edge foot, not an
    // interior stationary point".
    assert!(
        pr.orthogonality_u > 1.0,
        "an edge foot must report its residual honestly, got {pr:?}"
    );
}

#[test]
fn poison_input_refuses_typed_and_never_panics() {
    let s = bump_patch();
    let bad = s.project(Point3::new(f64::NAN, 0.0, 0.0));
    let e = bad.expect_err("NaN converges nowhere");
    assert!(e.last_distance.is_nan() || e.iterations > 0);
    // The Display carries the diagnosis without pretending to a foot.
    let msg = format!("{e}");
    assert!(msg.contains("surface projection inconclusive"), "{msg}");
}

#[test]
fn seeding_is_bit_deterministic() {
    let s = bump_patch();
    let p = Point3::new(0.31, 0.72, 1.4);
    let a = s.project_seed(p);
    let b = s.project_seed(p);
    assert_eq!(a.0.to_bits(), b.0.to_bits());
    assert_eq!(a.1.to_bits(), b.1.to_bits());
    let pa = s.project(p).unwrap();
    let pb = s.project(p).unwrap();
    assert_eq!(pa.u.to_bits(), pb.u.to_bits());
    assert_eq!(pa.distance.to_bits(), pb.distance.to_bits());
}

/// An **overflowing residual refuses**, and the inputs that produce
/// one are all finite — the surface half of
/// `tests/curves/projection.rs`'s overflow row.
///
/// The two rows are not two facts. They pin **one** property of a
/// shared helper: `crate::projection_policy::mid` is `x + ½(x − x)`, the
/// identity on every finite `x` and **NaN at ±∞**, and that
/// non-totality is *load-bearing at every caller* — it is what turns an
/// overflowed residual into the typed refusal instead of a converged
/// foot at infinite distance. `mid` has exactly two callers, this half
/// and the curve half, and each now has a row. Making `mid` total at
/// ±∞ turns **both** red; before this row it turned one red and changed
/// the other's certified path in silence.
///
/// `NurbsSurface::new` validates counts and weight positivity, never
/// coordinate magnitude, so a control net at 1e200 is a legal surface
/// reachable through the public door, and its squared distance to a
/// point at the origin overflows.
#[test]
fn an_overflowing_residual_refuses_rather_than_reporting_an_infinite_foot() {
    let huge = 1.0e200;
    let ku = KnotVector::unit_segment(1);
    let kv = KnotVector::unit_segment(1);
    // A bilinear patch, every corner at ~1e200.
    let control = vec![
        Point3::new(huge, huge, huge),
        Point3::new(huge, 2.0 * huge, huge),
        Point3::new(2.0 * huge, huge, huge),
        Point3::new(2.0 * huge, 2.0 * huge, huge),
    ];
    let s = NurbsSurface::new(ku, kv, control, vec![1.0; 4])
        .expect("a bilinear patch with unit weights is a valid surface");

    // Finite inputs throughout: the net's own coordinates and the query
    // point are all representable.
    assert!(
        s.control()
            .iter()
            .all(|p| p.x.is_finite() && p.z.is_finite())
    );
    let p = Point3::new(0.0, 0.0, 0.0);
    assert!(p.x.is_finite());

    // The residual itself is what overflows. Asserted BEFORE the
    // refusal: a fixture that stopped reaching its own precondition
    // would otherwise pass for the wrong reason.
    let d = s.eval(0.0, 0.0) - p;
    assert!(d.dot(d).is_infinite(), "the fixture must overflow");

    match s.project(p) {
        Err(e) => {
            assert!(
                e.last_distance.is_nan()
                    && e.last_orthogonality_u.is_nan()
                    && e.last_orthogonality_v.is_nan(),
                "the refusal carries the poisoned state honestly: {e:?}"
            );
        }
        Ok(pr) => panic!("an overflowed residual was reported as a foot point: {pr:?}"),
    }
}

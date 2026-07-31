//! M5 PR 7, substrate row: the **third-order** surface jet
//! ([`NurbsSurface::ders3`]) that the ℝ⁴ parametric×parametric SSI
//! trace needs for its cubic Taylor term (Hoffmann §6.3.2 via §6.2's
//! third-order approximant).
//!
//! Three obligations:
//!
//! 1. **No D9 fork.** `ders3`'s `k + l ≤ 2` block must be **bit
//!    identical** to `ders`'s — the third-order routine is an
//!    extension, not a second implementation of the same numbers. This
//!    is the row that would catch a well-meaning "tidy-up" of either
//!    expression.
//! 2. **The rational corrections are right.** Checked against central
//!    finite differences of the (independently exercised) second-order
//!    jet, on a genuinely **rational** patch — non-unit weights, so the
//!    Eq. 4.20 correction terms are all live. FD is the reference here
//!    precisely because it shares no code with the formula under test.
//! 3. **Poison totality.** An invalid span pair yields an all-poison
//!    jet, never a panic.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::spline::KnotVector;
use geom_core::{Point3, Vec3};
use geom_surfaces::NurbsSurface;

/// A bicubic × biquadratic **rational** patch with deliberately
/// irregular weights and a non-uniform interior knot in u: every term
/// of the rational third-derivative recursion is exercised.
fn rational_patch() -> NurbsSurface<f64> {
    // u: degree 3, one interior knot ⇒ 5 control columns.
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.4, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
    // v: degree 2, one interior knot ⇒ 4 control rows.
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
    let (nu, nv) = (5usize, 4usize);
    let mut control = Vec::with_capacity(nu * nv);
    let mut weights = Vec::with_capacity(nu * nv);
    for iu in 0..nu {
        for iv in 0..nv {
            let x = iu as f64 * 0.75;
            let y = iv as f64 * 0.6;
            // A saddle-ish height with cross terms, so no partial is
            // accidentally zero.
            let z = 0.3 * x * x - 0.2 * y * y + 0.45 * x * y + 0.1 * x * x * y;
            control.push(Point3::new(x, y, z));
            weights.push(0.6 + 0.35 * ((iu * 3 + iv * 5) % 7) as f64);
        }
    }
    NurbsSurface::new(ku, kv, control, weights).unwrap()
}

fn close(a: Vec3<f64>, b: Vec3<f64>, tol: f64, what: &str) {
    let d = (a - b).norm();
    assert!(d <= tol, "{what}: |{a:?} − {b:?}| = {d} > {tol}");
}

#[test]
fn ders3_agrees_with_ders_bit_for_bit_on_the_shared_block() {
    let s = rational_patch();
    // Interior of several distinct span cells, plus the clamped ends.
    for &(u, v) in &[
        (0.0, 0.0),
        (0.2, 0.3),
        (0.4, 0.5),
        (0.65, 0.25),
        (0.9, 0.8),
        (1.0, 1.0),
    ] {
        let a = s.ders(u, v);
        let b = s.ders3(u, v).jet;
        for (name, x, y) in [
            ("point", Vec3::new(a.point.x, a.point.y, a.point.z), {
                Vec3::new(b.point.x, b.point.y, b.point.z)
            }),
            ("du", a.du, b.du),
            ("dv", a.dv, b.dv),
            ("duu", a.duu, b.duu),
            ("duv", a.duv, b.duv),
            ("dvv", a.dvv, b.dvv),
        ] {
            assert_eq!(
                (x.x.to_bits(), x.y.to_bits(), x.z.to_bits()),
                (y.x.to_bits(), y.y.to_bits(), y.z.to_bits()),
                "ders/ders3 forked on {name} at ({u}, {v}) — D9 violation"
            );
        }
    }
}

#[test]
fn third_partials_match_central_differences_of_the_second_order_jet() {
    let s = rational_patch();
    // Richardson-extrapolated central differences: `(4·D(h) − D(2h))/3`
    // is O(h⁴) accurate, which drops the reference's truncation error
    // below 1e-8 on this patch. A plain O(h²) difference agrees only to
    // ~1e-5 relative here — enough to catch a dropped term, but the
    // extrapolated form makes the row sharp enough to catch a wrong
    // binomial coefficient in the *smallest* correction as well.
    let h = 1.0e-3;
    let rich = |f: &dyn Fn(f64) -> Vec3<f64>, x: f64| -> Vec3<f64> {
        let d1 = (f(x + h) - f(x - h)) / (2.0 * h);
        let d2 = (f(x + 2.0 * h) - f(x - 2.0 * h)) / (4.0 * h);
        (d1 * 4.0 - d2) / 3.0
    };
    // All four span cells, none of them ON a knot line: the patch is
    // only C¹ in v across the interior knot at v = 0.5, so a
    // difference stencil straddling it is comparing across a genuine
    // derivative jump, not measuring an error.
    for &(u, v) in &[(0.2, 0.3), (0.55, 0.25), (0.8, 0.7), (0.25, 0.75)] {
        let j = s.ders3(u, v);
        // ∂³/∂u³ = ∂/∂u of ∂²/∂u²; ∂³/∂u²∂v = ∂/∂v of ∂²/∂u²; etc.
        let duuu = rich(&|x| s.ders(x, v).duu, u);
        let duuv = rich(&|y| s.ders(u, y).duu, v);
        let duvv = rich(&|x| s.ders(x, v).dvv, u);
        let dvvv = rich(&|y| s.ders(u, y).dvv, v);
        close(j.duuu, duuu, 1.0e-6, "duuu");
        close(j.duuv, duuv, 1.0e-6, "duuv");
        close(j.duvv, duvv, 1.0e-6, "duvv");
        close(j.dvvv, dvvv, 1.0e-6, "dvvv");
    }
}

#[test]
fn third_partials_of_a_bilinear_patch_vanish_exactly() {
    // Degree 1 × degree 1, unit weights: every third partial is
    // identically zero (and so is every second partial except the
    // mixed one, which is also zero for a planar bilinear patch).
    let ku = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let control = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(0.0, 1.0, 0.3),
        Point3::new(1.0, 0.0, -0.2),
        Point3::new(1.0, 1.0, 0.9),
    ];
    let s = NurbsSurface::new(ku, kv, control, vec![1.0; 4]).unwrap();
    let j = s.ders3(0.37, 0.61);
    for (name, d) in [
        ("duuu", j.duuu),
        ("duuv", j.duuv),
        ("duvv", j.duvv),
        ("dvvv", j.dvvv),
    ] {
        assert!(d.norm() < 1.0e-14, "{name} = {d:?} on a bilinear patch");
    }
}

#[test]
fn invalid_span_pair_poisons_rather_than_panicking() {
    let s = rational_patch();
    let j = s.ders3_in_span(999, 999, 0.5, 0.5);
    assert!(j.jet.point.x.is_nan());
    assert!(j.duuu.x.is_nan());
    assert!(j.dvvv.z.is_nan());
}

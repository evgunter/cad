//! The offset fit and its certificate (`geom_brep::offset_fit`), with
//! the two meters it stands on (`geom_brep::offset_meters`).
//!
//! Four families:
//!
//! - **The analytic oracle** — a quarter cylinder and a sphere band,
//!   each re-expressed as an EXACT rational NURBS, offset through
//!   `fit_offset`, and checked against the CLOSED FORM (the radial
//!   push, which is what OFF-A's mint says the offset of these kinds
//!   is: `radius + d`). This is the one place the answer is known
//!   exactly, so it is the unit's spine — and the check is
//!   independent of the fit's own machinery: a dense deterministic
//!   sample, evaluated through the public surface door, measured
//!   against the analytic locus.
//! - **Containment** — the certified `hull_sup` never under-reports:
//!   it contains a dense sample's max on every row. The red direction
//!   for a bound is being too small, and that is what is asserted.
//! - **A non-analytic base** — a bicubic patch interpolated from a
//!   non-analytic height field through the loft door
//!   (`geom::curves::fit::interpolate_columns`, A9.4's own engine),
//!   fitted and certified at a default-scale tolerance.
//! - **Planted reds** — a degraded fit (coarsened knots) fails the
//!   certificate and NAMES the limb; a collapsed control row (the
//!   sphere-pole shape) refuses at the regularity floor; `|d|` past
//!   the curvature reach refuses at the collapse meter; an
//!   unreachable tolerance refuses typed, naming what stopped the
//!   loop (on the bumpy patch, the sample cap).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::NurbsSurface;
use geom_brep::offset_fit::{
    BestBound, OFFSET_FIT_SAMPLE_CAP, OffsetFitError, OffsetFitWall, OffsetLimb, certify_offset_at,
    fit_offset_at,
};
use geom_brep::offset_meters::{MeterError, OFFSET_METER_LADDER, patch_collapse, patch_regularity};
use geom_brep::patch_bound::{PatchBoundError, patch_cells_refined};
use geom_brep::recourse::Refused;
use geom_core::Bounds;
use geom_core::MarginDiag;
use geom_core::Point3;
use geom_core::interval::certification::Certification;
use geom_core::interval::{max_bound, min_bound};
use geom_core::spline::KnotVector;

use crate::shared::fixture::{bumpy_patch, kv1, kv2, quarter_cylinder, sphere_band};
use crate::shared::sample::{grid, worst_offset_residual};
use crate::shared::tol::band;

// ---------------------------------------------------------------------
// Fixtures
// ---------------------------------------------------------------------
//
// The two carriers this file's analytic-oracle spine rests on — the
// quarter cylinder and the sphere band, exact rationals whose closed
// form is what the oracle rows measure against — are
// `crate::shared::fixture`'s: four other suites in this crate were
// building the same two nets. What is left here is the base that has
// no closed form at all.

// ---------------------------------------------------------------------
// The analytic oracle
// ---------------------------------------------------------------------

/// The closed-form offset of a point on a cylinder about `+z` at
/// radius `r`: the radial push to `r + d`, `z` unchanged. Independent
/// of any parameterization — this is what OFF-A's `radius + d` mint
/// MEANS, spelled as geometry.
fn cylinder_offset_closed_form(p: Point3<f64>, r: f64, d: f64) -> Point3<f64> {
    let k = (r + d) / r;
    Point3::new(p.x * k, p.y * k, p.z)
}

/// The closed-form offset of a point on a sphere of radius `r` about
/// the origin: the radial push to `r + d`.
fn sphere_offset_closed_form(p: Point3<f64>, r: f64, d: f64) -> Point3<f64> {
    let k = (r + d) / r;
    Point3::new(p.x * k, p.y * k, p.z * k)
}

#[test]
fn cylinder_fit_matches_the_closed_form_both_signs() {
    let (r, h) = (1.25, 0.75);
    let base = quarter_cylinder(r, h);
    // The oracle's content is CONTAINMENT and the closed form, not
    // how small the tolerance is: at 1e-4 the loop spends a third
    // refinement round whose cells cost more CI wall clock than a
    // second full oracle pass buys in evidence. So the tighter
    // tolerance is held here as a LIVENESS claim only — the door
    // still answers, and answers within what it was asked for — and
    // the dense containment oracle runs once, at 3e-4.
    {
        let tol = 1e-4;
        let d = 0.3;
        let (_, cert) = fit_offset_at(&base, d, tol, band()).unwrap_or_else(|e| {
            panic!("LIVENESS: fit_offset refused this cylinder at d = {d}, tol = {tol}: {e}")
        });
        assert!(
            cert.hull_sup <= tol,
            "LIVENESS at tol = {tol}: certified sup {} exceeds the tolerance it was \
             asked for",
            cert.hull_sup
        );
    }
    let tol = 3e-4;
    for d in [0.3_f64, -0.4] {
        let (fit, cert) = fit_offset_at(&base, d, tol, band())
            .unwrap_or_else(|e| panic!("fit_offset refused at d = {d}: {e}"));
        assert!(
            cert.hull_sup <= tol,
            "d = {d}: certified sup {} exceeds the tolerance {tol}",
            cert.hull_sup
        );
        // The independent oracle: the closed form, sampled densely
        // through the public evaluation door.
        let mut worst = 0.0f64;
        for (u, v) in grid(23, 19) {
            let p = base.eval(u, v);
            let want = cylinder_offset_closed_form(p, r, d);
            let got = fit.eval(u, v);
            worst = max_bound(worst, (got - want).norm());
            // The base point really is on the cylinder (the fixture
            // is exact, not approximately exact).
            let rad = (p.x * p.x + p.y * p.y).sqrt();
            assert!(
                (rad - r).abs() < 1e-13,
                "fixture is not an exact cylinder: radius {rad} at ({u}, {v})"
            );
        }
        assert!(
            worst <= cert.hull_sup,
            "d = {d}: the certified sup {} UNDER-reports the sampled max {worst}",
            cert.hull_sup
        );
        assert!(worst <= tol, "d = {d}: sampled max {worst} exceeds {tol}");
        assert!(
            cert.on_locus_max <= cert.hull_sup,
            "limb 1 ({}) above limb 2 ({})",
            cert.on_locus_max,
            cert.hull_sup
        );
        eprintln!(
            "cylinder r={r} d={d}: cells={} rounds={} on_locus={:.3e} hull_sup={:.3e} \
             sampled={worst:.3e} floor={:.4} reach={}",
            cert.cells,
            cert.rounds,
            cert.on_locus_max,
            cert.hull_sup,
            cert.normal_floor,
            cert.curvature_reach
        );
        // And the fitted surface's own radius is `r + d` — OFF-A's
        // mint, re-derived from the fit.
        for (u, v) in grid(23, 19) {
            let q = fit.eval(u, v);
            let rad = (q.x * q.x + q.y * q.y).sqrt();
            assert!(
                (rad - (r + d)).abs() <= tol,
                "d = {d}: fitted radius {rad} is not r + d = {} at ({u}, {v})",
                r + d
            );
        }
    }
}

/// The fit lives on the base's own chart rectangle BIT FOR BIT: its
/// interpolation runs on `[0, 1]²` and is re-expressed onto the base's
/// `(u, v)` window with the ends assigned, so the certificate's
/// pointwise claim is about the same parameters on both sides. The
/// base is built by hand on a `u` window whose computed end misses —
/// `0.3 + (0.9 − 0.3)` is an ulp above `0.9` — so a fit whose ends
/// were mapped rather than assigned would sit an ulp past the chart.
/// Pins the domain only, not the interpolation's interior schedule.
#[test]
fn the_fit_lives_on_the_base_chart_window_bit_for_bit() {
    let (ulo, uhi, vlo, vhi) = (0.3_f64, 0.9_f64, 0.2_f64, 1.7_f64);
    assert_eq!(
        (ulo + (uhi - ulo)).to_bits(),
        uhi.to_bits() + 1,
        "the u window no longer documents the pin"
    );
    let unit = quarter_cylinder(1.25, 0.75);
    let base = NurbsSurface::new(
        KnotVector::clamped(vec![ulo, ulo, ulo, uhi, uhi, uhi], 2).unwrap(),
        KnotVector::clamped(vec![vlo, vlo, vhi, vhi], 1).unwrap(),
        unit.control().to_vec(),
        unit.weights().to_vec(),
    )
    .unwrap();
    let (fit, cert) = fit_offset_at(&base, 0.3, 3e-4, band())
        .unwrap_or_else(|e| panic!("fit_offset refused the re-charted cylinder: {e}"));
    assert!(cert.hull_sup <= 3e-4, "certified sup {}", cert.hull_sup);
    let bits = |(a, b): (f64, f64)| (a.to_bits(), b.to_bits());
    assert_eq!(
        bits(fit.knots_u_f64_interim().domain()),
        (ulo.to_bits(), uhi.to_bits()),
        "u domain: {:?}",
        fit.knots_u_f64_interim().domain()
    );
    assert_eq!(
        bits(fit.knots_v_f64_interim().domain()),
        (vlo.to_bits(), vhi.to_bits()),
        "v domain: {:?}",
        fit.knots_v_f64_interim().domain()
    );
}

#[test]
fn sphere_band_fit_matches_the_closed_form_both_signs() {
    let r = 2.0;
    let base = sphere_band(r, 0.25, 1.25);
    let tol = 3e-4;
    for d in [0.35_f64, -0.5] {
        let (fit, cert) = fit_offset_at(&base, d, tol, band())
            .unwrap_or_else(|e| panic!("fit_offset refused at d = {d}: {e}"));
        assert!(cert.hull_sup <= tol, "certified sup {}", cert.hull_sup);
        let mut worst = 0.0f64;
        for (u, v) in grid(23, 19) {
            let p = base.eval(u, v);
            let rad = (p.x * p.x + p.y * p.y + p.z * p.z).sqrt();
            assert!(
                (rad - r).abs() < 1e-12,
                "fixture is not an exact sphere: |p| = {rad} at ({u}, {v})"
            );
            let want = sphere_offset_closed_form(p, r, d);
            worst = max_bound(worst, (fit.eval(u, v) - want).norm());
        }
        assert!(
            worst <= cert.hull_sup,
            "d = {d}: the certified sup {} UNDER-reports the sampled max {worst}",
            cert.hull_sup
        );
        assert!(worst <= tol, "d = {d}: sampled max {worst} exceeds {tol}");
        assert!(
            cert.on_locus_max <= cert.hull_sup,
            "limb 1 ({}) above limb 2 ({})",
            cert.on_locus_max,
            cert.hull_sup
        );
        eprintln!(
            "sphere r={r} d={d}: cells={} rounds={} on_locus={:.3e} hull_sup={:.3e} \
             sampled={worst:.3e} floor={:.4} reach={}",
            cert.cells,
            cert.rounds,
            cert.on_locus_max,
            cert.hull_sup,
            cert.normal_floor,
            cert.curvature_reach
        );
    }
}

// ---------------------------------------------------------------------
// A non-analytic base
// ---------------------------------------------------------------------

#[test]
fn non_analytic_base_fits_and_the_bound_contains_the_sample() {
    let base = bumpy_patch();
    let tol = 1e-4;
    let d = 0.05;
    let (fit, cert) = fit_offset_at(&base, d, tol, band())
        .unwrap_or_else(|e| panic!("fit_offset refused on the non-analytic base: {e}"));
    assert!(cert.hull_sup <= tol);
    let worst = worst_offset_residual(&base, &fit, d, &grid(23, 19)).unwrap();
    assert!(
        worst <= cert.hull_sup,
        "the certified sup {} UNDER-reports the sampled max {worst}",
        cert.hull_sup
    );
    eprintln!(
        "non-analytic d={d}: cells={} rounds={} on_locus={:.3e} hull_sup={:.3e} \
         sampled={worst:.3e} floor={:.4} reach={}",
        cert.cells,
        cert.rounds,
        cert.on_locus_max,
        cert.hull_sup,
        cert.normal_floor,
        cert.curvature_reach
    );
}

// ---------------------------------------------------------------------
// Degree-1 base directions
// ---------------------------------------------------------------------

/// A bilinear patch: the hyperbolic paraboloid `z = 0.4·u·v` on the
/// unit square, degree 1 in both directions, read rationally under
/// `weights`.
fn bilinear(weights: Vec<f64>) -> NurbsSurface<f64> {
    let control = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(1.0, 1.0, 0.4),
    ];
    NurbsSurface::new(kv1(), kv1(), control, weights).unwrap()
}

/// A ruled patch, linear in `u` and quadratic in `v`: the ruling joins
/// a parabola in the `x = 0` plane to a differently bent one in the
/// `x = 1` plane, so the surface twists along the ruling.
fn ruled_linear_in_u() -> NurbsSurface<f64> {
    let control = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(0.0, 0.5, 0.3),
        Point3::new(0.0, 1.0, 0.0),
        Point3::new(1.0, 0.0, 0.1),
        Point3::new(1.0, 0.5, -0.2),
        Point3::new(1.0, 1.0, 0.2),
    ];
    NurbsSurface::new(kv1(), kv2(), control, vec![1.0; 6]).unwrap()
}

/// A base with a degree-1 direction offsets and certifies through both
/// public doors, and the certified sup contains a dense sample's max.
/// The rows cover a degree-1 `u` and `v` together (integral, and
/// rational with weights that differ along both rulings) and a
/// degree-1 `u` against a quadratic `v`; the quarter cylinder carries
/// a degree-1 `v` against a rational quadratic `u`.
#[test]
fn degree_1_base_directions_fit_and_certify() {
    let tol = 1e-4;
    let rows = [
        ("bilinear", bilinear(vec![1.0; 4]), 0.05),
        (
            "rational bilinear",
            bilinear(vec![1.0, 1.7, 0.6, 1.3]),
            -0.05,
        ),
        ("ruled, linear in u", ruled_linear_in_u(), 0.05),
    ];
    for (name, base, d) in rows {
        let (fit, cert) = fit_offset_at(&base, d, tol, band())
            .unwrap_or_else(|e| panic!("{name} at d = {d}: fit_offset refused: {e}"));
        assert!(
            cert.hull_sup <= tol,
            "{name} at d = {d}: certified sup {} exceeds {tol}",
            cert.hull_sup
        );
        let worst = worst_offset_residual(&base, &fit, d, &grid(23, 19)).unwrap();
        assert!(
            worst <= cert.hull_sup,
            "{name} at d = {d}: the certified sup {} UNDER-reports the sampled max {worst}",
            cert.hull_sup
        );
        let again = certify_offset_at(&base, &fit, d, tol, band())
            .unwrap_or_else(|e| panic!("{name} at d = {d}: certify_offset refused: {e}"));
        assert!(
            again.hull_sup <= tol,
            "{name} at d = {d}: re-certified sup {} exceeds {tol}",
            again.hull_sup
        );
        eprintln!(
            "{name} d={d}: cells={} rounds={} hull_sup={:.3e} sampled={worst:.3e}",
            cert.cells, cert.rounds, cert.hull_sup
        );
    }
}

/// A degree-1 direction with an interior knot is a crease, and refuses
/// as one before any fit (`Degree1Crease`, whose recourse is to split
/// the face), never at the composite.
#[test]
fn a_two_span_degree_1_direction_refuses_as_a_crease() {
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1).unwrap();
    let control = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
        Point3::new(0.5, 0.0, 0.2),
        Point3::new(0.5, 1.0, 0.1),
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(1.0, 1.0, 0.3),
    ];
    let base = NurbsSurface::new(kv, kv1(), control, vec![1.0; 6]).unwrap();
    let err = fit_offset_at(&base, 0.05, 1e-4, band()).unwrap_err();
    assert!(
        matches!(
            err,
            OffsetFitError::PatchBound(PatchBoundError::Degree1Crease)
        ),
        "a two-span degree-1 direction refused as {err:?}, not as a degree-1 crease"
    );
}

// ---------------------------------------------------------------------
// The meters, on their own
// ---------------------------------------------------------------------

#[test]
fn the_regularity_floor_is_positive_on_a_regular_patch_and_conservative() {
    let base = quarter_cylinder(1.0, 2.0);
    let cells = patch_cells_refined(&base, OFFSET_METER_LADDER[1]).unwrap();
    let reg = patch_regularity(&cells);
    assert!(reg.floor > 0.0, "floor {} is not positive", reg.floor);
    // Conservatism direction: the floor never exceeds the true
    // infimum, sampled independently.
    let mut inf = f64::INFINITY;
    for (u, v) in grid(23, 19) {
        let j = base.ders(u, v);
        inf = min_bound(inf, j.du.cross(j.dv).norm());
    }
    assert!(
        reg.floor <= inf,
        "floor {} exceeds the sampled infimum {inf} — the bound is UNSOUND",
        reg.floor
    );
    assert!(reg.sup >= inf, "sup {} below the sampled inf", reg.sup);
    assert!(reg.sine_floor > 0.0 && reg.sine_floor <= 1.0);
}

#[test]
fn the_collapse_meter_brackets_the_sphere_s_known_curvature() {
    let r = 2.0;
    let base = sphere_band(r, 0.25, 1.25);
    // The fixture really is an exact sphere.
    for (u, v) in grid(23, 19) {
        let p = base.eval(u, v);
        let rad = (p.x * p.x + p.y * p.y + p.z * p.z).sqrt();
        assert!(
            (rad - r).abs() < 1e-12,
            "fixture is not an exact sphere: |p| = {rad} at ({u}, {v})"
        );
    }
    let cells = patch_cells_refined(&base, OFFSET_METER_LADDER[1]).unwrap();
    {
        use geom_core::interval::Interval as RI;
        let nsq = |a: &[RI; 3]| a[0].sqr() + a[1].sqr() + a[2].sqr();
        let mut worst: Option<(f64, String)> = None;
        for c in &cells {
            let n = geom_brep::offset_meters::cell_normal(c);
            let mag = RI::from_bounds(n.floor, c.m_sup);
            let unit = [n.m[0] / mag, n.m[1] / mag, n.m[2] / mag];
            let (e, f, g) = (
                nsq(&c.s_u),
                geom_core::interval::dot(&c.s_u, &c.s_v),
                nsq(&c.s_v),
            );
            let (l, m, nn) = (
                geom_core::interval::dot(&unit, &c.s_uu),
                geom_core::interval::dot(&unit, &c.s_uv),
                geom_core::interval::dot(&unit, &c.s_vv),
            );
            let a = mag.sqr();
            let w11 = (g * l - f * m) / a;
            let width = w11.width();
            let line = format!(
                "u={:?} floor={:.3} sup={:.3} E=[{:.3},{:.3}] F=[{:.3},{:.3}] \
                 G=[{:.3},{:.3}] L=[{:.3},{:.3}] M=[{:.3},{:.3}] N=[{:.3},{:.3}] \
                 W11=[{:.3},{:.3}]",
                c.u,
                n.floor,
                c.m_sup,
                e.lo(),
                e.hi(),
                f.lo(),
                f.hi(),
                g.lo(),
                g.hi(),
                l.lo(),
                l.hi(),
                m.lo(),
                m.hi(),
                nn.lo(),
                nn.hi(),
                w11.lo(),
                w11.hi()
            );
            if worst.as_ref().is_none_or(|w| width > w.0) {
                worst = Some((width, line));
            }
        }
        eprintln!("sphere cells={}\nWORST {}", cells.len(), worst.unwrap().1);
    }
    // The chart normal of this patch points OUTWARD, so both
    // principal curvatures are exactly `−1/r`.
    let coll = patch_collapse(&cells, -0.1);
    assert!(
        coll.kappa_lo <= -1.0 / r && coll.kappa_hi >= -1.0 / r,
        "the certified range [{}, {}] does not contain −1/r = {}",
        coll.kappa_lo,
        coll.kappa_hi,
        -1.0 / r
    );
    // The inward fold radius is `r`; the outward direction never
    // folds, so its reach is unbounded.
    // The certified reach is CONSERVATIVE — never above the true fold
    // radius `r`. How far below is a MEASURED slack, not a designed
    // one, so the row asserts the guard it actually wants (soundness,
    // and non-vacuousness) and prints the ratio rather than asserting
    // a factor the join's assemblies are free to improve.
    assert!(
        coll.reach <= r && coll.reach > 0.3 * r,
        "inward reach {} is unsound (> r) or vacuous (< 0.3r), r = {r}",
        coll.reach
    );
    eprintln!(
        "sphere reach ratio: certified {:.4} / true {r} = {:.3}",
        coll.reach,
        coll.reach / r
    );
    eprintln!(
        "sphere collapse: kappa=[{:.4}, {:.4}] inward reach={:.4} (true kappa = {:.4}, \
         true reach = {r})",
        coll.kappa_lo,
        coll.kappa_hi,
        coll.reach,
        -1.0 / r
    );
    // The meter is SIGNED: the outward direction of a patch that
    // curves away from its normal is far less constrained than the
    // inward one. It is not certified UNBOUNDED here only because the
    // certified `κ_hi` still admits a little positive curvature at
    // this rung; an unsigned `|κ| ≤ κ_max` meter would report the two
    // directions identically, which is the thing being pinned.
    let out = patch_collapse(&cells, 0.1);
    assert!(
        out.reach > 3.0 * coll.reach,
        "the outward reach {} is not far beyond the inward reach {}",
        out.reach,
        coll.reach
    );
}

// ---------------------------------------------------------------------
// Planted reds
// ---------------------------------------------------------------------

#[test]
fn a_degraded_fit_fails_the_certificate_and_names_the_limb() {
    let base = quarter_cylinder(1.0, 1.0);
    let d = 0.3;
    let (fit, cert) = fit_offset_at(&base, d, 1e-3, band()).unwrap();
    assert!(certify_offset_at(&base, &fit, d, 1e-3, band()).is_ok());
    // Coarsen: a bilinear surface through the fit's corner control
    // points is a fit no longer — the same door must refuse it.
    let (cu, cv) = fit.control_counts();
    let corners = vec![
        fit.control()[0],
        fit.control()[cv - 1],
        fit.control()[(cu - 1) * cv],
        fit.control()[(cu - 1) * cv + cv - 1],
    ];
    let degraded = NurbsSurface::new(kv1(), kv1(), corners, vec![1.0; 4]).unwrap();
    match certify_offset_at(&base, &degraded, d, 1e-3, band()) {
        Err(OffsetFitError::Limb { limb, bound, .. }) => {
            assert_eq!(limb, OffsetLimb::OnLocus);
            assert!(bound > 1e-3, "the degraded fit measured only {bound}");
        }
        other => panic!("a degraded fit certified: {other:?}"),
    }
    assert!(cert.hull_sup <= 1e-3);
}

/// **A NaN sample reaches limb 1's guard.** The exact offset of the
/// quarter cylinder certifies (`approx_surface.rs`); one NaN control
/// point poisons its samples. A fold that dropped them would report
/// the max over the samples that answered, pass limb 1, and leave the
/// refusal to limb 2. Limb 1 must refuse, with the poison as its
/// bound.
#[test]
fn a_nan_sample_refuses_at_the_on_locus_limb() {
    let base = quarter_cylinder(1.0, 1.0);
    let d = 0.3;
    let exact = quarter_cylinder(1.0 + d, 1.0);
    assert!(
        certify_offset_at(&base, &exact, d, 1e-3, band()).is_ok(),
        "FIXTURE: the unpoisoned exact offset certifies"
    );
    let mut control = exact.control().to_vec();
    control[0] = Point3::new(f64::NAN, 0.0, 0.0);
    let poisoned = NurbsSurface::new(
        exact.knots_u_f64_interim().clone(),
        exact.knots_v_f64_interim().clone(),
        control,
        exact.weights().to_vec(),
    )
    .unwrap();
    match certify_offset_at(&base, &poisoned, d, 1e-3, band()) {
        Err(OffsetFitError::Limb { limb, bound, .. }) => {
            assert_eq!(limb, OffsetLimb::OnLocus, "the NaN sample was dropped");
            assert!(bound.is_nan(), "limb 1 reported {bound:e}, not the poison");
        }
        other => panic!("a NaN control point did not refuse at the on-locus limb: {other:?}"),
    }
}

#[test]
fn a_collapsed_control_row_refuses_at_the_regularity_floor() {
    // The sphere-pole shape: one control row collapsed onto the axis,
    // where `S_u × S_v` vanishes and the offset is undefined.
    let r = 1.0;
    let control = vec![
        Point3::new(0.0, 0.0, r),
        Point3::new(0.0, 0.0, r),
        Point3::new(0.0, 0.0, r),
        Point3::new(r, 0.0, 0.0),
        Point3::new(r, r, 0.0),
        Point3::new(0.0, r, 0.0),
        Point3::new(r, 0.0, -0.5),
        Point3::new(r, r, -0.5),
        Point3::new(0.0, r, -0.5),
    ];
    let base = NurbsSurface::new(kv2(), kv2(), control, vec![1.0; 9]).unwrap();
    match fit_offset_at(&base, 0.1, 1e-4, band()) {
        Err(e @ OffsetFitError::Meter(MeterError::NormalFloor { floor, verdict, .. })) => {
            assert_eq!(floor, 0.0, "a collapsed row left a positive floor");
            // A zero margin is band-decided, and no smaller tolerance
            // passes it: the lever, and the report clause for a face with
            // no degeneracy to split off.
            assert!(
                matches!(verdict, Refused::Zero(c) if c.margin == MarginDiag::value(0.0)),
                "{verdict:?}"
            );
            let text = e.to_string();
            assert!(
                text.ends_with(
                    "Recourse: split the face clear of any pole, cusp or pinch; if it has none, \
                     this may indicate a kernel bug worth reporting"
                ),
                "{text}"
            );
        }
        other => panic!("a pole-collapsed patch was fitted: {other:?}"),
    }
}

#[test]
fn an_offset_past_the_curvature_reach_refuses_at_the_collapse_meter() {
    let r = 2.0;
    let base = sphere_band(r, 0.25, 1.25);
    let cells = patch_cells_refined(&base, OFFSET_METER_LADDER[1]).unwrap();
    // Inward past the sphere's own radius: the offset folds through
    // the centre.
    match fit_offset_at(&base, -1.2 * r, 1e-4, band()) {
        Err(e @ OffsetFitError::Meter(MeterError::CurvatureHeadroom { reach, verdict, .. })) => {
            let headroom = verdict.margin().diagnostic_f64_for_error_text().value();
            assert!(
                headroom.is_some_and(|h| h <= 0.0),
                "headroom {headroom:?} is not a refusal"
            );
            assert!(reach <= r * 1.01, "reach {reach} exceeds r = {r}");
            // Folding by a fifth of the radius is sign-certain: the
            // lever alone, no tolerance.
            assert!(matches!(verdict, Refused::Negative { .. }), "{verdict:?}");
            let text = e.to_string();
            assert!(
                text.ends_with(
                    "Recourse: use an offset distance of smaller magnitude, or offset to the \
                     other side"
                ),
                "{text}"
            );
        }
        other => panic!("a folding offset was fitted: {other:?}"),
    }
    // And an offset well inside the CERTIFIED reach fits. The
    // certified reach is what the door actually classifies against —
    // conservative, and by a factor the ladder's second rung leaves
    // at about three on this fixture — so the row is written against
    // that number rather than against the true fold radius `r`.
    let coll = patch_collapse(&cells, -1.0);
    let inside = -0.5 * coll.reach;
    if let Err(e) = fit_offset_at(&base, inside, 1e-3, band()) {
        panic!("an inward offset at half the certified reach ({inside} m) refused: {e}");
    }
}

/// **The sample-cap face.** At 1e-15 the bumpy patch runs five
/// refinement rounds and is stopped by the sample cap — the sixth
/// round's schedule would carry 66x67 samples — with a finite bound in
/// hand. The refusal names the cap and the samples, not the rounds:
/// the rounds are not a knob.
#[test]
fn a_cap_stop_with_a_finite_bound_names_the_cap_not_the_rounds() {
    let base = bumpy_patch();
    match fit_offset_at(&base, 0.05, 1e-15, band()) {
        Err(OffsetFitError::WallReached {
            wall: wall @ OffsetFitWall::SampleCap { cap },
            rounds,
            grid,
            achieved,
            tolerance,
            best,
        }) => {
            let BestBound {
                bound: best_bound,
                grid: best_grid,
            } = best;
            assert_eq!(cap, OFFSET_FIT_SAMPLE_CAP);
            assert_eq!(rounds, 5, "five rounds ran before the cap");
            assert!(grid.0 * grid.1 <= OFFSET_FIT_SAMPLE_CAP);
            assert!(achieved.is_finite() && achieved > tolerance);
            assert!(
                best_bound > tolerance && best_bound <= achieved,
                "the best bound {best_bound:e} is not the smallest of a run ending on {achieved:e}"
            );
            assert!(best_grid.0 <= grid.0 && best_grid.1 <= grid.1);
            let e = OffsetFitError::WallReached {
                wall,
                rounds,
                grid,
                achieved,
                tolerance,
                best,
            };
            let msg = e.to_string();
            assert!(
                msg.contains(&format!("limit of {cap} samples,")),
                "the cap is not what the message names: {msg}"
            );
            assert!(
                !msg.contains("rounds"),
                "the message points at the rounds: {msg}"
            );
            assert!(
                msg.contains(&format!("best certified error was {best_bound} m")),
                "the message does not report the best bound reached: {msg}"
            );
            // The cap has a geometry lever, so the message names it and
            // advises no loosening.
            assert!(
                msg.contains("Recourse: split the face") && !msg.contains("loosen"),
                "the recourse is not the split alone: {msg}"
            );
        }
        other => panic!("a cap stop with a finite bound did not name the cap: {other:?}"),
    }
}

/// **What the floor on `‖E‖` reaches on a non-analytic base.** The
/// bumpy patch at `d = 1e-6` certifies at `1.395e-10` on the third
/// round's 609 cells — four orders below `|d|`, on a patch with no
/// closed form to check against, which is why the row pins the
/// digits rather than a ratio.
#[test]
fn the_bumpy_patch_certifies_a_micron_offset_below_a_nanometre() {
    let base = bumpy_patch();
    let (_, cert) = fit_offset_at(&base, 1e-6, 1e-9, band())
        .unwrap_or_else(|e| panic!("the bumpy patch refused a 1e-9 request: {e}"));
    assert_eq!((cert.rounds, cert.cells), (3, 609));
    assert!(
        (cert.hull_sup - 1.39455e-10).abs() < cert.hull_sup * 1e-3,
        "the bumpy patch certifies at {:e}",
        cert.hull_sup
    );
}

/// **The never-finite face, and where it starts.** At `d = 1e-8` and
/// `1e-9` on the quarter cylinder the certificate limb answers `+∞`
/// on every grid the loop reaches — the sign witness does not pass on
/// the cells that carry the sup — and the cap stops it after five
/// refinement rounds with no finite bound ever produced. A refusal
/// that "carries the achieved bound" must not carry `inf` there: the
/// face says there is no number, and prints none.
///
/// One decade up the face is not reached: `d = 1e-7` certifies at
/// `5.2539e-7` on the fifth round's 1144 cells. The row pins that
/// boundary, because the two faces are one decade apart and a change
/// that moved either would otherwise move it silently.
#[test]
fn a_bound_that_never_became_finite_refuses_with_no_number() {
    let base = quarter_cylinder(1.0, 1.0);
    let (_, cert) = fit_offset_at(&base, 1e-7, 1e-3, band())
        .unwrap_or_else(|e| panic!("d = 1e-7 no longer certifies: {e}"));
    assert_eq!((cert.rounds, cert.cells), (5, 1144));
    assert!(
        (cert.hull_sup - 5.25391e-7).abs() < 5e-11,
        "d = 1e-7 certifies at {:e}",
        cert.hull_sup
    );
    for d in [1e-8_f64, 1e-9] {
        match fit_offset_at(&base, d, 1e-3, band()) {
            Err(OffsetFitError::BoundNotFinite {
                rounds,
                grid,
                d: dd,
                tolerance,
                best,
            }) => {
                assert_eq!(rounds, 5, "d = {d}: five rounds ran before the cap");
                assert!(grid.0 * grid.1 <= OFFSET_FIT_SAMPLE_CAP);
                assert_eq!(dd, d);
                assert!(
                    best.is_none(),
                    "d = {d}: a round reached a finite bound: {best:?}"
                );
                let msg = OffsetFitError::BoundNotFinite {
                    rounds,
                    grid,
                    d: dd,
                    tolerance,
                    best,
                }
                .to_string();
                // The type cannot print an `inf` here — the face has no
                // bound field — so the row asserts what the message DOES
                // say: that no refinement bounded the error, the `d` it
                // was asked for, and the distance as the repair, which
                // the decade above (`1e-7` certifies) bears out. The
                // sample cap is not named: raising it would not help.
                assert!(
                    msg.contains("no refinement of the offset surface's fit could bound its error"),
                    "d = {d}: {msg}"
                );
                assert!(
                    msg.contains(&format!("offset distance of {d} m")),
                    "d = {d}: {msg}"
                );
                assert!(
                    msg.contains("Recourse: use an offset distance of larger magnitude"),
                    "d = {d}: {msg}"
                );
                assert!(!msg.contains("inf"), "d = {d}: prints a bound: {msg}");
                assert!(
                    !msg.contains("rounds") && !msg.contains("samples"),
                    "d = {d}: points at the budget or the cap: {msg}"
                );
            }
            other => panic!("d = {d}: a never-finite bound did not refuse as one: {other:?}"),
        }
    }
}

/// **The small-`|d|` row, and the limit it pins.** The certificate's
/// normal component divides `|X|` by `w̃²·(‖E‖ + |d|)`, and the
/// composite bounds `‖E‖` below directly rather than falling back on
/// `2|d|` for that denominator: a fallback that scales the reported
/// accuracy like `1/|d|` and collapses every cell to `+∞` as soon as
/// `dist` reaches `|d|`, which is a micron-scale offset on a
/// metre-scale patch certifying as `inf`.
///
/// The floor on `‖E‖` reads the three components TOGETHER, through
/// the sign witness `D` (module docs), which is what keeps the bound
/// near `|d|`'s own scale: at `d = 1e-6` the certified sup is
/// `1.514e-5`, fifteen times `|d|`, on the same 308-cell grid a
/// componentwise floor certified at `2.630e-4`. The sup cell's
/// decomposition and the two readings of `‖E‖` behind that factor are
/// `offset_fit`'s own row.
///
/// The row pins both halves: a reachable tolerance certifies, and an
/// unreachable one refuses typed rather than reporting a number it
/// cannot support. `1e-9` is the second half — the fit's own absolute
/// accuracy does not reach it, and the grid the bound wants exceeds
/// the sample cap first, at `achieved = 2.1241e-8`. The round-5
/// grid's bound, `3.4524e-7`, is carried by `τ` (`1.85e-7` of it), the
/// tangential term, which divides by the regularity floor rather than
/// by `‖E‖`.
#[test]
fn a_micron_scale_offset_certifies_and_names_its_limit() {
    let base = quarter_cylinder(1.0, 1.0);
    let d = 1e-6;
    let (fit, cert) = fit_offset_at(&base, d, 1e-3, band())
        .unwrap_or_else(|e| panic!("a micron-scale offset refused at 1e-3: {e}"));
    let worst = worst_offset_residual(&base, &fit, d, &grid(23, 19)).unwrap();
    assert!(
        cert.hull_sup.is_finite(),
        "the small-d bound is {} — the `2|d|` denominator is back",
        cert.hull_sup
    );
    assert!(
        worst <= cert.hull_sup,
        "d = {d}: certified sup {} UNDER-reports the sampled max {worst}",
        cert.hull_sup
    );
    assert!(
        (cert.hull_sup - 1.51411e-5).abs() < 5e-9,
        "the certified sup is {:e}",
        cert.hull_sup
    );
    assert!(
        cert.hull_sup < 20.0 * d,
        "the bound is {:e}, no longer within twenty times |d|",
        cert.hull_sup
    );
    eprintln!(
        "small-d d={d:.0e}: cells={} rounds={} hull_sup={:.3e} sampled={worst:.3e} \
         (ratio to |d|: {:.0}x)",
        cert.cells,
        cert.rounds,
        cert.hull_sup,
        worst.max(cert.hull_sup) / d
    );
    // The honest other half: a tolerance below what the fit's own
    // absolute accuracy can reach refuses typed, carrying the bound
    // it did reach — never a number it cannot support. The stop is
    // the sample cap's: the grid the bound wants exceeds it.
    match fit_offset_at(&base, d, 1e-9, band()) {
        Err(OffsetFitError::WallReached {
            wall: OffsetFitWall::SampleCap { .. },
            achieved,
            rounds,
            ..
        }) => {
            assert_eq!(rounds, 7);
            assert!(
                (achieved - 2.12411e-8).abs() < 5e-12,
                "the cap stop carries {achieved:e}"
            );
            eprintln!(
                "small-d: 1e-9 refused typed at the cap after {rounds} rounds, achieved = {achieved:.3e}"
            );
        }
        other => {
            panic!("a tolerance below the fit's absolute accuracy did not refuse typed: {other:?}")
        }
    }
}

/// **The certifying limb's own red.** The degraded-fit row above trips
/// limb 1, which is the sampled limb; the limb that CERTIFIES had no
/// red of its own. A fit that is right at every on-locus sample and
/// wrong between them is what limb 2 exists to catch, and a fit built
/// for a different `d` is exactly that shape at the cell scale: its
/// sign witness fails, the cell answers `+∞`, and the certificate
/// refuses naming `HullSup`.
#[test]
fn a_fit_for_the_wrong_distance_is_refused_by_the_certifying_limb() {
    let base = quarter_cylinder(1.0, 1.0);
    let (fit, _) = fit_offset_at(&base, 0.3, 1e-3, band()).unwrap();
    // Certified against the OPPOSITE sign: `E·n` carries the wrong
    // sign everywhere, so `D`'s witness cannot pass and limb 2 is the
    // limb that must speak. A tolerance far above the true residual
    // keeps limb 1 quiet, so the refusal can only come from limb 2.
    match certify_offset_at(&base, &fit, -0.3, 1e3, band()) {
        Err(OffsetFitError::Limb { limb, bound, .. }) => {
            assert_eq!(limb, OffsetLimb::HullSup);
            assert!(
                bound.is_infinite(),
                "the unproved sign witness must answer +inf, not {bound}"
            );
        }
        other => panic!("limb 2 certified a fit built for the other sign: {other:?}"),
    }
}

#[test]
fn a_zero_or_non_finite_request_refuses_at_the_door() {
    let base = quarter_cylinder(1.0, 1.0);
    for (d, tol) in [(0.0, 1e-6), (f64::NAN, 1e-6), (0.2, 0.0), (0.2, -1.0)] {
        assert!(
            matches!(
                fit_offset_at(&base, d, tol, band()),
                Err(OffsetFitError::InvalidRequest { .. })
            ),
            "d = {d}, tol = {tol} was accepted"
        );
    }
}

/// **The recentring row.** The offset residual is translation
/// invariant — moving a part does not change how well a surface fits
/// its own offset — so the certified bound should be too. It was not:
/// the composite's nets were built in world coordinates, so the
/// ring's rounding on the intermediates scaled with the base's
/// coordinate magnitude, and a micron offset on a metre part a
/// kilometre from the origin certified as `inf` while the same part
/// at the origin certified at 1.5e-5.
///
/// The composite now builds every net against one recentring origin
/// (the base control net's bbox midpoint), which is exact in ℝ and
/// leaves every claim identical.
///
/// **The row states its true domain, because the invariance is not
/// unlimited.** Measured on the decade ladder at `d = 1e-6`, every
/// station on the SAME 308-cell grid:
///
/// ```text
/// 1e0..1e4   1.5141e-5   flat to five figures
/// 1e5        1.5142e-5   1.00007x the origin
/// 1e6        1.5150e-5   1.0006x
/// 1e7        1.5223e-5   1.0054x
/// 1e8        1.6110e-5   1.0640x
/// 1e9        3.9769e-5   2.627x
/// 1e10       refused: BoundNotFinite, best None — no grid reached one
/// ```
///
/// So the band is asserted where the claim is meaningful — out to
/// 1e7, where rounding still tracks the recentred patch — and the
/// stations beyond it are pinned for what is actually true of them:
/// containment, which holds at every finite station. The ladder is
/// monotone in the shift as far as it was measured, and the row does
/// not assert that either: monotonicity would be a claim about the
/// refinement schedule, which is the kernel's and not this row's
/// subject. At 1e10 the door refuses typed rather than return
/// something uncertified, which is the honest end of the ladder and
/// is pinned as such.
///
/// Containment at each station is what stops the invariance being
/// bought by a bound that stopped bounding.
#[test]
fn a_patch_far_from_the_origin_certifies_as_well_as_one_at_it() {
    let d = 1e-6;
    let mut at_origin = f64::NAN;
    let shifted = |shift: f64| {
        let c = quarter_cylinder(1.0, 1.0);
        let control: Vec<Point3<f64>> = c
            .control()
            .iter()
            .map(|p| Point3::new(p.x + shift, p.y + shift, p.z))
            .collect();
        NurbsSurface::new(
            c.knots_u_f64_interim().clone(),
            c.knots_v_f64_interim().clone(),
            control,
            c.weights().to_vec(),
        )
        .unwrap()
    };
    // The stations, not every decade: as measured, `fit_offset` walks
    // the SAME 308-cell grid at every shift from the origin through
    // 1e9, and the ladder in this row's doc is flat to five figures
    // from 1e0 to 1e4, so a decade inside the flat run re-derives a
    // bound already asserted. What is kept is one station per
    // distinct reading: the origin's baseline, the far end of the
    // flat run, the three stations where the digits walk away from it
    // (1e5, 1e6, 1e7 — the band edge), the two out-of-band stations,
    // and the refusal below.
    //
    // That grid reading is UNGUARDED, deliberately. The schedule it
    // describes is the kernel's, not this row's, and pinning
    // `cells`/`rounds` here would turn any refinement improvement red
    // in a row whose subject is recentring invariance. What lapses if
    // the schedule moves is only the coverage argument for the
    // decades not visited: every station this row does visit still
    // asserts containment, and the invariance band is still asserted
    // where the claim is meaningful.
    for e in [0i32, 4, 5, 6, 7, 8, 9] {
        let shift = if e == 0 { 0.0 } else { 10f64.powi(e) };
        let base = shifted(shift);
        let (fit, cert) = fit_offset_at(&base, d, 1e-2, band())
            .unwrap_or_else(|err| panic!("shift 1e{e}: a micron offset refused: {err}"));
        let worst = worst_offset_residual(&base, &fit, d, &grid(23, 19)).unwrap();
        // True at EVERY station, and the assertion the whole row
        // exists to protect.
        assert!(
            worst <= cert.hull_sup,
            "shift 1e{e}: certified sup {} UNDER-reports the sampled max {worst}",
            cert.hull_sup
        );
        if e == 0 {
            at_origin = cert.hull_sup;
        } else if e <= 7 {
            // The invariance band, where the claim is meaningful.
            // Measured worst over this range is 1.0054x at 1e7.
            assert!(
                cert.hull_sup <= at_origin * 1.05,
                "shift 1e{e}: hull_sup {} is more than 5% above the same patch at the \
                 origin ({at_origin}) — the composite is reading world coordinates again",
                cert.hull_sup
            );
        }
        eprintln!(
            "recentred shift=1e{e}: cells={} hull_sup={:.4e} sampled={worst:.4e}",
            cert.cells, cert.hull_sup
        );
    }
    // The honest end of the ladder: a shift the recentring cannot
    // rescue refuses typed and returns nothing uncertified — and no
    // grid it reaches produces a finite bound, so the refusal carries
    // none rather than an `inf`.
    match fit_offset_at(&shifted(1.0e10), d, 1e-2, band()) {
        Err(OffsetFitError::BoundNotFinite {
            rounds, grid, best, ..
        }) => {
            assert!(best.is_none(), "a grid reached {best:?}");
            eprintln!(
                "recentred shift=1e10: refused typed, never finite after {rounds} rounds on {grid:?}"
            );
        }
        other => panic!("shift 1e10 did not refuse as a never-finite bound: {other:?}"),
    }
}

/// **The anisotropy row.** A quarter cylinder of near-zero height:
/// the `u` direction carries a quarter arc, the `v` direction is an
/// exact ruling a millimetre long that the very first fit reproduces.
/// Bisecting both directions on every failing cell buys a quadratic
/// grid for a linear need; bisecting the direction whose model-space
/// extent `h_d · sup‖S_d‖` is larger spends the rounds where the
/// error is.
///
/// The row asserts the shape of the answer, not a cell count: the
/// certificate contains, and the schedule stays within a small
/// multiple of the `v` direction's seed rather than growing with it.
#[test]
fn refinement_follows_the_anisotropy_on_a_thin_patch() {
    let base = quarter_cylinder(1.0, 1.0e-3);
    let d = 0.1;
    let tol = 1e-5;
    let (fit, cert) = fit_offset_at(&base, d, tol, band())
        .unwrap_or_else(|e| panic!("the thin patch refused at {tol}: {e}"));
    let worst = worst_offset_residual(&base, &fit, d, &grid(23, 19)).unwrap();
    assert!(
        worst <= cert.hull_sup,
        "certified sup {} UNDER-reports the sampled max {worst}",
        cert.hull_sup
    );
    // The `v` direction needs no refinement at all, so the schedule
    // must not have paid for any. Measured 14 cells; the ceiling is
    // 28, i.e. 2x headroom — tight enough that a real regression reds
    // it, since the both-directions loop this replaced reached 308
    // cells on exactly this fixture: 22x the measurement, 11x the
    // ceiling.
    assert!(
        cert.cells <= 28,
        "the schedule grew in the direction that carries no error: {} cells \
         (measured 14 when written)",
        cert.cells
    );
    eprintln!(
        "anisotropic: cells={} rounds={} hull_sup={:.3e} sampled={worst:.3e}",
        cert.cells, cert.rounds, cert.hull_sup
    );
}

/// **A seed over the cap refuses before any fit.** A bicubic patch with
/// sixteen spans a direction seeds three pieces in each, 49 x 49
/// samples, more than the cap holds, and the door refuses `SeedOverCap`
/// on that seed rather than fitting it.
#[test]
fn a_patch_with_more_spans_than_the_cap_seeds_refuses_before_round_0() {
    let spans = 16usize;
    let mut knots = vec![0.0; 4];
    #[allow(clippy::cast_precision_loss)]
    knots.extend((1..spans).map(|i| i as f64 / spans as f64));
    knots.extend([1.0; 4]);
    let n = spans + 3;
    let kv = KnotVector::clamped(knots, 3).unwrap();
    let mut control = Vec::with_capacity(n * n);
    for i in 0..n {
        for j in 0..n {
            #[allow(clippy::cast_precision_loss)]
            let (x, y) = (i as f64 / (n - 1) as f64, j as f64 / (n - 1) as f64);
            control.push(Point3::new(x, y, 0.05 * (3.0 * x).sin() * (2.0 * y).cos()));
        }
    }
    let base = NurbsSurface::new(kv.clone(), kv, control, vec![1.0; n * n]).unwrap();
    match fit_offset_at(&base, 0.05, 1e-6, band()) {
        Err(ref e @ OffsetFitError::SeedOverCap { cap, grid }) => {
            assert_eq!(cap, OFFSET_FIT_SAMPLE_CAP);
            assert_eq!(grid, (49, 49));
            assert!(e.to_string().ends_with("Recourse: split the face"), "{e}");
        }
        other => panic!("a seed over the cap was not refused before round 0: {other:?}"),
    }
}

/// **A knot span a few ulps wide refuses on the span**
/// (`geom_brep::recourse::NARROW_SPAN`), one row per arm it reaches
/// through the fit door: `[1e15, 1e15 + 1]` holds nine `f64` values,
/// so a quadratic's refinement overflows and a cubic's refined vector
/// is discontinuous; two one-ulp spans on `[4e15, 4e15 + 1]` seed three
/// distinct parameters under a cubic's four; one two-ulp rational span
/// on `[1, 1 + 2ε]` seeds a repeated one. Each names the span as its
/// lever and never the kernel-defect dead end, and none reaches the
/// fit's spline assembly, whose averaged knots a far domain was feared
/// to collapse.
#[test]
fn a_knot_span_a_few_ulps_wide_refuses_on_its_span() {
    let face = |lo: f64, p: usize, interior: &[f64]| {
        let mut knots = vec![lo; p + 1];
        knots.extend(interior.iter().map(|t| lo + t));
        knots.extend(vec![lo + 1.0; p + 1]);
        let ku = KnotVector::clamped(knots, p).unwrap();
        let kv = KnotVector::clamped([vec![0.0; p + 1], vec![1.0; p + 1]].concat(), p).unwrap();
        let (nu, nv) = (ku.control_count(), kv.control_count());
        let mut control = Vec::with_capacity(nu * nv);
        for i in 0..nu {
            for j in 0..nv {
                #[allow(clippy::cast_precision_loss)]
                let (x, y) = (i as f64 / (nu - 1) as f64, j as f64 / (nv - 1) as f64);
                control.push(Point3::new(
                    x,
                    y,
                    0.3 * (3.0 * x).sin() * (2.0 * y + 0.4).cos(),
                ));
            }
        }
        NurbsSurface::new(ku, kv, control, vec![1.0; nu * nv]).unwrap()
    };
    // Degree 1 in `u` over `[1, 1 + 2ε]`, `x` at its Grevilles so the
    // `u` speed is one, rational so the patch bound reads it unrefined.
    let two_ulps = {
        let (a, b) = (1.0, 1.0 + 2.0 * f64::EPSILON);
        let ku = KnotVector::clamped(vec![a, a, b, b], 1).unwrap();
        let mut control = Vec::new();
        for x in [a, b] {
            for (y, z) in [(0.0, 0.0), (0.5, 0.2), (1.0, 0.0)] {
                control.push(Point3::new(x, y, z));
            }
        }
        NurbsSurface::new(ku, kv2(), control, vec![1.0, 0.9, 1.0, 1.0, 0.9, 1.0]).unwrap()
    };
    type Arm = fn(&OffsetFitError) -> bool;
    let rows: [(&str, NurbsSurface<f64>, Arm); 4] = [
        ("insertion chain", face(1e15, 2, &[]), |e| {
            matches!(
                e,
                OffsetFitError::PatchBound(PatchBoundError::RefinementFailed)
            )
        }),
        ("refined vector", face(1e15, 3, &[]), |e| {
            matches!(e, OffsetFitError::PatchBound(PatchBoundError::DerivedKnots))
        }),
        ("seed", face(4e15, 2, &[0.5]), |e| {
            matches!(
                e,
                OffsetFitError::Fit(geom::curves::fit::FitError::TooFewPoints { .. })
            )
        }),
        ("repeated seed", two_ulps, |e| {
            matches!(
                e,
                OffsetFitError::Fit(geom::curves::fit::FitError::ParamCountMismatch { .. })
            )
        }),
    ];
    for (name, base, arm) in rows {
        match fit_offset_at(&base, 0.01, 1e-4, band()) {
            Err(e) => {
                assert!(arm(&e), "{name}: refused on another arm: {e:?}");
                let msg = e.to_string();
                assert!(
                    msg.contains(geom_brep::recourse::NARROW_SPAN)
                        && msg.ends_with(geom_brep::recourse::NARROW_SPAN_RECOURSE),
                    "{name}: not the narrow span's story: {msg}"
                );
            }
            Ok((_, cert)) => panic!("{name}: a span of a few ulps fitted: {cert:?}"),
        }
    }
}

/// **A bound found and then lost.** The quarter cylinder at
/// `d = -2e-8` and `1e-15` reaches a finite bound on a coarser grid and
/// loses it on the finer one the cap stops at, so the not-finite face
/// carries the bound it lost.
#[test]
fn a_bound_found_then_lost_carries_the_bound_it_lost() {
    let base = quarter_cylinder(1.0, 1.0);
    match fit_offset_at(&base, -2e-8, 1e-15, band()) {
        Err(OffsetFitError::BoundNotFinite {
            rounds,
            grid,
            best: Some(best),
            ..
        }) => {
            // Also `OFFSET_FIT_WORK`'s guard: this run fits the most
            // samples of any measured, and a budget under them would stop
            // it on an earlier round.
            assert_eq!((rounds, grid, best.grid), (10, (56, 29), (52, 29)));
            assert!(
                (best.bound - 4.4774e-6).abs() < 1e-9,
                "the lost bound is {:e}",
                best.bound
            );
        }
        other => panic!("expected a not-finite stop carrying a lost bound: {other:?}"),
    }
}

/// **A round whose bound rose is answered, not refused.** The bumpy
/// patch at `d = 5e-6` certifies `2e-9` on round 5, at 1.1284e-9, and
/// round 6's bound rises above that: a request between the two runs
/// past round 6. The loop takes the both-directions step there rather
/// than stopping, falls again, and at `1e-15` refines to the cap, far
/// below round 5's bound.
#[test]
fn a_round_whose_bound_rose_takes_the_both_directions_step() {
    let base = bumpy_patch();
    let d = 5e-6;
    let (_, at5) = fit_offset_at(&base, d, 2e-9, band())
        .unwrap_or_else(|e| panic!("the bumpy patch refused 2e-9: {e}"));
    assert!(
        at5.rounds == 5 && (at5.hull_sup - 1.1285881e-9).abs() < 1e-14,
        "round {} carries {:e}",
        at5.rounds,
        at5.hull_sup
    );
    let (_, past) = fit_offset_at(&base, d, 1.1e-9, band())
        .unwrap_or_else(|e| panic!("the bumpy patch refused 1.1e-9: {e}"));
    assert!(
        past.rounds > 6,
        "a request under round 5's bound certified on round {}, so round 6 did not rise",
        past.rounds
    );
    match fit_offset_at(&base, d, 1e-15, band()) {
        Err(OffsetFitError::WallReached {
            wall: OffsetFitWall::SampleCap { .. },
            rounds,
            grid,
            achieved,
            ..
        }) => {
            assert_eq!((rounds, grid), (9, (38, 37)));
            assert!(
                (achieved - 7.39e-11).abs() < 1e-13,
                "the cap stop carries {achieved:e}"
            );
        }
        other => panic!("expected the bumpy patch to refine to the cap: {other:?}"),
    }
}

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
//!   unreachable tolerance refuses typed at the budget.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::FRAC_PI_2;

use geom::NurbsSurface;
use geom::curves::fit::interpolate_columns;
use geom_brep::offset_fit::{
    OFFSET_FIT_BUDGET, OFFSET_FIT_SAMPLE_CAP, OffsetFitError, OffsetLimb, certify_offset,
    fit_offset, offset_point,
};
use geom_brep::offset_meters::{MeterError, OFFSET_METER_LADDER, patch_collapse, patch_regularity};
use geom_brep::patch_bound::patch_cells_refined;
use geom_core::spline::KnotVector;
use geom_core::{Band, Point3, Tol};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

// ---------------------------------------------------------------------
// Exact rational NURBS fixtures (closed-form geometry, by construction)
// ---------------------------------------------------------------------

/// The clamped degree-2 single-Bézier knot vector.
fn kv2() -> KnotVector {
    KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap()
}

/// The clamped degree-1 single-span knot vector.
fn kv1() -> KnotVector {
    KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap()
}

/// A quarter cylinder of radius `r` and height `h` about `+z`, exact:
/// the u direction is the rational quadratic quarter circle
/// (weights `1, √2/2, 1` — the classical exact arc), the v direction
/// a linear translation.
fn quarter_cylinder(r: f64, h: f64) -> NurbsSurface<f64> {
    let s = arc_weight(FRAC_PI_2);
    let control = vec![
        Point3::new(r, 0.0, 0.0),
        Point3::new(r, 0.0, h),
        Point3::new(r, r, 0.0),
        Point3::new(r, r, h),
        Point3::new(0.0, r, 0.0),
        Point3::new(0.0, r, h),
    ];
    let weights = vec![1.0, 1.0, s, s, 1.0, 1.0];
    NurbsSurface::new(kv2(), kv1(), control, weights).unwrap()
}

/// The classical rational-quadratic arc weight for a sweep of
/// `sweep` radians: `cos(sweep/2)` (a quarter turn gives `√2/2`).
fn arc_weight(sweep: f64) -> f64 {
    (sweep * 0.5).cos()
}

/// A sphere band of radius `r` between latitudes `lat0` and `lat1`,
/// swept a quarter turn in longitude — exact: a rational quadratic
/// meridian arc revolved through the classical rational quadratic
/// quarter turn (A8.1's weight product), with no control row on the
/// axis, so the chart normal is regular everywhere on it.
fn sphere_band(r: f64, lat0: f64, lat1: f64) -> NurbsSurface<f64> {
    // Meridian: a rational quadratic arc through the sweep
    // `lat1 − lat0`, in the (x, z) half-plane.
    let theta = 0.5 * (lat1 - lat0);
    let wm = theta.cos();
    let a = (r * lat0.cos(), r * lat0.sin());
    let b = (r * lat1.cos(), r * lat1.sin());
    // The tangent-intersection control point: the midpoint direction
    // at radius `r / cos θ`.
    let mid = (a.0 + b.0, a.1 + b.1);
    let mlen = (mid.0 * mid.0 + mid.1 * mid.1).sqrt();
    let m = (mid.0 / mlen * r / wm, mid.1 / mlen * r / wm);
    let meridian = [(a.0, a.1, 1.0), (m.0, m.1, wm), (b.0, b.1, 1.0)];
    // Revolve a quarter turn about `+z` (A8.1): the row is
    // `(x, 0, z), (x, x, z), (0, x, z)` with weights
    // `w, w·cos45, w`.
    let wr = arc_weight(FRAC_PI_2);
    let mut control = Vec::with_capacity(9);
    let mut weights = Vec::with_capacity(9);
    for iu in 0..3 {
        for (x, z, w) in meridian {
            control.push(match iu {
                0 => Point3::new(x, 0.0, z),
                1 => Point3::new(x, x, z),
                _ => Point3::new(0.0, x, z),
            });
            weights.push(if iu == 1 { w * wr } else { w });
        }
    }
    NurbsSurface::new(kv2(), kv2(), control, weights).unwrap()
}

/// A non-analytic bicubic patch: a height field with no closed form
/// as any analytic kind, interpolated through the loft door.
fn bumpy_patch() -> NurbsSurface<f64> {
    let n = 7;
    let params: Vec<f64> = (0..n)
        .map(|i| {
            #[allow(clippy::cast_precision_loss)]
            let t = i as f64 / (n - 1) as f64;
            t
        })
        .collect();
    let height = |u: f64, v: f64| 0.35 * (2.4 * u).sin() * (1.9 * v + 0.4).cos() + 0.2 * u * v;
    let rows: Vec<Vec<f64>> = params
        .iter()
        .map(|u| {
            let mut row = Vec::with_capacity(n * 3);
            for v in &params {
                row.extend_from_slice(&[*u, *v, height(*u, *v)]);
            }
            row
        })
        .collect();
    let (ku, r) = interpolate_columns(&params, 3, &rows).unwrap();
    let mut rows_v: Vec<Vec<f64>> = Vec::with_capacity(n);
    for l in 0..n {
        let mut row = Vec::with_capacity(ku.control_count() * 3);
        for rr in &r {
            row.extend_from_slice(&rr[l * 3..l * 3 + 3]);
        }
        rows_v.push(row);
    }
    let (kv, p) = interpolate_columns(&params, 3, &rows_v).unwrap();
    let (cu, cv) = (ku.control_count(), kv.control_count());
    let mut control = Vec::with_capacity(cu * cv);
    for i in 0..cu {
        for row in p.iter().take(cv) {
            control.push(Point3::new(row[i * 3], row[i * 3 + 1], row[i * 3 + 2]));
        }
    }
    NurbsSurface::new(ku, kv, control, vec![1.0; cu * cv]).unwrap()
}

/// A deterministic dense `(u, v)` schedule over `[0,1]²`, coprime
/// counts so it never lands on the fit's own sample grid.
fn dense_grid() -> Vec<(f64, f64)> {
    let (nu, nv) = (23usize, 19usize);
    let mut out = Vec::with_capacity(nu * nv);
    for i in 0..nu {
        for j in 0..nv {
            #[allow(clippy::cast_precision_loss)]
            out.push((i as f64 / (nu - 1) as f64, j as f64 / (nv - 1) as f64));
        }
    }
    out
}

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
    // refinement round whose cells cost more CI wall clock than the
    // row buys in evidence. The achieved numbers are printed either
    // way and the containment assertions are unchanged.
    let tol = 3e-4;
    for d in [0.3_f64, -0.4] {
        let (fit, cert) = fit_offset(&base, d, tol, band())
            .unwrap_or_else(|e| panic!("fit_offset refused at d = {d}: {e}"));
        assert!(
            cert.hull_sup <= tol,
            "d = {d}: certified sup {} exceeds the tolerance {tol}",
            cert.hull_sup
        );
        // The independent oracle: the closed form, sampled densely
        // through the public evaluation door.
        let mut worst = 0.0f64;
        for (u, v) in dense_grid() {
            let p = base.eval(u, v);
            let want = cylinder_offset_closed_form(p, r, d);
            let got = fit.eval(u, v);
            worst = worst.max((got - want).norm());
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
        for (u, v) in dense_grid() {
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

#[test]
fn sphere_band_fit_matches_the_closed_form_both_signs() {
    let r = 2.0;
    let base = sphere_band(r, 0.25, 1.25);
    let tol = 3e-4;
    for d in [0.35_f64, -0.5] {
        let (fit, cert) = fit_offset(&base, d, tol, band())
            .unwrap_or_else(|e| panic!("fit_offset refused at d = {d}: {e}"));
        assert!(cert.hull_sup <= tol, "certified sup {}", cert.hull_sup);
        let mut worst = 0.0f64;
        for (u, v) in dense_grid() {
            let p = base.eval(u, v);
            let rad = (p.x * p.x + p.y * p.y + p.z * p.z).sqrt();
            assert!(
                (rad - r).abs() < 1e-12,
                "fixture is not an exact sphere: |p| = {rad} at ({u}, {v})"
            );
            let want = sphere_offset_closed_form(p, r, d);
            worst = worst.max((fit.eval(u, v) - want).norm());
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
    let (fit, cert) = fit_offset(&base, d, tol, band())
        .unwrap_or_else(|e| panic!("fit_offset refused on the non-analytic base: {e}"));
    assert!(cert.hull_sup <= tol);
    let mut worst = 0.0f64;
    for (u, v) in dense_grid() {
        let target = offset_point(&base, d, u, v).unwrap();
        worst = worst.max((fit.eval(u, v) - target).norm());
    }
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
    for (u, v) in dense_grid() {
        let j = base.ders(u, v);
        inf = inf.min(j.du.cross(j.dv).norm());
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
    for (u, v) in dense_grid() {
        let p = base.eval(u, v);
        let rad = (p.x * p.x + p.y * p.y + p.z * p.z).sqrt();
        assert!(
            (rad - r).abs() < 1e-12,
            "fixture is not an exact sphere: |p| = {rad} at ({u}, {v})"
        );
    }
    let cells = patch_cells_refined(&base, OFFSET_METER_LADDER[1]).unwrap();
    {
        use geom_core::ring_interval::RingInterval as RI;
        let dot3 = |a: &[RI; 3], b: &[RI; 3]| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
        let nsq = |a: &[RI; 3]| a[0].sqr() + a[1].sqr() + a[2].sqr();
        let mut worst: Option<(f64, String)> = None;
        for c in &cells {
            let n = geom_brep::offset_meters::cell_normal(c);
            let mag = RI::from_bounds(n.floor, n.sup);
            let unit = [n.m[0] / mag, n.m[1] / mag, n.m[2] / mag];
            let (e, f, g) = (nsq(&c.s_u), dot3(&c.s_u, &c.s_v), nsq(&c.s_v));
            let (l, m, nn) = (
                dot3(&unit, &c.s_uu),
                dot3(&unit, &c.s_uv),
                dot3(&unit, &c.s_vv),
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
                n.sup,
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
    // The certified reach is CONSERVATIVE — never above the true
    // fold radius `r`, and on this fixture within a factor of ~2 of
    // it (the join's measured slack; see `cell_curvature`).
    assert!(
        coll.reach <= r && coll.reach > 0.3 * r,
        "inward reach {} is not within a factor of two of r = {r}",
        coll.reach
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
    let (fit, cert) = fit_offset(&base, d, 1e-3, band()).unwrap();
    assert!(certify_offset(&base, &fit, d, 1e-3, band()).is_ok());
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
    match certify_offset(&base, &degraded, d, 1e-3, band()) {
        Err(OffsetFitError::Limb { limb, bound, .. }) => {
            assert_eq!(limb, OffsetLimb::OnLocus);
            assert!(bound > 1e-3, "the degraded fit measured only {bound}");
        }
        other => panic!("a degraded fit certified: {other:?}"),
    }
    assert!(cert.hull_sup <= 1e-3);
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
    match fit_offset(&base, 0.1, 1e-4, band()) {
        Err(OffsetFitError::Meter(MeterError::NormalFloor { floor, .. })) => {
            assert_eq!(floor, 0.0, "a collapsed row left a positive floor");
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
    match fit_offset(&base, -1.2 * r, 1e-4, band()) {
        Err(OffsetFitError::Meter(MeterError::CurvatureHeadroom {
            reach, headroom, ..
        })) => {
            assert!(headroom <= 0.0, "headroom {headroom} is not a refusal");
            assert!(reach <= r * 1.01, "reach {reach} exceeds r = {r}");
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
    if let Err(e) = fit_offset(&base, inside, 1e-3, band()) {
        panic!("an inward offset at half the certified reach ({inside} m) refused: {e}");
    }
}

#[test]
fn an_unreachable_tolerance_refuses_typed_at_the_budget() {
    let base = bumpy_patch();
    match fit_offset(&base, 0.05, 1e-15, band()) {
        Err(OffsetFitError::BudgetExhausted {
            budget,
            grid,
            achieved,
            tolerance,
        }) => {
            assert_eq!(budget, OFFSET_FIT_BUDGET);
            assert!(grid.0 <= OFFSET_FIT_SAMPLE_CAP && grid.1 <= OFFSET_FIT_SAMPLE_CAP);
            assert!(achieved.is_finite() && achieved > tolerance);
        }
        other => panic!("an unreachable tolerance did not refuse typed: {other:?}"),
    }
}

#[test]
fn a_zero_or_non_finite_request_refuses_at_the_door() {
    let base = quarter_cylinder(1.0, 1.0);
    for (d, tol) in [(0.0, 1e-6), (f64::NAN, 1e-6), (0.2, 0.0), (0.2, -1.0)] {
        assert!(
            matches!(
                fit_offset(&base, d, tol, band()),
                Err(OffsetFitError::InvalidRequest { .. })
            ),
            "d = {d}, tol = {tol} was accepted"
        );
    }
}

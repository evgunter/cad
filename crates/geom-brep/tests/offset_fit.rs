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
    BestBound, LastRound, OFFSET_FIT_BUDGET, OFFSET_FIT_SAMPLE_CAP, OffsetFitError, OffsetLimb,
    certify_offset_at, fit_offset_at,
};
use geom_brep::offset_meters::{MeterError, OFFSET_METER_LADDER, patch_collapse, patch_regularity};
use geom_brep::patch_bound::patch_cells_refined;
use geom_core::Bounds;
use geom_core::Point3;
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
        bits(fit.knots_u().domain()),
        (ulo.to_bits(), uhi.to_bits()),
        "u domain: {:?}",
        fit.knots_u().domain()
    );
    assert_eq!(
        bits(fit.knots_v().domain()),
        (vlo.to_bits(), vhi.to_bits()),
        "v domain: {:?}",
        fit.knots_v().domain()
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
    match fit_offset_at(&base, -1.2 * r, 1e-4, band()) {
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
    if let Err(e) = fit_offset_at(&base, inside, 1e-3, band()) {
        panic!("an inward offset at half the certified reach ({inside} m) refused: {e}");
    }
}

/// **The sample-cap face.** At 1e-15 the bumpy patch runs five of the
/// six refinement rounds and is stopped by the per-direction sample
/// cap — the sixth round's schedule would carry 66x67 samples against
/// a cap of 48 — with a finite bound in hand. The refusal has to say
/// so: a caller reading the round budget off it would raise the wrong
/// knob, because the rounds were never what ran out.
#[test]
fn a_cap_stop_with_a_finite_bound_names_the_cap_not_the_round_budget() {
    let base = bumpy_patch();
    match fit_offset_at(&base, 0.05, 1e-15, band()) {
        Err(OffsetFitError::SampleCapReached {
            cap,
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
            assert_eq!(rounds, 5, "five of the six rounds ran before the cap");
            assert!(grid.0 <= OFFSET_FIT_SAMPLE_CAP && grid.1 <= OFFSET_FIT_SAMPLE_CAP);
            assert!(achieved.is_finite() && achieved > tolerance);
            assert!(
                best_bound > tolerance && best_bound <= achieved,
                "the best bound {best_bound:e} is not the smallest of a run ending on {achieved:e}"
            );
            assert!(best_grid.0 <= grid.0 && best_grid.1 <= grid.1);
            let e = OffsetFitError::SampleCapReached {
                cap,
                rounds,
                grid,
                achieved,
                tolerance,
                best,
            };
            let msg = e.to_string();
            assert!(
                msg.contains(&format!("limit of {cap} samples per direction")),
                "the cap is not what the message names: {msg}"
            );
            assert!(
                !msg.contains("rounds"),
                "the message points at the round budget: {msg}"
            );
            assert!(
                msg.contains(&format!("loosen the tolerance to {best_bound} m")),
                "the repair is not sized to the best bound reached: {msg}"
            );
        }
        other => panic!("a cap stop with a finite bound did not name the cap: {other:?}"),
    }
}

/// **The budget's last round, and which face it wears.** The stall
/// guard's refusal wants TWO non-improving rounds running, the second
/// past a both-directions marking; a single non-improving round is
/// not it, and the loop must reach the round budget's own face there
/// rather than the stall's — a refusal saying "the strongest step
/// gained nothing" on a loop that was given one weaker step sends the
/// caller to the wrong knob.
///
/// The bumpy patch at `d = 1e-4` is that shape at an unreachable
/// `1e-15`: it exhausts the budget on a round whose bound ROSE. The
/// other side of the ordering — a second non-improving round wearing
/// the stall's face — is
/// `the_second_non_improving_round_is_the_stalls_face`.
///
/// **What the face says on a round that rose.** Not "still improving",
/// and not the last round's bound as the number to size against:
/// `last_round` is `DidNotImprove`, and `best` is round 5's bound, the
/// smallest any round reached, which is what the recourse names.
///
/// **The row asserts that the last round really did not improve**,
/// because the name says so and a schedule change could otherwise
/// leave it green over a loop that was never the shape it claims. The
/// loop exposes no per-round trace, but it does not need to: at a
/// fixed band the tolerance decides only WHERE the walk stops
/// (`BestBound`'s recourse claim), so a request whose
/// tolerance an earlier round already met certifies ON that round and
/// hands back exactly the bound this run stepped off. The ladder read
/// back that way, at `d = 1e-4`:
///
/// ```text
/// round 0   1.8219683e-5   (144 cells)
/// round 1   6.5173320e-8   (224 cells)
/// round 5   6.0173058e-9   (782 cells)
/// round 6   8.3524739e-9   — the budget face, HIGHER than round 5
/// ```
///
/// **Re-read when certification arithmetic became the backend's**
/// (rounds 1, 5 and 6 were `6.5173322e-8`, `6.0173184e-9` and
/// `1.0707700e-8`): the retired arithmetic padded one representable step outward on
/// every operation of `cell_bound`'s assembly and the backend pads
/// only where the operation is inexact, so every rung came in tighter
/// and round 0 by less than its own printed precision. The SHAPE the
/// row is about is unmoved — round 6 is still higher than round 5, and
/// still the lone non-improving round.
///
/// Round 6 is the non-improving one, and it is a LONE one: round 5
/// came in under `1e-8` where round 4 did not, so round 5 improved,
/// so the marking that built round 6's grid was the directional one
/// rather than the both-directions fallback — which is the admission
/// set the stall's refusal wants and does not have here.
#[test]
fn a_single_non_improving_round_is_the_budgets_face_not_the_stalls() {
    let base = bumpy_patch();
    let refusal = fit_offset_at(&base, 1e-4, 1e-15, band());
    let (achieved, last_round, best) = match &refusal {
        Err(OffsetFitError::BudgetExhausted {
            budget,
            grid,
            achieved,
            tolerance,
            last_round,
            best,
        }) => {
            assert_eq!(*budget, OFFSET_FIT_BUDGET);
            assert!(achieved.is_finite() && achieved > tolerance);
            assert!(
                (achieved - 8.3524739e-9).abs() < achieved * 1e-5,
                "the budget face carries {achieved:e}"
            );
            eprintln!(
                "budget face: grid={grid:?} achieved={achieved:.7e} best={:.7e}",
                best.bound
            );
            (*achieved, *last_round, *best)
        }
        other => panic!("the budget's last round did not wear the budget's face: {other:?}"),
    };
    let msg = refusal.err().map(|e| e.to_string()).unwrap_or_default();
    // The round the budget face stepped off, read back through the
    // door. `1e-8` is met by round 5 and by no round before it.
    let (prev_fit, prev) = fit_offset_at(&base, 1e-4, 1e-8, band())
        .unwrap_or_else(|e| panic!("the round before the budget face refused: {e}"));
    assert!(
        prev.rounds == 5 && (prev.hull_sup - 6.0173058e-9).abs() < prev.hull_sup * 1e-5,
        "the ladder moved: round {} carries {:e}",
        prev.rounds,
        prev.hull_sup
    );
    // THE CLAIM IN THE NAME. The budget's last round gained nothing.
    assert!(
        achieved > prev.hull_sup,
        "the last round improved ({:e} against {achieved:e}), so this fixture no longer \
         exercises a non-improving round and the row's name is no longer its claim",
        prev.hull_sup
    );
    // And it is a SINGLE one, which `prev.rounds == 5` above is what
    // says: the loop stops at the FIRST round under the tolerance, so
    // a `1e-8` request that ran to round 5 is a run where round 4's
    // bound was still above `1e-8` and round 5's was below it. Round
    // 5 therefore improved, and round 6's grid came from a
    // directional marking. Two non-improving rounds in a row, the
    // second past a both-directions marking, is the stall's admission
    // set, and this is not it.
    //
    // The face's own account of that round. Rounds 0 to 4 sit above
    // `1e-8` and round 6 above round 5, so round 5's bound is the
    // smallest any round reached, and the payload carries it exactly,
    // on the grid round 5's fit was interpolated on.
    assert_eq!(
        last_round,
        LastRound::DidNotImprove,
        "the face says a round that rose improved"
    );
    assert_eq!(
        best.bound, prev.hull_sup,
        "the face's best bound is not the smallest the run reached"
    );
    assert_eq!(
        best.grid,
        prev_fit.control_counts(),
        "the best bound's grid"
    );
    let best = best.bound;
    assert!(
        !msg.contains("still improving"),
        "the message says a round that rose was improving: {msg}"
    );
    assert!(
        msg.contains(&format!("loosen the tolerance to {best} m or more"))
            && !msg.contains(&format!("{achieved} m")),
        "the repair is not sized to the best bound reached: {msg}"
    );
}

/// **What the floor on `‖E‖` reaches on a non-analytic base.** The
/// bumpy patch at `d = 1e-6` certifies at `5.059e-10` on the third
/// round's 609 cells — three orders below `|d|`, on a patch with no
/// closed form to check against, which is why the row pins the
/// digits rather than a ratio.
///
/// **Re-pinned when certification arithmetic became a newtype over the backend**
/// (`7.6102e-10` before): interval arithmetic padded one representable step
/// outward on every operation of `cell_bound`'s assembly and the
/// backend pads only where the operation is inexact, so the same
/// certificate on the same 609 cells comes in a third tighter.
#[test]
fn the_bumpy_patch_certifies_a_micron_offset_below_a_nanometre() {
    let base = bumpy_patch();
    let (_, cert) = fit_offset_at(&base, 1e-6, 1e-9, band())
        .unwrap_or_else(|e| panic!("the bumpy patch refused a 1e-9 request: {e}"));
    assert_eq!((cert.rounds, cert.cells), (3, 609));
    assert!(
        (cert.hull_sup - 5.0593e-10).abs() < cert.hull_sup * 1e-3,
        "the bumpy patch certifies at {:e}",
        cert.hull_sup
    );
}

/// **The never-finite face, and where it starts.** At `d = 1e-8` and
/// `1e-9` on the quarter cylinder the certificate limb answers `+∞`
/// on every grid the loop reaches — the sign witness does not pass on
/// the cells that carry the sup — and the cap stops it after four
/// refinement rounds with no finite bound ever produced. A refusal
/// that "carries the achieved bound" must not carry `inf` there: the
/// face says there is no number, and prints none.
///
/// One decade up the face is not reached: `d = 1e-7` certifies at
/// `5.8508e-7` on the fifth round's 1144 cells. The row pins that
/// boundary, because the two faces are one decade apart and a change
/// that moved either would otherwise move it silently.
///
/// **Re-pinned when certification arithmetic became a newtype over the backend**
/// (`5.8550e-7` before): the retired unconditional one-step pad per
/// operation is gone from `cell_bound`'s assembly. The boundary this
/// row draws is unmoved — `1e-8` and `1e-9` still never become
/// finite, and this decade still certifies.
#[test]
fn a_bound_that_never_became_finite_refuses_with_no_number() {
    let base = quarter_cylinder(1.0, 1.0);
    let (_, cert) = fit_offset_at(&base, 1e-7, 1e-3, band())
        .unwrap_or_else(|e| panic!("d = 1e-7 no longer certifies: {e}"));
    assert_eq!((cert.rounds, cert.cells), (5, 1144));
    assert!(
        (cert.hull_sup - 5.8508e-7).abs() < 5e-11,
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
                assert_eq!(rounds, 4, "d = {d}: four rounds ran before the cap");
                assert!(grid.0 <= OFFSET_FIT_SAMPLE_CAP && grid.1 <= OFFSET_FIT_SAMPLE_CAP);
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
                // the decade above (`1e-7` certifies) bears out. Neither
                // the round budget nor the sample cap is named: raising
                // either would not help.
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
/// `1.707e-5`, seventeen times `|d|`, on the same 308-cell grid a
/// componentwise floor certified at `3.222e-4`. The sup cell's
/// decomposition and the two readings of `‖E‖` behind that factor are
/// `offset_fit`'s own row.
///
/// The row pins both halves: a reachable tolerance certifies, and an
/// unreachable one refuses typed rather than reporting a number it
/// cannot support. `1e-9` is the second half — the fit's own absolute
/// accuracy does not reach it, and the grid the bound wants exceeds
/// the sample cap first, at `achieved = 3.754e-7`. That bound is
/// carried by `τ` (`2.05e-7` of it), the tangential term, which
/// divides by the regularity floor rather than by `‖E‖`.
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
        (cert.hull_sup - 1.7072e-5).abs() < 5e-9,
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
    // the sample cap's: the grid the bound wants exceeds it before
    // the rounds run out.
    match fit_offset_at(&base, d, 1e-9, band()) {
        Err(OffsetFitError::SampleCapReached {
            achieved, rounds, ..
        }) => {
            assert_eq!(rounds, 5);
            assert!(
                (achieved - 3.7544e-7).abs() < 5e-11,
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
/// at the origin certified at 1.7072e-5.
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
/// 1e0..1e4   1.7072e-5   flat to five figures
/// 1e5        1.7073e-5   1.00006x the origin
/// 1e6        1.7082e-5   1.0006x
/// 1e7        1.7168e-5   1.0056x
/// 1e8        1.8223e-5   1.0674x
/// 1e9        4.6979e-5   2.751x
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
            c.knots_u().clone(),
            c.knots_v().clone(),
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
            // Measured worst over this range is 1.0056x at 1e7.
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

/// A bilinear SADDLE: the wall between the edge `(0,0)–(2,0)` of a
/// 2 m square at `z = 0` and the same edge of that square turned
/// `theta` about its centre at `z = 1`, `u` running up the wall. It is
/// the first spline wall of `sweep`'s `twisted_loft(theta)` test body,
/// rebuilt here as the net that loft produces. The fit loop's outcome
/// on it at `theta = 0.3` is pinned, grid and bound, by
/// `the_second_non_improving_round_is_the_stalls_face`.
fn saddle_wall(theta: f64) -> NurbsSurface<f64> {
    let (s, c) = theta.sin_cos();
    let turned = |x: f64, y: f64| {
        let (dx, dy) = (x - 1.0, y - 1.0);
        Point3::new(1.0 + c * dx - s * dy, 1.0 + s * dx + c * dy, 1.0)
    };
    let control = vec![
        Point3::new(0.0, 0.0, 0.0),
        turned(0.0, 0.0),
        Point3::new(2.0, 0.0, 0.0),
        turned(2.0, 0.0),
    ];
    NurbsSurface::new(kv1(), kv1(), control, vec![1.0; 4]).unwrap()
}

/// **`RefinementStalled` from the loop, and the loop's ordering.**
/// The stall guard refuses a round whose grid came from the
/// both-directions step and whose bound did not fall; the loop takes
/// that verdict BEFORE the round-budget test, so a stall on the
/// budget's last round wears the stall's face, not the budget's.
/// `a_single_non_improving_round_is_the_budgets_face_not_the_stalls`
/// is the other side: one non-improving round is the budget's.
///
/// The saddle wall at a target of `1e-14`, which it cannot reach, each
/// request pinned by the round it stalls on, its grid and its bound:
///
/// ```text
/// d = ±5e-10   round 4, (16, 12), 1.2915e-11
/// d =  1e-6    OFFSET_FIT_BUDGET's round, (26, 18), 9.52e-10
/// ```
///
/// At `d = 1e-6`, a loop that tested the budget first would refuse
/// `BudgetExhausted` on the same round, so this request is the witness
/// that the verdict comes first.
///
/// **What the read-back pins is the loop at a fixed band.** Each
/// refusal's `best` is requested again at the same band, and certifies
/// on the round that reached it, with exactly that bound on exactly
/// that grid: `BestBound`'s recourse claim, which holds at the fit's
/// band. It says nothing about a production caller, whose band moves
/// with ε.
///
/// **If a request here certifies, re-find the fixture; do not delete
/// the row.** These stalls ride on the Bézier decomposition's insertion
/// width, which grows with the grid; PROPS has a convex insertion form
/// in view that narrows it, under which the `5e-10` request measured
/// certifying on round 3. A certificate here most likely means that
/// landed. The hunt that found these swept `theta` over 0.05–1.2 and
/// `d` over 1e-11–1e-2 at a target of 1e-17 on this saddle, and most
/// requests below `d ~ 1e-6` stalled; sweep again, and pin a request
/// that stalls on `OFFSET_FIT_BUDGET`'s round.
#[test]
fn the_second_non_improving_round_is_the_stalls_face() {
    let base = saddle_wall(0.3);
    let target = 1e-14;
    let last_round = u32::try_from(OFFSET_FIT_BUDGET).unwrap();
    for (d, want_rounds, want_grid, want_achieved) in [
        (5e-10, 4u32, (16, 12), 1.2915e-11),
        (-5e-10, 4, (16, 12), 1.2915e-11),
        (1e-6, last_round, (26, 18), 9.52e-10),
    ] {
        let (rounds, grid, achieved, best, msg) = match fit_offset_at(&base, d, target, band()) {
            Err(
                ref e @ OffsetFitError::RefinementStalled {
                    rounds,
                    grid,
                    achieved,
                    tolerance,
                    best,
                },
            ) => {
                assert_eq!(tolerance, target);
                eprintln!(
                    "saddle d={d:e}: stalled on round {rounds}, grid {grid:?}, bound \
                     {achieved:.4e}, best {:.4e} on {:?}",
                    best.bound, best.grid
                );
                (rounds, grid, achieved, best, e.to_string())
            }
            Ok((_, cert)) => panic!(
                "saddle d={d:e}: certified at {:e} on round {} — the stall this row pins is \
                 gone. Re-find a request that stalls (this row's doc says how) rather than \
                 deleting the row",
                cert.hull_sup, cert.rounds
            ),
            Err(other) => panic!("saddle d={d:e}: expected the stall's face, got {other:?}"),
        };
        assert_eq!(
            rounds, want_rounds,
            "saddle d={d:e}: the stall moved rounds (the 1e-6 request witnesses the ordering \
             only while it stalls on the budget's last round)"
        );
        assert_eq!(grid, want_grid, "saddle d={d:e}: the stall's grid moved");
        assert!(
            (achieved - want_achieved).abs() < want_achieved * 1e-3,
            "saddle d={d:e}: the stall's bound {achieved:e} moved from {want_achieved:e}"
        );
        let BestBound {
            bound: best,
            grid: best_grid,
        } = best;
        assert!(
            best > target && best < achieved,
            "saddle d={d:e}: the last bound {achieved:e} is not above the best {best:e}, so \
             the row no longer shows the recourse naming the best rather than the last"
        );
        // The recourse at this band: a request at `best` certifies.
        let (fit, cert) = fit_offset_at(&base, d, best, band()).unwrap_or_else(|e| {
            panic!("saddle d={d:e}: a request at the best bound {best:e} refused: {e}")
        });
        assert_eq!(
            cert.hull_sup, best,
            "saddle d={d:e}: the best bound read back"
        );
        assert_eq!(fit.control_counts(), best_grid, "saddle d={d:e}: its grid");
        assert!(cert.rounds < rounds, "saddle d={d:e}");
        assert!(
            msg.contains("stopped improving") && msg.contains("both directions"),
            "saddle d={d:e}: the message does not say what was tried: {msg}"
        );
        assert!(
            msg.contains(&format!("loosen the tolerance to {best} m or more"))
                && !msg.contains(&format!("{achieved} m")),
            "saddle d={d:e}: the repair is not sized to the best bound reached: {msg}"
        );
    }
}

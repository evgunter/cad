//! The chain→curve composition door (LIB-U4A-DOOR): the exactness
//! differential rows, the per-seam refusal rows, and the s_duct
//! oracle measurement.
//!
//! The door's contract lives in `geom_curves::compose`'s module docs;
//! this suite is what pins it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::spline::KnotVector;
use geom_core::{Band, Point3, Vec3};
use geom_curves::NurbsCurve3;
use geom_curves::compose::{ComposeError, SeamSide, compose_chain};

/// A pure band, per the geom-core test discipline (never
/// `Band::linear()` in a test: it forces the global `Tolerance`).
fn band() -> Band {
    Band::new(1e-9, 1e-8).unwrap()
}

/// The **locus-exact rational-quadratic chain** of a circular arc
/// (Book §7.3), written here exactly as `geom-brep`'s private
/// `rational_arc_chain` writes it: ≤ 90° Bézier segments, middle
/// weight `cos(θ/2)`, middle point the tangent intersection. This is
/// the "exact curve piece" the door is meant to consume; it is
/// test-local because the shipped copy is private and downstream (see
/// the unit's finding 1).
fn exact_arc(
    center: Point3<f64>,
    axis: Vec3<f64>,
    radius: f64,
    u_ref: Vec3<f64>,
    t0: f64,
    t1: f64,
) -> NurbsCurve3<f64> {
    let span = t1 - t0;
    assert!(span > 0.0 && span.is_finite());
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let n = (span / core::f64::consts::FRAC_PI_2).ceil().max(1.0) as usize;
    let cv = axis.cross(u_ref);
    let at = |t: f64| -> Point3<f64> {
        let (s, c) = t.sin_cos();
        center + (u_ref * c + cv * s) * radius
    };
    let seg = span / n as f64;
    let w_mid = (seg / 2.0).cos();
    let mut control = Vec::with_capacity(2 * n + 1);
    let mut weights = Vec::with_capacity(2 * n + 1);
    let mut knots = Vec::with_capacity(2 * n + 4);
    knots.extend([t0, t0, t0]);
    control.push(at(t0));
    weights.push(1.0);
    for i in 0..n {
        let a = t0 + seg * i as f64;
        let b = if i + 1 == n {
            t1
        } else {
            t0 + seg * (i + 1) as f64
        };
        let m = 0.5 * (a + b);
        let (s, c) = m.sin_cos();
        control.push(center + (u_ref * c + cv * s) * (radius / w_mid));
        weights.push(w_mid);
        control.push(at(b));
        weights.push(1.0);
        if i + 1 == n {
            knots.extend([b, b, b]);
        } else {
            knots.extend([b, b]);
        }
    }
    NurbsCurve3::new(KnotVector::clamped(knots, 2).unwrap(), control, weights).unwrap()
}

/// The exact degree-1 piece of a straight leg.
fn exact_line(a: Point3<f64>, b: Point3<f64>) -> NurbsCurve3<f64> {
    NurbsCurve3::new(
        KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
        vec![a, b],
        vec![1.0, 1.0],
    )
    .unwrap()
}

/// The same arc's chain rescaled onto `[0, 1]` — what the door's own
/// normalization produces, so the differential row compares like with
/// like.
fn on_unit_interval(curve: &NurbsCurve3<f64>) -> Vec<f64> {
    let (lo, hi) = curve.domain();
    curve
        .knots()
        .knots()
        .iter()
        .map(|k| (k - lo) / (hi - lo))
        .collect()
}

fn xz_circle_frame() -> (Vec3<f64>, Vec3<f64>) {
    (Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0))
}

// ---------------------------------------------------------------
// 1. The exactness differential rows
// ---------------------------------------------------------------

/// **The pinned exactness row.** Composing the two quarter arcs of a
/// semicircle reproduces the semicircle's own rational-quadratic chain
/// BIT FOR BIT on all three channels — control points, weights, and
/// (after the door's stated `[0, 1]` normalization) knots. No
/// sampling, no fitting, no drift: the door is the anti-interpolate.
#[test]
fn compose_two_quarter_arcs_is_the_exact_half_arc() {
    let (axis, u_ref) = xz_circle_frame();
    let c = Point3::new(0.0, 0.0, 0.0);
    let q1 = exact_arc(c, axis, 1.0, u_ref, 0.0, core::f64::consts::FRAC_PI_2);
    let q2 = exact_arc(
        c,
        axis,
        1.0,
        u_ref,
        core::f64::consts::FRAC_PI_2,
        core::f64::consts::PI,
    );
    let half = exact_arc(c, axis, 1.0, u_ref, 0.0, core::f64::consts::PI);

    let composed = compose_chain(&[q1, q2], band()).expect("the two quarter arcs compose");

    assert_eq!(
        composed.degree(),
        2,
        "the rational-quadratic degree survives"
    );
    assert_eq!(
        composed.knots().knots(),
        on_unit_interval(&half).as_slice(),
        "knots agree bit for bit with the half arc's own chain on [0, 1]"
    );
    assert_eq!(
        composed.weights(),
        half.weights(),
        "weights are the legs' verbatim"
    );
    for (index, (got, want)) in composed.control().iter().zip(half.control()).enumerate() {
        assert!(
            got.x.to_bits() == want.x.to_bits()
                && got.y.to_bits() == want.y.to_bits()
                && got.z.to_bits() == want.z.to_bits(),
            "control point {index} is not bit-identical: {got:?} vs {want:?}"
        );
    }
}

/// The seam knot keeps multiplicity `p` — exactly what the canonical
/// arc chain carries at its own 90° joint. The C¹ that holds there is
/// a fact about the control data, and the door never moves a control
/// point to buy knot-structural continuity.
#[test]
fn the_seam_knot_keeps_full_multiplicity() {
    let (axis, u_ref) = xz_circle_frame();
    let c = Point3::new(0.0, 0.0, 0.0);
    let composed = compose_chain(
        &[
            exact_arc(c, axis, 1.0, u_ref, 0.0, core::f64::consts::FRAC_PI_2),
            exact_arc(
                c,
                axis,
                1.0,
                u_ref,
                core::f64::consts::FRAC_PI_2,
                core::f64::consts::PI,
            ),
        ],
        band(),
    )
    .unwrap();
    let (mult, last) = composed
        .knots()
        .multiplicity_of(0.5)
        .expect("the seam knot is present");
    assert!(last > composed.degree(), "the seam is interior");
    assert_eq!(
        mult,
        composed.degree(),
        "seam multiplicity is p, as the arc chain's own joint is"
    );
}

/// The mixed-degree normalization, stated: a degree-1 line leg joined
/// to a degree-2 arc leg is raised through the crate's KnotAlgebra
/// route, so the composed net is the ELEVATED legs' data — not the
/// authored legs'. The LOCUS is untouched, which is what the row
/// measures.
#[test]
fn mixed_degree_legs_elevate_and_stay_on_locus() {
    let (axis, u_ref) = xz_circle_frame();
    let c = Point3::new(0.0, 0.0, 0.0);
    // A tangent line arriving at (1, 0, 0) along +x, then the quarter
    // arc that leaves (1, 0, 0) along +y... the arc's start tangent at
    // theta = 0 is +y (right-hand rule about +z), so the line must
    // arrive along +y too.
    let arc = exact_arc(c, axis, 1.0, u_ref, 0.0, core::f64::consts::FRAC_PI_2);
    let line = exact_line(Point3::new(1.0, -2.0, 0.0), Point3::new(1.0, 0.0, 0.0));

    let composed = compose_chain(&[line, arc], band()).expect("line + arc composes");
    assert_eq!(composed.degree(), 2, "the chain rises to the arc's degree");

    // Every sample on the arc half of the composed curve lies on the
    // unit circle; every sample on the line half lies on x = 1.
    let mut worst_arc = 0.0_f64;
    let mut worst_line = 0.0_f64;
    for i in 0..=200 {
        let u = f64::from(i) / 200.0;
        let p = composed.eval(u);
        if p.y >= 0.0 {
            worst_arc = worst_arc.max(((p.x * p.x + p.y * p.y).sqrt() - 1.0).abs());
        } else {
            worst_line = worst_line.max((p.x - 1.0).abs());
        }
        assert!(p.z.abs() < 1e-15, "the chain stays in the z = 0 plane");
    }
    assert!(
        worst_arc < 1e-14,
        "arc half stays on the exact circle: {worst_arc:e}"
    );
    assert!(
        worst_line < 1e-14,
        "line half stays on x = 1: {worst_line:e}"
    );
}

/// **C¹, measured.** The door equalizes the one-sided derivatives at
/// every seam by its choice of parameter intervals, so the composed
/// curve's derivative is continuous across the seam — not merely its
/// direction (G¹), which is all the input guarantees.
#[test]
fn the_join_is_c1_not_merely_g1() {
    let (axis, u_ref) = xz_circle_frame();
    let c = Point3::new(0.0, 0.0, 0.0);
    // Deliberately unequal natural parameterizations: the line leg is
    // 3 m long over [0, 1], the arc leg is a unit quarter over
    // [0, pi/2]. Only the door's rescaling can make this C¹.
    let line = exact_line(Point3::new(1.0, -3.0, 0.0), Point3::new(1.0, 0.0, 0.0));
    let arc = exact_arc(c, axis, 1.0, u_ref, 0.0, core::f64::consts::FRAC_PI_2);
    let composed = compose_chain(&[line, arc], band()).unwrap();

    let seam = composed
        .knots()
        .knots()
        .iter()
        .copied()
        .find(|k| *k > 0.0 && *k < 1.0)
        .expect("there is an interior seam knot");
    let h = 1e-7;
    let left = composed.deriv(seam - h);
    let right = composed.deriv(seam + h);
    let jump = (right - left).norm() / left.norm();
    assert!(
        jump < 1e-5,
        "the derivative is continuous across the seam (relative jump {jump:e})"
    );

    // And the C⁰ side, exactly: one control point carries the
    // junction, so the two one-sided limits are the same point.
    let gap = (composed.eval(seam + h) - composed.eval(seam - h)).norm();
    assert!(
        gap < 1e-6,
        "the position is continuous across the seam ({gap:e})"
    );
}

// ---------------------------------------------------------------
// 2. The per-seam refusal rows
// ---------------------------------------------------------------

#[test]
fn empty_chain_refuses() {
    assert_eq!(
        compose_chain(&[], band()).unwrap_err(),
        ComposeError::EmptyChain
    );
}

/// A definitely-separated pair refuses, naming the seam. This is the
/// backstop, not the C⁰ mechanism: it only ever refuses, and never
/// certifies coincidence.
#[test]
fn a_definite_gap_refuses_naming_the_seam() {
    let a = exact_line(Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0));
    let b = exact_line(Point3::new(1.5, 0.0, 0.0), Point3::new(3.0, 0.0, 0.0));
    match compose_chain(&[a, b], band()).unwrap_err() {
        ComposeError::SeamGap { seam, gap } => {
            assert_eq!(seam, 0);
            assert!(
                (gap - 0.5).abs() < 1e-15,
                "the refusal reports the separation: {gap}"
            );
        }
        other => panic!("expected SeamGap, got {other:?}"),
    }
}

/// A corner refuses as a non-tangent seam, naming the seam index and
/// the perpendicular deviation the predicate measured.
#[test]
fn a_corner_refuses_as_a_non_tangent_seam() {
    let a = exact_line(Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0));
    let b = exact_line(Point3::new(1.0, 0.0, 0.0), Point3::new(1.0, 1.0, 0.0));
    match compose_chain(&[a, b], band()).unwrap_err() {
        ComposeError::SeamNotTangent { seam, deviation } => {
            assert_eq!(seam, 0);
            assert!(
                deviation > 0.5,
                "a right-angle corner is grossly non-tangent: {deviation}"
            );
        }
        other => panic!("expected SeamNotTangent, got {other:?}"),
    }
}

/// A doubling-back chain is tangent but anti-parallel: a cusp, refused
/// by its own property so the caller can tell the two apart.
#[test]
fn a_cusp_refuses_as_a_reversed_seam() {
    let a = exact_line(Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0));
    let b = exact_line(Point3::new(1.0, 0.0, 0.0), Point3::new(0.0, 0.0, 0.0));
    assert_eq!(
        compose_chain(&[a, b], band()).unwrap_err(),
        ComposeError::SeamTangentReversed { seam: 0 }
    );
}

/// The refusal names the RIGHT seam in a longer chain.
#[test]
fn the_refusal_names_the_offending_seam_in_a_long_chain() {
    let a = exact_line(Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0));
    let b = exact_line(Point3::new(1.0, 0.0, 0.0), Point3::new(2.0, 0.0, 0.0));
    let c = exact_line(Point3::new(2.0, 0.0, 0.0), Point3::new(2.0, 0.0, 1.0));
    match compose_chain(&[a, b, c], band()).unwrap_err() {
        ComposeError::SeamNotTangent { seam, .. } => assert_eq!(seam, 1),
        other => panic!("expected SeamNotTangent at seam 1, got {other:?}"),
    }
}

/// A leg with a repeated end control point has no direction to verify
/// there — refused by side, not silently normalized.
#[test]
fn a_repeated_end_control_point_refuses_by_side() {
    let a = NurbsCurve3::new(
        KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap(),
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
        ],
        vec![1.0, 1.0, 1.0],
    )
    .unwrap();
    let b = exact_line(Point3::new(1.0, 0.0, 0.0), Point3::new(2.0, 0.0, 0.0));
    assert_eq!(
        compose_chain(&[a, b], band()).unwrap_err(),
        ComposeError::SeamTangentUndefined {
            seam: 0,
            side: SeamSide::Incoming
        }
    );
}

/// A single leg is a chain: it composes to itself on `[0, 1]`, with
/// its control net and weights untouched.
#[test]
fn a_single_leg_composes_to_itself_reparameterized() {
    let (axis, u_ref) = xz_circle_frame();
    let arc = exact_arc(
        Point3::new(0.0, 0.0, 0.0),
        axis,
        1.0,
        u_ref,
        0.0,
        core::f64::consts::FRAC_PI_2,
    );
    let composed = compose_chain(core::slice::from_ref(&arc), band()).unwrap();
    assert_eq!(composed.domain(), (0.0, 1.0));
    assert_eq!(composed.weights(), arc.weights());
    assert_eq!(composed.control().len(), arc.control().len());
}

// ---------------------------------------------------------------
// 3. The s_duct oracle (READ-ONLY use of the demo's intent)
// ---------------------------------------------------------------

/// The S path's two exact quarter arcs, as `demos/tour/src/skinned.rs`
/// (`s_duct`) MEANS them: radius `S_R` in the world `x = 0` plane,
/// tangent running +z → +y → +z. The demo itself is untouched; this is
/// its intent rebuilt in the door's own vocabulary.
fn s_duct_intent(radius: f64) -> [NurbsCurve3<f64>; 2] {
    [
        exact_arc(
            Point3::new(0.0, radius, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            radius,
            Vec3::new(0.0, -1.0, 0.0),
            0.0,
            core::f64::consts::FRAC_PI_2,
        ),
        exact_arc(
            Point3::new(0.0, radius, 2.0 * radius),
            Vec3::new(1.0, 0.0, 0.0),
            radius,
            Vec3::new(0.0, 0.0, -1.0),
            0.0,
            core::f64::consts::FRAC_PI_2,
        ),
    ]
}

/// The demo's own 17-point sampling, verbatim in shape.
fn s_duct_samples(radius: f64) -> Vec<Point3<f64>> {
    (0..=8)
        .map(|k| {
            let th = core::f64::consts::FRAC_PI_2 * f64::from(k) / 8.0;
            Point3::new(0.0, radius * (1.0 - th.cos()), radius * th.sin())
        })
        .chain((1..=8).map(|k| {
            let ph = core::f64::consts::FRAC_PI_2 * f64::from(k) / 8.0;
            Point3::new(
                0.0,
                radius + radius * ph.sin(),
                2.0 * radius - radius * ph.cos(),
            )
        }))
        .collect()
}

/// Distance from a point to the S path's exact locus (the nearer of
/// the two carrier circles, both in the `x = 0` plane), in metres.
fn off_locus(p: Point3<f64>, radius: f64) -> f64 {
    let lower = ((p.y - radius).powi(2) + p.z.powi(2)).sqrt() - radius;
    let upper = ((p.y - radius).powi(2) + (p.z - 2.0 * radius).powi(2)).sqrt() - radius;
    p.x.abs() + lower.abs().min(upper.abs())
}

/// **The s_duct oracle, quantified (P2).** The demo builds its S path
/// by sampling the intended shape at 17 points and interpolating at
/// degree 3. That interpolation is NOT the S: it leaves the exact
/// locus by a measurable amount that scales with the sampling. The
/// door's composition of the same intent is on the locus to rounding.
///
/// The row asserts the ORDERING (the composed curve is orders of
/// magnitude closer) and pins both numbers so the future U4a-proper
/// migration has its before/after on the record.
#[test]
fn s_duct_intent_beats_its_interpolation_by_orders_of_magnitude() {
    let radius = 2.0; // demos/tour/src/skinned.rs::S_R
    let composed = compose_chain(&s_duct_intent(radius), band()).expect("the S intent composes");
    let interpolated =
        NurbsCurve3::interpolate(&s_duct_samples(radius), 3).expect("the S path interpolates");

    let mut worst_exact = 0.0_f64;
    let mut worst_interp = 0.0_f64;
    let mut worst_at = 0.0_f64;
    // The inflection is where the intended path's curvature JUMPS from
    // +1/R to −1/R. A degree-3 interpolant cannot carry a curvature
    // jump, so the error concentrates there; the away-from-inflection
    // figure is the sampling error proper, and the two are worth
    // reporting apart.
    let mut worst_away = 0.0_f64;
    for i in 0..=400 {
        let u = f64::from(i) / 400.0;
        let (clo, chi) = composed.domain();
        let (ilo, ihi) = interpolated.domain();
        worst_exact = worst_exact.max(off_locus(composed.eval(clo + (chi - clo) * u), radius));
        let here = off_locus(interpolated.eval(ilo + (ihi - ilo) * u), radius);
        if here > worst_interp {
            worst_interp = here;
            worst_at = u;
        }
        if (u - 0.5).abs() > 0.1 {
            worst_away = worst_away.max(here);
        }
    }
    println!(
        "s_duct off-locus (R = {radius} m, 17 samples, degree 3): exact {worst_exact:e} m; \
         interpolated {worst_interp:e} m (peak at u = {worst_at}), {worst_away:e} m away from the inflection"
    );

    // Recorded for the design lane. The exact composition is on its
    // locus to rounding; the demo's interpolation is millimetres off
    // a 2 m-radius duct, concentrated at the inflection. Both bounds
    // are ceilings/floors on the measured values, so the row fails
    // loudly if either lane moves.
    assert!(
        worst_exact < 1e-14,
        "the composed S is on its locus to rounding: {worst_exact:e}"
    );
    assert!(
        worst_interp > 1e-3,
        "the interpolation's error is real, not noise: {worst_interp:e}"
    );
    assert!(
        worst_interp < 1e-2,
        "the interpolation's error is bounded as recorded: {worst_interp:e}"
    );
    assert!(
        (worst_at - 0.5).abs() < 0.1,
        "the interpolation's peak error sits at the inflection, as the curvature-jump \
         argument predicts (peak at u = {worst_at})"
    );
    assert!(
        worst_interp / worst_exact > 1e10,
        "exact composition beats interpolation by >= 10 orders: {worst_exact:e} vs {worst_interp:e}"
    );
}

/// The composed S path is a legal sweep spine: never reversed, so the
/// path-following frame the sweep consumer builds is total (the demo's
/// own stated requirement for this path).
#[test]
fn the_composed_s_path_never_reverses() {
    let composed = compose_chain(&s_duct_intent(2.0), band()).unwrap();
    let mut previous: Option<Vec3<f64>> = None;
    for i in 0..=200 {
        let u = f64::from(i) / 200.0;
        let d = composed.deriv(u);
        assert!(d.norm() > 1e-9, "the spine never stalls at u = {u}");
        if let Some(prev) = previous {
            assert!(d.dot(prev) > 0.0, "the tangent never reverses at u = {u}");
        }
        previous = Some(d);
    }
}

//! Adversarial e2e review artifact for M2 PR 2 (profile crate,
//! 2026-07-18), promoted from the reviewer's session worktree into CI
//! per the standing convention. These are **independent derivations** —
//! AutoCAD's bulge/center/apex formulas re-derived by hand, ulp-tied
//! canonicalization attacks, hand-solved ray/arc parity, 16-spoke
//! graze-exhaustion alignment, far-from-origin shoelace probes — do not
//! "simplify" them to match shipped fixtures; the independence is the
//! regression value.
//!
//! Promotion adaptations (mechanical, plus one semantic): the header,
//! and `hair_thin_near_full_arc_is_refused_but_mislabeled` updated to
//! the fix pass's behavior — the reviewer's SHOULD-2 finding (a
//! false-Zero arc_span regime on near-full arcs mislabeling the
//! rejection as NonSimple::Touch) is now the typed
//! `ProfileError::NearFullArc` rejection from the
//! `arc_diameter_clearance` gate; the original finding is preserved in
//! that test's comment. Everything else is verbatim.
//!
//! The K-hook section (the two `Probe` delegation pins) lives in
//! `review_m2_pr2_probe.rs` behind the `probe` feature; everything
//! here is f64 (plus the interval lane) and runs in the default
//! build.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unreachable
)]

mod common;

use common::{chain, profile, quarter_bulge, rect, tol};
// `lift` re-instantiates a fixture at another scalar; the only
// remaining consumer here is the interval-lane totality test.
#[cfg(feature = "interval")]
use common::lift;
use geom_core::{Point2, Sign};
use profile::RawLoop;
use profile::{
    ArcSweep, ContactKind, LoopRole, ProfileError, ProfileLoop, ProfileVertex, SegmentKind,
    SegmentRef, ValidatedProfile, bulge_from_center, bulge_from_via,
};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn sref(loop_index: usize, segment_index: usize) -> SegmentRef {
    SegmentRef {
        loop_index,
        segment_index,
    }
}

fn ok(p: &profile::Profile<f64>) -> ValidatedProfile<f64> {
    p.validate(tol()).expect("fixture must validate")
}

fn err(p: &profile::Profile<f64>) -> ProfileError {
    p.validate(tol()).expect_err("fixture must be rejected")
}

// ---------------------------------------------------------------- DXF --

/// True-DXF check: positive bulge = CCW sweep; for the quarter arc
/// (0,0) -> (2,0) with b = tan(pi/8) the center must be at (1, 1)
/// (LEFT of the chord) and the arc must bow DOWN (apex right of
/// travel). Numbers hand-derived from the AutoCAD bulge formulas
/// (theta = 4 atan b, r = L / (2 sin(theta/2)),
/// center = polar(P1, angle(P1->P2) + (pi - theta)/2, r)).
#[test]
fn dxf_quarter_arc_center_left_apex_right() {
    let b = quarter_bulge();
    // Embed the arc in a valid loop: quarter arc bows down below y=0,
    // rest of the loop well above.
    let lp = chain(&[
        (0.0, 0.0, b),
        (2.0, 0.0, 0.0),
        (2.0, 2.0, 0.0),
        (0.0, 2.0, 0.0),
    ]);
    let vp = ok(&profile(vec![lp]));
    let seg0 = vp.loops()[0].segments()[0];
    match seg0.kind {
        SegmentKind::Arc {
            center,
            radius,
            turn,
        } => {
            // Hand values: L = 2, r = L(1+b^2)/(4b) = sqrt(2),
            // apothem = L(1-b^2)/(4b) = 1 -> center = (1, 1).
            assert!((center.x - 1.0).abs() < 1e-12, "center.x = {}", center.x);
            assert!((center.y - 1.0).abs() < 1e-12, "center.y = {}", center.y);
            assert!((radius - std::f64::consts::SQRT_2).abs() < 1e-12);
            assert_eq!(turn, Sign::Positive, "positive bulge = CCW sweep");
        }
        SegmentKind::Line => panic!("quarter arc must classify as an arc"),
    }
    // The apex bows RIGHT of the chord direction (+x travel => below):
    // interior of this loop contains (1, 1) but NOT (1, -0.2) if the
    // arc bowed up... instead assert directly: the arc's lowest point
    // is (1, 1 - sqrt(2)) ~ (1, -0.414); the loop must be simple with
    // the rest of the boundary at y >= 0, which validation just
    // certified. Check the apex via the sagitta identity s = L*b/2.
    let sagitta = 2.0 * quarter_bulge() / 2.0;
    assert!((sagitta - (std::f64::consts::SQRT_2 - 1.0)).abs() < 1e-15);
}

/// Major-arc via form: the arc (1,0) -> (0,1) THROUGH (0,-1) is the
/// clockwise major arc of the unit circle: theta = -3pi/2,
/// b = tan(-3pi/8).
#[test]
fn bulge_from_via_major_arc_sign() {
    let b = bulge_from_via(p2(1.0, 0.0), p2(0.0, -1.0), p2(0.0, 1.0));
    let want = (-3.0 * std::f64::consts::FRAC_PI_8).tan();
    assert!((b - want).abs() < 1e-12, "b = {b}, want {want}");
}

/// The +1/+1 two-arc circle is CCW: its interior must contain the
/// center, and reversing the loop (all bulges -1) canonicalizes back to
/// the same bytes (winding invisible).
#[test]
fn two_arc_circle_is_ccw_and_winding_invisible() {
    let circle = chain(&[(-1.0, 0.0, 1.0), (1.0, 0.0, 1.0)]);
    // A tiny hole at the center proves the interior is where we think:
    // containment must classify the hole at depth 1.
    let hole = rect(-0.1, -0.1, 0.2, 0.2);
    let vp = ok(&profile(vec![circle.clone(), hole.clone()]));
    assert_eq!(vp.loops()[0].role(), LoopRole::Outer);
    assert_eq!(vp.loops()[1].role(), LoopRole::Hole);
    let vp_rev = ok(&profile(vec![circle.reversed(), hole]));
    assert_eq!(format!("{vp:?}"), format!("{vp_rev:?}"));
}

// --------------------------------------------- canonicalization attacks --

/// 1-ulp-separated lex-min candidates: two leftmost vertices with
/// x = 1.0 and x = 1.0 + ulp. The exact-order band must order them
/// definitely and rotation/reversal-invariantly.
#[test]
fn one_ulp_lex_min_tie_is_deterministic() {
    let x_lo = 1.0f64;
    let x_hi = 1.0f64.next_up(); // 1 + 2^-52
    let base = ProfileLoop::polygon([p2(x_lo, 0.0), p2(3.0, 0.0), p2(3.0, 2.0), p2(x_hi, 2.0)]);
    let canon = ok(&profile(vec![base.clone()]));
    let v0 = canon.loops()[0].vertices()[0].pos();
    assert_eq!(v0.x.to_bits(), x_lo.to_bits(), "lex-min must be x = 1.0");
    for r in 0..4 {
        for reversed in [false, true] {
            let n = base.vertices().len();
            let rotated = ProfileLoop::new(
                (0..n)
                    .map(|k| base.vertices()[(r + k) % n])
                    .collect::<Vec<_>>(),
            );
            let lp = if reversed {
                rotated.reversed()
            } else {
                rotated
            };
            let vp = ok(&profile(vec![lp]));
            assert_eq!(
                format!("{canon:?}"),
                format!("{vp:?}"),
                "rot {r} rev {reversed}"
            );
        }
    }
}

/// Symmetric square centered at the origin: automorphisms do not break
/// canonical-start uniqueness (vertices are distinct points).
#[test]
fn origin_centered_square_canonicalizes_uniquely() {
    let base = ProfileLoop::polygon([p2(-1.0, -1.0), p2(1.0, -1.0), p2(1.0, 1.0), p2(-1.0, 1.0)]);
    let canon = ok(&profile(vec![base.clone()]));
    let v0 = canon.loops()[0].vertices()[0].pos();
    assert_eq!((v0.x, v0.y), (-1.0, -1.0));
    for r in 0..4 {
        for reversed in [false, true] {
            let n = base.vertices().len();
            let rotated = ProfileLoop::new(
                (0..n)
                    .map(|k| base.vertices()[(r + k) % n])
                    .collect::<Vec<_>>(),
            );
            let lp = if reversed {
                rotated.reversed()
            } else {
                rotated
            };
            let vp = ok(&profile(vec![lp]));
            assert_eq!(format!("{canon:?}"), format!("{vp:?}"));
        }
    }
}

/// A loop that revisits a coordinate exactly (pinch at a bit-identical
/// non-adjacent vertex) is rejected by simplicity, so validated loops
/// can never contain two bit-identical vertices and lex-min stays
/// unique.
#[test]
fn self_pinch_at_repeated_vertex_is_rejected() {
    // Hourglass revisiting (1, 1).
    let p = profile(vec![chain(&[
        (0.0, 0.0, 0.0),
        (1.0, 1.0, 0.0),
        (2.0, 0.0, 0.0),
        (2.5, 2.0, 0.0),
        (1.0, 1.0, 0.0),
        (0.0, 2.0, 0.0),
    ])]);
    match err(&p) {
        ProfileError::NonSimple {
            kind: ContactKind::Touch,
            first,
            second,
        } => {
            assert_eq!(first.loop_index, 0);
            assert_eq!(second.loop_index, 0);
        }
        other => panic!("expected a pinch touch, got {other:?}"),
    }
}

// ---------------------------------------------------- simplicity attacks --

/// Two non-adjacent arcs crossing twice (lens crossing): both
/// intersection points interior to the same carrier pair.
#[test]
fn lens_crossing_arcs_are_a_crossing() {
    // seg0: (0,0)->(4,0) b=-1, upper semicircle (center (2,0), r=2).
    // seg2: (4,3)->(0,3) b=-1, lower semicircle (center (2,3), r=2).
    // Carriers meet at (2 +- sqrt(1.75), 1.5), both interior to both.
    let p = profile(vec![chain(&[
        (0.0, 0.0, -1.0),
        (4.0, 0.0, 0.0),
        (4.0, 3.0, -1.0),
        (0.0, 3.0, 0.0),
    ])]);
    assert_eq!(
        err(&p),
        ProfileError::NonSimple {
            first: sref(0, 0),
            second: sref(0, 2),
            kind: ContactKind::Crossing,
        }
    );
}

/// Partial collinear edge overlap between two loops, with no earlier
/// contact pair: must be Overlap, naming the overlapping pair.
#[test]
fn partial_edge_overlap_between_loops() {
    let outer = rect(0.0, 0.0, 2.0, 2.0);
    // Loop 1 starts with the segment lying ON outer's right edge
    // (x = 2, y in [0.5, 1.5]) so the overlap pair judges first.
    let bump = ProfileLoop::polygon([p2(2.0, 0.5), p2(2.0, 1.5), p2(3.0, 1.5), p2(3.0, 0.5)]);
    assert_eq!(
        err(&profile(vec![outer, bump])),
        ProfileError::NonSimple {
            first: sref(0, 1),
            second: sref(1, 0),
            kind: ContactKind::Overlap,
        }
    );
}

/// Cocircular partial arc overlap across loops: a second loop rides a
/// quarter of the lens's semicircular carrier.
#[test]
fn cocircular_partial_arc_overlap() {
    let lens = common::lens(); // semicircle (0,0)->(2,0), center (1,0), r=1
    // Points on the same carrier at angles 250 and 290 degrees.
    let at = |deg: f64| {
        let (s, c) = deg.to_radians().sin_cos();
        p2(1.0 + c, s)
    };
    let a = at(250.0);
    let b = at(290.0);
    // Two vertices: `a` leaves along the shared carrier to `b`, and `b`
    // closes back on the straight chord.
    let riding = <ProfileLoop<f64> as RawLoop<f64>>::new(vec![
        ProfileVertex::new(a, bulge_from_center(a, b, p2(1.0, 0.0), ArcSweep::Ccw)),
        ProfileVertex::new(b, 0.0),
    ]);
    match err(&profile(vec![lens, riding])) {
        ProfileError::NonSimple {
            kind: ContactKind::Overlap,
            first,
            second,
        } => {
            assert_eq!(first, sref(0, 0));
            assert_eq!(second.loop_index, 1);
        }
        other => panic!("expected cocircular overlap, got {other:?}"),
    }
}

/// Externally tangent separate loops: tangential contact, refused.
#[test]
fn externally_tangent_loops_are_tangential_contact() {
    // Unit circles centered (0,0) and (2,0), tangent at (1,0); both
    // split vertically so (1,0) is interior to arcs of both.
    let c1 = common::circle_v(0.0, 0.0, 1.0);
    let c2 = common::circle_v(2.0, 0.0, 1.0);
    match err(&profile(vec![c1, c2])) {
        ProfileError::TangentialContact { .. } => {}
        other => panic!("expected tangential contact, got {other:?}"),
    }
}

// ----------------------------------------------------- join smoothness --

/// The PR 3 handoff claim, attacked: a NEAR-tangent line->arc join
/// must escalate at validation (carrier_line_circle in-band), so the
/// dihedral predicate only ever sees definitely-smooth or
/// definitely-corner joins.
#[test]
fn near_tangent_join_escalates() {
    let eps = tol().eps();
    // Quarter arc leaving (2,0) with chord rotated by phi off the
    // exact-tangency direction (45 deg): carrier clearance to the
    // incoming line y=0 is r(1 - cos phi) ~ phi^2/2 with r = 1.
    let b = quarter_bulge();
    let build = |phi: f64, declare: bool| {
        let l = std::f64::consts::SQRT_2;
        let ang = std::f64::consts::FRAC_PI_4 + phi;
        let end = p2(2.0 + l * ang.cos(), l * ang.sin());
        // Joint 1 is the line->arc join under test. At phi = 0 the
        // vertical exit line x = end.x is tangent to the SAME carrier
        // at the arc's end -- joint 2, a second tangent joint, declared
        // under the same flag.
        let mut lp = chain(&[
            (0.0, 0.0, 0.0),
            (2.0, 0.0, b),
            (end.x, end.y, 0.0),
            (end.x, 3.0, 0.0),
            (0.0, 3.0, 0.0),
        ]);
        if declare {
            lp = lp.with_tangent_joints(vec![1, 2]);
        }
        profile(vec![lp])
    };
    // phi = 0: exact carrier tangency at the shared vertex -> smooth
    // join, accepted when DECLARED (#101: tangency is declared intent,
    // verified never trusted)...
    assert!(
        build(0.0, true).validate(tol()).is_ok(),
        "declared exact tangency accepts"
    );
    // ...and refused typed when the same exact tangency is undeclared.
    match build(0.0, false)
        .validate(tol())
        .expect_err("undeclared tangency")
    {
        ProfileError::UndeclaredTangency { .. } => {}
        other => panic!("expected undeclared tangency, got {other:?}"),
    }
    // phi = sqrt(10 eps): clearance ~ 5 eps, inside the band ->
    // escalation naming the tangency predicate (declaration cannot
    // rescue an in-band margin -- point 2 of the discipline).
    let phi = (10.0 * eps).sqrt();
    match build(phi, true)
        .validate(tol())
        .expect_err("near-tangent join")
    {
        ProfileError::Escalated { source, .. } => {
            assert_eq!(source.predicate, Some("carrier_line_circle"));
        }
        other => panic!("expected tangency escalation, got {other:?}"),
    }
    // phi large: a definite corner, accepted undeclared (transversal
    // joints are free geometry)...
    assert!(
        build(0.3, false).validate(tol()).is_ok(),
        "definite corner accepts"
    );
    // ...and a tangency DECLARATION on that definite corner is
    // contradicted (point 3: the flag is verified, never trusted).
    match build(0.3, true)
        .validate(tol())
        .expect_err("contradicted declaration")
    {
        ProfileError::TangencyContradicted {
            same_carrier: false,
            ..
        } => {}
        other => panic!("expected contradicted tangency, got {other:?}"),
    }
}

// ------------------------------------------------------- ray parity --

/// First candidate ray grazes an outer vertex placed exactly on it;
/// the golden-angle retry must still classify the hole correctly and
/// deterministically.
#[test]
fn forced_first_ray_graze_retries_deterministically() {
    let (s, c) = 0.5f64.sin_cos(); // ray 0 direction, bit-identical to validate's
    let graze_v = p2(3.0 * c, 3.0 * s); // exactly on ray 0 from (0,0)
    let outer = ProfileLoop::polygon([p2(-2.0, -2.0), p2(4.0, -2.0), graze_v, p2(-2.0, 4.0)]);
    let hole = rect(0.0, 0.0, 1.0, 1.0); // rep = (0,0) = ray origin
    let p = profile(vec![outer, hole]);
    let vp1 = ok(&p);
    let vp2 = ok(&p);
    assert_eq!(vp1.loops()[0].role(), LoopRole::Outer);
    assert_eq!(vp1.loops()[1].role(), LoopRole::Hole);
    assert_eq!(format!("{vp1:?}"), format!("{vp2:?}"));
}

/// All 16 candidate rays grazed: a 16-spoke outer polygon with one
/// vertex on every candidate ray from the hole's rep point. Must be
/// the typed RayCastingExhausted, never a guess.
#[test]
fn sixteen_spoke_alignment_exhausts_ray_casting() {
    const RAY_BASE: f64 = 0.5;
    const RAY_STEP: f64 = 2.399_963_229_728_653;
    let mut spokes: Vec<(f64, f64)> = (0..16)
        .map(|k| {
            let (s, c) = (RAY_BASE + (k as f64) * RAY_STEP).sin_cos();
            (
                (RAY_BASE + (k as f64) * RAY_STEP).rem_euclid(std::f64::consts::TAU),
                {
                    let _ = (s, c);
                    0.0
                },
            )
        })
        .collect();
    // Sort spoke angles so the polygon is convex/simple.
    spokes.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let outer = ProfileLoop::polygon(spokes.iter().map(|&(ang, _)| {
        let (s, c) = ang.sin_cos();
        p2(3.0 * c, 3.0 * s)
    }));
    let hole = rect(0.0, 0.0, 0.1, 0.1); // rep (0,0): every ray grazes a spoke
    match err(&profile(vec![outer, hole])) {
        ProfileError::RayCastingExhausted {
            loop_index,
            against_loop,
        } => {
            assert_eq!((loop_index, against_loop), (1, 0));
        }
        other => panic!("expected exhaustion, got {other:?}"),
    }
}

/// A ray entering and exiting the SAME arc segment counts 2 crossings:
/// a rect outside the circle whose first ray passes through the lower
/// semicircular arc twice must classify as outside (parity 0), giving
/// MultipleOuterLoops.
#[test]
fn ray_through_one_arc_twice_counts_two() {
    // Ray 0 from (-1.038, -1.466) at 0.5 rad crosses the unit circle's
    // LOWER semicircle at ~(-0.16, -0.987) and ~(0.917, -0.399) (hand
    // solve: line y = tan(0.5) x - 0.9ish); both points are on the
    // lower arc of circle_h, so one segment yields both crossings.
    let circle = common::circle_h(0.0, 0.0, 1.0);
    let outsider = rect(-1.038, -1.466, 0.3, 0.3);
    match err(&profile(vec![circle, outsider])) {
        ProfileError::MultipleOuterLoops { outer_loops } => {
            assert_eq!(outer_loops, vec![0, 1]);
        }
        other => panic!("expected two outers, got {other:?}"),
    }
}

// --------------------------------------------------- far-from-origin --

/// The shoelace translate-to-origin fix: a far-from-origin profile
/// still orients and validates at eps = 1e-9 (coordinates 1e8, where
/// naive shoelace terms would be 1e16 with ulp ~ 2).
#[test]
fn far_from_origin_rectangle_and_l_profile_validate() {
    let big = 1.0e8;
    let r = rect(big, big, 2.0, 1.0);
    let vp = ok(&profile(vec![r]));
    assert_eq!(vp.loops()[0].role(), LoopRole::Outer);
    let v0 = vp.loops()[0].vertices()[0].pos();
    assert_eq!((v0.x, v0.y), (big, big));
    // With a hole (ray casting + orientation of both loops far away).
    let vp = ok(&profile(vec![
        rect(big, big, 10.0, 4.0),
        rect(big + 4.0, big + 1.5, 1.0, 1.0),
    ]));
    assert_eq!(vp.loops()[1].role(), LoopRole::Hole);
}

/// Arcs at a moderate distance (1e4 m, inside the km session box):
/// circle + hole annulus still validates at eps = 1e-9.
#[test]
fn annulus_at_ten_kilometers_validates() {
    let c = 1.0e4;
    let vp = ok(&profile(vec![
        common::circle_h(c, c, 2.0),
        common::circle_h(c, c, 1.0),
    ]));
    assert_eq!(vp.loops()[0].role(), LoopRole::Outer);
    assert_eq!(vp.loops()[1].role(), LoopRole::Hole);
}

// --------------------------------------------------- near-full arcs --

/// A near-full arc (theta = 2pi - ~2e-3) closed by its chord still
/// validates: the chord is a secant of the carrier meeting it exactly
/// at the two shared vertices (both touches discounted).
#[test]
fn near_full_arc_with_chord_closure_validates() {
    // Half-gap angle, sized so the chord-to-carrier clearance
    // r(1 - cos delta) ~ delta^2/2 clears the escalation band (>= 20
    // eps) at whatever eps the CI row runs.
    let delta = (44.0 * tol().eps()).sqrt();
    let (s, c) = (delta.sin(), delta.cos());
    let a = p2(c, s);
    let b = p2(c, -s);
    // CCW from a up over the top, around, to b: theta = 2pi - 2delta.
    let theta = std::f64::consts::TAU - 2.0 * delta;
    let bulge = (theta / 4.0).tan();
    let lp = chain(&[(a.x, a.y, bulge), (b.x, b.y, 0.0)]);
    let vp = ok(&profile(vec![lp]));
    // Canonical start is the lex-min vertex (bit-identical x tie on
    // cos(delta), broken by least y => b), so the arc is segment 1.
    let arc = vp.loops()[0]
        .segments()
        .iter()
        .find(|s| matches!(s.kind, SegmentKind::Arc { .. }))
        .expect("near-full arc must stay an arc");
    match arc.kind {
        SegmentKind::Arc { center, radius, .. } => {
            assert!(center.x.abs() < 1e-9 && center.y.abs() < 1e-9);
            assert!((radius - 1.0).abs() < 1e-9);
        }
        SegmentKind::Line => unreachable!(),
    }
}

/// A hair-thin near-full arc (carrier clearance below eps) closed by
/// its chord is refused with the typed `NearFullArc` rejection.
///
/// ORIGINAL REVIEW FINDING (SHOULD-2, fixed in the PR 2 fix pass):
/// before the `arc_diameter_clearance` gate existed, this fixture
/// rejected as NonSimple::Touch naming the tangent foot — the foot
/// (c, 0) is 1e-5 m (arc length) from the arc's endpoints, yet
/// arc_span classified it Zero ("at an endpoint"): the chordal-defect
/// margin compresses arc-length distance by cos(theta/4) -> ~delta/2
/// near full arcs, so a point 1e-5 m away carried a 2.5e-11 m margin —
/// a false coincidence claim. No wrong-accept path existed (every
/// probe still rejected), but the error type was mislabeled. The fix
/// rejects the arc at segment construction: its half-span chord is
/// within tolerance of the carrier diameter.
#[test]
fn hair_thin_near_full_arc_is_refused_but_mislabeled() {
    // Clearance r(1-cos delta) ~ delta^2/2 well below eps (and the
    // diameter clearance ~ delta^2/4 below it too).
    let delta = (0.02 * tol().eps()).sqrt();
    let (s, c) = (delta.sin(), delta.cos());
    let theta = std::f64::consts::TAU - 2.0 * delta;
    let bulge = (theta / 4.0).tan();
    let lp = chain(&[(c, s, bulge), (c, -s, 0.0)]);
    assert_eq!(
        err(&profile(vec![lp])),
        ProfileError::NearFullArc(sref(0, 0))
    );
}

/// Interval-lane totality on poisoned input: NaN coordinates lift to
/// NaI enclosures and must produce a typed error, never a panic or an
/// accept.
#[cfg(feature = "interval")]
#[test]
fn interval_nan_totality() {
    use geom_core::Interval;
    let p = profile(vec![chain(&[
        (0.0, 0.0, 0.0),
        (f64::NAN, 0.0, 0.0),
        (1.0, 1.0, 0.0),
    ])]);
    assert!(lift::<Interval>(&p).validate(tol()).is_err());
    // Garbage battery at Interval: specials in every slot, no panics.
    let specials = [0.0, -0.0, 1.0, f64::NAN, f64::INFINITY, 1e300, -5e-324];
    for (i, &a) in specials.iter().enumerate() {
        for &b in &specials {
            let p = profile(vec![chain(&[
                (a, b, specials[i]),
                (b, a, 1.0),
                (1.0, 0.5, a),
            ])]);
            let _ = lift::<Interval>(&p).validate(tol());
        }
    }
}

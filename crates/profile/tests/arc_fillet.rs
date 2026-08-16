//! Arc-leg fillet corners (M5 S2, #101 R4): `LoopBuilder::fillet_corner`
//! on line×arc, arc×line and arc×arc corners.
//!
//! The suite covers the four acceptance groups of `docs/M5-S2-SPEC.md`
//! §5: one fixture per corner class (each verifying that the declared
//! tangency verifies clean and the profile closes and validates), the
//! refusal rows of the taxonomy, the definitely/exactly/in-band trio of
//! every named predicate the constructor fires, and the #100 bracket
//! demo shape re-expressed with an arc leg.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::{profile, tol};
use geom_core::Point2;
use profile::{
    ArcSweep, FilletLeg, FilletLegCarrier, FilletLegShape, NoCornerReason, Profile, ProfileError,
    ProfileLoop,
};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn arc(cx: f64, cy: f64, sweep: ArcSweep) -> FilletLegShape<f64> {
    FilletLegShape::Arc {
        center: p2(cx, cy),
        sweep,
    }
}

/// √3 — the y coordinate of the crossing points of the unit-spaced
/// radius-2 circles used by the arc×arc fixtures.
fn s3() -> f64 {
    3.0f64.sqrt()
}

/// The named predicate an escalation came from (panics with the actual
/// error when it is not a fillet-site escalation).
fn escalated_predicate(err: &ProfileError) -> &'static str {
    match err {
        ProfileError::Escalated {
            site: profile::EscalationSite::Fillet,
            source,
        } => source.predicate.unwrap_or("<unnamed>"),
        other => panic!("expected a fillet-site escalation, got {other:?}"),
    }
}

/// Every declared joint of `lp` must survive validation (the #101
/// discipline: declared tangency is verified, never trusted). The
/// constructor's declarations are asserted in the *input* indexing;
/// validation re-indexes into canonical order, so only the count
/// survives the comparison there — what matters is that both
/// declarations verify (a contradicted one is a hard refusal).
fn validates_with_declared_joints(lp: ProfileLoop<f64>, expected: &[usize]) -> Profile<f64> {
    assert_eq!(lp.tangent_joints, expected, "declared joints");
    let p = profile(vec![lp]);
    let vp = p
        .clone()
        .validate(tol())
        .expect("the fillet-authored loop must validate");
    assert_eq!(vp.loops()[0].tangent_joints().len(), expected.len());
    p
}

// ------------------------------------------- fixtures, per corner class

/// **line×arc, internal tangency** (the fillet curves the same way as
/// the arc leg): a quarter disc whose straight/circular corner at (2,0)
/// is rounded with r = 1/2. The fillet's center sits inside the leg
/// circle, so its offset carrier is the radius R − r circle.
fn line_arc_internal(radius: f64) -> Result<ProfileLoop<f64>, ProfileError> {
    Ok(ProfileLoop::builder(p2(0.0, 0.0))
        .fillet_corner(
            FilletLegShape::Line,
            p2(2.0, 0.0),
            arc(0.0, 0.0, ArcSweep::Ccw),
            p2(0.0, 2.0),
            radius,
            tol(),
        )?
        .arc_to_center(p2(0.0, 2.0), p2(0.0, 0.0), ArcSweep::Ccw)
        .close())
}

/// **line×arc, external tangency** (the fillet curves the other way
/// from the arc leg): the corner at (2,0) turns right onto a circle
/// centered at (3,0), so the fillet's offset carrier is the radius
/// R + r circle.
fn line_arc_external(radius: f64) -> Result<ProfileLoop<f64>, ProfileError> {
    Ok(ProfileLoop::builder(p2(0.0, 0.0))
        .fillet_corner(
            FilletLegShape::Line,
            p2(2.0, 0.0),
            arc(3.0, 0.0, ArcSweep::Ccw),
            p2(3.0, -1.0),
            radius,
            tol(),
        )?
        .arc_to_center(p2(3.0, -1.0), p2(3.0, 0.0), ArcSweep::Ccw)
        .line_to(p2(3.0, -2.0))
        .line_to(p2(0.0, -2.0))
        .close())
}

/// **arc×line**: the incoming leg is the circular one (a concave notch
/// on the radius-2 circle about the origin), the outgoing leg straight.
fn arc_line(radius: f64) -> Result<ProfileLoop<f64>, ProfileError> {
    Ok(ProfileLoop::builder(p2(0.0, 2.0))
        .fillet_corner(
            arc(0.0, 0.0, ArcSweep::Cw),
            p2(2.0, 0.0),
            FilletLegShape::Line,
            p2(4.0, 0.0),
            radius,
            tol(),
        )?
        .line_to(p2(4.0, 0.0))
        .line_to(p2(4.0, 3.0))
        .line_to(p2(-1.0, 3.0))
        .close())
}

/// **arc×arc, both tangencies internal**: the vesica of the two
/// radius-2 circles about (±1, 0), with its top corner (0, √3) rounded.
/// Both legs wind counterclockwise, so σ·τ = +1 on both and both offset
/// carriers are R − r circles.
fn arc_arc_internal(radius: f64) -> Result<ProfileLoop<f64>, ProfileError> {
    Ok(ProfileLoop::builder(p2(1.0, 0.0))
        .fillet_corner(
            arc(-1.0, 0.0, ArcSweep::Ccw),
            p2(0.0, s3()),
            arc(1.0, 0.0, ArcSweep::Ccw),
            p2(-1.0, 0.0),
            radius,
            tol(),
        )?
        .arc_to_center(p2(-1.0, 0.0), p2(1.0, 0.0), ArcSweep::Ccw)
        .close())
}

/// **arc×arc, one internal + one external**: the same crossing circles,
/// but the outgoing leg winds the other way, so the fillet is external
/// to the first carrier (R + r) and internal to the second (R − r).
fn arc_arc_mixed(radius: f64) -> Result<ProfileLoop<f64>, ProfileError> {
    Ok(ProfileLoop::builder(p2(1.0, 0.0))
        .fillet_corner(
            arc(-1.0, 0.0, ArcSweep::Ccw),
            p2(0.0, s3()),
            arc(1.0, 0.0, ArcSweep::Cw),
            p2(3.0, 0.0),
            radius,
            tol(),
        )?
        .arc_to_center(p2(3.0, 0.0), p2(1.0, 0.0), ArcSweep::Cw)
        .close())
}

/// The line×arc corner at the radius that consumes BOTH legs exactly
/// (r = 1): no lead-in piece, no lead-out piece, no declarations — the
/// fillet arc alone, closed straight. A half disc.
fn exact_fit_line_arc() -> ProfileLoop<f64> {
    ProfileLoop::builder(p2(0.0, 0.0))
        .fillet_corner(
            FilletLegShape::Line,
            p2(2.0, 0.0),
            arc(0.0, 0.0, ArcSweep::Ccw),
            p2(0.0, 2.0),
            1.0,
            tol(),
        )
        .expect("the exact-fit radius constructs")
        .close()
}

#[test]
fn line_arc_internal_validates_with_declared_tangency() {
    let lp = line_arc_internal(0.5).expect("the fillet fits");
    // (0,0) → T1 on y = 0 → fillet arc → T2 on the circle → (0,2).
    assert_eq!(lp.vertices.len(), 4);
    // T1 = (√2, 0) exactly: x² = (2−r)² − r² = 2 at r = 1/2.
    assert!((lp.vertices[1].pos.x - 2.0f64.sqrt()).abs() < 1e-15);
    assert_eq!(lp.vertices[1].pos.y, 0.0);
    // T2 sits on the leg carrier to rounding — tangency by construction.
    let t2 = lp.vertices[2].pos;
    // The claim is that T2 lies on the carrier, so measure THAT — the
    // radial residual — and not its square. |t|² − R² is 2R times the
    // radial error, so on this R = 2 carrier the squared form silently
    // asserts a quarter-ulp bound that no correctly-rounded construction
    // can meet; it passed before only because the old scaling happened to
    // cancel exactly here. The residual below is 1 ulp of 2.0.
    assert!(
        (t2.x.hypot(t2.y) - 2.0).abs() <= 2.0 * f64::EPSILON,
        "T2 off the carrier by {:e}",
        (t2.x.hypot(t2.y) - 2.0).abs()
    );
    validates_with_declared_joints(lp, &[1, 2]);
}

#[test]
fn line_arc_external_validates_with_declared_tangency() {
    let lp = line_arc_external(0.5).expect("the fillet fits");
    let t2 = lp.vertices[2].pos;
    // On the outgoing leg's carrier (center (3,0), R = 1).
    assert!(((t2.x - 3.0).powi(2) + t2.y.powi(2) - 1.0).abs() < 1e-15);
    validates_with_declared_joints(lp, &[1, 2]);
}

#[test]
fn arc_line_validates_with_declared_tangency() {
    let lp = arc_line(0.5).expect("the fillet fits");
    let t1 = lp.vertices[1].pos;
    // T1 on the incoming leg's carrier (origin, R = 2).
    assert!((t1.x.powi(2) + t1.y.powi(2) - 4.0).abs() < 1e-15);
    // T2 on the outgoing straight leg y = 0.
    assert!(lp.vertices[2].pos.y.abs() < 1e-15);
    validates_with_declared_joints(lp, &[1, 2]);
}

#[test]
fn arc_arc_internal_validates_with_declared_tangency() {
    let lp = arc_arc_internal(0.5).expect("the fillet fits");
    let t1 = lp.vertices[1].pos;
    let t2 = lp.vertices[2].pos;
    assert!(((t1.x + 1.0).powi(2) + t1.y.powi(2) - 4.0).abs() < 1e-14);
    assert!(((t2.x - 1.0).powi(2) + t2.y.powi(2) - 4.0).abs() < 1e-14);
    validates_with_declared_joints(lp, &[1, 2]);
}

#[test]
fn arc_arc_mixed_validates_with_declared_tangency() {
    let lp = arc_arc_mixed(0.5).expect("the fillet fits");
    let t1 = lp.vertices[1].pos;
    let t2 = lp.vertices[2].pos;
    assert!(((t1.x + 1.0).powi(2) + t1.y.powi(2) - 4.0).abs() < 1e-14);
    assert!(((t2.x - 1.0).powi(2) + t2.y.powi(2) - 4.0).abs() < 1e-14);
    validates_with_declared_joints(lp, &[1, 2]);
}

// ------------------------------------- the #100 bracket regression anchor

/// The #100 demo bracket with its incoming leg re-expressed as a shallow
/// arc (carrier: center (2, −2), R = √10, through both (3,1) and the
/// corner (1,1)) — the same L with the same r = 1/2 inner fillet,
/// authored through the arc-leg door.
#[test]
fn bracket_with_an_arc_leg_validates_and_declares() {
    let lp = ProfileLoop::builder(p2(0.0, 0.0))
        .line_to(p2(3.0, 0.0))
        .line_to(p2(3.0, 1.0))
        .fillet_corner(
            arc(2.0, -2.0, ArcSweep::Ccw),
            p2(1.0, 1.0),
            FilletLegShape::Line,
            p2(1.0, 3.0),
            0.5,
            tol(),
        )
        .expect("the arc-leg bracket fillet fits")
        .line_to(p2(1.0, 3.0))
        .line_to(p2(0.0, 3.0))
        .close();
    // The straight-leg bracket declares joints 3 and 4; so does this one
    // (same chain shape: start, (3,0), (3,1), T1, T2, (1,3), (0,3)).
    assert_eq!(lp.vertices.len(), 7);
    // T2 sits on the outgoing straight leg x = 1, above the corner.
    assert_eq!(lp.vertices[4].pos.x, 1.0);
    assert!(lp.vertices[4].pos.y > 1.0);
    // T1 sits on the incoming arc's carrier.
    let t1 = lp.vertices[3].pos;
    assert!(((t1.x - 2.0).powi(2) + (t1.y + 2.0).powi(2) - 10.0).abs() < 1e-14);
    validates_with_declared_joints(lp, &[3, 4]);
}

// ------------------------------------------------- the refusal taxonomy

#[test]
fn oversized_radius_on_an_arc_leg_names_the_carrier_and_angular_margin() {
    // The outgoing arc leg is only 10° long; r = 1/2 sets the tangent
    // point back further than that.
    let short = 10.0f64.to_radians();
    let err = ProfileLoop::builder(p2(0.0, 0.0))
        .fillet_corner(
            FilletLegShape::Line,
            p2(2.0, 0.0),
            arc(0.0, 0.0, ArcSweep::Ccw),
            p2(2.0 * short.cos(), 2.0 * short.sin()),
            0.5,
            tol(),
        )
        .expect_err("the short arc leg must refuse");
    match err {
        ProfileError::FilletDoesNotFit {
            leg,
            carrier:
                FilletLegCarrier::Arc {
                    radius,
                    angular_margin,
                },
            setback,
            leg_length,
        } => {
            assert_eq!(leg, FilletLeg::Outgoing);
            assert!((radius - 2.0).abs() < 1e-15, "carrier radius {radius}");
            assert!(angular_margin < 0.0, "angular margin {angular_margin}");
            assert!(setback > leg_length, "{setback} vs {leg_length}");
            // The angular margin is the arc-length margin over R.
            assert!((angular_margin - (leg_length - setback) / 2.0).abs() < 1e-15);
        }
        other => panic!("expected an arc-leg FilletDoesNotFit, got {other:?}"),
    }
}

#[test]
fn oversized_radius_on_a_straight_leg_still_names_the_straight_carrier() {
    // Same corner class, but the STRAIGHT leg is the short one.
    let err = ProfileLoop::builder(p2(1.9, 0.0))
        .fillet_corner(
            FilletLegShape::Line,
            p2(2.0, 0.0),
            arc(0.0, 0.0, ArcSweep::Ccw),
            p2(0.0, 2.0),
            0.5,
            tol(),
        )
        .expect_err("the short straight leg must refuse");
    match err {
        ProfileError::FilletDoesNotFit { leg, carrier, .. } => {
            assert_eq!(leg, FilletLeg::Incoming);
            assert_eq!(carrier, FilletLegCarrier::Line);
        }
        other => panic!("expected a straight-leg FilletDoesNotFit, got {other:?}"),
    }
}

#[test]
fn radius_too_large_for_the_corner_has_no_tangent_circle() {
    // The offset line y = r and the offset circle of radius 2 − r meet
    // only while r ≤ 1; beyond that no circle of radius r is tangent to
    // both carriers anywhere.
    match line_arc_internal(1.5).expect_err("no tangent circle exists") {
        ProfileError::NoCornerForFillet { reason, radius } => {
            assert_eq!(reason, NoCornerReason::OffsetCarriersDisjoint);
            assert_eq!(radius, 1.5);
        }
        other => panic!("expected NoCornerForFillet, got {other:?}"),
    }
}

#[test]
fn a_negative_radius_offsets_outside_the_corner_and_is_refused() {
    // r < 0 pushes both carriers AWAY from the turn side, so every
    // tangent circle touches past the corner. Unlike the straight-leg
    // `fillet` (which passes its one-sided gate and fails simplicity
    // downstream), the arc-leg door refuses at construction.
    match line_arc_external(-0.5).expect_err("a negative radius must refuse") {
        ProfileError::NoCornerForFillet { reason, radius } => {
            assert_eq!(reason, NoCornerReason::NoCornerSideCandidate);
            assert_eq!(radius, -0.5);
        }
        other => panic!("expected NoCornerForFillet, got {other:?}"),
    }
}

/// `NoCornerSideCandidate` on an **arc×arc** corner — unreachable before
/// the signed-setback fix (review MAJOR-1), because an arc leg's setback
/// was reduced into [0, 2π) and so could never classify Negative.
///
/// The corner is the origin, where a radius-1 carrier about (0, −1) meets
/// a radius-1/2 carrier about (1/2, 0). Both legs wind counterclockwise,
/// so the path arrives travelling −x and leaves travelling −y (a left
/// turn, σ = +1). A radius-2 fillet is larger than *both* carriers, so
/// both offset radii go negative (ρ₁ = −1, ρ₂ = −3/2) and the offset
/// circles still meet — twice. But a circle that big can only touch these
/// two small carriers on the far side from the origin, so both tangent
/// circles reach their legs past the corner and neither is a candidate.
#[test]
fn an_arc_arc_corner_can_have_no_corner_side_candidate() {
    // Far ends one radian along each carrier, so both legs have real
    // extent and the arm/turn gates pass cleanly.
    let along = |cx: f64, cy: f64, r: f64, delta: f64| {
        let a = f64::atan2(-cy, -cx) + delta;
        p2(cx + r * a.cos(), cy + r * a.sin())
    };
    let err = ProfileLoop::builder(along(0.0, -1.0, 1.0, -1.0))
        .fillet_corner(
            arc(0.0, -1.0, ArcSweep::Ccw),
            p2(0.0, 0.0),
            arc(0.5, 0.0, ArcSweep::Ccw),
            along(0.5, 0.0, 0.5, 1.0),
            2.0,
            tol(),
        )
        .expect_err("every tangent circle of radius 2 touches past the corner");
    match err {
        ProfileError::NoCornerForFillet { reason, radius } => {
            // NOT OffsetCarriersDisjoint — the offset carriers do meet.
            assert_eq!(reason, NoCornerReason::NoCornerSideCandidate);
            assert_eq!(radius, 2.0);
        }
        other => panic!("expected NoCornerSideCandidate, got {other:?}"),
    }
    // Trio parity (review MINOR-1): this definite refusal and the in-band
    // `fillet_leg_reach` escalation render the SAME recourse sentence.
    assert!(
        err.to_string().contains("can sit in the corner"),
        "recourse: {err}"
    );
}

/// The picked fillet circle of a 3-vertex vesica chain (start, T1, T2),
/// recovered through validation's segment classification: the one
/// radius-`r` arc. Returns (center, radius) and asserts both declared
/// joints verified on the way.
fn picked_fillet_circle(lp: ProfileLoop<f64>, r: f64) -> (Point2<f64>, f64) {
    let p = validates_with_declared_joints(lp, &[1, 2]);
    let vp = p.validate(tol()).expect("validates");
    vp.loops()[0]
        .segments()
        .iter()
        .find_map(|s| match s.kind {
            profile::SegmentKind::Arc { center, radius, .. } if (radius - r).abs() < 1e-12 => {
                Some((center, radius))
            }
            _ => None,
        })
        .expect("the fillet arc classifies at its authored radius")
}

/// The S8 ruling flips the M5 S2 refusal: the vesica of the two
/// crossing circles has TWO corners, and with both legs long enough to
/// contain both tangent circles the corner at (0, √3) now PICKS the
/// candidate nearest it — the top pocket — instead of refusing
/// `AmbiguousFilletBranch` (retired). Exact tangency asserted; both
/// junctions declared and verified.
#[test]
fn two_corner_side_candidates_pick_the_near_one() {
    let lp = ProfileLoop::builder(p2(0.0, -s3()))
        .fillet_corner(
            arc(-1.0, 0.0, ArcSweep::Ccw),
            p2(0.0, s3()),
            arc(1.0, 0.0, ArcSweep::Ccw),
            p2(0.0, -s3()),
            0.5,
            tol(),
        )
        .expect("the near candidate resolves the two-survivor corner")
        .close_arc_center(p2(1.0, 0.0), ArcSweep::Ccw);
    // start, T1, T2: trimmed incoming leg, fillet arc, closing leg.
    assert_eq!(lp.vertices.len(), 3);
    let t1 = lp.vertices[1].pos;
    let t2 = lp.vertices[2].pos;
    // Tangent points exactly on their leg carriers (tangency by
    // construction), and in the TOP pocket — the near candidate's.
    assert!(((t1.x + 1.0).powi(2) + t1.y.powi(2) - 4.0).abs() < 1e-14);
    assert!(((t2.x - 1.0).powi(2) + t2.y.powi(2) - 4.0).abs() < 1e-14);
    assert!(t1.y > 0.0 && t2.y > 0.0, "far pocket picked: {lp:?}");
    // The picked center is the near root of the offset circles (radius
    // 2 − r about (±1, 0)): (0, +√(1.5² − 1)) — its mirror below the
    // waist was the old refusal's other center.
    let (center, _) = picked_fillet_circle(lp, 0.5);
    assert!(center.x.abs() < 1e-14, "center {center:?}");
    assert!(
        (center.y - 1.25f64.sqrt()).abs() < 1e-14,
        "center {center:?}"
    );
}

/// The ruling's premise, pinned (S8 §2): the far tangent circle is
/// always deliberately authorable as the NEAR fillet of the OTHER
/// corner. Authoring the vesica corner at the second carrier
/// intersection (0, −√3), legs swept from there, yields the circle
/// that was the far candidate of the original corner — the recovered
/// center is asserted to agree with (0, −√(1.5² − 1)) to 1e-14, and
/// the tangent points to lie on their leg carriers to 1e-14 (the same
/// closed form up to rounding, not bit-identity: the two authorings
/// mirror the arithmetic, which f64 does not commute with exactly).
#[test]
fn the_far_pocket_is_authored_as_the_other_corners_near_fillet() {
    let lp = ProfileLoop::builder(p2(0.0, s3()))
        .fillet_corner(
            arc(-1.0, 0.0, ArcSweep::Cw),
            p2(0.0, -s3()),
            arc(1.0, 0.0, ArcSweep::Cw),
            p2(0.0, s3()),
            0.5,
            tol(),
        )
        .expect("the second intersection's near fillet constructs")
        .close_arc_center(p2(1.0, 0.0), ArcSweep::Cw);
    assert_eq!(lp.vertices.len(), 3);
    let t1 = lp.vertices[1].pos;
    let t2 = lp.vertices[2].pos;
    assert!(((t1.x + 1.0).powi(2) + t1.y.powi(2) - 4.0).abs() < 1e-14);
    assert!(((t2.x - 1.0).powi(2) + t2.y.powi(2) - 4.0).abs() < 1e-14);
    assert!(t1.y < 0.0 && t2.y < 0.0, "wrong pocket: {lp:?}");
    let (center, _) = picked_fillet_circle(lp, 0.5);
    assert!(center.x.abs() < 1e-14, "center {center:?}");
    assert!(
        (center.y + 1.25f64.sqrt()).abs() < 1e-14,
        "expected the original corner's far candidate, got {center:?}"
    );
}

/// S8 rung-3 pin at the constructor door: the symmetric lens is the
/// configuration whose candidate-swapping symmetry class motivates the
/// documented enumeration-order residual, and the whole selection must
/// be bit-deterministic run to run (the interval-lane twin asserts the
/// same pick from the other lane).
#[test]
fn symmetric_lens_pick_is_bit_deterministic_across_runs() {
    let build = || {
        ProfileLoop::builder(p2(0.0, -s3()))
            .fillet_corner(
                arc(-1.0, 0.0, ArcSweep::Ccw),
                p2(0.0, s3()),
                arc(1.0, 0.0, ArcSweep::Ccw),
                p2(0.0, -s3()),
                0.5,
                tol(),
            )
            .expect("constructs")
            .close_arc_center(p2(1.0, 0.0), ArcSweep::Ccw)
    };
    let a = build();
    let b = build();
    assert_eq!(a.tangent_joints, b.tangent_joints);
    assert_eq!(a.vertices.len(), b.vertices.len());
    for (va, vb) in a.vertices.iter().zip(&b.vertices) {
        assert_eq!(va.pos.x.to_bits(), vb.pos.x.to_bits());
        assert_eq!(va.pos.y.to_bits(), vb.pos.y.to_bits());
        assert_eq!(va.bulge.to_bits(), vb.bulge.to_bits());
    }
}

/// The hairline-asymmetric lens (S8 review MINOR-1): the authored
/// corner nudged ~1 ulp off the vesica's mirror axis. What is pinned is
/// each lane's OWN determinism — bit-identical output across runs and a
/// definite pocket committed to — NOT cross-lane agreement: a setback
/// gap below the interval channel's enclosure width may legally resolve
/// to the other pocket at Interval, and per the ruling both candidates
/// are valid fillets of the authored legs (the interval twin is
/// `ulp_perturbed_lens_pick_is_deterministic_at_interval`).
#[test]
fn ulp_perturbed_lens_pick_is_deterministic_within_the_lane() {
    let build = || {
        ProfileLoop::builder(p2(0.0, -s3()))
            .fillet_corner(
                arc(-1.0, 0.0, ArcSweep::Ccw),
                p2(f64::EPSILON, s3()),
                arc(1.0, 0.0, ArcSweep::Ccw),
                p2(0.0, -s3()),
                0.5,
                tol(),
            )
            .expect("the perturbed lens constructs")
            .close_arc_center(p2(1.0, 0.0), ArcSweep::Ccw)
    };
    let a = build();
    let b = build();
    assert_eq!(a.tangent_joints, b.tangent_joints);
    assert_eq!(a.vertices.len(), b.vertices.len());
    for (va, vb) in a.vertices.iter().zip(&b.vertices) {
        assert_eq!(va.pos.x.to_bits(), vb.pos.x.to_bits());
        assert_eq!(va.pos.y.to_bits(), vb.pos.y.to_bits());
        assert_eq!(va.bulge.to_bits(), vb.bulge.to_bits());
    }
    // One pocket was definitely committed to (which one is the lane's
    // own business).
    assert!(a.vertices[1].pos.y.abs() > 0.5);
}

#[test]
fn an_already_tangent_corner_asks_for_the_declaration_instead() {
    // The line y = 0 is tangent to the circle about (2,2) at (2,0):
    // there is no corner to cut.
    let err = ProfileLoop::builder(p2(0.0, 0.0))
        .fillet_corner(
            FilletLegShape::Line,
            p2(2.0, 0.0),
            arc(2.0, 2.0, ArcSweep::Ccw),
            p2(4.0, 2.0),
            0.5,
            tol(),
        )
        .expect_err("a tangent corner must refuse");
    match err {
        ProfileError::FilletCornerAlreadyTangent {
            reversed,
            margin,
            arm,
        } => {
            assert!(!reversed);
            assert_eq!(margin, 0.0);
            assert!(arm > 0.0);
        }
        other => panic!("expected FilletCornerAlreadyTangent, got {other:?}"),
    }
    assert!(
        err.to_string().contains("tangent_joints"),
        "recourse: {err}"
    );
}

#[test]
fn a_reverse_tangent_corner_is_named_as_a_cusp() {
    // Same carriers, opposite sweep: the outgoing leg leaves along the
    // REVERSE of the incoming tangent — a cusp, not a corner.
    let err = ProfileLoop::builder(p2(0.0, 0.0))
        .fillet_corner(
            FilletLegShape::Line,
            p2(2.0, 0.0),
            arc(2.0, 2.0, ArcSweep::Cw),
            p2(0.0, 2.0),
            0.5,
            tol(),
        )
        .expect_err("a cusp must refuse");
    match err {
        ProfileError::FilletCornerAlreadyTangent { reversed, .. } => assert!(reversed),
        other => panic!("expected a reversed FilletCornerAlreadyTangent, got {other:?}"),
    }
    // Review MINOR-2: a doubled-back corner has NO declaration door —
    // the material-wedge invariant refuses cusp wedges downstream, so
    // advising a tangency declaration here would only move the failure. The
    // reversed arm names the cusp class and #131 instead (PATHS-DESIGN
    // §4 item 1), and must not offer the smooth-tangent recourse.
    let text = err.to_string();
    assert!(
        !text.contains("declare the tangency"),
        "the cusp arm must not advise declaring tangency: {text}"
    );
    assert!(text.contains("cusp") && text.contains("#131"), "{text}");
}

#[test]
fn a_leg_with_no_extent_is_refused_before_any_angle_is_classified() {
    // Zero-length incoming leg: the corner IS the chain head, so the
    // corner's turn has no lever arm to be metered at (D4 ¶1).
    match ProfileLoop::builder(p2(2.0, 0.0))
        .fillet_corner(
            FilletLegShape::Line,
            p2(2.0, 0.0),
            arc(0.0, 0.0, ArcSweep::Ccw),
            p2(0.0, 2.0),
            0.5,
            tol(),
        )
        .expect_err("a zero-extent leg must refuse")
    {
        ProfileError::FilletLegDegenerate { leg, arm } => {
            assert_eq!(leg, FilletLeg::Incoming);
            assert_eq!(arm, 0.0);
        }
        other => panic!("expected FilletLegDegenerate, got {other:?}"),
    }
}

#[test]
fn an_arc_leg_with_no_extent_is_refused_the_same_way() {
    // The straight-leg row above collapses a CHORD; these two collapse an
    // arc leg the two ways `FilletLegDegenerate` documents — an empty
    // sweep (far end at the corner) and a zero-radius carrier (center at
    // the corner, whose tangent direction is poison). Both must come out
    // as the typed arm refusal naming the arc leg, never a panic and
    // never a classification taken on a collapsed lever arm (D4 ¶1).
    let empty_sweep = ProfileLoop::builder(p2(0.0, 0.0))
        .fillet_corner(
            FilletLegShape::Line,
            p2(2.0, 0.0),
            arc(0.0, 0.0, ArcSweep::Ccw),
            // `next` IS the corner: the outgoing leg sweeps nothing.
            p2(2.0, 0.0),
            0.5,
            tol(),
        )
        .expect_err("an empty-sweep arc leg must refuse");
    match empty_sweep {
        ProfileError::FilletLegDegenerate { leg, arm } => {
            assert_eq!(leg, FilletLeg::Outgoing);
            assert_eq!(arm, 0.0);
        }
        other => panic!("expected FilletLegDegenerate, got {other:?}"),
    }
    assert!(
        empty_sweep.to_string().contains("real extent"),
        "recourse: {empty_sweep}"
    );
    let zero_radius = ProfileLoop::builder(p2(0.0, 0.0))
        .fillet_corner(
            FilletLegShape::Line,
            p2(2.0, 0.0),
            // The carrier's center IS the corner: R = 0.
            arc(2.0, 0.0, ArcSweep::Ccw),
            p2(0.0, 2.0),
            0.5,
            tol(),
        )
        .expect_err("a zero-radius arc carrier must refuse");
    match zero_radius {
        ProfileError::FilletLegDegenerate { leg, arm } => {
            assert_eq!(leg, FilletLeg::Outgoing);
            assert_eq!(arm, 0.0);
        }
        other => panic!("expected FilletLegDegenerate, got {other:?}"),
    }
}

#[test]
fn a_line_line_corner_delegates_to_the_ratified_closed_form() {
    // Bit-identity, not near-equality: `fillet_corner` with two straight
    // legs must emit the SAME chain as `fillet` (one construction).
    let through_corner = ProfileLoop::builder(p2(0.0, 0.0))
        .line_to(p2(3.0, 0.0))
        .line_to(p2(3.0, 1.0))
        .fillet_corner(
            FilletLegShape::Line,
            p2(1.0, 1.0),
            FilletLegShape::Line,
            p2(1.0, 3.0),
            0.5,
            tol(),
        )
        .expect("fits")
        .line_to(p2(1.0, 3.0))
        .line_to(p2(0.0, 3.0))
        .close();
    let legacy = ProfileLoop::builder(p2(0.0, 0.0))
        .line_to(p2(3.0, 0.0))
        .line_to(p2(3.0, 1.0))
        .fillet(p2(1.0, 1.0), p2(1.0, 3.0), 0.5)
        .expect("fits")
        .line_to(p2(1.0, 3.0))
        .line_to(p2(0.0, 3.0))
        .close();
    assert_eq!(legacy.tangent_joints, through_corner.tangent_joints);
    for (a, b) in legacy.vertices.iter().zip(&through_corner.vertices) {
        assert_eq!(a.pos.x.to_bits(), b.pos.x.to_bits());
        assert_eq!(a.pos.y.to_bits(), b.pos.y.to_bits());
        assert_eq!(a.bulge.to_bits(), b.bulge.to_bits());
    }
}

// ------------------------ the definitely / exactly / in-band predicate trios
//
// The exact-order predicates (`fillet_leg_fit`, `fillet_leg_reach`) have
// no in-band row at `f64` by construction — no representable `f64` lies
// strictly inside the hairline band — so their in-band rows live in the
// interval lane (tests/interval_lane.rs), where a straddling enclosure
// escalates honestly. Everything else has all three rows here.

/// ε-relative offsets: `5·ε` sits strictly inside the run's ambiguity
/// band (ε, K·ε) at the default K = 10.
fn in_band() -> f64 {
    5.0 * tol().eps
}

#[test]
fn fillet_corner_arm_trio() {
    // definitely: every fixture above classifies the arm positive.
    assert!(line_arc_internal(0.5).is_ok());
    // exactly zero: the zero-extent leg row above.
    // in-band: a straight leg 5ε long.
    let tiny = in_band();
    let err = ProfileLoop::builder(p2(2.0 - tiny, 0.0))
        .fillet_corner(
            FilletLegShape::Line,
            p2(2.0, 0.0),
            arc(0.0, 0.0, ArcSweep::Ccw),
            p2(0.0, 2.0),
            0.5,
            tol(),
        )
        .expect_err("an in-band arm must escalate");
    assert_eq!(escalated_predicate(&err), "fillet_corner_arm");
    assert!(err.to_string().contains("real extent"), "recourse: {err}");
}

#[test]
fn fillet_corner_turn_trio() {
    // definitely: the fixtures. exactly zero: the tangent-corner row.
    // in-band: perturb the tangent configuration by δ so the turn
    // margin sin φ · arm ≈ δ lands inside (ε, K·ε).
    let delta = in_band();
    let err = ProfileLoop::builder(p2(0.0, 0.0))
        .fillet_corner(
            FilletLegShape::Line,
            p2(2.0, 0.0),
            arc(2.0 + delta, 2.0, ArcSweep::Ccw),
            p2(4.0 + delta, 2.0),
            0.5,
            tol(),
        )
        .expect_err("a near-tangent corner must escalate");
    assert_eq!(escalated_predicate(&err), "fillet_corner_turn");
    // The escalation renders the SAME recourse as the definite refusal
    // (the two-tolerance discipline: one situation, one message).
    assert!(
        err.to_string().contains("tangent_joints"),
        "recourse: {err}"
    );
}

#[test]
fn fillet_offset_line_circle_trio() {
    // definitely: r = 1/2 leaves margin (2 − r) − r = 1.
    assert!(line_arc_internal(0.5).is_ok());
    // exactly zero: r = 1 makes the offset carriers tangent — ONE
    // candidate, and it is an exact fit on both legs, so the chain is
    // the fillet arc alone (closed straight: a half disc).
    let lp = exact_fit_line_arc();
    assert_eq!(lp.vertices.len(), 2, "no lead-in, no lead-out: {lp:?}");
    profile(vec![lp])
        .validate(tol())
        .expect("the exact-offset fillet validates");
    // in-band: (2 − r) − r = 5ε.
    let err = line_arc_internal(0.5f64.mul_add(-in_band(), 1.0))
        .expect_err("an in-band offset clearance must escalate");
    assert_eq!(escalated_predicate(&err), "fillet_offset_line_circle");
    assert!(
        err.to_string().contains("smaller radius"),
        "recourse: {err}"
    );
}

#[test]
fn fillet_offset_circles_external_trio() {
    // definitely: r = 1/2 on the both-internal vesica corner leaves
    // (2 − r) + (2 − r) − 2 = 1.
    assert!(arc_arc_internal(0.5).is_ok());
    // exactly zero: r = 1 makes the two offset circles externally
    // tangent — one candidate, exact fit on both legs.
    let lp = ProfileLoop::builder(p2(1.0, 0.0))
        .fillet_corner(
            arc(-1.0, 0.0, ArcSweep::Ccw),
            p2(0.0, s3()),
            arc(1.0, 0.0, ArcSweep::Ccw),
            p2(-1.0, 0.0),
            1.0,
            tol(),
        )
        .expect("the tangent-offset case constructs")
        .close();
    assert_eq!(lp.vertices.len(), 2, "no lead-in, no lead-out: {lp:?}");
    profile(vec![lp])
        .validate(tol())
        .expect("the exact-offset arc×arc fillet validates");
    // in-band.
    let err = arc_arc_internal(0.5f64.mul_add(-in_band(), 1.0))
        .expect_err("an in-band external clearance must escalate");
    assert_eq!(escalated_predicate(&err), "fillet_offset_circles_external");
}

#[test]
fn fillet_offset_circles_internal_trio() {
    // The mixed corner offsets to R + r and R − r, so the INTERNAL
    // clearance d − |ρ₁ − ρ₂| = 2 − 2r is the one that closes.
    assert!(arc_arc_mixed(0.5).is_ok());
    let lp = ProfileLoop::builder(p2(1.0, 0.0))
        .fillet_corner(
            arc(-1.0, 0.0, ArcSweep::Ccw),
            p2(0.0, s3()),
            arc(1.0, 0.0, ArcSweep::Cw),
            p2(3.0, 0.0),
            1.0,
            tol(),
        )
        .expect("the tangent-offset case constructs")
        .close();
    assert_eq!(lp.vertices.len(), 2, "no lead-in, no lead-out: {lp:?}");
    profile(vec![lp])
        .validate(tol())
        .expect("the internally-tangent-offset fillet validates");
    let err = arc_arc_mixed(0.5f64.mul_add(-in_band(), 1.0))
        .expect_err("an in-band internal clearance must escalate");
    assert_eq!(escalated_predicate(&err), "fillet_offset_circles_internal");
}

#[test]
fn fillet_leg_fit_trio_definite_and_exact() {
    // definitely negative: the oversized-radius rows above.
    // definitely positive: every validating fixture.
    // exactly zero: r = 1 on the line×arc corner consumes the incoming
    // straight leg and the outgoing arc leg EXACTLY — neither trimmed
    // piece is emitted and neither tangent point is declared (the same
    // three-way rule the straight-leg constructor documents).
    let lp = exact_fit_line_arc();
    assert_eq!(lp.vertices.len(), 2);
    assert!(lp.tangent_joints.is_empty(), "{:?}", lp.tangent_joints);
    // The arc springs off the chain head and lands exactly on `next`.
    assert_eq!(lp.vertices[0].pos.x, 0.0);
    assert_eq!(lp.vertices[0].pos.y, 0.0);
    assert!((lp.vertices[1].pos.y - 2.0).abs() < 1e-15);
}

#[test]
fn fillet_leg_reach_trio_definite_and_exact() {
    // definitely positive: every validating fixture. definitely
    // negative: the negative-radius row and the arc×arc
    // no-corner-side row, both of which refuse `NoCornerForFillet` and
    // both of which render the SAME recourse as reach's in-band
    // escalation does in the interval lane (review MINOR-1 —
    // `an_arc_arc_corner_can_have_no_corner_side_candidate` and
    // `zero_radius_arc_fillet_escalates_at_interval` pin the two ends).
    //
    // exactly zero: r = 0 puts both tangent points ON the corner — the
    // gate passes and the degenerate zero-length arc is refused
    // downstream, exactly as the straight-leg constructor documents.
    let lp = line_arc_internal(0.0).expect("r = 0 passes the extent gates");
    let refused = profile(vec![lp])
        .validate(tol())
        .expect_err("a zero-length fillet arc must be refused at validation");
    assert!(
        matches!(
            refused,
            ProfileError::DegenerateSegment(_) | ProfileError::Escalated { .. }
        ),
        "got {refused:?}"
    );
}

/// **The extraction's bit-identity pin** (LIB-G2 §2). `fillet_corner`'s
/// candidate loop and selection now live in the shared, `Bounds`-free
/// `arc_fillet_trims` seam, with the S8 pick applied at this door — the
/// `line_line_fillet_trims` pattern one level up. The extraction is
/// behavior-preserving, and "preserving" here means BITWISE: the
/// literals below were captured from the pre-extraction build and must
/// keep reproducing exactly, for every corner class the seam carries.
///
/// A failure here is not a tolerance question. It means the shared seam
/// re-derived a quantity the shipped constructor derived differently,
/// and every downstream differential fixture (the algebra's included)
/// rests on these bits.
///
/// # Re-pinned once, deliberately (M8, `Leg::tangent_point`)
///
/// The literals moved exactly once since capture, when `tangent_point`
/// began dividing the spoke by its MEASURED length instead of its
/// nominal ρ. That is a change to the emitted geometry and so a D9
/// event, not a silent drift, and the size of it is the evidence that it
/// is the intended one: of the five corner classes below, **`arc_line`
/// and `arc_arc_internal` did not move at all**, and the other three
/// moved by **1, 2 and 4 ulps**. Well-conditioned corners are where the
/// old and new scalings agree to rounding, and that is what these
/// fixtures are.
///
/// The change buys, on ill-conditioned corners, up to 3.6e6x: a corner
/// whose outgoing offset radius is 1.7e-7 used to emit a tangent point
/// 1.2e-2 off its own carrier and a fillet radius 4.1e-3 wrong, and now
/// emits 2.2e-16 and 1.2e-9. Re-pinning a handful of ulps to remove that
/// is the trade; re-pinning for any smaller reason is not.
/// A vertex's `(x, y, bulge)` raw f64 bits — the channel the pin below
/// compares on, because "bit-identical" is the actual claim.
type VertexBits = (u64, u64, u64);

/// One pinned corner class: its name, the loop it builds, and the bits
/// the pre-extraction build produced.
type PinnedCase<'a> = (&'a str, ProfileLoop<f64>, &'a [VertexBits]);

#[test]
fn the_extracted_seam_reproduces_every_corner_class_bitwise() {
    let dump = |lp: &ProfileLoop<f64>| -> Vec<VertexBits> {
        lp.vertices
            .iter()
            .map(|v| (v.pos.x.to_bits(), v.pos.y.to_bits(), v.bulge.to_bits()))
            .collect()
    };
    let cases: [PinnedCase; 5] = [
        (
            "line_arc_internal",
            line_arc_internal(0.5).expect("fits"),
            &[
                (0, 0, 0),
                (4609047870845172685, 0, 4602837688965596815),
                (
                    4611170888069347941,
                    4604180019048437076,
                    4599397266714018680,
                ),
                (0, 4611686018427387904, 0),
            ],
        ),
        (
            "line_arc_external",
            line_arc_external(0.5).expect("fits"),
            &[
                (0, 0, 0),
                (4609820566382232627, 0, 13822769303568794489),
                (
                    4611814801016897895,
                    13823048456275842388,
                    4599397266714018680,
                ),
                (4613937818241073152, 13830554455654793216, 0),
                (4613937818241073152, 13835058055282163712, 0),
                (0, 13835058055282163712, 0),
            ],
        ),
        (
            "arc_line",
            arc_line(0.5).expect("fits"),
            &[
                (0, 4611686018427387904, 13823463879570942096),
                (
                    4611504036046923850,
                    4600877379321698714,
                    4600091842716166289,
                ),
                (4612698179346440494, 0, 0),
                (4616189618054758400, 0, 0),
                (4616189618054758400, 4613937818241073152, 0),
                (13830554455654793216, 4613937818241073152, 0),
            ],
        ),
        (
            "arc_arc_internal",
            arc_arc_internal(0.25).expect("fits"),
            &[
                (4607182418800017408, 0, 4598009223490746920),
                (
                    4594314991293244560,
                    4610070593513891235,
                    4599325607115144255,
                ),
                (
                    13817687028148020368,
                    4610070593513891235,
                    4598009223490746919,
                ),
                (13830554455654793216, 0, 0),
            ],
        ),
        (
            "arc_arc_mixed",
            arc_arc_mixed(0.25).expect("fits"),
            &[
                (4607182418800017408, 0, 4596857349751359594),
                (
                    4599676419421066584,
                    4609392389112809011,
                    13826831122041030757,
                ),
                (
                    4601392076421969630,
                    4611310551952855327,
                    13826067637668438915,
                ),
                (4613937818241073152, 0, 0),
            ],
        ),
    ];

    for (name, lp, want) in &cases {
        assert_eq!(
            &dump(lp)[..],
            *want,
            "{name}: the extracted seam moved a bit"
        );
    }
}

//! **The §2c arc-carrier fillet family** on line×arc, arc×line and
//! arc×arc corners: `fillet_arc`, `arc_fillet` and `arc_fillet_arc`,
//! whose corner is never named — it is DERIVED as the two authored
//! carriers' intersection, and each side is anchored by a real on-path
//! point (the entry anchor, the incoming ray's origin, the arrival
//! spec's `p`, or `Start`).
//!
//! The suite covers M5 S2's four acceptance groups in the algebra's
//! own vocabulary: one fixture per corner class
//! (each verifying that the constructed tangencies verify clean and the
//! loop closes and validates), the refusal rows of the taxonomy as
//! [`PathError`] variants, the definitely/exactly/in-band trio of every
//! named predicate the resolution fires, and the #100 bracket demo
//! shape re-expressed with an arc leg.
//!
//! Two shapes recur, because the surface has exactly two ways to end an
//! ARC arrival: `Center { .., p: Start }` closes the loop along the
//! arrival carrier (so a loop whose fillet arrives on an arc is authored
//! ROTATED, plain part first, fillet last), and an interior `Center`
//! anchor leaves the tip ON that carrier for the next fused verb. A
//! straight arrival keeps the ordinary binders (`.toward(..).to(p)`),
//! so an arc×line corner authors forward from the entry.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::{profile, tol};
use geom_core::Point2;
use profile::path::PathNoCornerReason;
use profile::{
    ArcSweep, Center, FilletLeg, FilletLegCarrier, NoCornerReason, Open, PathError, Profile,
    ProfileLoop, Start,
};
use geom_core::Tol;

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// √3 — the y coordinate of the crossing points of the unit-spaced
/// radius-2 circles used by the arc×arc fixtures.
fn s3() -> f64 {
    3.0f64.sqrt()
}

/// The named predicate an escalation came from (panics with the actual
/// error when it is not an escalation).
fn escalated_predicate(err: &PathError<f64>) -> &'static str {
    match err {
        PathError::Escalated { source } => source.predicate.unwrap_or("<unnamed>"),
        other => panic!("expected an escalation, got {other:?}"),
    }
}

/// Every declared joint of `lp` must survive validation (the #101
/// discipline: declared tangency is verified, never trusted). The
/// algebra's declarations are asserted in the *authoring* indexing;
/// validation re-indexes into canonical order, so only the count
/// survives the comparison there — what matters is that both
/// declarations verify (a contradicted one is a hard refusal).
fn validates_with_declared_joints(lp: ProfileLoop<f64>, expected: &[usize]) -> Profile<f64> {
    assert_eq!(lp.tangent_joints(), expected, "declared joints");
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
/// the arc side): a quarter disc whose straight/circular corner is
/// rounded with r = 1/2. The fillet's center sits inside the arrival
/// circle, so its offset carrier is the radius R − r circle.
///
/// The arrival is an ARC, so the loop is authored rotated: entry at
/// (0, 2), the straight run down to the origin, the incoming ray east,
/// and the fillet CLOSES along the circle about the origin — the
/// derived corner is (2, 0), where that ray meets that circle.
/// Vertex chain: (0,2) → (0,0) → T1 → T2 ⤾.
fn line_arc_internal(radius: f64) -> Result<ProfileLoop<f64>, PathError<f64>> {
    Open.at(p2(0.0, 2.0))
        .line_to(p2(0.0, 0.0), Tol::witness())?
        .toward(2.0, 0.0, Tol::witness())?
        .fillet_arc(
            radius,
            Center {
                c: p2(0.0, 0.0),
                winding: ArcSweep::Ccw,
                p: Start,
            },
            Tol::witness(),
        )
        .map(|closed| closed.loop_)
}

/// **line×arc, external tangency** (the fillet curves the other way
/// from the arc side): the derived corner at (2, 0) turns right onto a
/// circle centered at (3, 0), so the fillet's offset carrier is the
/// radius R + r circle. Rotated the same way as its internal twin.
/// Vertex chain: (3,-1) → (3,-2) → (0,-2) → (0,0) → T1 → T2 ⤾.
fn line_arc_external(radius: f64) -> Result<ProfileLoop<f64>, PathError<f64>> {
    Open.at(p2(3.0, -1.0))
        .line_to(p2(3.0, -2.0), Tol::witness())?
        .line_to(p2(0.0, -2.0), Tol::witness())?
        .line_to(p2(0.0, 0.0), Tol::witness())?
        .toward(2.0, 0.0, Tol::witness())?
        .fillet_arc(
            radius,
            Center {
                c: p2(3.0, 0.0),
                winding: ArcSweep::Ccw,
                p: Start,
            },
            Tol::witness(),
        )
        .map(|closed| closed.loop_)
}

/// **arc×line**: the incoming side is the circular one (a concave notch
/// on the radius-2 circle about the origin), the arrival straight — so
/// the entry fused verb authors the arc side ON its carrier at (0, 2)
/// and the ordinary far-end anchor ends the straight side at (4, 0).
/// The derived corner is (2, 0). Vertex chain:
/// (0,2) → T1 → T2 → (4,0) → (4,3) → (-1,3) ⤾.
fn arc_line(radius: f64) -> Result<ProfileLoop<f64>, PathError<f64>> {
    Open.arc_fillet(
        Center {
            c: p2(0.0, 0.0),
            winding: ArcSweep::Cw,
            p: p2(0.0, 2.0),
        },
        radius,
        Tol::witness(),
    )?
    .toward(1.0, 0.0, Tol::witness())?
    .to(p2(4.0, 0.0), Tol::witness())?
    .line_to(p2(4.0, 3.0), Tol::witness())?
    .line_to(p2(-1.0, 3.0), Tol::witness())?
    .line_to(Start, Tol::witness())
    .map(|closed| closed.loop_)
}

/// **arc×arc, both tangencies internal**: the vesica of the two
/// radius-2 circles about (±1, 0), with its top corner (0, √3) rounded
/// and the loop closed by the STRAIGHT diameter between the two
/// anchors — the sharp-after-arc-arrival shape the §2c dissolution
/// restored: the arrival lands on an ordinary directed point at
/// (-1, 0), and `line_to(Start)` is the sharp seam. Both sides wind
/// counterclockwise, so σ·τ = +1 on both and both offset carriers are
/// R − r circles. Authored coordinates are the raw-builder fixture's,
/// verbatim.
fn arc_arc_internal(radius: f64) -> Result<ProfileLoop<f64>, PathError<f64>> {
    Open.arc_fillet_arc(
        Center {
            c: p2(-1.0, 0.0),
            winding: ArcSweep::Ccw,
            p: p2(1.0, 0.0),
        },
        radius,
        Center {
            c: p2(1.0, 0.0),
            winding: ArcSweep::Ccw,
            p: p2(-1.0, 0.0),
        },
        Tol::witness(),
    )?
    .line_to(Start, Tol::witness())
    .map(|closed| closed.loop_)
}

/// **arc×arc, one internal + one external**: the same crossing circles,
/// but the outgoing leg winds the other way, so the fillet is external
/// to the first carrier (R + r) and internal to the second (R − r).
fn arc_arc_mixed(radius: f64) -> Result<ProfileLoop<f64>, PathError<f64>> {
    Open.arc_fillet_arc(
        Center {
            c: p2(-1.0, 0.0),
            winding: ArcSweep::Ccw,
            p: p2(1.0, 0.0),
        },
        radius,
        Center {
            c: p2(1.0, 0.0),
            winding: ArcSweep::Cw,
            p: p2(3.0, 0.0),
        },
        Tol::witness(),
    )?
    .line_to(Start, Tol::witness())
    .map(|closed| closed.loop_)
}

/// The line×arc corner at the radius that consumes BOTH sides exactly
/// (r = 1): no lead-in piece, no lead-out piece, no declarations — the
/// fillet arc alone, closed straight. A half disc.
fn exact_fit_line_arc() -> ProfileLoop<f64> {
    line_arc_internal(1.0).expect("the exact-fit radius constructs")
}

#[test]
fn line_arc_internal_validates_with_declared_tangency() {
    let lp = line_arc_internal(0.5).expect("the fillet fits");
    // (0,2) → (0,0) → T1 on y = 0 → fillet arc → T2 on the circle ⤾.
    assert_eq!(lp.vertices().len(), 4);
    // T1 = (√2, 0) exactly: x² = (2−r)² − r² = 2 at r = 1/2.
    assert!((lp.vertices()[2].pos().x - 2.0f64.sqrt()).abs() < 1e-15);
    assert_eq!(lp.vertices()[2].pos().y, 0.0);
    // T2 sits on the arrival carrier to rounding — tangency by
    // construction.
    let t2 = lp.vertices()[3].pos();
    // The claim is that T2 lies on the carrier, so measure THAT — the
    // radial residual — and not its square. |t|² − R² is 2R times the
    // radial error, so on this R = 2 carrier the squared form silently
    // asserts a quarter-ulp bound that no correctly-rounded construction
    // can meet. The residual below is 1 ulp of 2.0.
    assert!(
        (t2.x.hypot(t2.y) - 2.0).abs() <= 2.0 * f64::EPSILON,
        "T2 off the carrier by {:e}",
        (t2.x.hypot(t2.y) - 2.0).abs()
    );
    validates_with_declared_joints(lp, &[2, 3]);
}

#[test]
fn line_arc_external_validates_with_declared_tangency() {
    let lp = line_arc_external(0.5).expect("the fillet fits");
    assert_eq!(lp.vertices().len(), 6);
    let t2 = lp.vertices()[5].pos();
    // On the arrival side's carrier (center (3,0), R = 1).
    assert!(((t2.x - 3.0).powi(2) + t2.y.powi(2) - 1.0).abs() < 1e-15);
    validates_with_declared_joints(lp, &[4, 5]);
}

#[test]
fn arc_line_validates_with_declared_tangency() {
    let lp = arc_line(0.5).expect("the fillet fits");
    assert_eq!(lp.vertices().len(), 6);
    let t1 = lp.vertices()[1].pos();
    // T1 on the incoming side's carrier (origin, R = 2).
    assert!((t1.x.powi(2) + t1.y.powi(2) - 4.0).abs() < 1e-15);
    // T2 on the straight arrival side y = 0.
    assert!(lp.vertices()[2].pos().y.abs() < 1e-15);
    validates_with_declared_joints(lp, &[1, 2]);
}

#[test]
fn arc_arc_internal_validates_with_declared_tangency() {
    let lp = arc_arc_internal(0.5).expect("the fillet fits");
    let t1 = lp.vertices()[1].pos();
    let t2 = lp.vertices()[2].pos();
    assert!(((t1.x + 1.0).powi(2) + t1.y.powi(2) - 4.0).abs() < 1e-14);
    assert!(((t2.x - 1.0).powi(2) + t2.y.powi(2) - 4.0).abs() < 1e-14);
    validates_with_declared_joints(lp, &[1, 2]);
}

#[test]
fn arc_arc_mixed_validates_with_declared_tangency() {
    let lp = arc_arc_mixed(0.5).expect("the fillet fits");
    let t1 = lp.vertices()[1].pos();
    let t2 = lp.vertices()[2].pos();
    assert!(((t1.x + 1.0).powi(2) + t1.y.powi(2) - 4.0).abs() < 1e-14);
    assert!(((t2.x - 1.0).powi(2) + t2.y.powi(2) - 4.0).abs() < 1e-14);
    validates_with_declared_joints(lp, &[1, 2]);
}

// ------------------------------------- the #100 bracket regression anchor

/// The #100 demo bracket with its incoming side re-expressed as a
/// shallow arc (carrier: center (2, −2), R = √10, through both the
/// entry (3, 1) and the derived corner (1, 1)) — the same L with the
/// same r = 1/2 inner fillet, authored through the arc-carrier door.
///
/// The straight arrival lets the loop be authored forward from the arc
/// side's own anchor: entry ON the carrier at (3, 1), fillet, the
/// arrival side ending at (1, 3), then the plain run back to the seam.
#[test]
fn bracket_with_an_arc_leg_validates_and_declares() {
    let lp = Open
        .arc_fillet(
            Center {
                c: p2(2.0, -2.0),
                winding: ArcSweep::Ccw,
                p: p2(3.0, 1.0),
            },
            0.5,
            Tol::witness(),
        )
        .expect("the arc-carrier bracket fillet fits")
        .toward(0.0, 1.0, Tol::witness())
        .unwrap()
        .to(p2(1.0, 3.0), Tol::witness())
        .unwrap()
        .line_to(p2(0.0, 3.0), Tol::witness())
        .unwrap()
        .line_to(p2(0.0, 0.0), Tol::witness())
        .unwrap()
        .line_to(p2(3.0, 0.0), Tol::witness())
        .unwrap()
        .line_to(Start, Tol::witness())
        .unwrap()
        .loop_;
    // Same chain shape as the straight-leg bracket, rotated onto the
    // arc side's anchor: (3,1), T1, T2, (1,3), (0,3), (0,0), (3,0).
    assert_eq!(lp.vertices().len(), 7);
    // T2 sits on the straight arrival side x = 1, above the corner.
    assert_eq!(lp.vertices()[2].pos().x, 1.0);
    assert!(lp.vertices()[2].pos().y > 1.0);
    // T1 sits on the incoming side's carrier.
    let t1 = lp.vertices()[1].pos();
    assert!(((t1.x - 2.0).powi(2) + (t1.y + 2.0).powi(2) - 10.0).abs() < 1e-14);
    validates_with_declared_joints(lp, &[1, 2]);
}

// ------------------------------------------------- the refusal taxonomy

#[test]
fn oversized_radius_on_an_arc_side_names_the_carrier_and_angular_margin() {
    // The arrival arc side is only 10° long; r = 1/2 sets the tangent
    // point back further than that, so the trim would eat the anchor.
    let short = 10.0f64.to_radians();
    let err = Open
        .at(p2(0.0, 0.0))
        .toward(2.0, 0.0, Tol::witness())
        .unwrap()
        .fillet_arc(
            0.5,
            Center {
                c: p2(0.0, 0.0),
                winding: ArcSweep::Ccw,
                p: p2(2.0 * short.cos(), 2.0 * short.sin()),
            },
            Tol::witness(),
        )
        .expect_err("the short arc side must refuse");
    match err {
        PathError::AnchorOutsideTrimmedExtent {
            side,
            carrier:
                FilletLegCarrier::Arc {
                    radius,
                    angular_margin,
                },
            setback,
            available,
        } => {
            assert_eq!(side, FilletLeg::Outgoing);
            assert!((radius - 2.0).abs() < 1e-15, "carrier radius {radius}");
            assert!(angular_margin < 0.0, "angular margin {angular_margin}");
            assert!(setback > available, "{setback} vs {available}");
            // The angular margin is the arc-length margin over R.
            assert!((angular_margin - (available - setback) / 2.0).abs() < 1e-15);
        }
        other => panic!("expected an arc-side AnchorOutsideTrimmedExtent, got {other:?}"),
    }
}

#[test]
fn oversized_radius_on_a_straight_side_still_names_the_straight_carrier() {
    // Same corner class, but the STRAIGHT side is the short one: its
    // ray origin sits 1/10 short of the derived corner.
    let err = Open
        .at(p2(1.9, 0.0))
        .toward(1.0, 0.0, Tol::witness())
        .unwrap()
        .fillet_arc(
            0.5,
            Center {
                c: p2(0.0, 0.0),
                winding: ArcSweep::Ccw,
                p: p2(0.0, 2.0),
            },
            Tol::witness(),
        )
        .expect_err("the short straight side must refuse");
    match err {
        PathError::AnchorOutsideTrimmedExtent { side, carrier, .. } => {
            assert_eq!(side, FilletLeg::Incoming);
            assert_eq!(carrier, FilletLegCarrier::Line);
        }
        other => panic!("expected a straight-side AnchorOutsideTrimmedExtent, got {other:?}"),
    }
}

#[test]
fn radius_too_large_for_the_corner_has_no_tangent_circle() {
    // The offset line y = r and the offset circle of radius 2 − r meet
    // only while r ≤ 1; beyond that no circle of radius r is tangent to
    // both carriers anywhere. The constructor door's own vocabulary is
    // carried through rather than flattened.
    match line_arc_internal(1.5).expect_err("no tangent circle exists") {
        PathError::NoCornerForFillet { reason, radius } => {
            assert_eq!(
                reason,
                PathNoCornerReason::NoTangentCircle(NoCornerReason::OffsetCarriersDisjoint)
            );
            assert_eq!(radius, 1.5);
        }
        other => panic!("expected NoCornerForFillet, got {other:?}"),
    }
}

#[test]
fn a_negative_radius_is_refused_by_the_sign_gate_at_the_verb() {
    // r = 0 degenerates the arc and r < 0 mirrors the tangent points
    // past the corner: either way the declared-tangent construction the
    // fillet promises does not exist, so the radius is classified
    // through the funnel at the fillet verb itself, before an arrival
    // can be authored against it.
    match line_arc_external(-0.5).expect_err("a negative radius must refuse") {
        PathError::NonpositiveFilletRadius { radius } => assert_eq!(radius, -0.5),
        other => panic!("expected NonpositiveFilletRadius, got {other:?}"),
    }
}

/// `NoCornerSideCandidate` on an **arc×arc** corner — unreachable before
/// the signed-setback fix (review MAJOR-1), because an arc side's
/// setback was reduced into [0, 2π) and so could never classify
/// Negative.
///
/// The corner is the origin, where a radius-1 carrier about (0, −1)
/// meets a radius-1/2 carrier about (1/2, 0). Both sides wind
/// counterclockwise, so the path arrives travelling −x and leaves
/// travelling −y (a left turn, σ = +1). A radius-2 fillet is larger
/// than *both* carriers, so both offset radii go negative (ρ₁ = −1,
/// ρ₂ = −3/2) and the offset circles still meet — twice. But a circle
/// that big can only touch these two small carriers on the far side
/// from the origin, so both tangent circles reach their sides past the
/// corner and neither is a candidate.
#[test]
fn an_arc_arc_corner_can_have_no_corner_side_candidate() {
    // Anchors one radian along each carrier, so both sides have real
    // extent and the arm/turn gates pass cleanly.
    let along = |cx: f64, cy: f64, r: f64, delta: f64| {
        let a = f64::atan2(-cy, -cx) + delta;
        p2(cx + r * a.cos(), cy + r * a.sin())
    };
    let err = Open
        .arc_fillet_arc(
            Center {
                c: p2(0.0, -1.0),
                winding: ArcSweep::Ccw,
                p: along(0.0, -1.0, 1.0, -1.0),
            },
            2.0,
            Center {
                c: p2(0.5, 0.0),
                winding: ArcSweep::Ccw,
                p: along(0.5, 0.0, 0.5, 1.0),
            },
            Tol::witness(),
        )
        .expect_err("every tangent circle of radius 2 touches past the corner");
    match err {
        PathError::NoCornerForFillet { reason, radius } => {
            // NOT OffsetCarriersDisjoint — the offset carriers do meet.
            assert_eq!(
                reason,
                PathNoCornerReason::NoTangentCircle(NoCornerReason::NoCornerSideCandidate)
            );
            assert_eq!(radius, 2.0);
        }
        other => panic!("expected NoCornerSideCandidate, got {other:?}"),
    }
    // The refusal renders the constructor door's own sentence: the
    // radius fits nowhere on the corner SIDE of either carrier.
    assert!(
        err.to_string().contains("past the corner"),
        "recourse: {err}"
    );
}

/// The picked fillet circle of a 3-vertex vesica chain (entry, T1, T2),
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

/// The vesica of the two radius-2 circles about (±1, 0), authored as ONE
/// fused act: entry ON the `centre_in` lobe's carrier at `entry`, a
/// fillet of radius `r`, and an arrival that closes back along the lobe
/// about (1, 0). Both carriers pass through the entry anchor by
/// construction, so the crossing AT that anchor is derived and then
/// discarded by the advance gate — the surviving corner is the far
/// crossing, which is never authored.
fn vesica_lens(
    entry: Point2<f64>,
    centre_in: Point2<f64>,
    w: ArcSweep,
    r: f64,
) -> ProfileLoop<f64> {
    Open.arc_fillet_arc(
        Center {
            c: centre_in,
            winding: w,
            p: entry,
        },
        r,
        Center {
            c: p2(1.0, 0.0),
            winding: w,
            p: Start,
        },
        Tol::witness(),
    )
    .expect("the lens constructs")
    .loop_
}

/// The S8 ruling flips the M5 S2 refusal: with both sides long enough
/// to contain both tangent circles the corner at (0, √3) PICKS the
/// candidate nearest it — the top pocket — instead of refusing
/// `AmbiguousFilletBranch` (retired). Exact tangency asserted; both
/// junctions declared and verified.
#[test]
fn two_corner_side_candidates_pick_the_near_one() {
    let lp = vesica_lens(p2(0.0, -s3()), p2(-1.0, 0.0), ArcSweep::Ccw, 0.5);
    // entry, T1, T2: trimmed incoming run, fillet arc, closing run.
    assert_eq!(lp.vertices().len(), 3);
    let t1 = lp.vertices()[1].pos();
    let t2 = lp.vertices()[2].pos();
    // Tangent points exactly on their carriers (tangency by
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
/// corner. Entering at (0, √3) and winding the other way derives the
/// crossing at (0, −√3) as the corner, and yields the circle that was
/// the far candidate of the original corner — the recovered center is
/// asserted to agree with (0, −√(1.5² − 1)) to 1e-14, and the tangent
/// points to lie on their carriers to 1e-14 (the same closed form up to
/// rounding, not bit-identity: the two authorings mirror the
/// arithmetic, which f64 does not commute with exactly).
#[test]
fn the_far_pocket_is_authored_as_the_other_corners_near_fillet() {
    let lp = vesica_lens(p2(0.0, s3()), p2(-1.0, 0.0), ArcSweep::Cw, 0.5);
    assert_eq!(lp.vertices().len(), 3);
    let t1 = lp.vertices()[1].pos();
    let t2 = lp.vertices()[2].pos();
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

/// S8 rung-3 pin at the fused door: the symmetric lens is the
/// configuration whose candidate-swapping symmetry class motivates the
/// documented enumeration-order residual, and the whole selection must
/// be bit-deterministic run to run (the interval-lane twin asserts the
/// same pick from the other lane).
#[test]
fn symmetric_lens_pick_is_bit_deterministic_across_runs() {
    let build = || vesica_lens(p2(0.0, -s3()), p2(-1.0, 0.0), ArcSweep::Ccw, 0.5);
    let a = build();
    let b = build();
    assert_eq!(a.tangent_joints(), b.tangent_joints());
    assert_eq!(a.vertices().len(), b.vertices().len());
    for (va, vb) in a.vertices().iter().zip(b.vertices()) {
        assert_eq!(va.pos().x.to_bits(), vb.pos().x.to_bits());
        assert_eq!(va.pos().y.to_bits(), vb.pos().y.to_bits());
        assert_eq!(va.bulge().to_bits(), vb.bulge().to_bits());
    }
}

/// The hairline-asymmetric lens (S8 review MINOR-1). The corner is
/// DERIVED here, so the ulp of asymmetry is authored where the algebra
/// admits one — on a carrier CENTRE, which moves the derived corner off
/// the vesica's mirror axis by the same order. What is pinned is each
/// lane's OWN determinism — bit-identical output across runs and a
/// definite pocket committed to — NOT cross-lane agreement: a setback
/// gap below the interval channel's enclosure width may legally resolve
/// to the other pocket at Interval, and per the ruling both candidates
/// are valid fillets of the authored carriers. THE INTERVAL TWIN IS
/// GONE and the claim now rests on this lane alone: the fused door
/// derives its corners, a lens' carriers cross at BOTH tips, so the
/// entry anchor is itself a candidate whose zero advance straddles the
/// `signed_swept` fold on an enclosure — the Interval lane escalates
/// before any pick exists to pin
/// (`interval_lane::vesica_near_pick_escalates_at_interval_on_the_coincident_candidate`
/// pins that escalation, by predicate).
#[test]
fn ulp_perturbed_lens_pick_is_deterministic_within_the_lane() {
    let build = || {
        vesica_lens(
            p2(0.0, -s3()),
            p2(-1.0 + f64::EPSILON, 0.0),
            ArcSweep::Ccw,
            0.5,
        )
    };
    let a = build();
    let b = build();
    assert_eq!(a.tangent_joints(), b.tangent_joints());
    assert_eq!(a.vertices().len(), b.vertices().len());
    for (va, vb) in a.vertices().iter().zip(b.vertices()) {
        assert_eq!(va.pos().x.to_bits(), vb.pos().x.to_bits());
        assert_eq!(va.pos().y.to_bits(), vb.pos().y.to_bits());
        assert_eq!(va.bulge().to_bits(), vb.bulge().to_bits());
    }
    // One pocket was definitely committed to (which one is the lane's
    // own business).
    assert!(a.vertices()[1].pos().y.abs() > 0.5);
}

#[test]
fn an_already_tangent_corner_asks_for_the_declaration_instead() {
    // The line y = 0 is tangent to the circle about (2,2) at (2,0):
    // the carriers touch, so there is no corner to cut. One refusal for
    // the whole tangent/anti-tangent class — a doubled-back contact is
    // the same "no corner exists" story, not a second arm.
    let err = Open
        .at(p2(0.0, 0.0))
        .toward(2.0, 0.0, Tol::witness())
        .unwrap()
        .fillet_arc(
            0.5,
            Center {
                c: p2(2.0, 2.0),
                winding: ArcSweep::Ccw,
                p: p2(4.0, 2.0),
            },
            Tol::witness(),
        )
        .expect_err("a tangent corner must refuse");
    match err {
        PathError::NoCornerForFillet { reason, radius } => {
            assert_eq!(reason, PathNoCornerReason::CarriersParallel);
            assert_eq!(radius, 0.5);
        }
        other => panic!("expected NoCornerForFillet, got {other:?}"),
    }
    // The recourse is the structural tangency, not a wider fillet.
    assert!(err.to_string().contains(".tangent()"), "recourse: {err}");
}

#[test]
fn a_side_with_no_extent_is_refused_before_any_angle_is_classified() {
    // The incoming ray STARTS at the derived corner, so the corner is
    // not ahead of the side being authored and there is no lever arm to
    // meter the turn at (D4 ¶1).
    let err = Open
        .at(p2(2.0, 0.0))
        .toward(1.0, 0.0, Tol::witness())
        .unwrap()
        .fillet_arc(
            0.5,
            Center {
                c: p2(0.0, 0.0),
                winding: ArcSweep::Ccw,
                p: p2(0.0, 2.0),
            },
            Tol::witness(),
        )
        .expect_err("a zero-extent side must refuse");
    match err {
        PathError::NoCornerForFillet { reason, radius } => {
            assert_eq!(reason, PathNoCornerReason::BehindIncomingRay);
            assert_eq!(radius, 0.5);
        }
        other => panic!("expected NoCornerForFillet, got {other:?}"),
    }
}

#[test]
fn an_arc_side_with_no_extent_is_refused_the_same_way() {
    // The straight row above collapses the incoming ray onto the
    // corner; these two collapse an ARC side the two ways the algebra
    // can see — an incoming ray that leaves the arrival carrier no
    // reachable corner, and an arrival anchored at its own carrier's
    // CENTRE (a carrier with no radius, whose tangent direction is
    // poison). Both must come out as typed refusals, never a panic and
    // never a classification taken on a collapsed lever arm.
    let empty_sweep = Open
        .at(p2(0.0, 0.0))
        .toward(2.0, 0.0, Tol::witness())
        .unwrap()
        .fillet_arc(
            0.5,
            Center {
                // The ray STARTS at the arrival carrier's centre, so
                // every candidate corner sits behind the ray's own
                // origin once the offset carriers are taken.
                c: p2(0.0, 0.0),
                winding: ArcSweep::Ccw,
                p: p2(2.0, 0.0),
            },
            Tol::witness(),
        )
        .expect_err("an empty arrival extent must refuse");
    match empty_sweep {
        PathError::NoCornerForFillet { reason, radius } => {
            assert_eq!(reason, PathNoCornerReason::BehindIncomingRay);
            assert_eq!(radius, 0.5);
        }
        other => panic!("expected NoCornerForFillet, got {other:?}"),
    }
    assert!(
        empty_sweep
            .to_string()
            .contains("behind the incoming ray's start"),
        "recourse: {empty_sweep}"
    );
    let zero_radius = Open
        .at(p2(0.0, 0.0))
        .toward(2.0, 0.0, Tol::witness())
        .unwrap()
        .fillet_arc(
            0.5,
            Center {
                // The carrier's centre IS its anchor: R = 0, so the
                // winding selects no tangent.
                c: p2(2.0, 0.0),
                winding: ArcSweep::Ccw,
                p: p2(2.0, 0.0),
            },
            Tol::witness(),
        )
        .expect_err("a zero-radius arrival carrier must refuse");
    match zero_radius {
        PathError::DegenerateArcCenter { radius } => assert_eq!(radius, 0.0),
        other => panic!("expected DegenerateArcCenter, got {other:?}"),
    }
}

// ------------------------ the definitely / exactly / in-band predicate trios
//
// The exact-order predicates (`fillet_leg_fit`, `fillet_leg_reach`) have
// no in-band row at `f64` by construction — no representable `f64` lies
// strictly inside the hairline band — so their in-band rows live in the
// interval lane (tests/interval_lane.rs), where a straddling enclosure
// escalates honestly. Everything else has all three rows here.
//
// The algebra gates the DERIVED corner before it reaches the shared
// construction, so two of the constructor door's gates are shadowed by
// an earlier one that meters the same quantity: an in-band incoming
// extent escalates at `path_corner_advance` (never at
// `fillet_corner_arm`), and an in-band near-tangency escalates at
// `path_carrier_meet` (never at `fillet_corner_turn`) — the carriers
// have to MEET before there is a corner whose turn could be metered.

/// ε-relative offsets: `5·ε` sits strictly inside the run's ambiguity
/// band (ε, K·ε) at the default K = 10.
fn in_band() -> f64 {
    5.0 * tol().eps()
}

#[test]
fn corner_advance_trio() {
    // definitely: every fixture above classifies the advance positive.
    assert!(line_arc_internal(0.5).is_ok());
    // exactly zero: the zero-extent row above (`BehindIncomingRay`).
    // in-band: a ray origin 5ε short of the derived corner.
    let tiny = in_band();
    let err = Open
        .at(p2(2.0 - tiny, 0.0))
        .toward(1.0, 0.0, Tol::witness())
        .unwrap()
        .fillet_arc(
            0.5,
            Center {
                c: p2(0.0, 0.0),
                winding: ArcSweep::Ccw,
                p: p2(0.0, 2.0),
            },
            Tol::witness(),
        )
        .expect_err("an in-band advance must escalate");
    assert_eq!(escalated_predicate(&err), "path_corner_advance");
    // The two-tolerance discipline: the escalation renders the shared
    // sub-ε_input recourse, one situation, one sentence.
    assert!(
        err.to_string().contains("lower the tolerance"),
        "recourse: {err}"
    );
}

#[test]
fn carrier_meet_trio() {
    // definitely: the fixtures — ray and circle cross transversally.
    // exactly zero: the tangent-carrier row (`CarriersParallel`).
    // in-band: push the circle δ toward the ray so the two meet by a
    // margin R − |offset| = δ inside (ε, K·ε).
    let delta = in_band();
    let err = Open
        .at(p2(0.0, 0.0))
        .toward(2.0, 0.0, Tol::witness())
        .unwrap()
        .fillet_arc(
            0.5,
            Center {
                c: p2(2.0, 2.0 - delta),
                winding: ArcSweep::Ccw,
                p: p2(4.0, 2.0 - delta),
            },
            Tol::witness(),
        )
        .expect_err("a hairline-crossing carrier pair must escalate");
    assert_eq!(escalated_predicate(&err), "path_carrier_meet");
    assert!(
        err.to_string().contains("lower the tolerance"),
        "recourse: {err}"
    );
}

#[test]
fn fillet_offset_line_circle_trio() {
    // definitely: r = 1/2 leaves margin (2 − r) − r = 1.
    assert!(line_arc_internal(0.5).is_ok());
    // exactly zero: r = 1 makes the offset carriers tangent — ONE
    // candidate, and it is an exact fit on both sides, so the chain is
    // the entry, the authored ray origin, and the fillet arc closing
    // over both (a half disc).
    let lp = exact_fit_line_arc();
    assert_eq!(lp.vertices().len(), 2, "no lead-in, no lead-out: {lp:?}");
    profile(vec![lp])
        .validate(tol())
        .expect("the exact-offset fillet validates");
    // in-band: (2 − r) − r = 5ε.
    let err = line_arc_internal(0.5f64.mul_add(-in_band(), 1.0))
        .expect_err("an in-band offset clearance must escalate");
    assert_eq!(escalated_predicate(&err), "fillet_offset_line_circle");
    assert!(
        err.to_string().contains("lower the tolerance"),
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
    let lp = arc_arc_internal(1.0).expect("the tangent-offset case constructs");
    assert_eq!(lp.vertices().len(), 2, "no lead-in, no lead-out: {lp:?}");
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
    let lp = arc_arc_mixed(1.0).expect("the tangent-offset case constructs");
    assert_eq!(lp.vertices().len(), 2, "no lead-in, no lead-out: {lp:?}");
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
    // exactly zero: r = 1 on the line×arc corner consumes the straight
    // side and the arc side EXACTLY — neither trimmed piece is emitted
    // and neither tangent point is declared (the same three-way rule the
    // straight-side resolution documents).
    let lp = exact_fit_line_arc();
    assert_eq!(lp.vertices().len(), 2);
    assert!(lp.tangent_joints().is_empty(), "{:?}", lp.tangent_joints());
    // The entry anchor and the ray's origin survive verbatim; the fillet
    // arc springs off the origin and closes on the entry.
    assert!((lp.vertices()[0].pos().y - 2.0).abs() < 1e-15);
    assert_eq!(lp.vertices()[0].pos().x, 0.0);
    assert_eq!(lp.vertices()[1].pos().x, 0.0);
    assert_eq!(lp.vertices()[1].pos().y, 0.0);
}

#[test]
fn a_zero_radius_fillet_is_refused_at_the_verb() {
    // `fillet_leg_reach`'s definite rows are the negative-radius row and
    // the arc×arc no-corner-side row. Its exactly-zero row — both
    // tangent points ON the corner — is no longer reachable from the
    // surface: r = 0 names no fillet arc, and the funnel gate
    // (`path_fillet_radius`) classifies it at the verb, before a corner
    // is ever derived. A degenerate arc is refused where it is
    // authored, not downstream at validation.
    match line_arc_internal(0.0).expect_err("r = 0 must refuse at the verb") {
        PathError::NonpositiveFilletRadius { radius } => assert_eq!(radius, 0.0),
        other => panic!("expected NonpositiveFilletRadius, got {other:?}"),
    }
}

/// **The extraction's bit-identity pin** (LIB-G2 §2). The candidate loop
/// and selection live in the shared, `Bounds`-free `arc_fillet_trims`
/// seam, with the S8 pick applied at the algebra's door — the
/// `line_line_fillet_trims` pattern one level up. The extraction is
/// behavior-preserving, and "preserving" here means BITWISE: the
/// literals below were captured from the pre-extraction build and must
/// keep reproducing exactly, for every corner class the seam carries.
///
/// A failure here is not a tolerance question. It means the shared seam
/// re-derived a quantity the shipped construction derived differently,
/// and every downstream differential fixture (the algebra's included)
/// rests on these bits.
///
/// The fused verbs emit the same vertices in a ROTATED order wherever
/// the arrival is an arc (the arrival run is the closing segment), so
/// the pinned rows below are the captured tuples rotated onto each
/// fixture's authoring origin — the same (x, y, bulge) triples, in the
/// order the chain now emits them.
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
        lp.vertices()
            .iter()
            .map(|v| {
                (
                    v.pos().x.to_bits(),
                    v.pos().y.to_bits(),
                    v.bulge().to_bits(),
                )
            })
            .collect()
    };
    let cases: [PinnedCase; 5] = [
        (
            "line_arc_internal",
            line_arc_internal(0.5).expect("fits"),
            &[
                (0, 4611686018427387904, 0),
                (0, 0, 0),
                (4609047870845172685, 0, 4602837688965596815),
                (
                    4611170888069347941,
                    4604180019048437076,
                    4599397266714018680,
                ),
            ],
        ),
        (
            "line_arc_external",
            line_arc_external(0.5).expect("fits"),
            &[
                (4613937818241073152, 13830554455654793216, 0),
                (4613937818241073152, 13835058055282163712, 0),
                (0, 13835058055282163712, 0),
                (0, 0, 0),
                (4609820566382232627, 0, 13822769303568794489),
                (
                    4611814801016897895,
                    13823048456275842388,
                    4599397266714018680,
                ),
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

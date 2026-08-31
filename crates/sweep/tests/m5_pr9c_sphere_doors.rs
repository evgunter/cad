//! M5 PR 9c, scope item 1 (containment/pierce half): the SPHERE doors
//! of `topo::boolean::point_in_solid`.
//!
//! Before this unit `point_in_solid::face_geo` resolved `{Plane,
//! Cylinder}` only and refused every other kind as `CorruptFace` — so
//! a sphere operand could not be classified at all, which is the door
//! the cylinder×sphere boolean arm stands on. The arm this unit lands
//! covers the CLOSED sphere class: the faces sharing one sphere
//! surface close on each other, so their union is the whole chart and
//! no per-face chart trim is needed. That is exactly the ball the M5
//! inventory mints (`revolve_ball`'s V2 E2 F2: two half-bands on one
//! sphere key, joined along the seam meridian and its angle-π copy).
//!
//! Rows here:
//!
//! - **containment**: centre `In`, far point `Out`, a point ON the
//!   sphere `OnBoundary` — through the ray sweep, on a body whose only
//!   faces are spherical (so every row exercises the new arm and
//!   nothing else);
//! - **the group-closure refusal**: a PARTIAL revolve leaves the
//!   sphere band bounded by planar fan walls, so the group is not
//!   closed and the door refuses `PartialSphereFace` — the
//!   construction row for the chart-trim frontier this unit does NOT
//!   retire (the S9 flip pattern: every refusal is re-pinned as its
//!   construction row);
//! - **multi-ε honesty**: every probe offset is derived from the
//!   RESOLVED band (`Tol::witness().get().eps`), never a literal, so the
//!   rows mean the same thing in each ε lane.
//!
//! The Interval lane lives in `m5_pr9c_sphere_doors_interval.rs`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod revolve_common;

use geom_core::Point3;
use geom_core::Tol;
use profile::RawLoop;
use profile::{ProfileLoop, ProfileVertex};
use revolve_common::*;
use sweep::{Revolution, revolve};
use topo::boolean::{PointInSolidError, SolidContainment, point_in_solid};

/// The half-disc of the `ball` acceptance: a unit semicircle from
/// (0, −1) through (1, 0) to (0, 1), closed by the on-axis diameter.
fn half_disc() -> ProfileLoop<f64> {
    ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, -1.0), 1.0),
        ProfileVertex::new(p2(0.0, 1.0), 0.0),
    ])
}

/// The unit ball: two half-sphere bands on ONE sphere surface.
fn ball() -> topo::Body<f64> {
    let vp = validated(vec![half_disc()]);
    revolve(&vp, axis_y(), Revolution::Full, Tol::witness())
        .unwrap()
        .body
}

fn band() -> geom_core::Band {
    geom_core::Band::linear(Tol::witness()).unwrap()
}

/// The probe offset: a multiple of the RESOLVED band, so each ε lane
/// probes at its own scale rather than at a hard-coded distance.
fn away() -> f64 {
    (1e6 * Tol::witness().get().eps).max(0.25)
}

#[test]
fn sphere_door_classifies_the_ball_interior_exterior_and_boundary() {
    let body = ball();
    assert_all_tiers(&body);
    // Nothing but sphere faces — every row below is the new arm.
    assert_eq!(body.surfaces().count(), 1);
    let b = band();

    // Interior: the centre.
    assert_eq!(
        point_in_solid(&body, Point3::origin(), b, Tol::witness()).unwrap(),
        SolidContainment::In
    );
    // Interior: a band-scaled step inside the wall, off every axis so
    // no schedule ray runs along a symmetry.
    let inside = 1.0 - away();
    let d = inside / 3.0_f64.sqrt();
    assert_eq!(
        point_in_solid(&body, Point3::new(d, d, d), b, Tol::witness()).unwrap(),
        SolidContainment::In
    );
    // Exterior: the same direction, a band-scaled step outside.
    let outside = 1.0 + away();
    let e = outside / 3.0_f64.sqrt();
    assert_eq!(
        point_in_solid(&body, Point3::new(e, e, e), b, Tol::witness()).unwrap(),
        SolidContainment::Out
    );
    // Far exterior: the at-infinity side, reached with no crossing at
    // all on some schedule members.
    assert_eq!(
        point_in_solid(&body, Point3::new(10.0, 7.0, 3.0), b, Tol::witness()).unwrap(),
        SolidContainment::Out
    );
    // ON the sphere: the boundary pre-pass, before any ray is cast.
    for p in [
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
        Point3::new(0.0, 0.0, -1.0),
    ] {
        assert_eq!(
            point_in_solid(&body, p, b, Tol::witness()).unwrap(),
            SolidContainment::OnBoundary,
            "on-sphere probe {p:?}"
        );
    }
}

/// The pierce arm's own row: a query the boundary pre-pass cannot
/// answer (definitely off every face) whose verdict comes from the
/// ray/sphere quadratic — checked in BOTH directions across the wall
/// along a direction that is not in the schedule.
#[test]
fn sphere_pierce_reads_the_material_side_from_the_discriminant() {
    let body = ball();
    let b = band();
    let r = away();
    // A tangentially-placed pair: just inside / just outside the wall
    // at a generic point of the sphere.
    let n = Point3::new(0.6, 0.48, 0.64); // |n| = 1 exactly enough
    let s = n.distance(Point3::origin());
    let unit = (n - Point3::origin()) / s;
    let inner = Point3::origin() + unit * (1.0 - r);
    let outer = Point3::origin() + unit * (1.0 + r);
    assert_eq!(
        point_in_solid(&body, inner, b, Tol::witness()).unwrap(),
        SolidContainment::In
    );
    assert_eq!(
        point_in_solid(&body, outer, b, Tol::witness()).unwrap(),
        SolidContainment::Out
    );
}

/// The construction row for the frontier PR 9c did not retire and the
/// sphere CHART TRIM since has: a TRIMMED sphere face. A partial
/// revolve caps the sphere band with planar fan walls, so the
/// sphere-surface group has a boundary against another surface — and
/// that boundary is two meridian great circles, which is a chart
/// iso-line class. The face is therefore exactly the
/// `[azimuth] × [latitude]` rectangle its boundary pins, and the door
/// classifies through it instead of refusing.
///
/// The row is kept, per the S9 pattern, as the record of the frontier
/// it used to pin — and it now pins where the quarter lune DOES still
/// stop, which is a different door entirely: a ray from outside a
/// quarter ball can miss the body, and the at-infinity verdict then
/// needs the body's signed volume, which the closed-form props lane
/// will not certify for a rimless band whose meridians lie on two
/// different great circles (that arm hardcodes the azimuthal width at
/// π). Typed, and naming the volume rather than the chart.
#[test]
fn a_trimmed_sphere_face_is_classified_through_its_chart_rectangle() {
    let vp = validated(vec![half_disc()]);
    let t = revolve(
        &vp,
        axis_y(),
        Revolution::Partial(std::f64::consts::FRAC_PI_2),
        Tol::witness(),
    )
    .unwrap();
    // Inside the swept quarter (the fan walls are x = 0 and z = 0, with
    // material at x > 0, z < 0).
    assert_eq!(
        point_in_solid(&t.body, Point3::new(0.3, 0.1, -0.3), band(), Tol::witness()).unwrap(),
        SolidContainment::In
    );
    // The at-infinity side, where the props lane stops.
    let err =
        point_in_solid(&t.body, Point3::new(-0.3, 0.1, 0.3), band(), Tol::witness()).unwrap_err();
    let PointInSolidError::VolumeUncertified = err else {
        panic!("expected the at-infinity volume refusal, got {err:?}");
    };
    let msg = err.to_string();
    assert!(msg.contains("HEALTHY"), "{msg}");
    assert!(msg.contains("Recourse"), "{msg}");
}

/// The construction rows for the two frontiers this unit EXECUTED and
/// did not retire (M5-LOG PR 9c deviations 3 and 6). Both messages were
/// rewritten from "banked as PR 9c" — a promise that has now been kept
/// or refuted — to the blocker actually found, and both are pinned here
/// so a later unit cannot quietly restore a stale claim.
///
/// **Re-aimed at M5 S10.** Deviation 3 has since been RULED and landed:
/// `Face::sense` (option (a)) closed the representation gap, so this
/// refusal no longer says "there is nothing to write". What it says now
/// is that the remaining gap is **wiring** — S10 deliberately stopped at
/// the contract plus the consumer audit, every constructor still mints
/// `sense: true`, and making `revert` flip the bit is the follow-on
/// unit. The row is renamed to match what it actually pins, and it now
/// guards in BOTH directions: the new wiring statement must be present,
/// and the retired unrepresentability claim must be gone (a later unit
/// re-refusing on the old grounds is the regression this catches).
///
/// The parity finding is still pinned, unchanged, because it is the
/// reason the fix had to be a representation change at all — and it is
/// pinned in its SCOPED form (PR 9c review, F1). The first draft
/// claimed every axisymmetric chart normal is "always outward"; that is
/// true of the cylinder, cone and torus, whose normals are ODD in the
/// radius, and FALSE of the sphere, whose `r²·cos v·n̂` is EVEN — a
/// negative-radius sphere evaluates with an INWARD normal. The
/// conclusion survives because that sphere is rejected as a
/// representation rather than adopted: it violates the `radius > 0`
/// convention and inverts every `2r`-metered consumer, this file's own
/// arm included. Keeping these assertions is what stops the S10 rewrite
/// (or any later one) from quietly eroding F1's correction back to the
/// overclaim.
#[test]
fn curved_revert_reverts_the_ball_instead_of_refusing() {
    let body = ball();
    let before: Vec<bool> = body.faces().map(|(_, f)| f.sense).collect();
    let v = topo::mass_properties(&body, Tol::witness()).unwrap().volume;

    let rev = body.revert().expect("M5 S12 wired the curved arm");
    let after: Vec<bool> = rev.faces().map(|(_, f)| f.sense).collect();
    assert_eq!(after, before.iter().map(|s| !s).collect::<Vec<_>>());
    // The sphere CHART is untouched — no negative radius anywhere near
    // this, which is exactly what the parity finding demanded.
    for (k, s) in body.surfaces() {
        assert_eq!(
            format!("{s:?}"),
            format!("{:?}", rev.get_surface(k).unwrap()),
            "revert must not perturb a sphere chart"
        );
        let Some(geom::Surface::Sphere { radius, .. }) = rev.get_surface(k) else {
            panic!("still a sphere");
        };
        assert!(*radius > 0.0, "the radius > 0 convention survives revert");
    }
    // Complement currency: tier-2 valid, tier 3 exactly NegativeVolume,
    // volume bit-negated, and a bitwise involution.
    assert_eq!(topo::validate_closed(&rev), Ok(()));
    assert_eq!(
        topo::validate_geometric(&rev, Tol::witness()),
        Err(vec![topo::ValidationError::NegativeVolume])
    );
    assert_eq!(
        topo::mass_properties(&rev, Tol::witness())
            .unwrap()
            .volume
            .to_bits(),
        (-v).to_bits()
    );
    assert_eq!(format!("{:?}", rev.revert().unwrap()), format!("{body:?}"));
}

/// **Re-pinned again at M5 S13 (was
/// `the_die_pips_shape_still_refuses_at_the_narrowed_per_class_door`,
/// before that `curved_subtract_front_door_quotes_the_same_finding`).**
/// PR 9c's own smoke shape — the unit ball at the origin bitten out of
/// a slab standing on z = 0 — now PASSES the per-class door (S13 opened
/// the sphere class: the (Plane, Sphere) germ arm plus the
/// extent-certified fallback re-cut, pinned green in
/// `m5_s13_pips.rs`/`m5_s12_curved_ops.rs`), and what stops THIS
/// particular placement is its own geometry: the ball is exactly
/// TANGENT to the slab's top-face carrier (center on z = 0, radius 1,
/// face at z = 1), a touching configuration the crossing layer cannot
/// represent. The extent scan refuses it TYPED — never the S12
/// finding's silent vertex-probe answer — which keeps this row's
/// charter: PR 9c's smoke shape pinned at whatever the current door
/// says. What that is: the ball's seam great circle lies EXACTLY IN
/// the bottom face's carrier plane (center on z = 0) — the coplanar
/// conic class, routed structurally to endpoint-only treatment (the
/// M3 rule; S13 fix pass) — so the reduction finds no crossings and
/// the extent scan takes over, where the ball is exactly TANGENT to
/// the TOP face's carrier (center z = 0, radius 1, face z = 1): the
/// scan's tangency arm refuses TYPED. (Nudge the ball off both
/// coincidences and the S13 lanes cut it — the pips suite.)
#[test]
fn the_die_pips_shape_now_stops_typed_at_its_own_tangency() {
    let slab = validated(vec![profile::ProfileLoop::polygon([
        p2(-2.0, -2.0),
        p2(2.0, -2.0),
        p2(2.0, 2.0),
        p2(-2.0, 2.0),
    ])]);
    let a = sweep::extrude(&slab, sweep::Extrusion::Distance(1.0), Tol::witness())
        .unwrap()
        .body;
    let b = ball();
    let err = topo::boolean::subtract(&a, &b, Tol::witness()).unwrap_err();
    let topo::BooleanError::FallbackExtentUnsupported { what, .. } = err else {
        panic!("expected the extent scan's tangency arm, got {err:?}");
    };
    assert!(what.contains("tangent"), "{what}");
    // The retired claims must be GONE from the surfaced text: revert is
    // wired, the gate is not wholesale, and the sphere class is no
    // longer refused as a class.
    let msg = err.to_string();
    assert!(!msg.contains("no representation"), "{msg}");
    assert!(!msg.contains("no seam lane"), "{msg}");
    assert!(msg.contains("refused typed"), "{msg}");
}

/// NOTE row (PR 9c review, F4): the TANGENT ray. A schedule direction
/// that grazes the sphere has a zero discriminant, and the arm must
/// abandon that ray rather than fabricate a parity verdict — the
/// retry schedule then answers from a different direction. The probe
/// is a query point placed so that the FIRST schedule member (+x) is
/// exactly tangent: sitting on the plane `y = r` with `z = 0` makes
/// the +x ray touch the unit sphere at the north-pole circle.
#[test]
fn tangent_schedule_ray_grazes_and_the_retry_answers() {
    let body = ball();
    let b = band();
    // On the tangent plane y = 1, definitely outside the sphere.
    let q = Point3::new(-3.0, 1.0, 0.0);
    // The verdict still resolves — a later schedule member is not
    // tangent — and it is the right one.
    assert_eq!(
        point_in_solid(&body, q, b, Tol::witness()).unwrap(),
        SolidContainment::Out
    );
}

/// NOTE row (PR 9c review, F4): a MULTI-SHELL sphere body. Two
/// DISJOINT balls in one body exercise the group-representative rule
/// across SURFACES as well as faces — each sphere key gets its own
/// group and its own representative, so a ray through both folds two
/// crossing pairs and the closest-hit rule still reads the material
/// side. The body is built by the live curved UNION of two disjoint
/// balls, whose no-intersection lane is decided by `point_in_solid`
/// itself: the row therefore also pins the new arm driving the
/// boolean's own containment fallback, not just a direct query.
#[test]
fn two_ball_body_classifies_each_shell_independently() {
    let a = ball();
    let b = topo::transform_rigid(
        &a,
        &geom_core::Affine3::translation(geom_core::Vec3::new(4.0, 0.0, 0.0)),
        Tol::witness(),
    )
    .unwrap();
    let result = topo::boolean::union(&a, &b, Tol::witness()).unwrap();
    let topo::BooleanResult::Body(bb) = result else {
        panic!("two disjoint balls union to a real body");
    };
    let body = bb.body;
    assert_eq!(body.surfaces().count(), 2, "one sphere key per ball");
    let bd = band();
    // Inside ball A, inside ball B, and in the gap between them.
    assert_eq!(
        point_in_solid(&body, Point3::origin(), bd, Tol::witness()).unwrap(),
        SolidContainment::In
    );
    assert_eq!(
        point_in_solid(&body, Point3::new(4.0, 0.0, 0.0), bd, Tol::witness()).unwrap(),
        SolidContainment::In
    );
    assert_eq!(
        point_in_solid(&body, Point3::new(2.0, 0.0, 0.0), bd, Tol::witness()).unwrap(),
        SolidContainment::Out
    );
}

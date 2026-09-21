//! M5 S10 acceptance: the face orientation bit (`topo::Face::sense`).
//!
//! S10 landed the ratified fix for PR 9c's contract gap — a face's
//! outward normal is its surface's chart normal negated where `sense`
//! is `false`,
//! rather than the chart normal outright. At S10 every constructor
//! still minted `sense: true`; what the acceptance rows prove is that
//! the outward-normal consumers **actually read the bit**. Since M5
//! S11 the sweep constructors mint the honest bit (`false` on walls
//! whose material lies against the chart normal — concave arc walls
//! and revolve's inward walls), and the S10 `finding_*` rows below
//! flipped to construction rows pinning the fixed behaviour.
//!
//! The instrument is `Body::flipped_face_sense_for_tests`, which
//! inverts ONE face's bit and nothing else. That is deliberately an
//! incoherent body — by the interior-left rule a face's outer loop
//! winds CCW about its *outward* normal, so flipping the bit alone
//! puts the two encodings of orientation (the bit and the winding)
//! into disagreement. Each row below picks a consumer and asks whether
//! it noticed.
//!
//! **Tolerance shape.** These rows are STRUCTURAL, in the sense of the
//! `PartialSphereFace` precedent (M5 PR 9c): the sense is a `bool`
//! selecting a negation, never a decided quantity, so no row here has
//! an ε-relative margin to sweep — the discriminations are exact
//! arithmetic sign changes and typed refusals. Where a row does need a
//! numeric comparison it is against an ANALYTIC constant with a
//! generous absolute slack, not against a band.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::operands::pellet;
use crate::revolve_common;

use core::f64::consts::{FRAC_PI_8, PI};
use profile::RawLoop;

use geom::Surface;
use geom_core::Tol;
use geom_core::{Band, Point2, Point3};
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use revolve_common::{axis_y, p2, validated};
use sweep::{Extrusion, Revolution, extrude, revolve};
use topo::boolean::point_in_solid;
use topo::{Body, FaceKey};

/// The unit ball centred at the origin: two half-sphere bands on ONE
/// sphere surface, each **rimless** (bounded by meridians only).
fn ball() -> Body<f64> {
    ball_at(0.0)
}

/// The unit ball centred at `(0, cy, 0)`, revolved from the PR 9c
/// `ball` acceptance's half-disc (a unit semicircle closed by its
/// on-axis diameter). `cy ≠ 0` is what the anchored-versus-vector-area
/// row needs: it makes the `c·A⃗` terms nonzero.
fn ball_at(cy: f64) -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, cy - 1.0), 1.0),
        ProfileVertex::new(p2(0.0, cy + 1.0), 0.0),
    ]);
    revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body
}

/// The first face of `body` in arena order.
fn first_face(body: &Body<f64>) -> FaceKey {
    body.faces().next().unwrap().0
}

// =====================================================================
// Row: the props sign flip (the rimless sphere band).
// =====================================================================

/// **Acceptance row (props).** The unit ball's two half-bands are the
/// one face population in the kernel whose flux sign the BOUNDARY does
/// not encode: a rimless band has no rim to recover `s_f` from, and
/// its two meridians are traversed identically whichever side is
/// material. `props/curved.rs` used to hardcode `s_f = +1` there,
/// justified by "M2 sweeps emit single outward shells only". S10 makes
/// an inward band representable, so the hardcode became
/// the face's `sense` bit — and this row is what proves it is read.
///
/// With both bands outward the ball meters `4π/3`. Flip ONE band's
/// sense and that band's anchored term changes sign; since the sphere
/// is centred on the origin the `c·A⃗` terms vanish and the two bands'
/// contributions are equal and opposite, so the metered volume
/// collapses to zero. If the bit were ignored the volume would not
/// move at all — the discrimination is total, not marginal.
#[test]
fn props_flux_flips_with_the_rimless_band_sense() {
    let b = ball();
    let honest = topo::props::mass_properties(&b, Tol::witness()).unwrap();
    assert!(
        (honest.volume - 4.0 * PI / 3.0).abs() < 1e-9,
        "the unit ball meters 4π/3, got {}",
        honest.volume
    );

    let flipped = b.flipped_face_sense_for_tests(first_face(&b)).unwrap();
    let lied = topo::props::mass_properties(&flipped, Tol::witness()).unwrap();
    assert!(
        lied.volume.abs() < 1e-9,
        "flipping one band's sense must negate its flux (the two halves \
         then cancel); got {} — props is still ignoring Face::sense",
        lied.volume
    );
    // The surface area is a norm and must NOT move: it is
    // sense-invariant, and a consumer that flipped it too would be
    // double-counting.
    assert!(
        (lied.surface_area - honest.surface_area).abs() < 1e-9,
        "surface area is orientation-free and must not follow the bit"
    );
}

/// **Adopted review row (charter B): the double-count blind spot.**
///
/// The row above uses an ORIGIN-CENTRED ball, where every anchored
/// term is `c·A⃗ = 0` identically. That makes it blind to the one
/// mistake this unit's discipline exists to prevent: it cannot
/// distinguish a correct SINGLE application of the sense from a
/// double-count that also negates the winding-derived vector-area
/// term, because both give zero.
///
/// An OFF-CENTRE ball separates them. Under the correct discipline the
/// flip touches only the rimless band's `s_f`, so the two bands'
/// anchored halves still cancel and the `c·A⃗` terms — untouched,
/// winding-derived, and summing to zero over a closed shell — leave
/// the total at zero. If the sense were ALSO applied to the vector
/// area, `2·c·A⃗_band ≠ 0` would survive. Zero here is therefore
/// evidence of single application, not merely of symmetry.
#[test]
fn offcentre_flipped_ball_still_cancels() {
    let b = ball_at(5.0);
    let honest = topo::props::mass_properties(&b, Tol::witness()).unwrap();
    assert!(
        (honest.volume - 4.0 * PI / 3.0).abs() < 1e-9,
        "the off-centre unit ball still meters 4π/3, got {}",
        honest.volume
    );

    let flipped = b.flipped_face_sense_for_tests(first_face(&b)).unwrap();
    let lied = topo::props::mass_properties(&flipped, Tol::witness()).unwrap();
    assert!(
        lied.volume.abs() < 1e-9,
        "off-centre flipped ball must STILL meter zero (anchored halves \
         cancel; the A⃗ terms are winding-derived and close). Got {} — a \
         nonzero here means the sense is ALSO reaching a winding-derived \
         term: the double-count this unit's discipline forbids",
        lied.volume
    );
}

/// **Adopted review row (charter E): wrong-but-NONZERO.**
///
/// Both ball rows land on zero, which is a suspiciously tidy answer —
/// a consumer that returned zero for some unrelated reason would pass
/// them. This row removes that escape: a disjoint assembly of the ball
/// and a cuboid, built through the PUBLIC `union`. Flipping one band
/// kills exactly the ball's `4π/3` and leaves the cuboid's `6` standing,
/// so the sense's effect is read off as a specific wrong number rather
/// than as an absence.
#[test]
fn assembly_flip_is_wrong_but_nonzero() {
    let ball = ball_at(0.0);
    let lp = <ProfileLoop<f64> as RawLoop<f64>>::polygon([
        p2(5.0, 0.0),
        p2(6.0, 0.0),
        p2(6.0, 2.0),
        p2(5.0, 2.0),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let cuboid = extrude(&vp, Extrusion::Distance(3.0), Tol::witness())
        .unwrap()
        .body;
    let r = topo::boolean::union(&ball, &cuboid, Tol::witness()).unwrap();
    let body = &r.body().expect("a disjoint assembly is a body").body;
    let honest = topo::props::mass_properties(body, Tol::witness()).unwrap();
    assert!(
        (honest.volume - (4.0 * PI / 3.0 + 6.0)).abs() < 1e-9,
        "the assembly meters ball + cuboid, got {}",
        honest.volume
    );

    let band_face = body
        .faces()
        .find(|(_, f)| matches!(body.get_surface(f.surface), Some(Surface::Sphere { .. })))
        .map(|(k, _)| k)
        .expect("the assembly keeps the ball's sphere bands");
    let flipped = body.flipped_face_sense_for_tests(band_face).unwrap();
    let lied = topo::props::mass_properties(&flipped, Tol::witness()).unwrap();
    assert!(
        (lied.volume - 6.0).abs() < 1e-9,
        "wrong-but-nonzero: the bands cancel and the cuboid stands, so \
         the flip must meter exactly 6; got {}",
        lied.volume
    );
}

/// **Adopted review row (charter E): the tier-3 refusal is SURGICAL.**
///
/// Row 1 asserts that check 6 fires. That is weaker than it looks: a
/// refusal that also threw incidental errors at every other face would
/// still satisfy it, and would mean the gate is detecting collateral
/// damage rather than the winding-vs-sense disagreement itself. This
/// row flips EVERY face of a cuboid in turn and requires that the
/// error set is non-empty and contains nothing but `LoopRoleInverted`
/// naming that exact face.
#[test]
fn tier_three_refusal_is_surgical() {
    let lp = <ProfileLoop<f64> as RawLoop<f64>>::polygon([
        p2(0.0, 0.0),
        p2(2.0, 0.0),
        p2(2.0, 1.0),
        p2(0.0, 1.0),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let body = extrude(&vp, Extrusion::Distance(1.0), Tol::witness())
        .unwrap()
        .body;
    assert_eq!(
        topo::validate::validate_geometric(&body, Tol::witness()),
        Ok(())
    );
    for (face, _) in body.faces() {
        let flipped = body.flipped_face_sense_for_tests(face).unwrap();
        let errs = topo::validate::validate_geometric(&flipped, Tol::witness()).unwrap_err();
        assert!(!errs.is_empty(), "face {face:?}: the flip must be refused");
        assert!(
            errs.iter().all(|e| matches!(
                e,
                topo::ValidationError::LoopRoleInverted { face: f, .. } if *f == face
            )),
            "face {face:?}: incidental errors leaked: {errs:?}"
        );
    }
}

// =====================================================================
// Construction rows (M5 S11): concave sweep walls mint sense: false.
// These began life as S10 `finding_*` rows pinning the pre-S11 defect
// (concave walls stamped `true`, the cylinder door misreporting the
// notch, `union` silently swallowing a disjoint pellet); S11 taught
// extrude the exact turn-sign criterion and the rows flipped to pin
// the CORRECT behaviour, per the S9 finding→construction pattern.
// =====================================================================

/// The mixed convex/concave extrusion of `review_m2_pr4`'s assignment
/// 5: a square whose bottom edge is a CONVEX arc and whose top edge is
/// a CONCAVE one (bowing into the region).
fn mixed_turn_arcs() -> sweep::Extruded<f64> {
    let b = FRAC_PI_8.tan();
    // Leaving bulges: the bottom arc bows out (+b), the top one bows
    // into the region (-b); the two sides are straight.
    let lp = <ProfileLoop<f64> as RawLoop<f64>>::new(vec![
        ProfileVertex::new(p2(0.0, 0.0), b),
        ProfileVertex::new(p2(2.0, 0.0), 0.0),
        ProfileVertex::new(p2(2.0, 1.5), -b),
        ProfileVertex::new(p2(0.0, 1.5), 0.0),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    extrude(&vp, Extrusion::Distance(1.0), Tol::witness()).unwrap()
}

/// **Construction row (M5 S11, flipped from S10's finding).**
///
/// `extrude` mints a cylinder wall for every arc segment, and the
/// cylinder's chart normal is unconditionally the **radially outward**
/// radial. For a CONCAVE arc the material lies OUTSIDE that cylinder,
/// so the face's outward normal is radially INWARD: the honest sense
/// is `false`, decided from the profile's stored turn sign against
/// the loop's canonical winding (S11 — never a numeric derivation).
///
/// This row pins the two halves of the fix:
///
/// 1. **Structure**: the concave wall (material outside its carrier,
///    `|interior probe − axis| > radius`) carries `sense: false`;
///    every other wall of the fixture keeps `true`.
/// 2. **Consequence**: `point_in_solid` — whose cylinder door reads
///    the chart normal with `sense` folded in as outward — now reports `Out`
///    throughout the notch the concave arc cuts. At `x = 1` the true
///    boundary is `y = 2.5 − √2 ≈ 1.0858`; before S11 the door did
///    not turn over until `y ≈ 1.5`.
#[test]
fn fixed_concave_arc_wall_sense_is_false() {
    let t = mixed_turn_arcs();
    // (1) The concave wall's material is OUTSIDE its cylinder, and
    // exactly that wall carries the reversed bit.
    let mut saw_concave = false;
    for &fk in &t.side_faces[0] {
        let sk = t.body.get_face(fk).unwrap().surface;
        let Surface::Cylinder { origin, radius, .. } = *t.body.get_surface(sk).unwrap() else {
            // Planar walls stay `true` (Newell-outward by
            // construction).
            assert!(t.body.get_face(fk).unwrap().sense);
            continue;
        };
        // The region's interior probe, in the sketch plane.
        let d = Point2::new(1.0, 0.75) - Point2::new(origin.x, origin.y);
        if d.norm() > radius {
            saw_concave = true;
            assert!(
                !t.body.get_face(fk).unwrap().sense,
                "the concave wall's material lies against the chart \
                 normal: S11 mints sense: false here"
            );
        } else {
            assert!(
                t.body.get_face(fk).unwrap().sense,
                "the convex wall keeps sense: true"
            );
        }
    }
    assert!(saw_concave, "the fixture must contain a concave arc wall");

    // (2) The consequence: point_in_solid is honest in the notch.
    let band = Band::linear(Tol::witness()).unwrap();
    let truth_hi = 2.5 - 2.0_f64.sqrt(); // ≈ 1.0858
    let inside = point_in_solid(&t.body, Point3::new(1.0, 0.5, 0.5), band, Tol::witness()).unwrap();
    assert_eq!(inside, topo::boolean::SolidContainment::In);
    for y in [1.2_f64, 1.3, 1.4] {
        assert!(y > truth_hi, "these probes are OUTSIDE the solid");
        assert_eq!(
            point_in_solid(&t.body, Point3::new(1.0, y, 0.5), band, Tol::witness()).unwrap(),
            topo::boolean::SolidContainment::Out,
            "the cylinder door reads the sense-signed radial as \
             outward, so the concave notch reads as void (S11)"
        );
    }
    // And still In just below the true arc boundary.
    assert_eq!(
        point_in_solid(
            &t.body,
            Point3::new(1.0, truth_hi - 0.05, 0.5),
            band,
            Tol::witness()
        )
        .unwrap(),
        topo::boolean::SolidContainment::In,
        "the fix must not overshoot: just inside the arc is material"
    );
}

/// **Construction row (M5 S11, e2e half — flipped from S10's
/// finding, per its own flip instruction).**
///
/// `union(notched, pellet)` on two DISJOINT solids has no surface
/// intersections, so the operation falls back to mutual containment
/// to decide the arrangement — and it asks `point_in_solid`. Before
/// S11 the concave wall's `sense: true` made the door answer `In` for
/// the pellet's points, so the PUBLIC `union` shipped a well-formed,
/// tier-3-valid body that was silently **missing a solid** (volume
/// `3.000` instead of `3.008`, one shell instead of two). With the
/// honest bit the pellet survives as its own shell and the volumes
/// add exactly.
#[test]
fn fixed_union_keeps_a_pellet_in_a_concave_notch() {
    let a = mixed_turn_arcs().body;
    let b = pellet::<f64>();
    let vol_a = topo::props::mass_properties(&a, Tol::witness())
        .unwrap()
        .volume;
    let vol_b = topo::props::mass_properties(&b, Tol::witness())
        .unwrap()
        .volume;
    assert!(
        (vol_b - 0.008).abs() < 1e-12,
        "the pellet meters 0.008, got {vol_b}"
    );

    let r = topo::boolean::union(&a, &b, Tol::witness()).unwrap();
    let out = r.body().expect("union of two non-empty solids");
    let vol = topo::props::mass_properties(&out.body, Tol::witness())
        .unwrap()
        .volume;
    let shells = out.body.shells().count();

    assert!(
        (vol - (vol_a + vol_b)).abs() < 1e-9,
        "the disjoint union must keep both solids: expected \
         {} (= {vol_a} + {vol_b}), got {vol}",
        vol_a + vol_b
    );
    assert_eq!(
        shells, 2,
        "two disjoint solids union into two shells; the pre-S11 door \
         swallowed the pellet into one"
    );
}

// =====================================================================
// ATREST-2 rows: what the sense bit means on an arc-capped loft.
//
// A PERF-6 reviewer found that inverting EVERY face's sense on the
// three-station arc loft leaves tier 3 green with an unchanged positive
// enclosure, while the same inversion on the square loft refuses
// `LoopRoleInverted`. These three rows pin the three measurements that
// explain it — one per answer, each red when its answer changes.
//
// They live here and not in `topo`'s `tier3_tests` because the bodies
// are `sweep::loft_body` output: `topo` cannot reach the constructor
// that builds them, and a hand-assembled stand-in would pin a fixture
// rather than the loft the finding is about.
// =====================================================================

/// The three-station arc loft, the finding's body: four profile
/// vertices, one of them bulged, so each cap loop carries one circular
/// arc among its lines.
fn atrest2_arc_loft() -> Body<f64> {
    sweep::loft_body::<f64>(
        &[
            crate::common::arc_section(1.0),
            crate::common::arc_section(1.0),
            crate::common::arc_section(1.0),
        ],
        &crate::common::stacked(&[0.0, 1.0, 2.0], 1.0),
        2,
        Tol::witness(),
    )
    .expect("the arc loft lofts")
    .body
}

/// The two-station square loft, the control: the same construction with
/// an unbulged profile, so every cap loop is line-bounded.
fn atrest2_square_loft() -> Body<f64> {
    sweep::loft_body::<f64>(
        &[
            crate::common::quad([(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]),
            crate::common::quad([(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]),
        ],
        &crate::common::stacked(&[0.0, 2.0], 1.0),
        1,
        Tol::witness(),
    )
    .expect("the square loft lofts")
    .body
}

/// Every face's sense inverted, through the PUBLIC door
/// [`topo::Body::set_face_sense`] — not the `_for_tests` hand-flip.
fn atrest2_inverted(body: &Body<f64>) -> Body<f64> {
    let mut out = body.clone();
    let flips: Vec<(FaceKey, bool)> = out.faces().map(|(k, f)| (k, f.sense)).collect();
    for (k, sense) in flips {
        out.set_face_sense(k, !sense).expect("the face is live");
    }
    out
}

/// Whether `face`'s outer loop is bounded by lines alone — check 6's
/// planar arm's `all_lines` gate, re-derived here from the same stored
/// carriers the arm reads.
fn atrest2_outer_is_line_bounded(body: &Body<f64>, face: FaceKey) -> bool {
    let f = body.get_face(face).expect("the face is live");
    let ld = body.get_loop(f.outer).expect("the loop is live");
    let topo::entity::LoopBoundary::Cycle { first } = ld.boundary else {
        return true; // bounds no area
    };
    body.loop_cycle(first)
        .expect("the cycle resolves")
        .iter()
        .all(|&he| {
            body.get_half_edge(he)
                .and_then(|hd| body.get_edge(hd.edge))
                .and_then(|e| body.get_curve_geom(e.curve))
                .and_then(topo::null::CurveGeom::certified)
                .is_some_and(|c| matches!(c.carrier(), geom::Curve3::Line { .. }))
        })
}

/// **Answer 1.** The only at-rest site that produces `LoopRoleInverted`
/// is tier 3's check 6 PLANAR arm, and the only thing that silences it
/// on the arc loft is that arm's `all_lines` gate: a cap loop carrying
/// one circular arc is skipped, a cap loop of four lines is not.
///
/// The row states both halves as runtime facts. On the square loft the
/// refusal set is non-empty, is `LoopRoleInverted` and NOTHING else,
/// and its faces are exactly the line-bounded ones — so the four NURBS
/// walls contribute no refusal on either body and the discriminant is
/// the cap. On the arc loft the same inversion validates clean, and
/// each of its caps carries a non-line carrier.
///
/// **How it goes red.** Widening the planar arm past line carriers
/// (`verbs-1031b-assigner-checker-divergence`'s open question) makes
/// `validate_geometric` on `inverted_arc` return `Err`, and the
/// `is_ok` assertion fails. A curved arm that stopped exempting spline
/// charts would put wall faces into `square_faces` and fail the
/// exactly-the-line-bounded-faces assertion. A loft that stopped
/// emitting an arc carrier for a bulged segment fails the last
/// assertion. The runtime values are the error vector from
/// `validate_geometric` and the stored carrier discriminants.
#[test]
fn only_the_line_bounded_cap_refuses_a_whole_body_sense_inversion() {
    let tol = Tol::witness();
    let arc = atrest2_arc_loft();
    let square = atrest2_square_loft();
    assert!(
        topo::validate_geometric(&arc, tol).is_ok(),
        "the arc loft is honest at rest"
    );
    assert!(
        topo::validate_geometric(&square, tol).is_ok(),
        "the square loft is honest at rest"
    );

    let errs = topo::validate_geometric(&atrest2_inverted(&square), tol)
        .expect_err("a whole-body inversion of the square loft is refused");
    let mut square_faces: Vec<FaceKey> = errs
        .iter()
        .map(|e| match e {
            topo::ValidationError::LoopRoleInverted { face, .. } => *face,
            other => panic!("only check 6's planar arm should speak here, got {other:?}"),
        })
        .collect();
    square_faces.sort();
    let mut line_bounded: Vec<FaceKey> = square
        .faces()
        .map(|(k, _)| k)
        .filter(|&k| atrest2_outer_is_line_bounded(&square, k))
        .collect();
    line_bounded.sort();
    assert!(
        !line_bounded.is_empty(),
        "the square loft has line-bounded caps"
    );
    assert_eq!(
        square_faces, line_bounded,
        "the refusing faces must be exactly the line-bounded ones — check 6's \
         planar arm is the sole raiser and `all_lines` is its whole gate"
    );

    assert!(
        topo::validate_geometric(&atrest2_inverted(&arc), tol).is_ok(),
        "MEASURED GAP: the same whole-body inversion of the arc loft is clean at \
         rest. This row pins the gap; when a check closes it, re-cut the row \
         rather than loosening it"
    );
    for (k, _) in arc.faces() {
        if matches!(
            arc.get_surface(arc.get_face(k).unwrap().surface),
            Some(geom::Surface::Plane { .. })
        ) {
            assert!(
                !atrest2_outer_is_line_bounded(&arc, k),
                "an arc loft cap must carry a non-line carrier — that is what \
                 `all_lines` rejects, and the reason the arm never runs on it"
            );
        }
    }
}

/// **Answer 2.** No at-rest check reads `Face::sense` on the arc loft
/// at all, and the row pins the three gates that stop each reader
/// rather than the absence itself:
///
/// - check 6's PLANAR arm reads the bit through `plane_outward_normal`
///   and is gated on `all_lines` — every planar face here fails it;
/// - check 6's CURVED arm reads `face.sense` directly and skips
///   `Plane` and spline charts — every face here is one or the other;
/// - tier 2's C7 material arm reads `sense` on both sides of a
///   definitely-smooth edge, behind `nurbs_adjacent` — every edge here
///   has a spline-chart face, so the short-circuit fires first;
/// - check 7 reads the bit only at `props::curved_face`'s rimless-band
///   site, which no loft face reaches; the row states that as the
///   volume being bit-identical under the inversion.
///
/// **How it goes red.** If `loft_body` ever mints an analytic cylinder
/// for a circular-arc profile segment — the obvious improvement — that
/// wall is neither `Plane` nor a spline chart, the second assertion
/// fails, and the third fails with it because its edges stop being
/// nurbs-adjacent. If the quadrature ever folds the bit into a loft
/// face's flux, the bit-identical assertion fails. The runtime values
/// are the stored `Surface` discriminants, the per-edge face pair, and
/// the `f64` bits of the metered volume.
#[test]
fn every_sense_reading_gate_shuts_on_the_arc_loft() {
    let tol = Tol::witness();
    let arc = atrest2_arc_loft();

    // The planar arm's gate: no planar face of this body is
    // line-bounded, so `all_lines` rejects every one of them.
    for (k, f) in arc.faces() {
        if matches!(
            arc.get_surface(f.surface),
            Some(geom::Surface::Plane { .. })
        ) {
            assert!(
                !atrest2_outer_is_line_bounded(&arc, k),
                "planar face {k:?} is line-bounded — check 6's planar arm now \
                 reads `Face::sense` on this body and this row's answer has changed"
            );
        }
    }

    // The curved arm's gate: `Plane` or a spline chart, face by face.
    for (k, f) in arc.faces() {
        let s = arc.get_surface(f.surface).expect("the surface is live");
        assert!(
            matches!(s, geom::Surface::Plane { .. }) || s.spline_chart().is_some(),
            "face {k:?} is neither planar nor a spline chart — check 6's curved \
             arm would now run on it and this row's answer has changed"
        );
    }

    // The C7 material arm's gate: every edge nurbs-adjacent.
    for (k, e) in arc.edges() {
        let is_spline = |he| {
            arc.face_of_half_edge(he)
                .and_then(|fk| arc.get_face(fk))
                .and_then(|f| arc.get_surface(f.surface))
                .is_some_and(|s| s.spline_chart().is_some())
        };
        assert!(
            is_spline(e.he_plus) || is_spline(e.he_minus),
            "edge {k:?} has no spline-chart face — tier 2's `nurbs_adjacent` \
             short-circuit no longer covers it and the material arm's \
             `Face::sense` read is now reachable here"
        );
    }

    // Check 7's blindness: the enclosure is bit-identical under the
    // whole-body inversion, and positive both ways.
    let honest = topo::mass_properties(&arc, tol).expect("the arc loft meters");
    let lied =
        topo::mass_properties(&atrest2_inverted(&arc), tol).expect("the inverted arc loft meters");
    assert!(
        honest.volume > 0.0,
        "the honest arc loft encloses positive volume"
    );
    assert_eq!(
        honest.volume.to_bits(),
        lied.volume.to_bits(),
        "the metered enclosure must not move under a sense inversion this body's \
         flux never reads — if it moved, some lane started folding the bit in"
    );
}

/// **Answer 3.** The inverted body is reachable through the PUBLIC API:
/// `atrest2_inverted` builds it with [`topo::Body::set_face_sense`]
/// alone, which is `pub`, not `#[doc(hidden)]`, and callable from
/// outside `topo` — this integration test is the witness. So the
/// finding is a real gap, not an artefact of the `_for_tests` door.
///
/// The control half is what makes the row a guard rather than a
/// restatement: the same public door on the square loft DOES earn a
/// refusal, so the door genuinely writes the bit and the arc loft's
/// silence is the checks', not the door's.
///
/// **How it goes red.** Making `set_face_sense` private or hiding it
/// breaks compilation, which is this row failing. A gate that catches
/// the public-door inversion on the arc loft fails the `is_ok`
/// assertion; one that stops catching it on the square loft fails the
/// `is_err` one. The runtime values are the two error vectors.
#[test]
fn the_public_sense_door_builds_an_inverted_arc_loft_tier_3_accepts() {
    let tol = Tol::witness();
    let arc = atrest2_arc_loft();
    let inverted = atrest2_inverted(&arc);
    for ((_, before), (_, after)) in arc.faces().zip(inverted.faces()) {
        assert_ne!(before.sense, after.sense, "every face's bit is inverted");
    }
    assert!(
        topo::validate_geometric(&inverted, tol).is_ok(),
        "MEASURED GAP: a body built inverted through the public `set_face_sense` \
         door validates clean at rest"
    );
    assert!(
        topo::validate_geometric(&atrest2_inverted(&atrest2_square_loft()), tol).is_err(),
        "the same public door on the line-bounded control IS refused — the door \
         writes the bit, so the arc loft's silence is check 6's gate and not a \
         no-op write"
    );
}

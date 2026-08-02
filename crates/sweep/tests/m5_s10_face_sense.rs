//! M5 S10 acceptance: the face orientation bit (`topo::Face::sense`).
//!
//! S10 landed the ratified fix for PR 9c's contract gap — a face's
//! outward normal is its surface's chart normal times `sense_sign`,
//! rather than the chart normal outright. Every constructor in this
//! build still mints `sense: true`, so the whole battery is
//! bit-identical; what these rows prove is that the outward-normal
//! consumers **actually read the bit**, i.e. that the follow-on
//! revert-wiring unit will be a flip and not a rewrite.
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
//! `PartialSphereFace` precedent (M5 PR 9c): `sense_sign` is a `±1`
//! selected by a `bool`, never a decided quantity, so no row here has
//! an ε-relative margin to sweep — the discriminations are exact
//! arithmetic sign changes and typed refusals. Where a row does need a
//! numeric comparison it is against an ANALYTIC constant with a
//! generous absolute slack, not against a band.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod revolve_common;

use core::f64::consts::{FRAC_PI_8, PI};

use geom_core::{Band, Point2, Point3, Tolerance};
use geom_surfaces::Surface;
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
        ProfileVertex {
            pos: p2(0.0, cy - 1.0),
            bulge: 1.0,
        },
        ProfileVertex {
            pos: p2(0.0, cy + 1.0),
            bulge: 0.0,
        },
    ]);
    revolve(&validated(vec![lp]), axis_y(), Revolution::Full)
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
/// `s_f = sense_sign` — and this row is what proves it is read.
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
    let honest = topo::props::mass_properties(&b).unwrap();
    assert!(
        (honest.volume - 4.0 * PI / 3.0).abs() < 1e-9,
        "the unit ball meters 4π/3, got {}",
        honest.volume
    );

    let flipped = b.flipped_face_sense_for_tests(first_face(&b)).unwrap();
    let lied = topo::props::mass_properties(&flipped).unwrap();
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
    let honest = topo::props::mass_properties(&b).unwrap();
    assert!(
        (honest.volume - 4.0 * PI / 3.0).abs() < 1e-9,
        "the off-centre unit ball still meters 4π/3, got {}",
        honest.volume
    );

    let flipped = b.flipped_face_sense_for_tests(first_face(&b)).unwrap();
    let lied = topo::props::mass_properties(&flipped).unwrap();
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
    let lp = ProfileLoop::builder(p2(5.0, 0.0))
        .line_to(p2(6.0, 0.0))
        .line_to(p2(6.0, 2.0))
        .line_to(p2(5.0, 2.0))
        .close();
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tolerance::get())
        .unwrap();
    let cuboid = extrude(&vp, Extrusion::Distance(3.0)).unwrap().body;
    let r = topo::boolean::union(&ball, &cuboid).unwrap();
    let body = &r.body().expect("a disjoint assembly is a body").body;
    let honest = topo::props::mass_properties(body).unwrap();
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
    let lied = topo::props::mass_properties(&flipped).unwrap();
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
    let lp = ProfileLoop::builder(p2(0.0, 0.0))
        .line_to(p2(2.0, 0.0))
        .line_to(p2(2.0, 1.0))
        .line_to(p2(0.0, 1.0))
        .close();
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tolerance::get())
        .unwrap();
    let body = extrude(&vp, Extrusion::Distance(1.0)).unwrap().body;
    assert_eq!(topo::validate::validate_geometric(&body), Ok(()));
    for (face, _) in body.faces() {
        let flipped = body.flipped_face_sense_for_tests(face).unwrap();
        let errs = topo::validate::validate_geometric(&flipped).unwrap_err();
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
// Finding: concave sweep walls already violate the pre-S10 contract.
// =====================================================================

/// The mixed convex/concave extrusion of `review_m2_pr4`'s assignment
/// 5: a square whose bottom edge is a CONVEX arc and whose top edge is
/// a CONCAVE one (bowing into the region).
fn mixed_turn_arcs() -> sweep::Extruded<f64> {
    let b = FRAC_PI_8.tan();
    let lp = ProfileLoop::builder(p2(0.0, 0.0))
        .arc_to(p2(2.0, 0.0), b)
        .line_to(p2(2.0, 1.5))
        .arc_to(p2(0.0, 1.5), -b)
        .close();
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tolerance::get())
        .unwrap();
    extrude(&vp, Extrusion::Distance(1.0)).unwrap()
}

/// **FINDING (S10 deviation 1), pinned as the current behaviour.**
///
/// The premise S10 was specified against — "at M5 every constructor
/// mints material-agrees-with-chart faces, so `sense: true` everywhere
/// is correct" — is FALSE in this build, and was false before S10.
/// `extrude` mints a cylinder wall for every arc segment, and the
/// cylinder's chart normal is unconditionally the **radially outward**
/// radial. For a CONCAVE arc the material lies OUTSIDE that cylinder,
/// so the face's outward normal is radially INWARD: the face's true
/// sense is `false`, and this build stamps `true`.
///
/// This row pins the two halves of the finding:
///
/// 1. **Geometry**: the concave wall's cylinder has the material on
///    its outside (`|centroid − axis| > radius`), i.e. the stored
///    chart normal points INTO the solid.
/// 2. **Consequence**: `point_in_solid` — whose cylinder door reads
///    the chart-outward radial as the outward normal — therefore
///    reports `In` for points that are outside the solid, in the
///    notch the concave arc cuts. At `x = 1` the true boundary is
///    `y = 2.5 − √2 ≈ 1.0858`; the door does not turn over until
///    `y ≈ 1.5`.
///
/// The assertions below deliberately pin the WRONG answer (the
/// `finding_*` convention of the review suites): S10 does not fix
/// this. Fixing it means teaching the sweep constructors to mint
/// `sense: false` on concave arc walls, which is a behaviour change
/// across the boolean layer and its own unit — and it is a REQUIRED
/// predecessor of the revert-wiring unit, since reverting a body whose
/// senses are already wrong flips a lie into another lie.
#[test]
fn finding_concave_arc_wall_sense_is_wrong_today() {
    let t = mixed_turn_arcs();
    // (1) The concave wall's material is OUTSIDE its cylinder.
    let mut saw_concave = false;
    for &fk in &t.side_faces[0] {
        let sk = t.body.get_face(fk).unwrap().surface;
        let Surface::Cylinder { origin, radius, .. } = *t.body.get_surface(sk).unwrap() else {
            continue;
        };
        // The region's interior probe, in the sketch plane.
        let d = Point2::new(1.0, 0.75) - Point2::new(origin.x, origin.y);
        if d.norm() > radius {
            saw_concave = true;
            assert!(
                t.body.get_face(fk).unwrap().sense,
                "this build stamps sense: true even here — that IS the finding"
            );
        }
    }
    assert!(saw_concave, "the fixture must contain a concave arc wall");

    // (2) The consequence: point_in_solid is wrong in the notch.
    let band = Band::linear().unwrap();
    let truth_hi = 2.5 - 2.0_f64.sqrt(); // ≈ 1.0858
    let inside = point_in_solid(&t.body, Point3::new(1.0, 0.5, 0.5), band).unwrap();
    assert_eq!(inside, topo::boolean::SolidContainment::In);
    for y in [1.2_f64, 1.3, 1.4] {
        assert!(y > truth_hi, "these probes are OUTSIDE the solid");
        assert_eq!(
            point_in_solid(&t.body, Point3::new(1.0, y, 0.5), band).unwrap(),
            topo::boolean::SolidContainment::In,
            "PINNED DEFECT: the cylinder door reads the chart-outward \
             radial as outward, so the concave notch reads as material"
        );
    }
}

/// A small cuboid strictly inside the concave notch — `x ∈ [0.9, 1.1]`,
/// `y ∈ [1.25, 1.35]`, `z ∈ [0.3, 0.7]`, volume `0.2·0.1·0.4 = 0.008`.
/// Every point of it is genuinely OUTSIDE `mixed_turn_arcs` (the notch
/// floor at `x = 1` is `y ≈ 1.0858`), so the two solids are disjoint.
fn pellet() -> Body<f64> {
    let lp = ProfileLoop::builder(p2(0.9, 1.25))
        .line_to(p2(1.1, 1.25))
        .line_to(p2(1.1, 1.35))
        .line_to(p2(0.9, 1.35))
        .close();
    let plane = SketchPlane::from_frame(
        Point3::new(0.0, 0.0, 0.3),
        geom_core::Vec3::new(1.0, 0.0, 0.0),
        geom_core::Vec3::new(0.0, 1.0, 0.0),
    );
    let vp = Profile::new(plane, vec![lp])
        .validate(Tolerance::get())
        .unwrap();
    extrude(&vp, Extrusion::Distance(0.4)).unwrap().body
}

/// **FINDING (S10 deviation 1, e2e half) — a WRONG BODY ships from a
/// public call today.** Pinned as current behaviour, per the
/// `finding_*` convention.
///
/// The row above shows the concave wall's sense is wrong and that
/// `point_in_solid` misreports inside the notch. That is a door-level
/// statement, and on its own it understates the urgency. This row
/// carries it end-to-end through the PUBLIC `union`:
///
/// `union(notched, pellet)` on two DISJOINT solids has no surface
/// intersections, so the operation falls back to mutual containment to
/// decide the arrangement — and it asks `point_in_solid`. The door
/// answers `In` for the pellet's points because the concave wall's
/// chart normal points into material, so the pellet is judged
/// swallowed by the notched body and its shell is dropped. The result
/// is a well-formed, tier-3-valid body that is simply **missing a
/// solid**: volume `3.000` instead of `3.008`, one shell instead of
/// two. No error, no escalation — a silent wrong answer.
///
/// This is the strongest available statement of why the concave-fix
/// unit must follow immediately, and why S10's bit is the enabling
/// infrastructure rather than a nicety: the fix IS "mint `sense: false`
/// on concave arc walls", which was unrepresentable before this PR.
///
/// **When the concave-fix unit lands, this row FLIPS**: rename it
/// `fixed_union_keeps_a_pellet_in_a_concave_notch` and invert the
/// assertions to `vol == vol_a + vol_b` and `shells == 2`. The
/// assertions below are deliberately written so that inversion is a
/// one-line edit.
#[test]
fn finding_union_silently_swallows_a_pellet_in_a_concave_notch() {
    let a = mixed_turn_arcs().body;
    let b = pellet();
    let vol_a = topo::props::mass_properties(&a).unwrap().volume;
    let vol_b = topo::props::mass_properties(&b).unwrap().volume;
    assert!(
        (vol_b - 0.008).abs() < 1e-12,
        "the pellet meters 0.008, got {vol_b}"
    );

    let r = topo::boolean::union(&a, &b).unwrap();
    let out = r.body().expect("union of two non-empty solids");
    let vol = topo::props::mass_properties(&out.body).unwrap().volume;
    let shells = out.body.shells().count();

    // TRUTH would be `vol == vol_a + vol_b` in 2 shells. PINNED DEFECT:
    // the pellet is gone.
    assert!(
        (vol - vol_a).abs() < 1e-9,
        "PINNED DEFECT: the pellet is swallowed, so the union must meter \
         the notched body alone ({vol_a}); got {vol} — if this now equals \
         {} the concave-fix unit has landed and this row should be \
         flipped to `fixed_*`",
        vol_a + vol_b
    );
    assert_eq!(
        shells, 1,
        "PINNED DEFECT: the dropped pellet leaves one shell where two \
         disjoint solids should give two"
    );
}

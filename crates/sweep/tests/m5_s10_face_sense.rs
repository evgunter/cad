//! M5 S10 acceptance: the face orientation bit (`topo::Face::sense`).
//!
//! S10 landed the ratified fix for PR 9c's contract gap — a face's
//! outward normal is its surface's chart normal times `sense_sign`,
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
//! `PartialSphereFace` precedent (M5 PR 9c): `sense_sign` is a `±1`
//! selected by a `bool`, never a decided quantity, so no row here has
//! an ε-relative margin to sweep — the discriminations are exact
//! arithmetic sign changes and typed refusals. Where a row does need a
//! numeric comparison it is against an ANALYTIC constant with a
//! generous absolute slack, not against a band.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

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
///    `sense_sign · chart normal` as outward — now reports `Out`
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

/// A small cuboid strictly inside the concave notch — `x ∈ [0.9, 1.1]`,
/// `y ∈ [1.25, 1.35]`, `z ∈ [0.3, 0.7]`, volume `0.2·0.1·0.4 = 0.008`.
/// Every point of it is genuinely OUTSIDE `mixed_turn_arcs` (the notch
/// floor at `x = 1` is `y ≈ 1.0858`), so the two solids are disjoint.
fn pellet() -> Body<f64> {
    let lp = <ProfileLoop<f64> as RawLoop<f64>>::polygon([
        p2(0.9, 1.25),
        p2(1.1, 1.25),
        p2(1.1, 1.35),
        p2(0.9, 1.35),
    ]);
    let plane = SketchPlane::from_frame(
        Point3::new(0.0, 0.0, 0.3),
        geom_core::Vec3::new(1.0, 0.0, 0.0),
        geom_core::Vec3::new(0.0, 1.0, 0.0),
    );
    let vp = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .unwrap();
    extrude(&vp, Extrusion::Distance(0.4), Tol::witness())
        .unwrap()
        .body
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
    let b = pellet();
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

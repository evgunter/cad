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
//! **Two instruments, and every row says which it used.**
//! `Body::flipped_face_sense_for_tests` inverts ONE face's bit and
//! nothing else; `Body::set_face_sense` is the PUBLIC door, and the
//! rows that use it invert EVERY face of a body at once. Either way
//! the result is deliberately an incoherent body — by the
//! interior-left rule a face's outer loop winds CCW about its
//! *outward* normal, so writing the bit alone puts the two encodings
//! of orientation (the bit and the winding) into disagreement. Each
//! row below picks a consumer and asks whether it noticed.
//!
//! **Tolerance shape.** These rows are STRUCTURAL, in the sense of the
//! `PartialSphereFace` precedent (M5 PR 9c): the sense is a `bool`
//! selecting a negation, never a decided quantity, so the
//! discriminations are exact arithmetic sign changes and typed
//! refusals. Where a row needs a numeric comparison it is against an
//! ANALYTIC constant with a generous absolute slack, not against a
//! band. That is not the same as "nothing here is ε-coupled": this
//! corpus contains bodies whose walls are RATIONAL, and on those the
//! fixed quadrature schedule honestly runs out of budget at a tight ε.
//! A row over such a body states its claim over the door's whole
//! OUTCOME — a refusal is as much an output as a number is — rather
//! than over a volume that may not exist at every point of the matrix.

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
    let honest = topo::mass_properties(&b, Tol::witness()).unwrap();
    assert!(
        (honest.volume - 4.0 * PI / 3.0).abs() < 1e-9,
        "the unit ball meters 4π/3, got {}",
        honest.volume
    );

    let flipped = b.flipped_face_sense_for_tests(first_face(&b)).unwrap();
    let lied = topo::mass_properties(&flipped, Tol::witness()).unwrap();
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
    let honest = topo::mass_properties(&b, Tol::witness()).unwrap();
    assert!(
        (honest.volume - 4.0 * PI / 3.0).abs() < 1e-9,
        "the off-centre unit ball still meters 4π/3, got {}",
        honest.volume
    );

    let flipped = b.flipped_face_sense_for_tests(first_face(&b)).unwrap();
    let lied = topo::mass_properties(&flipped, Tol::witness()).unwrap();
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
    let honest = topo::mass_properties(body, Tol::witness()).unwrap();
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
    let lied = topo::mass_properties(&flipped, Tol::witness()).unwrap();
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
    let vol_a = topo::mass_properties(&a, Tol::witness()).unwrap().volume;
    let vol_b = topo::mass_properties(&b, Tol::witness()).unwrap().volume;
    assert!(
        (vol_b - 0.008).abs() < 1e-12,
        "the pellet meters 0.008, got {vol_b}"
    );

    let r = topo::boolean::union(&a, &b, Tol::witness()).unwrap();
    let out = r.body().expect("union of two non-empty solids");
    let vol = topo::mass_properties(&out.body, Tol::witness())
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
// What the sense bit means on an arc-capped loft, and where tier 3
// reads it.
//
// Inverting EVERY face's sense on the three-station arc loft is refused
// at its two planar caps, exactly as on the square loft: check 6's
// planar arm winds a loop of `Line` and `Circle` carriers exactly (the
// chord polygon plus each arc's circular segment), so an arc cap is as
// falsifiable as a polygonal one. The spline walls stay silent on both
// bodies, and a planar loop riding an `Ellipse` or NURBS carrier stays
// outside the arm; the rows below pin both residues so their silence
// stays visible.
//
// They live here and not in `topo`'s `tier3_tests` because the bodies
// are `sweep` output: `topo` cannot reach the constructors that build
// them, and a hand-assembled stand-in would pin a fixture rather than
// the verbs' own bodies.
//
// The two lofts are the crate's shared `arc_prism` and `square_prism`
// (`common`), which differ in the bulge, the station count and the
// v-degree; the arc carrier alone decided the verdict under the
// line-only arm (the 2×2 in
// `work/atrest/sense-inversion-is-invisible-to-tier-3-on-arc-capped-lofts.md`).
// =====================================================================

/// Every face's sense inverted, through the PUBLIC door
/// [`topo::Body::set_face_sense`] — not the `_for_tests` hand-flip.
fn sense_inverted_everywhere(body: &Body<f64>) -> Body<f64> {
    let mut out = body.clone();
    let flips: Vec<(FaceKey, bool)> = out.faces().map(|(k, f)| (k, f.sense)).collect();
    for (k, sense) in flips {
        out.set_face_sense(k, !sense).expect("the face is live");
    }
    out
}

/// `face`'s sense alone inverted, through the same public door.
fn sense_inverted_at(body: &Body<f64>, face: FaceKey) -> Body<f64> {
    let mut out = body.clone();
    let sense = out.get_face(face).expect("the face is live").sense;
    out.set_face_sense(face, !sense).expect("the face is live");
    out
}

/// Whether `face`'s surface is a spline chart — the discriminant of
/// check 6's curved-arm skip and of tier 3's `nurbs_adjacent`
/// short-circuit, asked of a FACE key.
fn is_spline_chart_face(body: &Body<f64>, face: FaceKey) -> bool {
    body.get_face(face)
        .and_then(|f| body.get_surface(f.surface))
        .is_some_and(|s| s.spline_chart().is_some())
}

/// The certified carriers of `l`'s cycle, in cycle order; empty for a
/// loop that is not a `Cycle`.
fn loop_carriers(body: &Body<f64>, l: topo::LoopKey) -> Vec<geom::Curve3<f64>> {
    let Some(topo::entity::LoopBoundary::Cycle { first }) = body.get_loop(l).map(|ld| ld.boundary)
    else {
        return Vec::new();
    };
    body.loop_cycle(first)
        .expect("a live cycle closes")
        .iter()
        .map(|&he| {
            body.get_half_edge(he)
                .and_then(|hd| body.get_edge(hd.edge))
                .and_then(|e| body.get_curve_geom(e.curve))
                .and_then(topo::null::CurveGeom::certified)
                .expect("every edge of an at-rest body carries a certified curve")
                .carrier()
                .clone()
        })
        .collect()
}

/// **The (face, loop) pairs check 6's PLANAR arm examines** on `body`,
/// re-derived here from the same stored data the arm reads, in the
/// arm's order:
///
/// - the face's surface is a `Plane` (the arm `continue`s on every
///   other kind — a predicate that omitted this would drag the spline
///   walls in the day `loft_body` mints `Line` carriers for straight
///   rails);
/// - the loop is the outer loop OR one of `face.rings`;
/// - the loop's boundary is a `Cycle` (an empty ring bounds no area and
///   is NOT examined);
/// - every certified carrier on the cycle is a `Line` or a `Circle` —
///   an `Ellipse`, spiric or NURBS carrier puts the loop outside.
///
/// A `(face, loop)` PAIR and not a face, because `LoopRoleInverted`
/// names both and a face can refuse once per loop.
fn planar_arm_reaches(body: &Body<f64>) -> Vec<(FaceKey, topo::LoopKey)> {
    let mut out = Vec::new();
    for (fk, f) in body.faces() {
        if !matches!(body.get_surface(f.surface), Some(Surface::Plane { .. })) {
            continue;
        }
        for l in core::iter::once(f.outer).chain(f.rings.iter().copied()) {
            let carriers = loop_carriers(body, l);
            if !carriers.is_empty()
                && carriers
                    .iter()
                    .all(|c| matches!(c, geom::Curve3::Line { .. } | geom::Curve3::Circle { .. }))
            {
                out.push((fk, l));
            }
        }
    }
    out.sort();
    out
}

/// Whether any loop of `pairs` carries a `Circle` — the premise that
/// makes a row over them a row about ARC-bearing loops.
fn some_loop_rides_an_arc(body: &Body<f64>, pairs: &[(FaceKey, topo::LoopKey)]) -> bool {
    pairs.iter().any(|&(_, l)| {
        loop_carriers(body, l)
            .iter()
            .any(|c| matches!(c, geom::Curve3::Circle { .. }))
    })
}

/// The `(face, loop)` pairs of `LoopRoleInverted` in `errs`, sorted;
/// panics on any other refusal, naming it.
fn role_inversions(errs: &[topo::ValidationError]) -> Vec<(FaceKey, topo::LoopKey)> {
    let mut refused: Vec<(FaceKey, topo::LoopKey)> = errs
        .iter()
        .map(|e| match e {
            topo::ValidationError::LoopRoleInverted { face, r#loop } => (*face, *r#loop),
            other => panic!(
                "only check 6's planar arm should speak here; a `CurvedSenseInverted` \
                 is what a curved arm that stopped exempting spline charts would \
                 raise. Got {other:?}"
            ),
        })
        .collect();
    refused.sort();
    refused
}

/// **A whole-body inversion refuses at the planar caps, arcs included.**
/// On the square prism and the arc prism alike, the refusal set is
/// `LoopRoleInverted` and nothing else, and its `(face, loop)` pairs
/// are exactly the ones check 6's planar arm examines — the two caps.
/// On the arc prism each examined loop carries a `Circle`, so the
/// verdict is the arc-exact winding's.
///
/// The four spline walls contribute no refusal on either body: the
/// curved arm exempts spline charts, which is
/// `work/verdict/m6-sense-gate-recorded-residuals.md`'s residual 3.
///
/// **How it goes red.** An arm that went back to skipping arc-bearing
/// loops leaves the inverted arc prism `Ok`, and `expect_err` fails. A
/// curved arm that stopped exempting spline charts raises
/// `CurvedSenseInverted`, which `role_inversions` names in its panic. A
/// loft that stopped minting an arc carrier for a bulged segment fails
/// the arc premise. The runtime values are the error vectors from
/// `validate_geometric` and the stored carrier discriminants.
#[test]
fn a_whole_body_sense_inversion_refuses_at_every_planar_cap_arcs_included() {
    let tol = Tol::witness();
    for (name, body) in [
        ("square prism", crate::common::square_prism()),
        ("arc prism", crate::common::arc_prism()),
    ] {
        assert!(
            topo::validate_geometric(&body, tol).is_ok(),
            "the {name} is honest at rest"
        );
        let examined = planar_arm_reaches(&body);
        assert_eq!(
            examined.len(),
            2,
            "check 6's planar arm examines the {name}'s two caps; got {examined:?}"
        );
        let errs = topo::validate_geometric(&sense_inverted_everywhere(&body), tol)
            .expect_err("a whole-body inversion is refused");
        assert_eq!(
            role_inversions(&errs),
            examined,
            "every (face, loop) the planar arm examines on the {name} refuses under a \
             whole-body inversion, and nothing else does"
        );
    }
    let arc = crate::common::arc_prism();
    assert!(
        some_loop_rides_an_arc(&arc, &planar_arm_reaches(&arc)),
        "the arc prism's caps carry a `Circle` — the premise of the arc half"
    );
}

/// **An inverted arc-bounded planar face refuses by name.** One cap of
/// the arc prism has its bit inverted through the public door; the
/// refusal is exactly `LoopRoleInverted` naming THAT face and ITS outer
/// loop — the loop of three lines and one arc.
///
/// **How it goes red.** An arm that skips arc-bearing loops leaves the
/// body `Ok`; one that mis-signed the arc's segment term would still
/// see the three-line chord polygon's sign here (the quarter-circle's
/// bulge is small beside the square), so the SIGN of the bulge is
/// pinned by the ring row below, where the chord term is zero. The
/// runtime value is the error vector.
#[test]
fn an_inverted_arc_bounded_planar_cap_refuses_naming_its_face_and_loop() {
    let tol = Tol::witness();
    let arc = crate::common::arc_prism();
    let caps = planar_arm_reaches(&arc);
    assert!(some_loop_rides_an_arc(&arc, &caps), "the caps carry arcs");
    for &(face, l) in &caps {
        assert_eq!(
            arc.get_face(face).map(|f| f.outer),
            Some(l),
            "a cap's loop is outer"
        );
        let errs = topo::validate_geometric(&sense_inverted_at(&arc, face), tol)
            .expect_err("an arc cap whose bit disagrees with its winding is refused");
        assert_eq!(
            role_inversions(&errs),
            vec![(face, l)],
            "the refusal names the inverted cap and its outer loop, and nothing else"
        );
    }
}

/// The extruded washer: a unit-radius disc with a concentric hole of
/// radius ½, each loop a full circle of two semicircular arcs, extruded
/// one unit. Both planar caps carry an outer loop and a ring of
/// `Circle` carriers only, so every chord polygon is a DIGON: its
/// Newell term is zero and the winding is the arcs' segment terms alone.
fn washer() -> Body<f64> {
    let circle = |r: f64| {
        ProfileLoop::new(vec![
            ProfileVertex::new(p2(-r, 0.0), 1.0),
            ProfileVertex::new(p2(r, 0.0), 1.0),
        ])
    };
    let prof = Profile::new(SketchPlane::xy(), vec![circle(1.0), circle(0.5)])
        .validate(Tol::witness())
        .expect("the washer profile validates");
    extrude(&prof, Extrusion::Distance(1.0), Tol::witness())
        .expect("the washer extrudes")
        .body
}

/// **A ring carrying an arc refuses too.** Each cap of the extruded
/// washer has its bit inverted in turn; the refusal names that face
/// twice — its outer loop AND its ring — and nothing else. Every loop
/// here is a digon of two semicircles, so the chord term is zero and
/// the verdict is the arcs' segment terms alone: an arm that dropped
/// the bulge would see `Zero` and exempt both loops, and one that
/// applied it with the wrong sign would refuse the HONEST washer.
///
/// **How it goes red.** An arm that skips arc-bearing loops, or one
/// that drops the segment term, leaves the inverted cap `Ok`; a
/// mis-signed segment term refuses the honest body in the first
/// assertion. The runtime values are the error vectors.
#[test]
fn an_inverted_cap_refuses_at_its_arc_ring_as_well_as_its_outline() {
    let tol = Tol::witness();
    let body = washer();
    assert!(
        topo::validate_geometric(&body, tol).is_ok(),
        "the washer is honest at rest"
    );
    let examined = planar_arm_reaches(&body);
    let caps: Vec<FaceKey> = body
        .faces()
        .filter(|(_, f)| !f.rings.is_empty())
        .map(|(k, _)| k)
        .collect();
    assert_eq!(caps.len(), 2, "the washer has two ringed caps");
    for face in caps {
        let f = body.get_face(face).expect("the cap is live");
        let mut expected = vec![(face, f.outer), (face, f.rings[0])];
        expected.sort();
        assert!(
            expected.iter().all(|pair| examined.contains(pair)),
            "check 6's planar arm examines both of the cap's loops"
        );
        let errs = topo::validate_geometric(&sense_inverted_at(&body, face), tol)
            .expect_err("a washer cap whose bit disagrees with its winding is refused");
        assert_eq!(
            role_inversions(&errs),
            expected,
            "the refusal names the cap's outline and its arc ring, and nothing else"
        );
    }
}

/// **The residue: a planar loop riding an `Ellipse` stays outside the
/// arm.** `tilted_cut_upper`'s cut face is a plane bounded by one exact
/// `Ellipse`; its bottom cap is bounded by circle arcs. Inverting the
/// bit of the circle cap is refused; inverting the bit of the ellipse
/// face is not — check 6's planar arm answers on `Line` and `Circle`
/// carriers only, and a loop riding an ellipse (or a NURBS carrier) is
/// not examined.
///
/// **How it goes red.** The day the arm reaches ellipse carriers the
/// `is_ok` fails, and this row is re-cut to the new verdict rather than
/// loosened. A split that stopped minting an `Ellipse` for the tilted
/// cut fails the carrier premise. The runtime values are the stored
/// carrier discriminants and the two validation results.
#[test]
fn an_ellipse_bounded_planar_face_stays_outside_the_planar_arm() {
    let tol = Tol::witness();
    let body = crate::common::tilted_cut_upper();
    assert!(
        topo::validate_geometric(&body, tol).is_ok(),
        "the tilted cut is honest at rest"
    );
    let planar = |pred: fn(&geom::Curve3<f64>) -> bool| -> Vec<(FaceKey, topo::LoopKey)> {
        body.faces()
            .filter(|(_, f)| matches!(body.get_surface(f.surface), Some(Surface::Plane { .. })))
            .filter(|(_, f)| loop_carriers(&body, f.outer).iter().any(pred))
            .map(|(k, f)| (k, f.outer))
            .collect()
    };
    let elliptic = planar(|c| matches!(c, geom::Curve3::Ellipse { .. }));
    let circular = planar(|c| matches!(c, geom::Curve3::Circle { .. }));
    assert_eq!(elliptic.len(), 1, "the tilted cut face rides an `Ellipse`");
    assert_eq!(circular.len(), 1, "the bottom cap rides circle arcs");
    assert!(
        !planar_arm_reaches(&body).contains(&elliptic[0]),
        "the ellipse-bounded loop is outside the planar arm"
    );

    let (cap, cap_loop) = circular[0];
    let errs = topo::validate_geometric(&sense_inverted_at(&body, cap), tol)
        .expect_err("the circle-bounded cap's inversion is refused");
    assert_eq!(role_inversions(&errs), vec![(cap, cap_loop)]);

    assert!(
        topo::validate_geometric(&sense_inverted_at(&body, elliptic[0].0), tol).is_ok(),
        "MEASURED RESIDUE: an ellipse-bounded planar face inverted through the public \
         door is clean at rest"
    );
}

/// **The planar arm is the only sense reader on the arc loft.** Check
/// 6's planar arm now reads the bit on both caps; the other three
/// readers stay gated shut on this body, and the row pins each gate:
///
/// - check 6's CURVED arm reads `face.sense` directly and skips `Plane`
///   and spline charts — every face here is one or the other;
/// - tier 3's **check 4 MATERIAL arm** reads `sense` on both sides of a
///   definitely-smooth edge, behind `nurbs_adjacent` — every edge here
///   has a spline-chart face, so the short-circuit fires first;
/// - check 7 reads the bit only at `props::curved_face`'s rimless-band
///   site, which no loft face reaches; the row states that as the
///   reporting door giving the SAME reading under the inversion — a
///   bit-identical volume where it computes, the same typed refusal
///   where this body's rational walls honestly run out of budget.
///
/// So the walls' bits are read by nothing at rest (residual 3), and the
/// whole-body inversion is caught at the caps alone.
///
/// **How it goes red.** An arm that stops examining the caps fails the
/// first assertion. If `loft_body` ever mints an analytic cylinder for
/// a circular-arc profile segment, that wall is neither `Plane` nor a
/// spline chart and the second assertion fails, the third with it. If
/// the quadrature ever folds the bit into a loft face's flux, the
/// same-reading assertion fails. The runtime values are the stored
/// `Surface` discriminants, the per-edge face pair, and the `f64` bits
/// of the metered volume (or the typed `MassPropsError` where the
/// schedule refuses).
#[test]
fn the_planar_arm_is_the_only_sense_reader_on_the_arc_loft() {
    let tol = Tol::witness();
    let arc = crate::common::arc_prism();

    let mut planar_outers: Vec<(FaceKey, topo::LoopKey)> = arc
        .faces()
        .filter(|(_, f)| matches!(arc.get_surface(f.surface), Some(Surface::Plane { .. })))
        .map(|(k, f)| (k, f.outer))
        .collect();
    planar_outers.sort();
    assert_eq!(
        planar_arm_reaches(&arc),
        planar_outers,
        "check 6's planar arm examines every planar face's loop of the arc prism"
    );

    // The curved arm's gate: `Plane` or a spline chart, face by face.
    for (k, f) in arc.faces() {
        let s = arc.get_surface(f.surface).expect("the surface is live");
        assert!(
            matches!(s, Surface::Plane { .. }) || is_spline_chart_face(&arc, k),
            "face {k:?} is neither planar nor a spline chart ({s:?}) — check 6's \
             curved arm would now run on it and this row's answer has changed"
        );
    }

    // Check 4's MATERIAL arm's gate: every edge nurbs-adjacent.
    for (k, e) in arc.edges() {
        let spline_side = |he| {
            arc.face_of_half_edge(he)
                .is_some_and(|fk| is_spline_chart_face(&arc, fk))
        };
        assert!(
            spline_side(e.he_plus) || spline_side(e.he_minus),
            "edge {k:?} has no spline-chart face — tier 3's `nurbs_adjacent` \
             short-circuit no longer covers it and check 4's MATERIAL arm's \
             `Face::sense` read is now reachable here"
        );
    }

    // Check 7's blindness: the reporting door gives the SAME reading
    // under the whole-body inversion. Phrased over the whole outcome,
    // not over a volume, because this body's walls are rational and the
    // fixed schedule honestly runs out of budget at a tight ε (the m8-3
    // posture) — a refusal is as much the door's output as a number is,
    // and both must be unmoved by a bit no lane here reads.
    let honest = topo::mass_properties(&arc, tol);
    let lied = topo::mass_properties(&sense_inverted_everywhere(&arc), tol);
    match (&honest, &lied) {
        (Ok(h), Ok(l)) => {
            assert!(
                h.volume > 0.0,
                "the honest arc prism encloses positive volume; got {}",
                h.volume
            );
            assert_eq!(
                h.volume.to_bits(),
                l.volume.to_bits(),
                "the metered enclosure must not move under a sense inversion this \
                 body's flux never reads — if it moved, some lane started folding \
                 the bit in"
            );
        }
        (Err(h), Err(l)) => assert_eq!(
            h, l,
            "the door's typed refusal must not move under the inversion either"
        ),
        _ => panic!("the inversion changed WHETHER the enclosure computes: {honest:?} vs {lied:?}"),
    }
}

/// **The public door's inversion of an arc loft is refused at rest.**
/// [`sense_inverted_everywhere`] builds the inverted body with
/// [`topo::Body::set_face_sense`] alone, which is `pub` and callable
/// from outside `topo` — this integration test is the witness that the
/// state is reachable through the public API, and tier 3 refuses it at
/// exactly the caps its planar arm examines, as it does the square
/// control. The refusal SET is asserted, not `is_err`: an `is_err`
/// would still pass the day either body started refusing for an
/// unrelated reason.
///
/// **How it goes red.** Making `set_face_sense` private breaks
/// compilation. An arm that stops catching the inversion on either body
/// fails its refusal-set assertion. The runtime values are the two
/// error vectors and the per-face `sense` bits.
#[test]
fn the_public_sense_door_inversion_of_an_arc_loft_is_refused_at_its_caps() {
    let tol = Tol::witness();
    let arc = crate::common::arc_prism();
    let inverted = sense_inverted_everywhere(&arc);
    // Keyed, not positional: the claim is about each face's own bit,
    // and a `zip` over two iterators would rest on the unstated premise
    // that arena order survives `clone`.
    for (k, before) in arc.faces() {
        let after = inverted
            .get_face(k)
            .expect("the clone keeps every face key");
        assert_ne!(before.sense, after.sense, "face {k:?}'s bit is inverted");
    }
    let errs = topo::validate_geometric(&inverted, tol)
        .expect_err("a body built inverted through the public door is refused at rest");
    assert_eq!(role_inversions(&errs), planar_arm_reaches(&arc));

    let square = crate::common::square_prism();
    let errs = topo::validate_geometric(&sense_inverted_everywhere(&square), tol)
        .expect_err("the same public door on the line-bounded control IS refused");
    assert_eq!(role_inversions(&errs), planar_arm_reaches(&square));
}

/// PROBE (temporary): the reviewer's C-shape.
#[test]
fn probe_c_shape_cap_orientation() {
    let tol = Tol::witness();
    let d = |deg: f64, r: f64| p2(r * deg.to_radians().cos(), r * deg.to_radians().sin());
    let section = || {
        vec![ProfileLoop::new(vec![
            ProfileVertex::new(d(5.0, 1.0), 87.5f64.to_radians().tan()),
            ProfileVertex::new(d(355.0, 1.0), 0.0),
            ProfileVertex::new(d(355.0, 0.9), 0.0),
            ProfileVertex::new(d(270.0, 0.9), 0.0),
            ProfileVertex::new(d(180.0, 0.9), 0.0),
            ProfileVertex::new(d(90.0, 0.9), 0.0),
            ProfileVertex::new(d(5.0, 0.9), 0.0),
        ])]
    };
    let mut report = String::new();
    let caps = |body: &Body<f64>| -> String {
        body.faces()
            .filter_map(|(k, f)| match body.get_surface(f.surface) {
                Some(Surface::Plane { origin, normal, .. }) => Some(format!(
                    "{k:?} z0={:.3} nz={:.3} sense={}",
                    origin.z, normal.z, f.sense
                )),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("; ")
    };
    match Profile::new(SketchPlane::xy(), section()).validate(tol) {
        Err(e) => report += &format!("PROFILE REFUSES: {e:?}\n"),
        Ok(vp) => {
            report += "profile ok\n";
            match extrude(&vp, Extrusion::Distance(1.0), tol) {
                Err(e) => report += &format!("EXTRUDE REFUSES: {e:?}\n"),
                Ok(x) => {
                    report += &format!(
                        "extrude ok: caps [{}]; validate: {:?}\n",
                        caps(&x.body),
                        topo::validate_geometric(&x.body, tol)
                    );
                }
            }
        }
    }
    match sweep::loft_body::<f64>(
        &[section(), section()],
        &crate::common::stacked(&[0.0, 1.0], 1.0),
        1,
        tol,
    ) {
        Err(e) => report += &format!("LOFT REFUSES: {e:?}\n"),
        Ok(l) => {
            report += &format!(
                "loft ok: caps [{}]; validate: {:?}\n",
                caps(&l.body),
                topo::validate_geometric(&l.body, tol)
            );
        }
    }
    panic!("PROBE REPORT\n{report}");
}

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

/// The half-disc of the PR 9c `ball` acceptance: a unit semicircle
/// from (0, −1) through (1, 0) to (0, 1), closed by the on-axis
/// diameter.
fn half_disc() -> ProfileLoop<f64> {
    ProfileLoop::new(vec![
        ProfileVertex {
            pos: p2(0.0, -1.0),
            bulge: 1.0,
        },
        ProfileVertex {
            pos: p2(0.0, 1.0),
            bulge: 0.0,
        },
    ])
}

/// The unit ball centred at the origin: two half-sphere bands on ONE
/// sphere surface, each **rimless** (bounded by meridians only).
fn ball() -> Body<f64> {
    let vp = validated(vec![half_disc()]);
    revolve(&vp, axis_y(), Revolution::Full).unwrap().body
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

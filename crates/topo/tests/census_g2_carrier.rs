//! **The declared planar pair's world-carrier chart** (census gap 2,
//! #1063; `docs/CENSUS-G2-SPEC.md`).
//!
//! These rows are about the PREDICATE, not the census: they call
//! [`topo::declared_pair_overlap`] directly, so each one can hand Door
//! 1's verdict in and pin what Door 2 does with it. The census-level
//! rows — the flush seat certifying end to end — live with the seat, in
//! `m9_c1_rest_face_rung.rs`.
//!
//! **ε posture.** Two kinds of row here, deliberately separated:
//!
//! - the SEAT rows are exact at every ε the matrix runs (the two
//!   descriptions' shared coordinates are the same `f64` literals, so
//!   the carrier residue is a bit-zero, not a small number);
//! - the CARRIER-TILT rows ride the band ON PURPOSE — the whole content
//!   of `chart_region_carrier_tilt` is where a residue falls relative to
//!   ε — so their fixtures are built ε-RELATIVE from
//!   [`Tol::witness`]`.eps()`. A fixed literal tilt would pass at one ε
//!   row and contradict at the next, which would be a fact about the
//!   fixture rather than about the predicate.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use geom::Surface;
use geom_core::{Band, Point3, Tol};
use topo::{
    Body, ChartOverlap, ChartRegionError, ContactVerdict, FaceKey, FaceSurface,
    declared_pair_overlap,
};

fn band() -> Band {
    let tol = Tol::witness();
    Band::new(tol.eps(), tol.k() * tol.eps()).unwrap()
}

/// The one planar face of `body` whose outward normal points along
/// `sign · ẑ`. Read by NORMAL, never by height: a sheared plate's
/// underside plane has its origin at the face centroid, which is not
/// on the contact line.
fn z_face(body: &Body<f64>, sign: f64) -> FaceKey {
    let mut found = None;
    for (k, f) in body.faces() {
        if let Some(Surface::Plane { normal, .. }) = body.get_surface(f.surface) {
            let out = if f.sense { *normal } else { -*normal };
            if out.z * sign > 0.5 {
                assert!(
                    found.is_none(),
                    "the z-facing face is unique in this fixture"
                );
                found = Some(k);
            }
        }
    }
    found.expect("a z-facing planar face")
}

// ---------------------------------------------------------------------
// Fixtures — two independently authored bodies, never one arena, so no
// structural chart identity exists and the world-carrier arm is the
// only authority in play. The shape is #943's own: a plate seated on a
// slab, FLUSH with the slab's `x = 0` end and narrower in y, so the two
// trims share a boundary segment and the region walk must refuse it
// before the interior-witness rung can answer. (A pair whose trims are
// bit-identical takes `bit_equal_cyclic`'s fast path and never reaches
// the rung at all — which is why the fixture is not two equal squares.)
// ---------------------------------------------------------------------

/// The slab, its TOP the A-side face: `[0, len] × [0, 1]` at `z = 1`.
fn slab(len: f64) -> Body<f64> {
    common::mapped_cube(|x, y, z| Point3::new(x * len, y, z))
}

/// The plate, its UNDERSIDE the B-side face: `[0, len] × [0.25, 0.75]`,
/// TILTED by `slope`, so the two carriers meet exactly at `x = 0` and
/// are `slope · len` apart at the far end.
fn plate(len: f64, slope: f64) -> Body<f64> {
    common::mapped_cube(|x, y, z| Point3::new(x * len, 0.25 + y * 0.5, 1.0 + slope * x * len + z))
}

/// The declared pair of a slab/plate fixture: the slab's top and the
/// plate's underside.
fn pair(slab: &Body<f64>, plate: &Body<f64>) -> (FaceKey, FaceKey) {
    (z_face(slab, 1.0), z_face(plate, -1.0))
}

// ---------------------------------------------------------------------
// The world carrier answers where the structural rung cannot
// ---------------------------------------------------------------------

/// INVARIANT: two independently authored coincident planar faces share
/// no `SurfaceKey` and no `GeomSource`, and the world-carrier arm
/// answers them anyway — with definite area, not a guess. This is the
/// gap #1063 names, closed.
#[test]
fn a_declared_planar_pair_with_no_structural_chart_certifies_on_the_world_carrier() {
    let (a, b) = (slab(1.0), plate(1.0, 0.0));
    let (fa, fb) = pair(&a, &b);
    // The structural door still refuses it — the arm is an ADDITION
    // below that rung, never a weakening of it.
    match topo::chart_region_overlap(&a, fa, &b, fb, band()) {
        Err(ChartRegionError::ChartDivergence { .. }) => {}
        other => panic!("the structural rung must still refuse: {other:?}"),
    }
    assert_eq!(
        declared_pair_overlap(&a, fa, &b, fb, ContactVerdict::Definite, band()).unwrap(),
        ChartOverlap::PositiveArea,
    );
}

/// INVARIANT (argument-order symmetry, the spec's third binding
/// condition): `overlap(A, B)` and `overlap(B, A)` agree. The verdict is
/// a property of the pair, not of which description was taken as the
/// representative frame — the lemma at `world_carrier`.
#[test]
fn the_verdict_does_not_depend_on_which_frame_is_representative() {
    let eps = Tol::witness().eps();
    let cases: Vec<(&str, Body<f64>, Body<f64>, ContactVerdict)> = vec![
        (
            "coincident",
            slab(1.0),
            plate(1.0, 0.0),
            ContactVerdict::Definite,
        ),
        (
            "a peg-extent tilt the pair absorbs",
            slab(1.0),
            plate(1.0, eps / 2.0),
            ContactVerdict::Definite,
        ),
        (
            "a table-extent tilt the pair does not",
            slab(100.0),
            plate(100.0, eps / 2.0),
            ContactVerdict::Definite,
        ),
        (
            "a bridged pair, whose witness rung declines",
            slab(1.0),
            plate(1.0, 0.0),
            ContactVerdict::Bridged,
        ),
    ];
    for (what, a, b, verdict) in cases {
        let (fa, fb) = pair(&a, &b);
        let ab = declared_pair_overlap(&a, fa, &b, fb, verdict, band());
        let ba = declared_pair_overlap(&b, fb, &a, fa, verdict, band());
        assert_eq!(
            format!("{ab:?}"),
            format!("{ba:?}"),
            "{what}: the two argument orders disagreed — {ab:?} vs {ba:?}"
        );
    }
}

// ---------------------------------------------------------------------
// `chart_region_carrier_tilt` — the lever is the PAIR'S OWN EXTENT
// ---------------------------------------------------------------------

/// INVARIANT (the spec's second binding condition): the residue is
/// metered at the pair's own chart extent, so ONE tilt gets TWO honest
/// answers depending on how big the contact is. Door 1's pinned 1 m
/// lever arm cannot tell these two apart — it sees the same angle and
/// passes both.
#[test]
fn one_tilt_two_extents_two_answers() {
    let slope = Tol::witness().eps() / 2.0;

    // A peg: the same slope subtends half an ε over the whole contact,
    // so the two carriers agree everywhere the trims reach.
    let (a, b) = (slab(1.0), plate(1.0, slope));
    let (fa, fb) = pair(&a, &b);
    assert_eq!(
        declared_pair_overlap(&a, fa, &b, fb, ContactVerdict::Definite, band()).unwrap(),
        ChartOverlap::PositiveArea,
        "at a peg's extent the tilt is below the band and the carrier is a chart",
    );

    // A table: the same slope, a hundredfold extent, and the far end of
    // the contact is definitely off the representative carrier.
    let (a, b) = (slab(100.0), plate(100.0, slope));
    let (fa, fb) = pair(&a, &b);
    match declared_pair_overlap(&a, fa, &b, fb, ContactVerdict::Definite, band()) {
        Err(ChartRegionError::CarrierTilt) => {}
        other => panic!("at the table's own extent the same tilt must refuse typed, got {other:?}"),
    }
}

/// INVARIANT (the row's THIRD outcome, pinned rather than asserted in
/// prose): a carrier disagreement that lands IN the band escalates
/// typed and names the predicate. Without this row two of
/// `chart_region_carrier_tilt`'s outcomes are exercised and the
/// genuine-residue one is only claimed.
///
/// The fixture is ε-relative for the reason the module header gives:
/// the whole content of this row is where a residue falls relative to
/// ε. `3ε` of separation over the pair's own extent is strictly inside
/// `(ε, Kε)` at every ε the matrix runs, since `K = 10`.
#[test]
fn an_in_band_carrier_disagreement_escalates_naming_the_row() {
    let eps = Tol::witness().eps();
    // Extent 1 m, so the far end's separation IS the slope: 3ε.
    let (a, b) = (slab(1.0), plate(1.0, 3.0 * eps));
    let (fa, fb) = pair(&a, &b);
    match declared_pair_overlap(&a, fa, &b, fb, ContactVerdict::Definite, band()) {
        Err(ChartRegionError::Escalated(diag)) => {
            assert_eq!(
                diag.predicate,
                Some("chart_region_carrier_tilt"),
                "the escalation names the row that could not decide: {diag:?}"
            );
        }
        other => panic!("an in-band carrier residue escalates typed, got {other:?}"),
    }
}

/// INVARIANT: the tilt refusal is a REFUSAL, not a contradiction and
/// not an `Empty`. The pair may well be in contact; what the door says
/// is that it cannot certify the overlap in either description's frame,
/// which is the honest third outcome.
#[test]
fn the_tilt_refusal_is_never_an_empty() {
    let slope = Tol::witness().eps() / 2.0;
    let (a, b) = (slab(100.0), plate(100.0, slope));
    let (fa, fb) = pair(&a, &b);
    let v = declared_pair_overlap(&a, fa, &b, fb, ContactVerdict::Definite, band());
    assert!(
        !matches!(v, Ok(ChartOverlap::Empty)),
        "a carrier the door cannot read is not evidence of disjointness: {v:?}"
    );
}

// ---------------------------------------------------------------------
// Door 1's verdict, consumed
// ---------------------------------------------------------------------

/// INVARIANT (the spec's first binding condition, where it bites): the
/// interior-witness rung discharges `contfp`'s "q already on the plane
/// of `face`" precondition by carrier agreement, and a `Bridged`
/// verdict says that agreement rests on the DECLARATION rather than on
/// the geometry. A precondition may not be discharged by the claim
/// under test, so the rung declines and the region walk's typed refusal
/// stands.
#[test]
fn a_bridged_door_one_declines_the_interior_witness_rung() {
    // A pair sharing their whole boundary: the region walk refuses
    // `TouchingBoundary` and only the witness rung can answer.
    let (a, b) = (slab(1.0), plate(1.0, 0.0));
    let (fa, fb) = pair(&a, &b);
    assert_eq!(
        declared_pair_overlap(&a, fa, &b, fb, ContactVerdict::Definite, band()).unwrap(),
        ChartOverlap::PositiveArea,
        "on the geometry's own evidence the rung proves the disc",
    );
    match declared_pair_overlap(&a, fa, &b, fb, ContactVerdict::Bridged, band()) {
        Err(ChartRegionError::TouchingBoundary) => {}
        other => panic!("a bridged pair must keep the region walk's refusal, got {other:?}"),
    }
}

// ---------------------------------------------------------------------
// What stays refused
// ---------------------------------------------------------------------

/// INVARIANT (the spec's scope line): cross-instance CURVED declared
/// pairs stay refused. Two independently authored curved descriptions
/// of one locus differ in `u_ref` and seam, no world embedding
/// arbitrates that, and there is no isometry lemma to be had — the arm
/// is PLANAR by the same reasoning that licenses it.
#[test]
fn a_declared_curved_cross_instance_pair_is_still_refused() {
    let a: common::Prism<f64> = common::prism_z(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0)], 0.0, 1.0);
    let b: common::Prism<f64> = common::prism_z(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0)], 1.0, 2.0);
    // Re-describe both interface faces as CYLINDERS of one radius: two
    // independently authored curved descriptions, each in its own
    // arena, with no shared key and no `GeomSource`.
    let (mut a_body, mut b_body) = (a.body, b.body);
    let cyl = || Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, 1.0),
        axis: geom_core::Vec3::unit_z(),
        radius: 1.0,
        u_ref: geom_core::Vec3::unit_x(),
    };
    a_body
        .set_face_surface(a.top_face, FaceSurface::New(cyl()))
        .unwrap();
    b_body
        .set_face_surface(b.bottom_face, FaceSurface::New(cyl()))
        .unwrap();
    match declared_pair_overlap(
        &a_body,
        a.top_face,
        &b_body,
        b.bottom_face,
        ContactVerdict::Definite,
        band(),
    ) {
        Err(ChartRegionError::ChartDivergence { .. }) => {}
        other => panic!("a curved cross-instance declared pair stays refused: {other:?}"),
    }
}

// ---------------------------------------------------------------------
// Argument-order symmetry, under ROTATIONS (the #1063 fix pass)
// ---------------------------------------------------------------------
//
// The corpus above builds both bodies from ONE cube mapper, so the map
// between the two frames is always a reflection with axis-aligned
// `u_ref`s — which exercises the orientation half of the lemma and
// nothing else. The rows below rotate the B-side body in its own plane
// by angles that are not multiples of a right angle, so the map between
// the two chart frames is a genuine rotation and the trims land on
// edges of every length ratio against each other.
//
// This is where the ONE real asymmetry was found: `chart_region_parallel`
// levered its determinant by the FIRST polygon's edge length, so a short
// edge against a long one at a near-parallel angle read a margin a
// length-ratio apart in the two orders — mid-band one way, definite the
// other, which is a different GATE VERDICT and not the few-ulp frame
// variance the lemma concedes. Both levers are union-max forms now.

/// A plate whose underside is the rectangle `[0, w] × [0, d]` at
/// `z = 1`, ROTATED by `deg` about the z-axis through the origin and
/// then translated by `(tx, ty)`.
fn turned_plate(w: f64, d: f64, deg: f64, tx: f64, ty: f64) -> Body<f64> {
    let (c, s) = (deg.to_radians().cos(), deg.to_radians().sin());
    common::mapped_cube(|x, y, z| {
        let (px, py) = (x * w, y * d);
        Point3::new(px * c - py * s + tx, px * s + py * c + ty, 1.0 + z)
    })
}

/// Both argument orders of one configuration, rendered for comparison.
fn both_orders(a: &Body<f64>, b: &Body<f64>) -> (String, String) {
    let (fa, fb) = pair(a, b);
    (
        format!(
            "{:?}",
            declared_pair_overlap(a, fa, b, fb, ContactVerdict::Definite, band())
        ),
        format!(
            "{:?}",
            declared_pair_overlap(b, fb, a, fa, ContactVerdict::Definite, band())
        ),
    )
}

/// INVARIANT (the spec's third binding condition, under rotations): the
/// gate's verdict is a property of the PAIR. Not "the certified answers
/// agree when both are certified" — the two calls must land on the same
/// outcome, refusals included, because a declaration that certifies only
/// when the mate happens to name the post before the shelf is not a
/// certification.
#[test]
fn the_verdict_is_argument_order_symmetric_under_rotation() {
    let base = slab(1.0);
    let eps = Tol::witness().eps();
    // The angle list is NOT all coarse. Three of these are ε-relative
    // near-parallel angles, and they are the regime the one-sided lever
    // actually broke in: at a coarse angle the determinant is definite
    // whichever length levers it, so a battery of coarse rotations is
    // wide and shallow and passes a predicate that is wrong. `10ε` and
    // `100ε` at a 20:1 edge-length ratio put the two one-sided margins
    // on opposite sides of the band.
    let near = [
        (2.0 * eps).atan().to_degrees(),
        (10.0 * eps).atan().to_degrees(),
        (100.0 * eps).atan().to_degrees(),
    ];
    let mut angles = vec![0.0, 1.0, 7.0, 17.0, 30.0, 45.0, 63.5, 89.0, 91.0, 137.0];
    angles.extend(near);
    // And the 90°-neighbourhood of each near-parallel angle, where the
    // SHORT edge is the near-parallel partner rather than the long one.
    angles.extend(near.iter().map(|a| 90.0 - a));
    let mut checked = 0usize;
    for &deg in &angles {
        for &(w, d) in &[(1.0, 1.0), (0.05, 0.05), (0.9, 0.05), (0.05, 0.9)] {
            for &(tx, ty) in &[(0.0, 0.0), (0.3, 0.2), (0.5, 0.5), (0.95, 0.0)] {
                let b = turned_plate(w, d, deg, tx, ty);
                let (ab, ba) = both_orders(&base, &b);
                assert_eq!(
                    ab, ba,
                    "deg = {deg}, plate = {w} x {d}, at ({tx}, {ty}): the two \
                     argument orders disagreed — {ab} vs {ba}"
                );
                checked += 1;
            }
        }
    }
    assert_eq!(checked, 256, "the battery is the size it says it is");
}

/// INVARIANT, the R1 probe that found the defect, kept as a row: a SHORT
/// edge crossing a LONG edge's line at a near-parallel angle, far off
/// the segment, with a fat overlap between the two trims.
///
/// Before the fix this pair read `Ok(PositiveArea)` one way and
/// `Err(Escalated)` the other, because the parallel row's margin was
/// `|denom| / |r|` — the perpendicular height of the SECOND edge across
/// the first's line, which is a length-ratio apart from its mirror. The
/// union-max lever makes the margin `max(|r|, |s|) · sin θ` in both
/// orders. The pair is a plainly-overlapping one, so the symmetric
/// answer it settles on is also the RIGHT one: certified, not refused.
#[test]
fn the_short_against_long_near_parallel_edge_pair_agrees_both_ways() {
    // Angle chosen so the near-parallel margin sits where the two
    // one-sided levers straddled the band: the long edge's height
    // across the short edge's line is definite, the short edge's across
    // the long one's is not.
    let eps = Tol::witness().eps();
    let deg = (100.0 * eps).atan().to_degrees();
    let base = slab(1.0);
    let b = turned_plate(0.05, 0.9, deg, 0.3, 0.05);
    let (ab, ba) = both_orders(&base, &b);
    assert_eq!(ab, ba, "the probe disagreed: {ab} vs {ba}");
    assert_eq!(
        ab, "Ok(PositiveArea)",
        "and a pair whose trims plainly overlap certifies rather than \
         escalating: {ab}"
    );
}

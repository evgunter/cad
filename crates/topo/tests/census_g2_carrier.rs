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
                assert!(found.is_none(), "the z-facing face is unique in this fixture");
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
    common::mapped_cube(|x, y, z| {
        Point3::new(x * len, 0.25 + y * 0.5, 1.0 + slope * x * len + z)
    })
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
        other => panic!(
            "at the table's own extent the same tilt must refuse typed, got {other:?}"
        ),
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

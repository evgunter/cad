//! M9-3 PR-B — the zip and the marks: the two-peg kernel path
//! (CONTACT-DESIGN's considered-not-built demo, now built): plate P
//! with two pegs, plate Q with two through-bores, mated on one plane;
//! three declared `Rest` contact groups (one planar + two
//! cylindrical); the union removes all three patches as interior,
//! the bore walls vanish (full engagement), and the volume is exactly
//! additive (the C7-lane statement).
//!
//! Acceptance (ii) is the kissing rounds: two convex quarter-round
//! walls touching externally along one ruling, a wedge-2π rim. (The
//! wedge-π G1 "tube chain" is a different fixture, the torus chain in
//! `mate7a_torus_rest.rs`.)

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::operands::{plate6 as plate, plate6_cyl};
use geom_core::{Affine3, Point2, Tol, Vec3};
use profile::{Profile, RawLoop, SketchPlane, test_support::bulge_loop};
use sweep::{Extrusion, extrude};
use topo::readback::euler_counts;
use topo::{
    Body, BooleanDeclarations, BooleanResult, ContactClass, FacePairDeclaration, mass_properties,
};

fn body_of(r: BooleanResult<f64>) -> Body<f64> {
    match r {
        BooleanResult::Body(b) => b.body,
        BooleanResult::Empty => panic!("a two-peg operand cannot be empty"),
    }
}

/// Plate P: base [0,1] with two pegs rising to z = 2 (embedded boss
/// unions — the shipped transverse lane).
fn plate_with_pegs() -> Body<f64> {
    let p0 = plate(0.0);
    let p1 = body_of(topo::union(&p0, &plate6_cyl(2.0, 0.4, 1.6, 0.5), Tol::witness()).unwrap());
    body_of(topo::union(&p1, &plate6_cyl(4.0, 0.4, 1.6, 0.5), Tol::witness()).unwrap())
}

/// Plate Q: z ∈ [1, 2] with two through-bores (the shipped transverse
/// subtracts).
fn plate_with_bores() -> Body<f64> {
    let q0 = plate(1.0);
    let q1 = body_of(topo::subtract(&q0, &plate6_cyl(2.0, 0.8, 1.4, 0.5), Tol::witness()).unwrap());
    body_of(topo::subtract(&q1, &plate6_cyl(4.0, 0.8, 1.4, 0.5), Tol::witness()).unwrap())
}

/// The cylinder faces of a body whose axis x is near `cx`.
fn walls_at(body: &Body<f64>, cx: f64) -> Vec<topo::FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Cylinder { origin, .. }) if (origin.x - cx).abs() < 0.5
            )
        })
        .map(|(k, _)| k)
        .collect()
}

/// The planar face at height `z` facing `up`.
fn plane_face(body: &Body<f64>, z: f64, up: bool) -> topo::FaceKey {
    let hits: Vec<_> = body
        .faces()
        .filter(|(_, f)| match body.get_surface(f.surface) {
            Some(geom::Surface::Plane { origin, normal, .. }) => {
                (origin.z - z).abs() < 1e-12 && (normal.z > 0.5) == up
            }
            _ => false,
        })
        .map(|(k, _)| k)
        .collect();
    let [f] = hits[..] else {
        panic!("expected exactly one z = {z} face (up = {up}), got {hits:?}");
    };
    f
}

/// The three declared contact groups: the mating plane (P top × Q
/// bottom) and the two cylinder bands, each peg declared against its
/// own bore's walls only (cross-peg pairs are DISTINCT carriers and
/// would be contradicted — correctly).
fn declarations(p: &Body<f64>, q: &Body<f64>) -> BooleanDeclarations {
    let mut decls = BooleanDeclarations::none();
    decls.coincident_faces.push(FacePairDeclaration::new(
        plane_face(p, 1.0, true),
        plane_face(q, 1.0, false),
        ContactClass::Rest,
    ));
    for cx in [2.0, 4.0] {
        for &fa in &walls_at(p, cx) {
            for &fb in &walls_at(q, cx) {
                decls
                    .coincident_faces
                    .push(FacePairDeclaration::new(fa, fb, ContactClass::Rest));
            }
        }
    }
    decls
}

/// Acceptance (i): the two-peg kernel path — three declared contacts,
/// union succeeds, volume EXACTLY additive (vol(P∪Q) = vol(P)+vol(Q):
/// interiors are disjoint, nothing is discarded — and the π terms of
/// the pegs and bores cancel against 48 exactly).
#[test]
fn two_peg_plate_union_is_exactly_additive() {
    let p = plate_with_pegs();
    let q = plate_with_bores();
    let vp = mass_properties(&p, Tol::witness()).unwrap().volume;
    let vq = mass_properties(&q, Tol::witness()).unwrap().volume;
    let decls = declarations(&p, &q);
    let out =
        topo::union_with(&p, &q, &decls, Tol::witness()).expect("the two-peg kernel path unions");
    let BooleanResult::Body(bb) = out else {
        panic!("a two-peg union cannot be empty");
    };
    let body = bb.body;
    let v = mass_properties(&body, Tol::witness()).unwrap().volume;
    assert_eq!(v, vp + vq, "exactly additive (the C7-lane statement)");
    // And the closed form: the peg and bore π-terms cancel exactly —
    // vol(P) + vol(Q) = (24 + π/2) + (24 − π/2) = 48, bitwise.
    assert_eq!(v, 48.0, "the closed-form oracle");
    // The bore walls vanished with full engagement: no cylinder
    // surface survives anywhere in the result.
    assert!(
        body.faces().all(|(_, f)| !matches!(
            body.get_surface(f.surface),
            Some(geom::Surface::Cylinder { .. })
        )),
        "full-engagement patch removal deletes every wall face"
    );
    if let Err(errs) = topo::validate_geometric(&body, Tol::witness()) {
        panic!("the mated pair must be tier-3 valid: {errs:?}");
    }
    // Topology pinned: ONE shell, genus 0 (every handle the bores
    // opened is closed by its peg). Euler–Poincaré with rings:
    // V − E + F − R = 2(S − H); each peg's circular seam survives as
    // an inner ring on the surrounding planar face, so R = 2.
    let counts = euler_counts(&body);
    assert_eq!(counts.s, 1, "one shell");
    assert_eq!(counts.r, 2, "one surviving circular ring per peg seam");
    assert_eq!(
        counts.genus(),
        Ok(0),
        "Euler–Poincaré of a genus-0 single shell"
    );
    if let Err(errs) = topo::validate_pseudomanifold(&body, &bb.contacts, Tol::witness()) {
        panic!("the mated pair must be pseudomanifold-clean: {errs:?}");
    }
}

// -------------------------------------------------------------------
// Acceptance (ii): the kissing-rounds rim on the DEV-1 carriers — two
// EQUAL-RADIUS convex quarter-round walls (axes (2,·,0) and (2,·,2))
// kissing externally along one ruling, opposite outward normals there
// (the parallel-cylinder witness lane), mated by a declared planar
// Rest with the wall pairs declared Tangent. The rim survives the zip
// as the wedge-2π kiss, a slit interior to material, and carries the
// INTRINSIC
// `TangentIntersection` description (the D6 smooth ladder's mint —
// the jet is determinate: κ_rel = 1/r + 1/r definite).
// -------------------------------------------------------------------

/// Sketch frame: sketch x → world z, sketch y → world x, extrusion
/// along +y (the lying frame).
fn lying_plane() -> SketchPlane<f64> {
    SketchPlane::new(Affine3::from_parts(
        geom_core::Mat3::from_cols(Vec3::unit_z(), Vec3::unit_x(), Vec3::unit_y()),
        Vec3::new(0.0, 0.0, 0.0),
    ))
}

fn lying_extrude(vertices: Vec<(Point2<f64>, f64)>, tangent_joints: Vec<usize>) -> Body<f64> {
    let profile = Profile::new(
        lying_plane(),
        vec![bulge_loop(vertices).with_tangent_joints(tangent_joints)],
    )
    .validate(Tol::witness())
    .unwrap();
    extrude(&profile, Extrusion::Distance(4.0), Tol::witness())
        .unwrap()
        .body
}

/// Body A: slab x ∈ [0,3], z ∈ [0,1], its top-right profile edge
/// rounded by a radius-1 quarter arc tangent to z = 1 at x = 2
/// (cylinder axis (2, ·, 0)); y ∈ [0, 4].
fn quarter_round_below() -> Body<f64> {
    let b90 = (core::f64::consts::PI / 8.0).tan();
    lying_extrude(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(1.0, 0.0), 0.0),
            (Point2::new(1.0, 2.0), b90),
            (Point2::new(0.0, 3.0), 0.0),
        ],
        vec![2],
    )
}

/// Body B: slab x ∈ [0.5, 3], z ∈ [1, 3], its bottom-right profile
/// edge rounded by a radius-1 quarter arc tangent to z = 1 at x = 2
/// (cylinder axis (2, ·, 2)); rests on A's top face; y ∈ [0, 4].
fn quarter_round_above() -> Body<f64> {
    let b90 = (core::f64::consts::PI / 8.0).tan();
    lying_extrude(
        vec![
            (Point2::new(1.0, 0.5), 0.0),
            (Point2::new(1.0, 2.0), -b90),
            (Point2::new(2.0, 3.0), 0.0),
            (Point2::new(3.0, 3.0), 0.0),
            (Point2::new(3.0, 0.5), 0.0),
        ],
        vec![1, 2],
    )
}

fn cyl_face(body: &Body<f64>) -> topo::FaceKey {
    let hits: Vec<_> = body
        .faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Cylinder { .. })
            )
        })
        .map(|(k, _)| k)
        .collect();
    let [f] = hits[..] else {
        panic!("expected exactly one wall face, got {hits:?}");
    };
    f
}

#[test]
fn kissing_rounds_rim_unions_and_carries_the_tangent_intersection() {
    let a = quarter_round_below();
    let b = quarter_round_above();
    let va = mass_properties(&a, Tol::witness()).unwrap().volume;
    let vb = mass_properties(&b, Tol::witness()).unwrap().volume;
    let mut decls = BooleanDeclarations::none();
    // The mate: B rests on A's top face.
    decls.coincident_faces.push(FacePairDeclaration::new(
        plane_face(&a, 1.0, true),
        plane_face(&b, 1.0, false),
        ContactClass::Rest,
    ));
    // The tangencies, all in the DEV-1 witness lane: wall × wall
    // (parallel cylinders), and each wall against the other body's
    // mating plane (plane × cylinder along the same ruling).
    decls.coincident_faces.push(FacePairDeclaration::new(
        cyl_face(&a),
        cyl_face(&b),
        ContactClass::Tangent,
    ));
    decls.coincident_faces.push(FacePairDeclaration::new(
        plane_face(&a, 1.0, true),
        cyl_face(&b),
        ContactClass::Tangent,
    ));
    decls.coincident_faces.push(FacePairDeclaration::new(
        cyl_face(&a),
        plane_face(&b, 1.0, false),
        ContactClass::Tangent,
    ));
    let out =
        topo::union_with(&a, &b, &decls, Tol::witness()).expect("the kissing-rounds union runs");
    let BooleanResult::Body(bb) = out else {
        panic!("a kissing-rounds union cannot be empty");
    };
    let body = bb.body;
    let v = mass_properties(&body, Tol::witness()).unwrap().volume;
    // Additive to re-association rounding: the seam chord at x = 0.5
    // SPLITS A's top face, so the same per-edge flux terms are summed
    // in a different association — each term is O(V) and one
    // re-association perturbs the sum by O(ulp(V)); the measured
    // residual is 0.5 ulp. Bound: 4·ulp(va + vb), headroom included
    // and still orders under any geometric signal. (The two-peg
    // path's patches are WHOLE faces, which is why ITS row is
    // bitwise.)
    let ulp = (va + vb) * f64::EPSILON;
    assert!(
        (v - (va + vb)).abs() <= 4.0 * ulp,
        "additive volume: {v} vs {} (allowed {})",
        va + vb,
        4.0 * ulp
    );

    // The rim: the seam edges between the two cylinder walls — the
    // wedge-2π kiss — carry the INTRINSIC tangency
    // description on their line carrier (D6's smooth ladder; U2's
    // taxonomy, no new variant).
    let face_kind = |he| {
        body.get_half_edge(he)
            .and_then(|h| body.get_loop(h.parent_loop))
            .and_then(|l| body.get_face(l.face))
            .and_then(|f| body.get_surface(f.surface))
            .map(geom_brep::SurfaceKind::of)
    };
    let mut rim_edges = Vec::new();
    for (k, e) in body.edges() {
        let Some(c) = body.get_curve_geom(e.curve).and_then(|g| g.certified()) else {
            continue;
        };
        if face_kind(e.he_plus) == Some(geom_brep::SurfaceKind::Cylinder)
            && face_kind(e.he_minus) == Some(geom_brep::SurfaceKind::Cylinder)
        {
            rim_edges.push(k);
            assert!(
                matches!(
                    c.description(),
                    geom_brep::EdgeDescription::TangentIntersection { .. }
                ),
                "the kissing rim is intrinsically described: {:?}",
                c.description()
            );
        }
    }
    // ONE rim edge: the two fused tangent edges survive as the single
    // wall–wall ruling.
    assert_eq!(rim_edges.len(), 1, "the rim seam is one fused ruling");
    // The rim is a wedge-2π edge — the ruling's "kissing union, a slit
    // interior to material" — and its tangency is jet-determinate, so
    // under D1's second-order arm it is legal at rest, derived from the
    // body exactly as a π seam is. The intent was declared where the
    // tangency was created: this op was GIVEN the wall × wall `Tangent`
    // mate above.
    if let Err(errs) = topo::validate_geometric(&body, Tol::witness()) {
        panic!("the kissing-rounds body must be tier-3 valid: {errs:?}");
    }
    // The tier-3 contact mark agrees: the rim EDGE ITSELF is the
    // must-carry's own regime (jet-determinate tangency), satisfied
    // by the mint — tied to the rim, not a body-wide census.
    let marks = topo::contact_marks(&body, Tol::witness()).expect("marks derive at rest");
    for &k in &rim_edges {
        assert_eq!(
            marks.get(k).copied(),
            Some(topo::ContactMark::Tangent),
            "the rim edge carries the Tangent contact mark"
        );
    }
    // And the census is pinned: exactly TWO tangent-marked edges —
    // the rim, plus the upper profile's own authored G1 joint at
    // (2,3) (its flat top wall meeting its quarter-round wall), which
    // survives the union untouched. The two profiles' joints AT the
    // shared ruling fused INTO the rim edge itself.
    let tangent: Vec<_> = marks
        .iter()
        .filter(|&(_, m)| *m == topo::ContactMark::Tangent)
        .map(|(k, _)| k)
        .collect();
    assert_eq!(tangent.len(), 2, "the rim plus one authored joint");
    for k in tangent {
        if rim_edges.contains(&k) {
            continue;
        }
        // The non-rim tangent edge is the authored wall-top joint:
        // one plane flank, one cylinder flank.
        let e = body.get_edge(k).expect("marked edge exists");
        let kinds = [face_kind(e.he_plus), face_kind(e.he_minus)];
        assert!(
            kinds.contains(&Some(geom_brep::SurfaceKind::Plane))
                && kinds.contains(&Some(geom_brep::SurfaceKind::Cylinder)),
            "the surviving authored joint is the plane-wall/round-wall seam: {kinds:?}"
        );
    }
    // Topology pinned: ONE shell, genus 0, no rings:
    // V − E + F − R = 2(S − H).
    let counts = euler_counts(&body);
    assert_eq!(counts.s, 1, "one shell");
    assert_eq!(counts.r, 0, "no ring loops in the kissing rounds");
    assert_eq!(
        counts.genus(),
        Ok(0),
        "Euler–Poincaré of a genus-0 single shell"
    );
    // 3′ judges the rim as tier 3 does — its local battery reads no
    // record — over the contacts the op itself emitted.
    if let Err(errs) = topo::validate_pseudomanifold(&body, &bb.contacts, Tol::witness()) {
        panic!("the kissing rounds must be pseudomanifold-clean: {errs:?}");
    }
}

/// A slab under the upper quarter round, wide enough that the round's
/// rim vertices land INSIDE its top face (x ∈ [0, 5], y ∈ [−1, 5],
/// z ∈ [0, 1]).
fn wide_slab_below() -> Body<f64> {
    let plane = SketchPlane::new(Affine3::from_parts(
        geom_core::Mat3::from_cols(Vec3::unit_z(), Vec3::unit_x(), Vec3::unit_y()),
        Vec3::new(0.0, -1.0, 0.0),
    ));
    let profile = Profile::new(
        plane,
        vec![bulge_loop(vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(1.0, 0.0), 0.0),
            (Point2::new(1.0, 5.0), 0.0),
            (Point2::new(0.0, 5.0), 0.0),
        ])],
    )
    .validate(Tol::witness())
    .unwrap();
    extrude(&profile, Extrusion::Distance(6.0), Tol::witness())
        .unwrap()
        .body
}

/// **A declared-`Tangent` CURVED sector at a vertex on a face is lumped
/// whole.** The quarter round rests on a slab whose top face contains
/// its rim vertices, so the vertex-on-face door meets the round's wall
/// sector with both bounds `On`: the tangency ruling, and the arc,
/// which departs in the plane. Undeclared, that curved on-carrier
/// sector is the typed frontier; declared `Tangent`, the door lumps it
/// and the classification completes — the slab minus the round is the
/// slab, and their intersection is empty. (The union is not asserted:
/// at `ε = 1e-6` it refuses `ClassificationInvariant` at the run's germ,
/// filed with the band-edge split below.)
///
/// What tells the whole-sector lump from a per-bound reading here is a
/// defect: read per bound, the ruling is `On` and so is the arc, whose
/// lever arm is the sliver the band-edge split leaves
/// (`work/hone/an-arc-tangent-to-a-face-at-its-end-is-split-at-the-edge-of-the-band.md`),
/// and two consecutive `On`s refuse. With the split fixed the arc reads
/// its own side and per bound should pass too
/// (`work/tang/vtxfac-tangent-sector-should-descend-per-bound.md`).
#[test]
fn a_tangent_curved_sector_on_a_face_lumps_whole() {
    let a = wide_slab_below();
    let b = quarter_round_above();
    let va = mass_properties(&a, Tol::witness()).unwrap().volume;
    let mut decls = BooleanDeclarations::none();
    decls.coincident_faces.push(FacePairDeclaration::new(
        plane_face(&a, 1.0, true),
        plane_face(&b, 1.0, false),
        ContactClass::Rest,
    ));
    let undeclared = topo::subtract_with(&a, &b, &decls, Tol::witness())
        .expect_err("an undeclared curved on-carrier sector is the typed frontier");
    assert!(
        matches!(
            undeclared,
            topo::BooleanError::CurvedBooleanUnsupported { face, .. } if face == cyl_face(&b)
        ),
        "the frontier names the round's wall: {undeclared:?}"
    );
    decls.coincident_faces.push(FacePairDeclaration::new(
        plane_face(&a, 1.0, true),
        cyl_face(&b),
        ContactClass::Tangent,
    ));
    let diff = topo::subtract_with(&a, &b, &decls, Tol::witness())
        .expect("the declared tangent sector lumps and the difference runs");
    let BooleanResult::Body(diff) = diff else {
        panic!("the slab minus a body resting on it is the slab, not empty");
    };
    let v = mass_properties(&diff.body, Tol::witness()).unwrap().volume;
    assert!(
        (v - va).abs() <= 4.0 * va * f64::EPSILON,
        "the slab minus the round is the slab: {v} vs {va}"
    );
    let meet = topo::intersect_with(&a, &b, &decls, Tol::witness())
        .expect("the declared tangent sector lumps and the intersection runs");
    assert!(
        matches!(meet, BooleanResult::Empty),
        "a resting contact encloses no volume"
    );
}

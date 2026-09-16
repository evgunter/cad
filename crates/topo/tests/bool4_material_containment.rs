//! **Material containment in the census's instance arm** (issue 750).
//!
//! The instance-containment arm used to answer from extent boxes where
//! the question is material: a part sitting in a concavity has its box
//! inside the container's box while sharing no material, so every
//! L-bracket, pocket and cavity assembly refused as undecidable by any
//! declaration. Now the box is the GATE and the material test decides
//! — the contained instance's vertices probed against the container's
//! material through the per-solid point-in-solid door
//! (`topo::point_in_solid_of`). These rows are the issue's geometry
//! verbatim plus the placements the spec names, each pinned at the
//! base first (the verdicts that moved are recorded in the PR).
//!
//! Neither of the issue's two falsifications is retried here or in the
//! arm: no contact record is consulted (a declaration must never turn
//! an examination off), and no separating plane is derived from the
//! container's own faces (unsound on exactly the non-convex containers
//! this is about).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;
use geom_core::{Band, Point3, Tol};
use topo::{
    Body, CensusContact, ContactRecords, EntityId, FaceKey, SolidContainment, SolidKey,
    ValidationError, VfContact, VoidContainment, VoidEvidence, insert_void, point_in_solid_of,
    validate_pseudomanifold,
};

/// The issue's L profile: counterclockwise from `+z`, reflex at (1, 1).
const L_PROFILE: [(f64, f64); 6] = [
    (0.0, 0.0),
    (3.0, 0.0),
    (3.0, 1.0),
    (1.0, 1.0),
    (1.0, 3.0),
    (0.0, 3.0),
];

/// The pair as one two-instance arena: `a` keeps its keys, `b` is
/// grafted (fresh keys, equal geometry).
fn assembly(a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    let mut out = a.clone();
    topo::graft_disjoint(&mut out, b, Tol::witness()).unwrap();
    out
}

/// Every vertex of `body` whose point satisfies `pick`.
fn vertices_where(body: &Body<f64>, pick: impl Fn(Point3<f64>) -> bool) -> Vec<topo::VertexKey> {
    body.vertices()
        .filter(|(_, v)| pick(*body.get_point(v.point).unwrap()))
        .map(|(k, _)| k)
        .collect()
}

fn point_of(body: &Body<f64>, v: topo::VertexKey) -> Point3<f64> {
    *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
}

/// The two solids of a two-instance arena, in arena order.
fn two_solids(body: &Body<f64>) -> (SolidKey, SolidKey) {
    let solids: Vec<SolidKey> = body.solids().map(|(k, _)| k).collect();
    let [a, b] = solids[..] else {
        panic!("a two-instance arena: {solids:?}");
    };
    (a, b)
}

/// **The L-bracket** (issue 750, verbatim): the container over the L
/// profile, `z ∈ [0, 1]`; the part `x ∈ [1, 2]`, `y ∈ [1.2, 2]`,
/// `z ∈ [0.2, 0.8]`, resting flat on the inner wall `x = 1`, wholly
/// outside the bracket's material. `dx` shifts the part along `+x`;
/// `declared` adds the four v-on-f records on the wall.
fn lbracket(declared: bool, dx: f64) -> (Body<f64>, ContactRecords) {
    let l = common::prism_z::<f64>(&L_PROFILE, 0.0, 1.0);
    // The side face over profile segment 3, (1, 1) → (1, 3): the wall.
    let wall: FaceKey = l.side_faces[3];
    let part = common::brick::<f64>((1.0 + dx, 2.0 + dx), (1.2, 2.0), (0.2, 0.8));
    let body = assembly(&l.body, &part);
    let mut records = ContactRecords::default();
    if declared {
        for v in vertices_where(&body, |p| {
            (p.x - (1.0 + dx)).abs() < 1e-12 && (1.1..2.1).contains(&p.y)
        }) {
            records.b_on_a.push(VfContact {
                vertex: v,
                face: wall,
            });
        }
        assert_eq!(
            records.b_on_a.len(),
            4,
            "the fixture's four resting corners"
        );
    }
    (body, records)
}

/// A cube of side `side` with its minimum corner at `(dx, dy, dz)`.
fn cube(side: f64, dx: f64, dy: f64, dz: f64) -> Body<f64> {
    common::mapped_cube(|x, y, z| Point3::new(side * x + dx, side * y + dy, side * z + dz))
}

/// **The embedded cube** (`h14_census_deferrals`' fixture): 1 m in
/// 4 m, flush at `z = 0`, its four bottom corners declared v-on-f on
/// the big cube's bottom face, every record true. Flush means one box
/// margin is exactly zero, so the box gate cannot separate the pair;
/// the material test decides it.
fn embedded() -> (Body<f64>, ContactRecords) {
    let body = assembly(&cube(4.0, 0.0, 0.0, 0.0), &cube(1.0, 1.0, 1.0, 0.0));
    let big_bottom = body
        .faces()
        .map(|(f, _)| f)
        .find(|&f| {
            let face = body.get_face(f).unwrap();
            let l = body.get_loop(face.outer).unwrap();
            let topo::LoopBoundary::Cycle { first } = l.boundary else {
                return false;
            };
            body.loop_cycle(first).unwrap().into_iter().all(|he| {
                let p = point_of(&body, body.get_half_edge(he).unwrap().start);
                p.z.abs() < 1e-12 && !(0.9..2.1).contains(&p.x)
            })
        })
        .unwrap();
    let mut records = ContactRecords::default();
    for v in vertices_where(&body, |p| {
        p.z.abs() < 1e-12 && (0.9..2.1).contains(&p.x) && (0.9..2.1).contains(&p.y)
    }) {
        records.b_on_a.push(VfContact {
            vertex: v,
            face: big_bottom,
        });
    }
    assert_eq!(records.b_on_a.len(), 4);
    (body, records)
}

/// **The cavity**: a 3 m cube hollowed by a 1 m void (two shells, one
/// solid), with a 0.6 m part floating in the void — inside the
/// container's box, inside its VOID, outside its material.
fn cavity() -> Body<f64> {
    let mut dst = common::brick::<f64>((0.0, 3.0), (0.0, 3.0), (0.0, 3.0));
    let hole = common::brick::<f64>((1.0, 2.0), (1.0, 2.0), (1.0, 2.0));
    let (solid, _) = dst.solids().next().unwrap();
    let evidence = VoidEvidence {
        shells: hole
            .shells()
            .map(|(s, _)| (s, VoidContainment::Probed(SolidContainment::In)))
            .collect(),
    };
    insert_void(&mut dst, solid, hole, &evidence, Tol::witness()).unwrap();
    assert_eq!(
        dst.shells().count(),
        2,
        "the container carries its void shell"
    );
    let part = common::brick::<f64>((1.2, 1.8), (1.2, 1.8), (1.2, 1.8));
    assembly(&dst, &part)
}

/// **The planar pocket**: a U-shaped block (the pocket `x ∈ [1, 2]`,
/// `y ∈ [1, 3]`, open on `+y`), with a part floating in the pocket.
fn pocket() -> Body<f64> {
    let u = common::prism_z::<f64>(
        &[
            (0.0, 0.0),
            (3.0, 0.0),
            (3.0, 3.0),
            (2.0, 3.0),
            (2.0, 1.0),
            (1.0, 1.0),
            (1.0, 3.0),
            (0.0, 3.0),
        ],
        0.0,
        1.0,
    );
    let part = common::brick::<f64>((1.2, 1.8), (1.5, 2.5), (0.2, 0.8));
    assembly(&u.body, &part)
}

/// **Every vertex on the boundary**: a slab `x ∈ [0, 2]` through a
/// 2 m cube — each of its eight vertices lies in one of the cube's
/// side faces, its interior lies in the cube's material, and no vertex
/// is strictly anywhere.
fn all_on_boundary() -> Body<f64> {
    let container = common::brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let part = common::brick::<f64>((0.0, 2.0), (0.5, 1.5), (0.5, 1.5));
    assembly(&container, &part)
}

/// The arm-2 findings among `errors`: every `CensusUndecidable` naming
/// two SOLIDS, and every `InstanceInterference`.
fn placement_findings(errors: &[ValidationError]) -> Vec<&ValidationError> {
    errors
        .iter()
        .filter(|e| {
            matches!(
                e,
                ValidationError::CensusUndecidable {
                    a: EntityId::Solid(_),
                    b: EntityId::Solid(_),
                    ..
                } | ValidationError::InstanceInterference { .. }
            )
        })
        .collect()
}

fn undecidable_what(e: &ValidationError) -> Option<&'static str> {
    match e {
        ValidationError::CensusUndecidable { what, .. } => Some(what),
        _ => None,
    }
}

/// **The red-first row.** Declared, the L-bracket certifies: the part
/// is inside the bracket's box and outside its material, and the
/// material test says so (at the base this drew `CensusUndecidable
/// { Solid, Solid, "one instance's extent box inside another's — the
/// interference class (recorded gate-skips do not exist yet)" }`).
#[test]
fn the_declared_l_bracket_certifies() {
    let (body, records) = lbracket(true, 0.0);
    assert_eq!(
        validate_pseudomanifold(&body, &records, Tol::witness()),
        Ok(())
    );
}

/// Undeclared, the L-bracket refuses on its eight touch findings — the
/// four resting corners and the four resting edges on the wall — and
/// on NOTHING about placement: a touch leaves the arm's invariant
/// intact, so the material test still runs and clears. (At the base:
/// nine errors, the ninth the containment refusal.)
#[test]
fn the_undeclared_l_bracket_carries_no_placement_finding() {
    let (body, _) = lbracket(false, 0.0);
    let errors = validate_pseudomanifold(&body, &ContactRecords::default(), Tol::witness())
        .expect_err("undeclared touches refuse");
    assert_eq!(errors.len(), 8, "{errors:?}");
    let corners = errors
        .iter()
        .filter(|e| {
            matches!(
                e,
                ValidationError::UndeclaredContact {
                    contact: CensusContact::VertexOnFace { .. },
                    ..
                }
            )
        })
        .count();
    let edges = errors
        .iter()
        .filter(|e| {
            matches!(
                e,
                ValidationError::UndeclaredContact {
                    contact: CensusContact::EdgeFaceOverlap { .. },
                    ..
                }
            )
        })
        .count();
    assert_eq!((corners, edges), (4, 4), "{errors:?}");
    assert!(placement_findings(&errors).is_empty(), "{errors:?}");
}

/// **The decided interference.** The embedded cube refuses as
/// `InstanceInterference`, its witness the first inner vertex in arena
/// order that is strictly inside — a top corner, the bottom four being
/// ON the boundary and skipped. No undecidable verdict rides beside
/// it, and no record is refuted (the loudness is the arm's own). At
/// the base this drew the in-band `CensusUndecidable`.
#[test]
fn the_embedded_cube_is_a_decided_interference() {
    let (body, records) = embedded();
    let errors = validate_pseudomanifold(&body, &records, Tol::witness())
        .expect_err("an instance inside another's material never clears");
    let (outer, inner) = two_solids(&body);
    let [finding] = &errors[..] else {
        panic!("exactly the one decided finding: {errors:?}");
    };
    let ValidationError::InstanceInterference {
        outer: o,
        inner: i,
        witness,
    } = finding
    else {
        panic!("the decided interference: {finding:?}");
    };
    assert_eq!((*o, *i), (outer, inner));
    let w = point_of(&body, *witness);
    assert_eq!(
        (w.x, w.y, w.z),
        (1.0, 1.0, 1.0),
        "the first strictly-inside corner"
    );
    // The Display is the kernel's own sentence, not a struct dump.
    let text = finding.to_string();
    assert!(text.contains("interference fit"), "{text}");
    assert!(text.contains("material contains vertex"), "{text}");
    assert!(text.contains("recorded gate-skips do not exist"), "{text}");
    assert!(!text.contains("InstanceInterference {"), "{text}");
    assert!(!text.contains("witness:"), "{text}");
}

/// The embedded cube UNDECLARED decides the same way: no record is
/// consulted in either direction.
#[test]
fn the_embedded_cube_decides_the_same_undeclared() {
    let (body, _) = embedded();
    let errors = validate_pseudomanifold(&body, &ContactRecords::default(), Tol::witness())
        .expect_err("refuses undeclared too");
    let placements = placement_findings(&errors);
    assert_eq!(placements.len(), 1, "{errors:?}");
    assert!(
        matches!(placements[0], ValidationError::InstanceInterference { .. }),
        "{errors:?}"
    );
}

/// The cavity assembly clears: the part's first vertex is in the void
/// — the closest hit from it is the void shell's face, whose material
/// side faces away — so it is strictly outside the container's
/// material. (At the base: "one instance's extent box inside
/// another's".)
#[test]
fn a_part_in_a_cavity_clears() {
    assert_eq!(
        validate_pseudomanifold(&cavity(), &ContactRecords::default(), Tol::witness()),
        Ok(())
    );
}

/// The planar pocket clears. (At the base: "one instance's extent box
/// inside another's".)
#[test]
fn a_part_in_a_planar_pocket_clears() {
    assert_eq!(
        validate_pseudomanifold(&pocket(), &ContactRecords::default(), Tol::witness()),
        Ok(())
    );
}

/// An instance whose every vertex lies on the container's boundary is
/// not a placement a witness decides: the typed refusal, beside the
/// sixteen touch findings, and never an interference verdict (the slab
/// IS inside the cube's material, and the arm says it cannot tell —
/// which is true of what it reads). At the base: the in-band
/// `CensusUndecidable`.
#[test]
fn every_vertex_on_the_boundary_refuses_typed() {
    let body = all_on_boundary();
    let errors = validate_pseudomanifold(&body, &ContactRecords::default(), Tol::witness())
        .expect_err("refuses");
    let placements = placement_findings(&errors);
    assert_eq!(placements.len(), 1, "{errors:?}");
    let what = undecidable_what(placements[0]).expect("the typed refusal");
    assert!(
        what.contains("every vertex of the contained instance lies on the containing"),
        "{what}"
    );
    assert_eq!(errors.len(), 17, "{errors:?}");
}

/// **The witness at the band edge**: the part shifted off the wall by
/// a distance inside the run's ambiguity band — within ε of the
/// container's boundary, not on it. The vertex-face and edge-face
/// sweeps escalate on the same residual first, an escalation names no
/// entity, and the arm refuses the material test to a pair whose
/// boundaries are not certified crossing-free — the typed in-band
/// refusal, with no clear and no interference. `delta` is taken from
/// the run's band, so the row is the same statement at every
/// `CAD_TOLERANCE_EPS` row of the matrix (default, 1e-6, 1e-12).
#[test]
fn a_witness_at_the_band_edge_refuses_typed_at_this_eps() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let delta = (band.zero() * band.escalate()).sqrt();
    assert!(band.zero() < delta && delta < band.escalate());
    let (body, _) = lbracket(false, delta);
    let errors = validate_pseudomanifold(&body, &ContactRecords::default(), tol)
        .expect_err("in band refuses");
    assert!(
        errors
            .iter()
            .any(|e| matches!(e, ValidationError::CensusEscalated { .. })),
        "the sweeps escalate on the in-band residual: {errors:?}"
    );
    let placements = placement_findings(&errors);
    assert_eq!(placements.len(), 1, "{errors:?}");
    let what = undecidable_what(placements[0]).expect("the typed refusal");
    assert!(what.contains("not certified crossing-free"), "{what}");
    // And just past the band the part floats in the concavity and
    // clears — the refusal above is the band's, not the placement's.
    let (body, _) = lbracket(false, 10.0 * band.escalate());
    assert_eq!(
        validate_pseudomanifold(&body, &ContactRecords::default(), tol),
        Ok(())
    );
}

/// **The per-solid door itself**, on the L-bracket arena: the bracket's
/// material answers `In`, the concavity answers `Out` (the ray from it
/// may meet no face of the bracket at all — the at-infinity side is
/// read off the BRACKET's volume, not the arena's), the wall answers
/// `OnBoundary`; and the same points asked of the PART's solid answer
/// for the part alone.
#[test]
fn the_per_solid_door_answers_for_one_solid_of_the_arena() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let (body, _) = lbracket(false, 0.0);
    let (bracket, part) = two_solids(&body);
    let ask = |solid, p: (f64, f64, f64)| {
        point_in_solid_of(&body, solid, Point3::new(p.0, p.1, p.2), band, tol).unwrap()
    };
    // In the bracket's material (the short arm), and in its concavity.
    assert_eq!(ask(bracket, (2.0, 0.5, 0.5)), SolidContainment::In);
    assert_eq!(ask(bracket, (2.0, 2.0, 0.5)), SolidContainment::Out);
    assert_eq!(ask(bracket, (1.5, 1.6, 0.5)), SolidContainment::Out);
    // On the wall, inside the part's footprint and outside it.
    assert_eq!(ask(bracket, (1.0, 1.6, 0.5)), SolidContainment::OnBoundary);
    assert_eq!(ask(bracket, (1.0, 2.5, 0.5)), SolidContainment::OnBoundary);
    // The part's solid: its own material, the bracket's material, the
    // shared wall.
    assert_eq!(ask(part, (1.5, 1.6, 0.5)), SolidContainment::In);
    assert_eq!(ask(part, (2.0, 0.5, 0.5)), SolidContainment::Out);
    assert_eq!(ask(part, (1.0, 1.6, 0.5)), SolidContainment::OnBoundary);
    // A key the arena does not hold is an arena claim, typed.
    assert!(matches!(
        point_in_solid_of(
            &body,
            SolidKey::default(),
            Point3::new(0.0, 0.0, 0.0),
            band,
            tol
        ),
        Err(topo::PointInSolidError::NoSuchSolid { .. })
    ));
}

/// The L-bracket's far outer wall (`x = 0`, two metres from the part,
/// so arm 1's reach test clears every pair it is in) re-described as a
/// NURBS net lying exactly in that plane.
fn nurbs_wall(y: (f64, f64), z: (f64, f64)) -> geom::Surface<f64> {
    let k = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let ys = [y.0 - 1.0, 0.5 * (y.0 + y.1), y.1 + 1.0];
    let zs = [z.0 - 1.0, 0.5 * (z.0 + z.1), z.1 + 1.0];
    let (mut control, mut weights) = (Vec::new(), Vec::new());
    for &yy in &ys {
        for &zz in &zs {
            control.push(Point3::new(0.0, yy, zz));
            weights.push(1.0);
        }
    }
    let n = geom::NurbsSurface::new(k.clone(), k, control, weights).unwrap();
    assert!(!n.is_placeholder());
    geom::Surface::Nurbs(std::sync::Arc::new(n))
}

/// The L-bracket with its far wall on the NURBS net, the wall's four
/// edges re-described on the NURBS lane (the M7-8 recipe
/// `m4_pr2_transform.rs` builds): the one way a described spline face
/// reaches the public door at all.
fn nurbs_walled_bracket() -> Body<f64> {
    let l = common::prism_z::<f64>(&L_PROFILE, 0.0, 1.0);
    let far_wall: FaceKey = l.side_faces[5];
    let mut body = l.body;
    let wall = body
        .set_face_surface(
            far_wall,
            topo::FaceSurface::New(nurbs_wall((0.0, 3.0), (0.0, 1.0))),
        )
        .unwrap();
    let face_surface_of_he = |body: &Body<f64>, he: topo::HalfEdgeKey| {
        let he_data = body.get_half_edge(he).unwrap();
        let loop_data = body.get_loop(he_data.parent_loop).unwrap();
        body.get_face(loop_data.face).unwrap().surface
    };
    let edges: Vec<_> = body.edges().map(|(k, e)| (k, e.clone())).collect();
    let mut lane_edges = 0;
    for (edge_key, edge) in edges {
        let s1 = face_surface_of_he(&body, edge.he_plus);
        let s2 = face_surface_of_he(&body, edge.he_minus);
        if s1 != wall && s2 != wall {
            continue;
        }
        let start = body.get_half_edge(edge.he_plus).unwrap().start;
        let end = body.half_edge_end(edge.he_plus).unwrap();
        let p0 = point_of(&body, start);
        let p1 = point_of(&body, end);
        let kv = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let carrier = geom::Curve3::Nurbs(std::sync::Arc::new(
            geom::NurbsCurve3::new(kv, vec![p0, p1], vec![1.0, 1.0]).unwrap(),
        ));
        body.set_edge_curve_nurbs_lane(
            edge_key,
            geom_brep::EdgeCurveSpec {
                description: geom_brep::EdgeDescriptionSpec::Intersection {
                    s1,
                    s2,
                    witness: p0.lerp(p1, 0.5),
                },
                carrier,
                param_start: 0.0,
                param_end: 1.0,
            },
            Tol::witness(),
        )
        .unwrap();
        lane_edges += 1;
    }
    assert_eq!(lane_edges, 4, "the far wall has four M7-8 edges");
    topo::mint_pcurves(&mut body, Tol::witness()).unwrap();
    body
}

/// **A container carrying a face kind the material test does not
/// serve — measured at the public door.** The spline-walled bracket
/// is refused at tier 3 (check 7 cannot integrate the spline face:
/// `VolumeUncomputable`), so no body carrying a described spline face
/// reaches the census through `validate_pseudomanifold` today, and the
/// arm's own typed cause for the kind is pinned one door down, through
/// `census_and_certify`, in `census.rs`'s
/// `an_unserved_face_kind_on_the_container_refuses_naming_the_cause`.
/// This row pins the public-door fact so that the day check 7 admits
/// such a face, the census row is what a reader is sent to.
#[test]
fn a_spline_walled_container_is_refused_at_tier_3_before_the_census() {
    let container = nurbs_walled_bracket();
    assert_eq!(topo::validate_closed(&container), Ok(()));
    let part = common::brick::<f64>((1.2, 1.8), (1.5, 2.5), (0.2, 0.8));
    let body = assembly(&container, &part);
    let errors = validate_pseudomanifold(&body, &ContactRecords::default(), Tol::witness())
        .expect_err("tier 3 refuses the spline face's quadrature");
    assert!(
        errors
            .iter()
            .all(|e| matches!(e, ValidationError::VolumeUncomputable { .. })),
        "nothing from the census, only check 7: {errors:?}"
    );
    assert!(placement_findings(&errors).is_empty(), "{errors:?}");
}

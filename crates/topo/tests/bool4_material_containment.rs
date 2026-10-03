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
    topo::graft_disjoint(&mut out, b).unwrap();
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
    let l = common::prism_z::<f64>(&L_PROFILE, 0.0, 1.0, Tol::witness());
    // The side face over profile segment 3, (1, 1) → (1, 3): the wall.
    let wall: FaceKey = l.side_faces[3];
    let part = common::brick::<f64>((1.0 + dx, 2.0 + dx), (1.2, 2.0), (0.2, 0.8), Tol::witness());
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
    common::mapped_cube(
        |x, y, z| Point3::new(side * x + dx, side * y + dy, side * z + dz),
        Tol::witness(),
    )
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
    let mut dst = common::brick::<f64>((0.0, 3.0), (0.0, 3.0), (0.0, 3.0), Tol::witness());
    let hole = common::brick::<f64>((1.0, 2.0), (1.0, 2.0), (1.0, 2.0), Tol::witness());
    let (solid, _) = dst.solids().next().unwrap();
    let evidence = VoidEvidence {
        shells: hole
            .shells()
            .map(|(s, _)| (s, VoidContainment::Probed(SolidContainment::In)))
            .collect(),
    };
    insert_void(&mut dst, solid, hole, &evidence).unwrap();
    assert_eq!(
        dst.shells().count(),
        2,
        "the container carries its void shell"
    );
    let part = common::brick::<f64>((1.2, 1.8), (1.2, 1.8), (1.2, 1.8), Tol::witness());
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
        Tol::witness(),
    );
    let part = common::brick::<f64>((1.2, 1.8), (1.5, 2.5), (0.2, 0.8), Tol::witness());
    assembly(&u.body, &part)
}

/// **Every vertex on the boundary**: a slab `x ∈ [0, 2]` through a
/// 2 m cube — each of its eight vertices lies in one of the cube's
/// side faces, its interior lies in the cube's material, and no vertex
/// is strictly anywhere.
fn all_on_boundary() -> Body<f64> {
    let container = common::brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0), Tol::witness());
    let part = common::brick::<f64>((0.0, 2.0), (0.5, 1.5), (0.5, 1.5), Tol::witness());
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

/// Declared, the L-bracket certifies: the part is inside the bracket's
/// box and outside its material, every vertex of each instance is
/// outside-or-on the other, and the four declared rests are one-sided.
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
/// on NOTHING about placement: each touch is a rest by the local cone
/// analysis, so the clear stands beside them.
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
/// ON the boundary. No undecidable verdict rides beside it, and no
/// record is refuted (the loudness is the arm's own).
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
    assert!(
        text.contains("two instances overlap: a corner of one lies inside the other"),
        "{text}"
    );
    assert!(
        text.ends_with("Recourse: move the instances apart"),
        "{text}"
    );
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

/// The cavity assembly clears: every vertex of the part is in the void
/// — the closest hit from each is the void shell's face, whose material
/// side faces away — so strictly outside the container's material, and
/// every vertex of the container is outside the part; no touch stands.
#[test]
fn a_part_in_a_cavity_clears() {
    assert_eq!(
        validate_pseudomanifold(&cavity(), &ContactRecords::default(), Tol::witness()),
        Ok(())
    );
}

/// The planar pocket clears on the same conditions.
#[test]
fn a_part_in_a_planar_pocket_clears() {
    assert_eq!(
        validate_pseudomanifold(&pocket(), &ContactRecords::default(), Tol::witness()),
        Ok(())
    );
}

/// An instance whose every vertex lies on the container's boundary is
/// not a placement its vertices decide, and never an interference
/// verdict. The slab IS inside the cube's material, and the touch
/// analysis says so: with the probe undecided, the findings are read,
/// and their reason — a touch whose cones overlap — is the one
/// refusal, beside the sixteen touch findings.
#[test]
fn every_vertex_on_the_boundary_refuses_typed() {
    let body = all_on_boundary();
    let errors = validate_pseudomanifold(&body, &ContactRecords::default(), Tol::witness())
        .expect_err("refuses");
    let placements = placement_findings(&errors);
    assert_eq!(placements.len(), 1, "{errors:?}");
    let what = undecidable_what(placements[0]).expect("the typed refusal");
    assert!(
        what.contains("one passes into the other where they touch"),
        "{what}"
    );
    assert_eq!(errors.len(), 17, "{errors:?}");
}

/// **The witness at the band edge**: the part shifted off the wall by
/// a distance inside the run's ambiguity band — within ε of the
/// container's boundary, not on it. The vertex-face and edge-face
/// sweeps escalate on that residual, and the material test's own
/// boundary pre-pass escalates on it in BOTH orderings — the part's
/// vertices against the wall's plane, and the wall's vertices against
/// the part's near face's plane (the pre-pass decides a plane residual
/// before it asks the region) — so neither ordering decides, and with
/// the probe undecided the findings are read: the sweeps' escalations
/// stand, so the one refusal is the pair's boundaries left unchecked,
/// with no clear and no interference. `delta`
/// is taken from the run's band, so the row is the same statement at
/// every `CAD_TOLERANCE_EPS` row of the matrix (default, 1e-6, 1e-12).
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
    assert!(
        what.contains("another finding left their boundaries unchecked"),
        "{what}"
    );
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

/// **A hexagonal prism straddling the wall with its waist AT the
/// wall.** The part spans `x ∈ [0, 2]` through the bracket's tall arm,
/// `z ∈ [0.25, 0.75]`, its profile a hexagon whose two waist corners
/// sit at `x = 1` (`y = 1.4` and `y = 2.6`, inside the wall's region):
/// its four vertices at `x = 1` lie in the wall and its two vertical
/// edges there lie in the wall, its four vertices at `x = 0` lie in the
/// bracket's outer face, and its vertices at `x = 2` float in the
/// concavity. No vertex of either instance is strictly inside the
/// other, and nothing pierces: the wall is crossed AT the part's
/// vertices and edges. The materials overlap over `x ∈ [0, 1]`.
fn split_straddle() -> (Body<f64>, ContactRecords) {
    let l = common::prism_z::<f64>(&L_PROFILE, 0.0, 1.0, Tol::witness());
    let part = common::prism_z::<f64>(
        &[
            (0.0, 1.5),
            (1.0, 1.4),
            (2.0, 1.5),
            (2.0, 2.5),
            (1.0, 2.6),
            (0.0, 2.5),
        ],
        0.25,
        0.75,
        Tol::witness(),
    );
    let body = assembly(&l.body, &part.body);
    // Every touch the sweeps report, declared as the v-on-f records it
    // is: all true (each vertex IS in that face's region).
    let errors = validate_pseudomanifold(&body, &ContactRecords::default(), Tol::witness())
        .expect_err("undeclared touches refuse");
    let mut records = ContactRecords::default();
    for e in &errors {
        if let ValidationError::UndeclaredContact {
            contact: CensusContact::VertexOnFace { vertex, face },
            ..
        } = e
        {
            records.b_on_a.push(VfContact {
                vertex: *vertex,
                face: *face,
            });
        }
    }
    assert_eq!(records.b_on_a.len(), 8, "{errors:?}");
    (body, records)
}

/// **A mixed-side touch is a crossing at a lower-dimensional feature,
/// and it BLOCKS.** At each vertex on the wall the part's edges leave
/// the wall's plane to `x < 1` (the bracket's material) and to `x > 1`
/// (the concavity); the vertical edges in the wall have their two
/// adjacent faces on both sides. Undeclared, the sweeps report the
/// touches and the cone analysis refuses the clear typed; declared,
/// the same analysis runs over the records and refuses the same way —
/// a record certifies a coincidence, never a side. No interference is
/// claimed (no vertex is inside) and nothing clears.
#[test]
fn a_mixed_side_touch_blocks_the_clear_declared_or_not() {
    let (body, records) = split_straddle();
    for (name, recs) in [
        ("undeclared", ContactRecords::default()),
        ("declared", records),
    ] {
        let Err(errors) = validate_pseudomanifold(&body, &recs, Tol::witness()) else {
            panic!("{name}: the straddle must not certify");
        };
        let placements = placement_findings(&errors);
        assert_eq!(placements.len(), 1, "{name}: {errors:?}");
        let what = undecidable_what(placements[0]).expect("the typed refusal");
        assert!(
            what.contains("one passes into the other where they touch"),
            "{name}: {what}"
        );
        assert!(
            !errors
                .iter()
                .any(|e| matches!(e, ValidationError::InstanceInterference { .. })),
            "{name}: {errors:?}"
        );
    }
}

//! Review probes for the census's material-containment arm (issue 750).
//!
//! Each row MEASURES a verdict the arm's invariant argument makes a
//! claim about, and pins what the tree actually does at the reviewed
//! head so the record is executable rather than reasoned. The doc on
//! each row says what the measurement means.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;
use geom_core::{Band, Point3, Tol};
use topo::{
    Body, CensusContact, ContactRecords, EntityId, SolidContainment, SolidKey, ValidationError,
    VfContact, VoidContainment, VoidEvidence, insert_void, point_in_solid, point_in_solid_of,
    validate_pseudomanifold,
};

const L_PROFILE: [(f64, f64); 6] = [
    (0.0, 0.0),
    (3.0, 0.0),
    (3.0, 1.0),
    (1.0, 1.0),
    (1.0, 3.0),
    (0.0, 3.0),
];

fn assembly(a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    let mut out = a.clone();
    topo::graft_disjoint(&mut out, b, Tol::witness()).unwrap();
    out
}

fn solids(body: &Body<f64>) -> Vec<SolidKey> {
    body.solids().map(|(k, _)| k).collect()
}

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

fn crossings(errors: &[ValidationError]) -> Vec<&ValidationError> {
    errors
        .iter()
        .filter(|e| {
            matches!(
                e,
                ValidationError::UndeclaredContact {
                    contact: CensusContact::EdgeFacePierce { .. }
                        | CensusContact::EdgeEdgeCross { .. },
                    ..
                }
            )
        })
        .collect()
}

fn point_of(body: &Body<f64>, v: topo::VertexKey) -> Point3<f64> {
    *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
}

/// **A part straddling the L-bracket's inner wall with every crossing
/// edge lying IN the other body's faces.** The part `x ∈ [0.5, 1.5]`,
/// `y ∈ [1, 3]`, `z ∈ [0, 1]` shares the bracket's full `y`- and
/// `z`-extents on that side, so the bracket's wall `x = 1` passes
/// through the part's interior with all four of its edges in the
/// part's faces, and the part's face `x = 0.5` lies inside the
/// bracket's material with its edges in the bracket's faces. The two
/// materials overlap in `[0.5, 1] × [1, 3] × [0, 1]` — a genuine
/// interference — yet the exact sweeps see ONLY touches (no
/// `EdgeFacePierce`, no `EdgeEdgeCross`), because every point of the
/// crossing curve lies on an edge of one body inside a face of the
/// other.
///
/// The arm's precondition reads touches as "invariant intact", the
/// gate cannot separate the pair (y and z flush), and the part's third
/// vertex in arena order, `(1.5, 3, 0)`, is strictly outside the
/// bracket — so the material test CLEARS the pair. The reverse ordering
/// separates on the box. This row pins what that measures.
#[test]
fn a_straddling_part_with_touch_only_crossings_is_cleared_by_the_material_test() {
    let l = common::prism_z::<f64>(&L_PROFILE, 0.0, 1.0);
    let part = common::brick::<f64>((0.5, 1.5), (1.0, 3.0), (0.0, 1.0));
    let body = assembly(&l.body, &part);
    let errors = validate_pseudomanifold(&body, &ContactRecords::default(), Tol::witness())
        .expect_err("the undeclared touches refuse");
    for e in &errors {
        println!("straddle: {e:?}");
    }
    assert!(
        crossings(&errors).is_empty(),
        "no pierce and no edge-edge cross among the findings: {errors:?}"
    );
    let placements = placement_findings(&errors);
    println!("straddle placement findings: {placements:?}");
    // The materials overlap, and the arm's own per-solid door says so
    // of an interior point of the overlap.
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let [bracket, part_solid] = solids(&body)[..] else {
        panic!()
    };
    let q = Point3::new(0.75, 2.0, 0.5);
    assert_eq!(
        point_in_solid_of(&body, bracket, q, band, tol).unwrap(),
        SolidContainment::In
    );
    assert_eq!(
        point_in_solid_of(&body, part_solid, q, band, tol).unwrap(),
        SolidContainment::In
    );
    // MEASURED: the placement is cleared — no InstanceInterference and
    // no CensusUndecidable naming the solid pair.
    assert!(
        placements.is_empty(),
        "measured at the head: the arm pushes nothing on this pair; got {placements:?}"
    );
}

/// The same straddle, with every touch the sweeps report declared as
/// far as the record vocabulary reaches (v-on-f both ways, v-v), so
/// the confirm pass has nothing to refute: what remains undeclared is
/// the door's whole verdict. Prints the residue so a reader can see
/// whether a fully-declared straddle would certify.
#[test]
fn the_straddle_declared_as_far_as_the_vocabulary_reaches() {
    let l = common::prism_z::<f64>(&L_PROFILE, 0.0, 1.0);
    let part = common::brick::<f64>((0.5, 1.5), (1.0, 3.0), (0.0, 1.0));
    let body = assembly(&l.body, &part);
    let errors = validate_pseudomanifold(&body, &ContactRecords::default(), Tol::witness())
        .expect_err("the undeclared touches refuse");
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
    let residue = validate_pseudomanifold(&body, &records, Tol::witness());
    println!(
        "straddle declared ({} v-on-f records): {residue:?}",
        records.b_on_a.len()
    );
    let kinds: Vec<String> = match &residue {
        Ok(()) => vec![],
        Err(errs) => errs
            .iter()
            .map(|e| match e {
                ValidationError::UndeclaredContact { contact, .. } => {
                    format!("Undeclared:{}", contact_kind(contact))
                }
                other => format!("{other:?}").chars().take(40).collect(),
            })
            .collect(),
    };
    println!("straddle declared residue kinds: {kinds:?}");
}

fn contact_kind(c: &CensusContact) -> &'static str {
    match c {
        CensusContact::VertexVertex { .. } => "VertexVertex",
        CensusContact::VertexOnFace { .. } => "VertexOnFace",
        CensusContact::VertexOnEdge { .. } => "VertexOnEdge",
        CensusContact::EdgeEdgeOverlap { .. } => "EdgeEdgeOverlap",
        CensusContact::EdgeFaceOverlap { .. } => "EdgeFaceOverlap",
        CensusContact::EdgeFacePierce { .. } => "EdgeFacePierce",
        CensusContact::EdgeEdgeCross { .. } => "EdgeEdgeCross",
        CensusContact::ConformalPatch { .. } => "ConformalPatch",
    }
}

/// **Two cubes overlapping by half, sharing y and z extents.** The same
/// touch-only crossing shape as the straddle, but here the box GATE
/// separates both orderings (neither hull sits inside the other's
/// reach), so the material test never runs: the pair is cleared at the
/// gate, as it was at the base. Pinned to show the straddle's clear is
/// the material test's, not the gate's.
#[test]
fn two_half_overlapping_cubes_are_cleared_at_the_gate() {
    let a = common::brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let b = common::brick::<f64>((1.0, 3.0), (0.0, 2.0), (0.0, 2.0));
    let body = assembly(&a, &b);
    let errors = validate_pseudomanifold(&body, &ContactRecords::default(), Tol::witness())
        .expect_err("the undeclared touches refuse");
    assert!(crossings(&errors).is_empty(), "{errors:?}");
    let placements = placement_findings(&errors);
    println!("half-overlap placement findings: {placements:?}");
    assert!(placements.is_empty(), "{placements:?}");
}

/// **A three-solid arena**: the L-bracket, a part floating in its
/// concavity (which clears as a pair), and a third box piercing the
/// bracket's far arm. The pierce names the bracket, so the
/// bracket × part pair is refused on the precondition — conservative,
/// and measured here so the cost is on record.
#[test]
fn a_pierce_between_a_and_c_blocks_the_a_b_material_test() {
    let l = common::prism_z::<f64>(&L_PROFILE, 0.0, 1.0);
    let part = common::brick::<f64>((1.2, 1.8), (1.5, 2.5), (0.2, 0.8));
    let piercer = common::brick::<f64>((2.3, 2.7), (-0.5, 0.5), (0.3, 0.7));
    let mut body = assembly(&l.body, &part);
    topo::graft_disjoint(&mut body, &piercer, Tol::witness()).unwrap();
    let errors = validate_pseudomanifold(&body, &ContactRecords::default(), Tol::witness())
        .expect_err("the pierce refuses");
    assert!(!crossings(&errors).is_empty(), "{errors:?}");
    let placements = placement_findings(&errors);
    println!("three-solid placement findings: {placements:?}");
    let [bracket, part_solid, _] = solids(&body)[..] else {
        panic!()
    };
    assert!(
        placements.iter().any(|e| matches!(
            e,
            ValidationError::CensusUndecidable {
                a: EntityId::Solid(o),
                b: EntityId::Solid(i),
                what,
            } if *o == bracket && *i == part_solid && what.contains("not certified crossing-free")
        )),
        "{placements:?}"
    );
    // And the pair without the third solid clears — the refusal above
    // is the precondition's, not the placement's.
    assert_eq!(
        validate_pseudomanifold(
            &assembly(&l.body, &part),
            &ContactRecords::default(),
            Tol::witness()
        ),
        Ok(())
    );
}

/// **A hollow part (two shells) floating in the concavity**: the part's
/// void-shell vertices are probed like its outer ones, all answer
/// `Out` of the bracket, and the pair clears.
#[test]
fn a_hollow_part_in_the_concavity_clears() {
    let l = common::prism_z::<f64>(&L_PROFILE, 0.0, 1.0);
    let mut part = common::brick::<f64>((1.2, 1.8), (1.5, 2.5), (0.2, 0.8));
    let hole = common::brick::<f64>((1.4, 1.6), (1.8, 2.2), (0.4, 0.6));
    let (solid, _) = part.solids().next().unwrap();
    let evidence = VoidEvidence {
        shells: hole
            .shells()
            .map(|(s, _)| (s, VoidContainment::Probed(SolidContainment::In)))
            .collect(),
    };
    insert_void(&mut part, solid, hole, &evidence, Tol::witness()).unwrap();
    assert_eq!(part.shells().count(), 2);
    let body = assembly(&l.body, &part);
    assert_eq!(
        validate_pseudomanifold(&body, &ContactRecords::default(), Tol::witness()),
        Ok(())
    );
}

/// **The at-infinity side is the SELECTION's sign.** An arena holding a
/// reverted unit cube (a complement: its material is everything
/// outside it) and an ordinary far-away cube whose volume dominates
/// the total. A point far from both, whose schedule rays meet no
/// face: the per-solid door on the complement reads the complement's
/// own negative volume and answers `In`; the whole-body door reads
/// the positive total and answers `Out`. (If every ray hits something
/// the row says so rather than deciding.)
#[test]
fn the_per_solid_door_reads_the_container_s_own_sign_at_infinity() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let complement = common::brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0))
        .revert()
        .unwrap();
    let far = common::brick::<f64>((50.0, 53.0), (50.0, 53.0), (50.0, 53.0));
    let body = assembly(&complement, &far);
    let [comp, _] = solids(&body)[..] else {
        panic!()
    };
    let whole_props = topo::mass_properties(&body, tol).unwrap();
    println!("total volume {}", whole_props.volume);
    assert!(whole_props.volume > 0.0);
    let q = Point3::new(-7.0, 11.0, 23.0);
    let per_solid = point_in_solid_of(&body, comp, q, band, tol);
    let whole = point_in_solid(&body, q, band, tol);
    println!("complement at infinity: per-solid {per_solid:?}, whole-body {whole:?}");
    assert_eq!(per_solid.unwrap(), SolidContainment::In);
    assert_eq!(whole.unwrap(), SolidContainment::Out);
}

/// The embedded cube's witness in arena order, for the record: the
/// four bottom corners are on the boundary and the first top corner
/// decides.
#[test]
fn embedded_witness_is_the_fifth_vertex_in_arena_order() {
    let big = common::brick::<f64>((0.0, 4.0), (0.0, 4.0), (0.0, 4.0));
    let small = common::brick::<f64>((1.0, 2.0), (1.0, 2.0), (0.0, 1.0));
    let body = assembly(&big, &small);
    let errors = validate_pseudomanifold(&body, &ContactRecords::default(), Tol::witness())
        .expect_err("embedded refuses");
    let witness = errors.iter().find_map(|e| match e {
        ValidationError::InstanceInterference { witness, .. } => Some(*witness),
        _ => None,
    });
    let w = witness.expect("decided");
    let p = point_of(&body, w);
    println!("embedded witness {w:?} at {p:?}");
    assert_eq!((p.x, p.y, p.z), (1.0, 1.0, 1.0));
}

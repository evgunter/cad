//! The mated cross-lap union — the REST-contact frontier, CLOSED
//! (#91 C1 → #102 R7 → M5 S1).
//!
//! History: the original `demo_tripwires.rs` crosslap wire expected
//! M4 PR 5's Declare to glue the mate. PR 5 opened the CLASSIFICATION
//! half exactly as predicted — but the mate is a pure REST contact
//! (the half-depth notches interlock exactly; the two interiors are
//! DISJOINT), so the union then refused typed at the JOIN — the M3
//! envelope's boundary-on-boundary class (iii). The mechanism (S1's
//! diagnosis, `boolean::rest` module docs): at a REST site a seam
//! direction lies in FOUR coincident planes (two per solid, coplanar
//! via the declared rung), the two end records of one segment can
//! resolve that ambiguity onto DIFFERENT face pairs, and the join's
//! germ-identity match (face pairs agree) then never fires — the
//! chords existed; their identity keys disagreed. The re-armed wire
//! (this file's previous life) sat on that join-stage frontier until
//! M5 S1 landed the declared-REST union zip; the wire FIRED with the
//! exact expected volume and was retired per its own instructions.
//! What remains are the certified pins at BOTH doors:
//!
//! - UNDECLARED, the mate still refuses at the coincidence door
//!   (rung (b) — value equality never classifies; the ladder is law).
//! - DECLARED, the mate BUILDS: exact dyadic volume
//!   2·(BEAM_VOL − NOTCH_VOL), gate-ladder green (tiers + 3′ with the
//!   rest records consumed into structure), watertight STL and STEP
//!   rows, and bit-identical on re-run (naming-key stability).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::{flush_declarations, prism_z};
use geom_core::Tol;
use topo::{
    BooleanError, BooleanResult, mass_properties, subtract, union, union_with, validate_geometric,
    validate_pseudomanifold,
};

const NOTCH_VOL: f64 = 0.5 * 0.5 * 0.25;
const BEAM_VOL: f64 = 4.0 * 0.5 * 0.5;

fn notched_beams() -> (topo::Body<f64>, topo::Body<f64>) {
    let beam_a = prism_z::<f64>(
        &[(0.0, 1.75), (4.0, 1.75), (4.0, 2.25), (0.0, 2.25)],
        0.0,
        0.5,
    );
    let cut_a = prism_z::<f64>(
        &[(1.75, 1.5), (2.25, 1.5), (2.25, 2.5), (1.75, 2.5)],
        0.25,
        0.75,
    );
    let BooleanResult::Body(a) =
        subtract(&beam_a.body, &cut_a.body, Tol::witness()).expect("notch A")
    else {
        panic!("notch A yields a body");
    };
    let beam_b = prism_z::<f64>(
        &[(1.75, 0.0), (2.25, 0.0), (2.25, 4.0), (1.75, 4.0)],
        0.0,
        0.5,
    );
    let cut_b = prism_z::<f64>(
        &[(1.5, 1.75), (2.5, 1.75), (2.5, 2.25), (1.5, 2.25)],
        -0.25,
        0.25,
    );
    let BooleanResult::Body(b) =
        subtract(&beam_b.body, &cut_b.body, Tol::witness()).expect("notch B")
    else {
        panic!("notch B yields a body");
    };
    for (label, notched) in [("A", &a), ("B", &b)] {
        assert_eq!(
            mass_properties(&notched.body, Tol::witness())
                .unwrap()
                .volume,
            BEAM_VOL - NOTCH_VOL,
            "notched beam {label}: exact dyadic volume"
        );
    }
    (a.body, b.body)
}

/// The glued union (the declared door), shared by the pins below.
fn glued() -> topo::BooleanBody<f64> {
    let (a, b) = notched_beams();
    match union_with(&a, &b, &flush_declarations(&a, &b), Tol::witness())
        .expect("declared mate unions")
    {
        BooleanResult::Body(body) => body,
        BooleanResult::Empty => panic!("mated union cannot be empty"),
    }
}

/// The narrowing pin, unchanged: UNDECLARED, the mate refuses at the
/// coincidence door (rung (b) — post-PR 5, value equality never
/// classifies). The M5 S1 lane is reached exclusively through the
/// declared rung; this door must never widen.
#[test]
fn undeclared_crosslap_refuses_at_the_coincidence_door() {
    let (a, b) = notched_beams();
    match union(&a, &b, Tol::witness()) {
        Err(BooleanError::UndeclaredCoincidence { .. }) => {}
        other => panic!("expected UndeclaredCoincidence, got {other:?}"),
    }
}

/// The certified pass at the declared door (the retired wire's
/// promise): the DECLARED mated cross-lap BUILDS — one seamed body,
/// exact dyadic volume, gate-ladder green, every rest record consumed
/// into seam structure (the census's consumed class: 3′ ≡ tier 3).
#[test]
fn declared_crosslap_rest_union_builds() {
    let glued = glued();
    assert_eq!(glued.kind, topo::BooleanResultKind::Seamed);
    assert_eq!(
        mass_properties(&glued.body, Tol::witness()).unwrap().volume,
        2.0 * (BEAM_VOL - NOTCH_VOL),
        "exact dyadic volume additivity (disjoint interiors)"
    );
    assert_eq!(
        validate_geometric(&glued.body, Tol::witness()),
        Ok(()),
        "tier 3"
    );
    assert_eq!(
        validate_pseudomanifold(&glued.body, &glued.contacts, Tol::witness()),
        Ok(()),
        "3′ with the surviving records"
    );
    assert!(
        glued.contacts.vv.is_empty()
            && glued.contacts.a_on_b.is_empty()
            && glued.contacts.b_on_a.is_empty(),
        "every REST record is consumed into seam structure: {:?}",
        glued.contacts
    );
    assert!(
        !glued.naming.seam_edges.is_empty(),
        "the zip mints real seam edges"
    );
}

/// Watertight export rows: the glued union tessellates to a checked
/// (watertight, outward-oriented) mesh whose signed volume converges
/// on the exact one, writes as binary STL, and exports STEP.
#[test]
fn declared_crosslap_union_exports_watertight() {
    let glued = glued();
    let mesh = mesh::tessellate(&glued.body, 1e-2, Tol::witness()).expect("tessellate");
    mesh::validate::check_mesh(&mesh).expect("watertight, consistently oriented");
    let v_mesh = mesh::validate::signed_volume(&mesh);
    let exact = 2.0 * (BEAM_VOL - NOTCH_VOL);
    assert!(
        ((v_mesh - exact) / exact).abs() < 1e-9,
        "planar mesh volume matches the exact one: {v_mesh} vs {exact}"
    );
    let mut stl = Vec::new();
    stl::write_binary(&mesh, &stl::BinaryOptions::default(), &mut stl).expect("STL row");
    assert!(!stl.is_empty());
    let step = step_export::step_string(
        &glued.body,
        &step_export::StepOptions::default(),
        Tol::witness(),
    )
    .expect("STEP row");
    assert!(step.contains("ADVANCED_BREP_SHAPE_REPRESENTATION"));
}

/// Naming-key stability: the glued union re-runs bit-identical — the
/// naming emission (seam edges, fusions, graft rows, merge groups)
/// and the exported STL bytes are equal across independent runs
/// (D9; parallel-vs-sequential mint identity rides the same
/// determinism).
#[test]
fn declared_crosslap_union_rerun_is_bit_identical() {
    let g1 = glued();
    let g2 = glued();
    assert_eq!(g1.naming.seam_edges, g2.naming.seam_edges);
    assert_eq!(g1.naming.vertex_merges, g2.naming.vertex_merges);
    assert_eq!(g1.naming.graft_vertices, g2.naming.graft_vertices);
    assert_eq!(g1.naming.graft_edges, g2.naming.graft_edges);
    assert_eq!(g1.naming.graft_faces, g2.naming.graft_faces);
    assert_eq!(g1.naming.merge_groups, g2.naming.merge_groups);
    assert_eq!(g1.naming.face_fragments_a, g2.naming.face_fragments_a);
    assert_eq!(g1.naming.face_fragments_b, g2.naming.face_fragments_b);
    let stl_of = |g: &topo::BooleanBody<f64>| {
        let mesh = mesh::tessellate(&g.body, 1e-2, Tol::witness()).expect("tessellate");
        let mut buf = Vec::new();
        stl::write_binary(&mesh, &stl::BinaryOptions::default(), &mut buf).expect("stl");
        buf
    };
    assert_eq!(stl_of(&g1), stl_of(&g2), "STL bytes bit-identical");
}

//! **One local material-cone analysis for every touch kind** at the
//! census's instance arm.
//!
//! A touch between two solids is admitted as a rest only when the two
//! materials' local cones at the touch point have disjoint interiors;
//! a touch is the degenerate shape of a crossing, so anything else —
//! the cones overlap, or the analysis cannot tell — refuses typed.
//! These rows pose one container (the L-bracket) against a block in
//! its concavity, so the extent gate hands every pair to the material
//! test and the touches are what decide. Each rest row refused at the
//! base on a touch kind that had no analysis (a coincident vertex pair,
//! a vertex on an edge, a collinear edge overlap); each crossing row
//! is a pose whose materials overlap while the exact sweeps see only
//! touches and no vertex lies inside the other, so the touch analysis
//! is the only thing standing between it and a clear.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;
use geom_core::{Band, Point3, Tol};
use topo::{
    Body, CensusContact, ContactRecords, EntityId, SolidContainment, SolidKey, ValidationError,
    VvContact, point_in_solid_of, validate_pseudomanifold,
};

/// The L profile: counterclockwise from `+z`, reflex at (1, 1).
const L_PROFILE: [(f64, f64); 6] = [
    (0.0, 0.0),
    (3.0, 0.0),
    (3.0, 1.0),
    (1.0, 1.0),
    (1.0, 3.0),
    (0.0, 3.0),
];

/// The bracket over the L, `z ∈ [0, 1]`, with `part` grafted beside it.
fn bracket_with(part: &Body<f64>) -> Body<f64> {
    let mut out = common::prism_z::<f64>(&L_PROFILE, 0.0, 1.0, Tol::witness()).body;
    topo::graft_disjoint(&mut out, part, Tol::witness()).unwrap();
    out
}

fn block(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<f64> {
    common::brick::<f64>(x, y, z, Tol::witness())
}

/// The parallelepiped `p + u·a + v·b + w·c` over the unit cube
/// (`det[a, b, c] > 0`, so an ordinary solid).
fn parallelepiped(p: [f64; 3], a: [f64; 3], b: [f64; 3], c: [f64; 3]) -> Body<f64> {
    common::mapped_cube(
        move |u, v, w| {
            Point3::new(
                p[0] + u * a[0] + v * b[0] + w * c[0],
                p[1] + u * a[1] + v * b[1] + w * c[1],
                p[2] + u * a[2] + v * b[2] + w * c[2],
            )
        },
        Tol::witness(),
    )
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

/// The one placement refusal's `what`, asserting there is exactly one
/// and that it is not an interference.
fn the_refusal(errors: &[ValidationError]) -> &'static str {
    let placements = placement_findings(errors);
    assert_eq!(placements.len(), 1, "{errors:?}");
    let ValidationError::CensusUndecidable { what, .. } = placements[0] else {
        panic!("a typed refusal, not a decided interference: {placements:?}");
    };
    what
}

/// Every touch kind the sweeps reported, by name.
fn touch_kinds(errors: &[ValidationError]) -> Vec<&'static str> {
    let mut kinds: Vec<&'static str> = errors
        .iter()
        .filter_map(|e| match e {
            ValidationError::UndeclaredContact { contact, .. } => Some(match contact {
                CensusContact::VertexVertex { .. } => "VertexVertex",
                CensusContact::VertexOnFace { .. } => "VertexOnFace",
                CensusContact::VertexOnEdge { .. } => "VertexOnEdge",
                CensusContact::EdgeEdgeOverlap { .. } => "EdgeEdgeOverlap",
                CensusContact::EdgeFaceOverlap { .. } => "EdgeFaceOverlap",
                CensusContact::EdgeFacePierce { .. } => "EdgeFacePierce",
                CensusContact::EdgeEdgeCross { .. } => "EdgeEdgeCross",
                CensusContact::ConformalPatch { .. } => "ConformalPatch",
            }),
            _ => None,
        })
        .collect();
    kinds.sort_unstable();
    kinds.dedup();
    kinds
}

fn refused(body: &Body<f64>, records: &ContactRecords) -> Vec<ValidationError> {
    validate_pseudomanifold(body, records, Tol::witness()).expect_err("the touches refuse")
}

/// **A block standing against the wall** — `[1, 2.5] × [1.2, 3] × [0,
/// 1]`, flat on the bracket's inner wall `x = 1`, flush with the leg's
/// end `y = 3`, the floor and the ceiling. Its corners `(1, 3, z)`
/// coincide with the bracket's, its corners `(1, 1.2, z)` sit on the
/// wall's floor and ceiling edges, its edges along `x = 1` run
/// collinear with the bracket's, and its edge `(1, 1.2)` lies in the
/// wall: a coincident vertex pair, a vertex on an edge, a collinear
/// edge overlap and an edge in a face, every one with the block on the
/// `x > 1` side and the bracket on `x < 1`. The pair clears; at the
/// base it refused on the vertex pair.
#[test]
fn a_block_standing_against_the_wall_clears_through_every_touch_kind() {
    let body = bracket_with(&block((1.0, 2.5), (1.2, 3.0), (0.0, 1.0)));
    let errors = refused(&body, &ContactRecords::default());
    assert_eq!(
        touch_kinds(&errors),
        [
            "EdgeEdgeOverlap",
            "EdgeFaceOverlap",
            "VertexOnEdge",
            "VertexVertex"
        ]
    );
    assert!(placement_findings(&errors).is_empty(), "{errors:?}");
}

/// **The same block sunk into the leg** — `[0, 1.5] × [1, 3] × [0, 1]`,
/// flush with the bracket's outer face `x = 0`. The two materials
/// overlap over the leg's `[0, 1] × [1, 3] × [0, 1]`, yet every point
/// where the boundaries meet is a touch: the corners `(0, 3, z)`
/// coincide with material on the same side, the corners `(0, 1, z)`
/// sit on the bracket's outer edges, the edges along `x = 0` run
/// collinear with the bracket's inside the same wedge, and the wall's
/// edges lie in the block's faces. No pierce, no edge cross, and no
/// vertex of either strictly inside the other — the probe cannot see
/// it, and the extent gate hands it on only because the block's hull
/// sits inside the bracket's. It refuses as a crossing; at the base it
/// refused on the unanalysed vertex pair.
#[test]
fn a_block_sunk_into_the_leg_refuses_as_a_crossing() {
    let body = bracket_with(&block((0.0, 1.5), (1.0, 3.0), (0.0, 1.0)));
    let errors = refused(&body, &ContactRecords::default());
    let kinds = touch_kinds(&errors);
    for kind in ["VertexVertex", "VertexOnEdge", "EdgeEdgeOverlap"] {
        assert!(kinds.contains(&kind), "{kind}: {kinds:?}");
    }
    assert!(
        !kinds.contains(&"EdgeFacePierce") && !kinds.contains(&"EdgeEdgeCross"),
        "{kinds:?}"
    );
    // The overlap is real, by the arm's own per-solid door.
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let solids: Vec<SolidKey> = body.solids().map(|(k, _)| k).collect();
    let q = Point3::new(0.5, 2.0, 0.5);
    for s in solids {
        assert_eq!(
            point_in_solid_of(&body, s, q, band, tol).unwrap(),
            SolidContainment::In
        );
    }
    assert!(
        the_refusal(&errors).contains("one passes into the other where they touch"),
        "{errors:?}"
    );
}

/// **A block seated in the inner corner, off the floor** — `[1, 2] ×
/// [1, 2] × [0.2, 0.8]`. Its vertical edge runs collinear with the
/// bracket's REFLEX edge `(1, 1)` and its corners sit on it: the
/// bracket's cone there is the complement of a convex wedge, and the
/// block's lies inside that wedge — the peg in a concave corner, which
/// no single plane separates and the complement test certifies. The
/// pair clears; at the base it refused on the vertex-on-edge touch.
#[test]
fn a_block_seated_in_the_inner_corner_clears_on_the_complement() {
    let body = bracket_with(&block((1.0, 2.0), (1.0, 2.0), (0.2, 0.8)));
    let errors = refused(&body, &ContactRecords::default());
    let kinds = touch_kinds(&errors);
    assert!(kinds.contains(&"EdgeEdgeOverlap"), "{kinds:?}");
    assert!(kinds.contains(&"VertexOnEdge"), "{kinds:?}");
    assert!(placement_findings(&errors).is_empty(), "{errors:?}");
}

/// **A block flush with the arm's top edge** — `[1.5, 2.5] × [1, 2] ×
/// [0.5, 1]`, flat on the wall `y = 1` with its top edge along the
/// bracket's convex edge `y = 1, z = 1`: the two wedges share that edge
/// and one plane, `y = 1`, separates them. The pair clears; at the base
/// it refused on the vertex-on-edge touch.
#[test]
fn a_block_flush_with_the_arm_top_clears_on_its_shared_edge() {
    let body = bracket_with(&block((1.5, 2.5), (1.0, 2.0), (0.5, 1.0)));
    let errors = refused(&body, &ContactRecords::default());
    let kinds = touch_kinds(&errors);
    assert!(kinds.contains(&"EdgeEdgeOverlap"), "{kinds:?}");
    assert!(placement_findings(&errors).is_empty(), "{errors:?}");
}

/// **A block seated in the inner corner ON the floor** — `[1, 2] × [1,
/// 2] × [0, 0.5]`: its corner `(1, 1, 0)` coincides with the bracket's
/// inner-corner vertex, which is a SADDLE (a reflex vertical edge, two
/// convex floor edges, a floor sector of 270°). No plane through the
/// corner separates the two cones and the bracket's complement there is
/// not convex, so neither sufficient test certifies the rest: the
/// analysis refuses typed as unanalysed (filed as
/// `work/contact/a-touch-at-a-saddle-corner-refuses-unanalysed.md`).
/// Unmoved from the base, where the vertex pair had no analysis at all.
#[test]
fn a_block_on_the_floor_of_the_inner_corner_refuses_unanalysed() {
    let body = bracket_with(&block((1.0, 2.0), (1.0, 2.0), (0.0, 0.5)));
    let errors = refused(&body, &ContactRecords::default());
    assert!(
        the_refusal(&errors).contains("neither convex nor concave"),
        "{errors:?}"
    );
}

/// The tilted block whose corner meets the bracket's corner `(3, 1,
/// 1)` and nothing else: the block's cone lies in `y ≥ 1` and the
/// bracket's in `y ≤ 1`.
fn tilted_on_the_corner() -> Body<f64> {
    bracket_with(&parallelepiped(
        [3.0, 1.0, 1.0],
        [-0.3, 0.3, -1.0],
        [-1.0, 0.3, -0.3],
        [-0.3, 1.0, -0.3],
    ))
}

/// **A tilted block touching the bracket corner to corner**: the only
/// touch is the coincident vertex pair. Undeclared, the pair refuses
/// on that finding alone and carries no placement finding; declared
/// with a v-v record, the record takes the same analysis and the body
/// certifies. At the base both refused, the declared one on the
/// record's unanalysed side.
#[test]
fn a_tilted_block_touching_a_corner_clears_declared_or_not() {
    let body = tilted_on_the_corner();
    let errors = refused(&body, &ContactRecords::default());
    assert_eq!(touch_kinds(&errors), ["VertexVertex"]);
    assert!(placement_findings(&errors).is_empty(), "{errors:?}");
    let (a, b) = errors
        .iter()
        .find_map(|e| match e {
            ValidationError::UndeclaredContact {
                contact: CensusContact::VertexVertex { a, b },
                ..
            } => Some((*a, *b)),
            _ => None,
        })
        .unwrap();
    let records = ContactRecords {
        vv: vec![VvContact { a, b }],
        ..ContactRecords::default()
    };
    assert_eq!(
        validate_pseudomanifold(&body, &records, Tol::witness()),
        Ok(())
    );
}

/// **A sliver corner is read at its faces' levers.** A parallelepiped
/// rests a corner on the wall `x = 1` with a 3 cm edge off the wall and
/// a 0.8 m edge leaving it at an elevation `e` just past the run band's
/// escalation threshold. Every reading is levered at the face it stands
/// for (`census.rs`, `touch_lever`), so the long edge's faces read `e`
/// or more — definitely off the wall, on the concavity's side — and the
/// corner is a rest: the pair carries no placement finding.
#[test]
fn a_sliver_corner_clears_on_its_long_edge_s_own_reading() {
    let band = Band::linear(Tol::witness()).unwrap();
    let e = 2.0 * band.escalate();
    let body = bracket_with(&parallelepiped(
        [1.0, 2.0, 0.5],
        [0.03, 0.0, 0.0],
        [e, 0.8, 0.0],
        [0.0, 0.0, 0.3],
    ));
    let errors = refused(&body, &ContactRecords::default());
    assert!(
        !errors
            .iter()
            .any(|e| matches!(e, ValidationError::CensusEscalated { .. })),
        "no sweep escalates: {errors:?}"
    );
    assert!(placement_findings(&errors).is_empty(), "{errors:?}");
}

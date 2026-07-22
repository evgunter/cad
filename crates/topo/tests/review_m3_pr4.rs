//! M3 PR 4 adversarial review suite — INDEPENDENT derivations (censuses
//! hand-derived from geometry before reading the shipped tests; see the
//! review report). Falsification targets: Table III / edge-edge tie
//! engine (wedge-touch, notch double-tie, reflex residue), the 15.7
//! sign chain (mirrored resting fixtures), BOB-routing (pinch contexts
//! produce no join input), contact completeness, operands untouched
//! (byte-compare), pierce-ring structure, plane_eq NaN door.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::prism_z;
use geom_core::Decide;
use topo::{Body, BooleanError, BooleanOp, BooleanReduction, boolean_reduce, validate};

fn brick<T: Decide>(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<T> {
    prism_z::<T>(&[(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)], z.0, z.1).body
}

/// Full geometric dump of a body: every vertex's point coordinates (via
/// Debug — bit-faithful for f64) plus entity counts. Operand-untouched
/// checks compare this, not just counts.
fn dump<T: Decide>(b: &Body<T>) -> String {
    let mut s = String::new();
    for (vk, v) in b.vertices() {
        let p = b.get_point(v.point).unwrap();
        s.push_str(&format!("{vk:?}:{p:?};"));
    }
    s.push_str(&format!(
        "|V{} E{} F{}",
        b.vertices().count(),
        b.edges().count(),
        b.faces().count()
    ));
    s
}

fn reduce_ok<T: Decide>(op: BooleanOp, a: &Body<T>, b: &Body<T>) -> BooleanReduction<T> {
    let (da, db) = (dump(a), dump(b));
    let red = boolean_reduce(op, a, b).unwrap();
    assert_eq!(dump(a), da, "operand A mutated");
    assert_eq!(dump(b), db, "operand B mutated");
    validate(&red.a).unwrap();
    validate(&red.b).unwrap();
    red
}

const ALL_OPS: [BooleanOp; 3] = [BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract];

/// Independent two-brick census (derived from geometry: corner-overlap
/// bricks A=[0,2]^3, B=[1,3]^3; three A-edges pierce B faces at
/// (1,2,2),(2,1,2),(2,2,1); three B-edges pierce A faces at
/// (2,1,1),(1,2,1),(1,1,2); no v-v, no edge-edge). Each pierce = one
/// non-dangling run edge in the piercing body + one dangling ring strut
/// in the pierced body, one pair, one ring record.
#[test]
fn census_two_bricks_independent() {
    for op in ALL_OPS {
        let a = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
        let b = brick::<f64>((1.0, 3.0), (1.0, 3.0), (1.0, 3.0));
        let red = reduce_ok(op, &a, &b);
        assert_eq!(
            (
                red.contacts.vv.len(),
                red.contacts.a_on_b.len(),
                red.contacts.b_on_a.len()
            ),
            (0, 3, 3),
            "op {op:?}"
        );
        assert_eq!(red.null_pairs.len(), 6);
        assert_eq!(red.pierce_rings.len(), 6);
        let (dangling, run): (Vec<&topo::BoolNullEdgeRecord>, Vec<_>) =
            red.null_edges.iter().partition(|e| e.dangling);
        assert_eq!((dangling.len(), run.len()), (6, 6));
        // F3 witness: every piercing-side run edge keeps the classified
        // vertex as its below (IN) end — the copy took the OUT side.
        for e in &run {
            assert_eq!(e.attr.below_end, e.at_vertex, "op {op:?}");
        }
    }
}

/// A post through a slab: op-independent double pierce. 8 B-vertices on
/// A faces (4 on top z=2, 4 on bottom z=0), no A-on-B, no v-v; 8 pairs,
/// 8 run edges + 8 ring struts, 8 rings.
#[test]
fn census_post_through_slab() {
    for op in ALL_OPS {
        let a = brick::<f64>((0.0, 3.0), (0.0, 3.0), (0.0, 2.0));
        let b = brick::<f64>((1.0, 2.0), (1.0, 2.0), (-1.0, 3.0));
        let red = reduce_ok(op, &a, &b);
        assert_eq!(
            (
                red.contacts.vv.len(),
                red.contacts.a_on_b.len(),
                red.contacts.b_on_a.len()
            ),
            (0, 0, 8),
            "op {op:?}"
        );
        assert_eq!(red.null_pairs.len(), 8, "op {op:?}");
        assert_eq!(red.pierce_rings.len(), 8);
        assert_eq!(red.null_edges.len(), 16);
        assert_eq!(red.null_edges.iter().filter(|e| e.dangling).count(), 8);
        // Structural probe of one pierce ring: the ring vertex must live
        // on a RING loop of the pierced face, and the outer loop must
        // NOT contain it (kemr put the lone vertex on the ring side).
        let ring = red.pierce_rings[0];
        let face = red.a.get_face(ring.face).unwrap();
        let on_loop = |lk, vk| -> bool {
            let l = red.a.get_loop(lk).unwrap();
            match l.boundary {
                topo::LoopBoundary::Cycle { first } => red
                    .a
                    .loop_cycle(first)
                    .unwrap()
                    .iter()
                    .any(|&he| red.a.get_half_edge(he).unwrap().start == vk),
                topo::LoopBoundary::Empty { vertex } => vertex == vk,
            }
        };
        assert!(
            !on_loop(face.outer, ring.ring_vertex),
            "ring vertex leaked into the outer loop"
        );
        assert!(
            face.rings.iter().any(|&r| on_loop(r, ring.ring_vertex)),
            "ring vertex not on any ring loop of the pierced face"
        );
    }
}

/// The 15.7 sign chain, mirrored (F3): B resting corner-down on A's TOP
/// face and the mirror B hanging corner-up under A's BOTTOM face. For
/// ∩ and ∖ the ⁻ row lumps every coplanar sector Out and the departing
/// edges are geometrically Out — NO surgery. With the printed
/// (uninverted) 15.7 sign the departing edges would read In, the
/// neighborhoods would transition, and spurious seams would be minted:
/// this fixture executes the difference.
#[test]
fn sign_chain_mirrored_resting() {
    // Above.
    let a = brick::<f64>((0.0, 3.0), (0.0, 3.0), (0.0, 2.0));
    let b = brick::<f64>((1.0, 2.0), (1.0, 2.0), (2.0, 4.0));
    // Below (mirror).
    let c = brick::<f64>((1.0, 2.0), (1.0, 2.0), (-2.0, 0.0));
    for op in [BooleanOp::Intersect, BooleanOp::Subtract] {
        for other in [&b, &c] {
            let red = reduce_ok(op, &a, other);
            assert_eq!(red.contacts.b_on_a.len(), 4, "op {op:?}");
            assert!(red.null_edges.is_empty(), "op {op:?}: spurious surgery");
            assert!(red.pierce_rings.is_empty());
        }
    }
    // Union: the resting wall is a genuine boundary crossing of the
    // merged solid — 4 pierce rings both ways (mirror-symmetric).
    for other in [&b, &c] {
        let red = reduce_ok(BooleanOp::Union, &a, other);
        assert_eq!(red.pierce_rings.len(), 4);
        assert_eq!(red.null_pairs.len(), 4);
    }
}

/// BOB routing / pinch contexts (feeds the #61 fork): two bricks
/// sharing exactly one edge (diagonal wedge touch). Anti-parallel
/// edge-edge tie: touching, never crossing — NO join input for ANY op;
/// the two shared-endpoint v-v contacts are the entire record.
#[test]
fn wedge_touch_no_join_input_any_op() {
    let a = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    let b = brick::<f64>((1.0, 2.0), (0.0, 1.0), (1.0, 2.0));
    for op in ALL_OPS {
        let red = reduce_ok(op, &a, &b);
        assert_eq!(red.contacts.vv.len(), 2, "op {op:?}");
        assert!(red.contacts.a_on_b.is_empty() && red.contacts.b_on_a.is_empty());
        assert!(
            red.null_pairs.is_empty() && red.null_edges.is_empty() && red.pierce_rings.is_empty(),
            "op {op:?}: pinch context routed into join input"
        );
    }
}

/// Cross-stacked bricks: coplanar face overlap whose four seam corners
/// are ALL discovered through the noncoplanar-neighbor-face catch (the
/// coplanar pair itself is skipped): B's bottom edges cross A's top rim
/// edges at interior points of both ⇒ 4 v-v pairs. ∪ seams the overlap
/// rim: 4 pairs, 8 run edges, 0 rings; ∩/∖ (⁻ row): contact only.
#[test]
fn cross_stack_neighbor_face_catch() {
    let a = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let b = brick::<f64>((0.5, 1.5), (-1.0, 3.0), (2.0, 4.0));
    for op in ALL_OPS {
        let red = reduce_ok(op, &a, &b);
        assert_eq!(red.contacts.vv.len(), 4, "op {op:?}");
        assert!(red.contacts.a_on_b.is_empty() && red.contacts.b_on_a.is_empty());
        if matches!(op, BooleanOp::Union) {
            assert_eq!(red.null_pairs.len(), 4);
            assert_eq!(red.null_edges.len(), 8);
            assert!(red.null_edges.iter().all(|e| !e.dangling));
            assert!(red.pierce_rings.is_empty());
        } else {
            assert!(
                red.null_pairs.is_empty() && red.null_edges.is_empty(),
                "op {op:?}"
            );
        }
    }
}

/// Stacked identical-footprint bricks, EXACT census (the double-tie
/// neighborhoods that forced the membership-rule redesign): each of the
/// four corner v-v sites carries two seam germs for ∪ ⇒ 4 pairs, 8 run
/// edges; ∩/∖ cancel everything.
#[test]
fn stacked_double_tie_exact() {
    let a = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let b = brick::<f64>((0.0, 2.0), (0.0, 2.0), (2.0, 4.0));
    let red = reduce_ok(BooleanOp::Union, &a, &b);
    assert_eq!(red.contacts.vv.len(), 4);
    assert_eq!(red.null_pairs.len(), 4);
    assert_eq!(red.null_edges.len(), 8);
    assert!(red.null_edges.iter().all(|e| !e.dangling));
    for op in [BooleanOp::Intersect, BooleanOp::Subtract] {
        let red = reduce_ok(op, &a, &b);
        assert!(red.null_pairs.is_empty(), "op {op:?}");
        assert!(red.null_edges.is_empty(), "op {op:?}");
    }
}

/// Reflex-dihedral residue, benign side: a triangular prism nestled in
/// the notch of an L-prism, sharing only the reflex vertical edge.
/// Touch, no crossing — expect clean success with v-v contacts only OR
/// the documented loud refusal; NEVER a silent seam.
#[test]
fn reflex_edge_touch_benign() {
    let a = prism_z::<f64>(
        &[
            (0.0, 0.0),
            (2.0, 0.0),
            (2.0, 1.0),
            (1.0, 1.0),
            (1.0, 2.0),
            (0.0, 2.0),
        ],
        0.0,
        1.0,
    )
    .body;
    let b = prism_z::<f64>(&[(1.0, 1.0), (2.0, 1.4), (1.4, 2.0)], 0.0, 1.0).body;
    for op in ALL_OPS {
        match boolean_reduce(op, &a, &b) {
            Ok(red) => {
                validate(&red.a).unwrap();
                validate(&red.b).unwrap();
                assert!(
                    red.null_pairs.is_empty() && red.null_edges.is_empty(),
                    "op {op:?}: seam minted at a touch-only reflex edge"
                );
            }
            Err(
                e @ (BooleanError::ClassificationInvariant { .. } | BooleanError::Escalated { .. }),
            ) => {
                eprintln!("reflex touch refused (documented residue): {e}");
            }
            Err(e) => panic!("unexpected error class: {e}"),
        }
    }
}

/// Reflex-dihedral residue, crossing side: the triangle straddles the
/// +x arm of the L (genuine material overlap; a correct engine must
/// germ here). The convex-wedge membership limitation makes the two
/// solids disagree — the PR claims a LOUD typed refusal. A silent
/// Ok with no seam would be a wrong answer (BLOCKER-grade).
#[test]
fn reflex_edge_crossing_refuses_loudly() {
    let a = prism_z::<f64>(
        &[
            (0.0, 0.0),
            (2.0, 0.0),
            (2.0, 1.0),
            (1.0, 1.0),
            (1.0, 2.0),
            (0.0, 2.0),
        ],
        0.0,
        1.0,
    )
    .body;
    let b = prism_z::<f64>(&[(1.0, 1.0), (2.0, 0.6), (2.0, 1.4)], 0.0, 1.0).body;
    for op in ALL_OPS {
        match boolean_reduce(op, &a, &b) {
            Ok(red) => {
                assert!(
                    !red.null_pairs.is_empty(),
                    "op {op:?}: SILENT wrong answer — material overlap with no seam"
                );
            }
            Err(BooleanError::ClassificationInvariant { what }) => {
                eprintln!("op {op:?}: loud refusal: {what}");
            }
            Err(BooleanError::Escalated { .. } | BooleanError::UndeclaredCoincidence { .. }) => {}
            Err(e) => panic!("unexpected error class: {e}"),
        }
    }
}

/// Notch-fill: the brick exactly fills the L-prism's notch (every
/// contact is a declared coplanar tie; union = full box). Runs the
/// densest tie surface in the corpus: shared walls opposite-oriented,
/// shared top/bottom SAME-oriented (⁺ row live), six v-v sites, the
/// reflex edge shared. Expect: success with seams only where the ∪ rim
/// demands, or the documented loud refusal — never a quiet wrong shape.
#[test]
fn notch_fill_dense_ties() {
    let a = prism_z::<f64>(
        &[
            (0.0, 0.0),
            (2.0, 0.0),
            (2.0, 1.0),
            (1.0, 1.0),
            (1.0, 2.0),
            (0.0, 2.0),
        ],
        0.0,
        1.0,
    )
    .body;
    let b = brick::<f64>((1.0, 2.0), (1.0, 2.0), (0.0, 1.0));
    for op in ALL_OPS {
        match boolean_reduce(op, &a, &b) {
            Ok(red) => {
                validate(&red.a).unwrap();
                validate(&red.b).unwrap();
                eprintln!(
                    "op {op:?}: vv={} pairs={} edges={} rings={}",
                    red.contacts.vv.len(),
                    red.null_pairs.len(),
                    red.null_edges.len(),
                    red.pierce_rings.len()
                );
                assert_eq!(red.contacts.vv.len(), 6, "op {op:?}");
            }
            Err(
                e @ (BooleanError::ClassificationInvariant { .. }
                | BooleanError::Escalated { .. }
                | BooleanError::UndeclaredCoincidence { .. }),
            ) => {
                eprintln!("op {op:?}: refused: {e}");
            }
            Err(e) => panic!("unexpected error class: {e}"),
        }
    }
}

/// plane_eq door: bit-DIFFERENT NaN normals must never compare Same
/// (the PR 1 NaN lesson at the new seam) — and must not silently pass
/// as Distinct either; axis-plane revert (zero-sign folding) must land
/// SameOpposite.
#[test]
fn plane_eq_nan_and_negzero() {
    use geom_core::{Band, Point3, Vec3};
    use topo::{PlaneRelation, oriented_plane_eq};
    let band = Band::linear().unwrap();
    let mk = |n: Vec3<f64>, o: Point3<f64>| topo::boolean::plane_eq::PlaneDesc {
        origin: o,
        normal: n,
    };
    let nan1 = f64::from_bits(0x7ff8_0000_0000_0001);
    let nan2 = f64::from_bits(0x7ff8_0000_0000_0002);
    let p1 = mk(Vec3::new(0.0, 0.0, nan1), Point3::new(0.0, 0.0, 0.0));
    let p2 = mk(Vec3::new(0.0, 0.0, nan2), Point3::new(0.0, 0.0, 0.0));
    let r = oriented_plane_eq(&p1, &p2, 1.0, band);
    assert!(r.is_err(), "bit-different NaN planes decided {r:?}");
    // Zero-sign folding: negating an axis plane flips 0.0 signs; the
    // canonicalization must still land SameOpposite.
    let q1 = mk(Vec3::new(0.0, 0.0, 1.0), Point3::new(0.0, 0.0, 5.0));
    let q2 = mk(Vec3::new(-0.0, -0.0, -1.0), Point3::new(0.0, 0.0, 5.0));
    assert_eq!(
        oriented_plane_eq(&q1, &q2, 1.0, band).unwrap(),
        PlaneRelation::SameOpposite
    );
}

// ---- Interval lane spot checks on the review fixtures. ----
#[cfg(feature = "interval")]
mod interval {
    use super::*;
    use geom_core::Interval;

    #[test]
    fn post_through_interval() {
        let a = brick::<Interval>((0.0, 3.0), (0.0, 3.0), (0.0, 2.0));
        let b = brick::<Interval>((1.0, 2.0), (1.0, 2.0), (-1.0, 3.0));
        let red = reduce_ok(BooleanOp::Subtract, &a, &b);
        assert_eq!(red.null_pairs.len(), 8);
    }

    #[test]
    fn wedge_touch_interval() {
        let a = brick::<Interval>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
        let b = brick::<Interval>((1.0, 2.0), (0.0, 1.0), (1.0, 2.0));
        for op in ALL_OPS {
            let red = reduce_ok(op, &a, &b);
            assert!(red.null_pairs.is_empty(), "op {op:?}");
        }
    }
}

/// Generic (tie-free) edge-edge coincidence, the Fig. 19 pair: A's
/// vertical corner edge at (1,1) is collinear with a mid-span of B's
/// vertical edge; B's dihedral wedge either interleaves A's (mixed
/// angular order — the wedge straddles A's −y wall: sectors alternate
/// B(−107°), A(−90°), B(−58°), A(180°) around the line ⇒ crossing) or
/// nests strictly outside (no intersection). The derived membership
/// rule must reproduce TOG's mixed-order criterion in both directions,
/// op-independently (no coplanar ties anywhere).
#[test]
fn generic_edge_edge_mixed_order_pair() {
    let a = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    // Crossing twin: wedge contains A's −y bound.
    let b_cross = prism_z::<f64>(&[(1.0, 1.0), (0.7, 0.0), (1.5, 0.2)], -0.5, 1.5).body;
    // Touching twin: wedge strictly inside A's exterior quadrant.
    let b_touch = prism_z::<f64>(&[(1.0, 1.0), (2.0, 1.2), (1.2, 2.0)], -0.5, 1.5).body;
    for op in ALL_OPS {
        // FINDING R-1: the mixed-order (interleaved-wedge) collinear
        // overlap — TOG Fig. 19-left's analogue — currently refuses
        // with an ODD surviving-germ count (the transverse crossing
        // germ and the along-line overlap event do not pair up). Loud,
        // not wrong — but the printed TOG criterion says this
        // configuration INTERSECTS and should classify. Pinned here so
        // the fix (or the documented scope exclusion) is visible.
        match boolean_reduce(op, &a, &b_cross) {
            Ok(red) => {
                assert!(
                    !red.null_pairs.is_empty(),
                    "op {op:?}: mixed-order edge-edge crossing produced no germ"
                );
            }
            Err(BooleanError::ClassificationInvariant { what }) => {
                eprintln!("op {op:?}: R-1 loud refusal: {what}");
            }
            Err(e) => panic!("op {op:?}: unexpected error class: {e}"),
        }
        let red = reduce_ok(op, &a, &b_touch);
        assert_eq!(red.contacts.vv.len(), 2, "op {op:?}");
        assert!(
            red.null_pairs.is_empty() && red.null_edges.is_empty(),
            "op {op:?}: non-interleaved touch minted a seam"
        );
    }
}

/// The F5 curved gate itself (the shipped `curved_operand_refuses`
/// exercises only the SCAFFOLDING arm): a face swapped to a cylinder
/// must refuse as CurvedBooleanUnsupported with the offending operand
/// and face named.
#[test]
fn curved_face_gate_witness() {
    use geom_core::{Point3, Vec3};
    let a = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    let mut b = brick::<f64>((2.0, 3.0), (0.0, 1.0), (0.0, 1.0));
    let (face, _) = b.faces().next().unwrap();
    b.set_face_surface(
        face,
        topo::FaceSurface::New(geom_surfaces::Surface::Cylinder {
            origin: Point3::new(0.0, 0.0, 1.0),
            axis: Vec3::new(1.0, 0.0, 0.0),
            radius: 1.0,
            u_ref: Vec3::new(0.0, 0.0, 1.0),
        }),
    )
    .unwrap();
    match boolean_reduce(BooleanOp::Union, &a, &b) {
        Err(BooleanError::CurvedBooleanUnsupported {
            operand: topo::Operand::B,
            face: f,
        }) => assert_eq!(f, face),
        other => panic!("expected CurvedBooleanUnsupported(B), got {other:?}"),
    }
}

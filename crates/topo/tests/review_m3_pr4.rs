//! M3 PR 4 adversarial review suite — INDEPENDENT derivations (censuses
//! hand-derived from geometry before reading the shipped tests; see the
//! review report). Falsification targets: Table III / edge-edge tie
//! engine (wedge-touch, notch double-tie, reflex edge), the 15.7
//! sign chain (mirrored resting fixtures), BOB-routing (pinch contexts
//! produce no join input), contact completeness, operands untouched
//! (byte-compare), pierce-ring structure, plane_eq NaN door.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{brick, finished, flush_declarations, prism_z};
use geom_core::Decide;
use geom_core::Tol;
use topo::{AtRestBody, Body, BooleanError, BooleanOp, BooleanReduction, boolean_reduce, validate};

/// Full geometric dump of a body: every vertex's point coordinates (via
/// Debug — bit-faithful for f64) plus entity counts. Operand-untouched
/// checks compare this, not just counts.
fn dump<T: Decide>(b: &Body<T>) -> String {
    let mut s = String::new();
    for (vk, p) in b.vertex_points() {
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

fn reduce_ok<T: Decide + geom_core::Bounds + topo::AtRestPolicy>(
    op: BooleanOp,
    a: &AtRestBody<T>,
    b: &AtRestBody<T>,
) -> BooleanReduction<T> {
    let (da, db) = (dump(a), dump(b));
    // M4 PR 5: the review corpus declares its intended flush contacts.
    let red = topo::boolean_reduce_declared(
        op,
        a,
        b,
        &flush_declarations(a, b, Tol::witness()),
        Tol::witness(),
    )
    .unwrap();
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
        let a = finished(
            "operand A",
            brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0), Tol::witness()),
            Tol::witness(),
        );
        let b = finished(
            "operand B",
            brick::<f64>((1.0, 3.0), (1.0, 3.0), (1.0, 3.0), Tol::witness()),
            Tol::witness(),
        );
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
        let (dangling, run): (Vec<&topo::BoolNullEdgeRecord<f64>>, Vec<_>) =
            red.null_edges.iter().partition(|e| e.dangling);
        assert_eq!((dangling.len(), run.len()), (6, 6));
        // Per-operand view (the PR 5 ergonomics accessor): 3 pierces
        // each way ⇒ each clone carries 3 run edges + 3 ring struts.
        assert_eq!(red.null_edges_of(topo::Operand::A).count(), 6);
        assert_eq!(red.null_edges_of(topo::Operand::B).count(), 6);
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
        let a = finished(
            "operand A",
            brick::<f64>((0.0, 3.0), (0.0, 3.0), (0.0, 2.0), Tol::witness()),
            Tol::witness(),
        );
        let b = finished(
            "operand B",
            brick::<f64>((1.0, 2.0), (1.0, 2.0), (-1.0, 3.0), Tol::witness()),
            Tol::witness(),
        );
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
    let a = finished(
        "operand A",
        brick::<f64>((0.0, 3.0), (0.0, 3.0), (0.0, 2.0), Tol::witness()),
        Tol::witness(),
    );
    let b = finished(
        "operand B",
        brick::<f64>((1.0, 2.0), (1.0, 2.0), (2.0, 4.0), Tol::witness()),
        Tol::witness(),
    );
    // Below (mirror).
    let c = finished(
        "the mirrored operand",
        brick::<f64>((1.0, 2.0), (1.0, 2.0), (-2.0, 0.0), Tol::witness()),
        Tol::witness(),
    );
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
    let a = finished(
        "operand A",
        brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), Tol::witness()),
        Tol::witness(),
    );
    let b = finished(
        "operand B",
        brick::<f64>((1.0, 2.0), (0.0, 1.0), (1.0, 2.0), Tol::witness()),
        Tol::witness(),
    );
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
    let a = finished(
        "operand A",
        brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0), Tol::witness()),
        Tol::witness(),
    );
    let b = finished(
        "operand B",
        brick::<f64>((0.5, 1.5), (-1.0, 3.0), (2.0, 4.0), Tol::witness()),
        Tol::witness(),
    );
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
    let a = finished(
        "operand A",
        brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0), Tol::witness()),
        Tol::witness(),
    );
    let b = finished(
        "operand B",
        brick::<f64>((0.0, 2.0), (0.0, 2.0), (2.0, 4.0), Tol::witness()),
        Tol::witness(),
    );
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

/// The L-prism of the two reflex-edge rows: unit height, area 3, its
/// 270° wedge on the vertical edge over (1, 1).
fn l_prism_unit() -> AtRestBody<f64> {
    let l = prism_z::<f64>(
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
        Tol::witness(),
    )
    .body;
    finished("the L-prism", l, Tol::witness())
}

/// `op` on `a` and `b` under `decls` builds a body that passes tiers 2
/// and 3′ and the at-rest certificate at the closed-form `volume`, or
/// the empty result when `volume` is zero.
fn assert_sound(
    op: BooleanOp,
    a: &AtRestBody<f64>,
    b: &AtRestBody<f64>,
    decls: &topo::BooleanDeclarations,
    volume: f64,
) {
    let tol = Tol::witness();
    let res = match op {
        BooleanOp::Union => topo::union_with(a, b, decls, tol),
        BooleanOp::Intersect => topo::intersect_with(a, b, decls, tol),
        BooleanOp::Subtract => topo::subtract_with(a, b, decls, tol),
    };
    let bb = match res {
        Ok(topo::BooleanResult::Empty) if volume == 0.0 => return,
        Ok(topo::BooleanResult::Body(bb)) if volume > 0.0 => bb,
        other => panic!("op {op:?}: {other:?}"),
    };
    assert_eq!(topo::validate_closed(&bb.body), Ok(()), "op {op:?}: tier 2");
    assert_eq!(
        topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol),
        Ok(()),
        "op {op:?}: tier 3′"
    );
    assert!(
        topo::validate_geometric_certificate(&bb.body, tol).is_ok(),
        "op {op:?}: the at-rest certificate"
    );
    let v = topo::mass_properties(&bb.body, tol).unwrap().volume;
    assert!(
        (v - volume).abs() < 1e-9,
        "op {op:?}: volume {v}, want {volume}"
    );
}

/// Reflex edge, touch: a triangular prism nestled in the notch of an
/// L-prism, its corner on the reflex vertical edge, adding its area to
/// the L's 3 with nothing in common.
/// - Sharing only that edge (area 0.42), no declaration is needed: the
///   reduction finds the two v-v contacts and no seam. Both of the
///   triangle's flankers lie outside both of the L's flanking planes, so
///   no verdict splits and the wedge's extent is never asked.
/// - Flush along the +x arm's wall (area 0.4, declared): the wall's
///   anti-parallel tie reads the On ladder, which splits the verdicts,
///   and the union is right only if the 270° wedge reads reflex.
#[test]
fn reflex_edge_touch_benign() {
    let tol = Tol::witness();
    let a = l_prism_unit();
    let edge = finished(
        "the touching triangle",
        prism_z::<f64>(&[(1.0, 1.0), (2.0, 1.4), (1.4, 2.0)], 0.0, 1.0, tol).body,
        tol,
    );
    for (op, volume) in [
        (BooleanOp::Union, 3.42),
        (BooleanOp::Intersect, 0.0),
        (BooleanOp::Subtract, 3.0),
    ] {
        let red = boolean_reduce(op, &a, &edge, tol).unwrap();
        assert_eq!(red.contacts.vv.len(), 2, "op {op:?}");
        assert!(
            red.null_pairs.is_empty() && red.null_edges.is_empty(),
            "op {op:?}: seam minted at a touch-only reflex edge"
        );
        assert_sound(op, &a, &edge, &topo::BooleanDeclarations::default(), volume);
    }
    let flush = finished(
        "the flush triangle",
        prism_z::<f64>(&[(1.0, 1.0), (2.0, 1.0), (1.5, 1.8)], 0.0, 1.0, tol).body,
        tol,
    );
    let decls = flush_declarations(&a, &flush, tol);
    for (op, volume) in [
        (BooleanOp::Union, 3.4),
        (BooleanOp::Intersect, 0.0),
        (BooleanOp::Subtract, 3.0),
    ] {
        assert_sound(op, &a, &flush, &decls, volume);
    }
}

/// Reflex edge, crossing: the triangle straddles the +x arm of the L
/// from the reflex edge, so the edge-edge site reads the L's 270° wedge
/// and must germ there. Undeclared, the coplanar caps refuse at the
/// coincidence door; declared flush, each op builds the closed form:
/// the L's 3 and the triangle's 0.4 share its lower half, 0.2.
#[test]
fn reflex_edge_crossing_builds_sound() {
    let a = l_prism_unit();
    let b = finished(
        "the crossing triangle",
        prism_z::<f64>(
            &[(1.0, 1.0), (2.0, 0.6), (2.0, 1.4)],
            0.0,
            1.0,
            Tol::witness(),
        )
        .body,
        Tol::witness(),
    );
    let decls = flush_declarations(&a, &b, Tol::witness());
    for (op, volume) in [
        (BooleanOp::Union, 3.2),
        (BooleanOp::Intersect, 0.2),
        (BooleanOp::Subtract, 2.8),
    ] {
        let err = boolean_reduce(op, &a, &b, Tol::witness()).unwrap_err();
        assert!(
            matches!(err, BooleanError::UndeclaredCoincidence { .. }),
            "op {op:?}: undeclared caps, got {err:?}"
        );
        assert_sound(op, &a, &b, &decls, volume);
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
    let a = finished(
        "operand A",
        prism_z::<f64>(
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
            Tol::witness(),
        )
        .body,
        Tol::witness(),
    );
    let b = finished(
        "operand B",
        brick::<f64>((1.0, 2.0), (1.0, 2.0), (0.0, 1.0), Tol::witness()),
        Tol::witness(),
    );
    for op in ALL_OPS {
        match boolean_reduce(op, &a, &b, Tol::witness()) {
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
/// as Distinct either. Post-retirement (M4 PR 5): an axis-plane
/// revert pair decides SameOpposite through the SAME-SOURCE rung
/// (reverted orient), and WITHOUT sources the same values refuse
/// Undeclared — value equality never glues (rung (b)).
#[test]
fn plane_eq_nan_and_negzero() {
    use geom_core::{Band, Point3, Vec3};
    use topo::{GeomSource, PlaneIdentity, PlaneRelation, oriented_plane_eq};
    let band = Band::linear(Tol::witness()).unwrap();
    let mk = |n: Vec3<f64>, o: Point3<f64>| topo::boolean::plane_eq::PlaneDesc {
        origin: o,
        normal: n,
    };
    let nan1 = f64::from_bits(0x7ff8_0000_0000_0001);
    let nan2 = f64::from_bits(0x7ff8_0000_0000_0002);
    let p1 = mk(Vec3::new(0.0, 0.0, nan1), Point3::new(0.0, 0.0, 0.0));
    let p2 = mk(Vec3::new(0.0, 0.0, nan2), Point3::new(0.0, 0.0, 0.0));
    let r = oriented_plane_eq(&p1, &p2, PlaneIdentity::NONE, &metre_ball(1.0), band);
    assert!(r.is_err(), "bit-different NaN planes decided {r:?}");
    // Same-source revert pair (the post-retirement declared rung):
    // orient split decides SameOpposite with zero numerics.
    let q1 = mk(Vec3::new(0.0, 0.0, 1.0), Point3::new(0.0, 0.0, 5.0));
    let q2 = mk(Vec3::new(-0.0, -0.0, -1.0), Point3::new(0.0, 0.0, 5.0));
    let src = GeomSource::minted(1, 0);
    let src_rev = src.reverted();
    assert_eq!(
        oriented_plane_eq(
            &q1,
            &q2,
            PlaneIdentity {
                s1: Some(&src),
                s2: Some(&src_rev),
                declared: false
            },
            &metre_ball(1.0),
            band
        )
        .unwrap(),
        PlaneRelation::SameOpposite
    );
    // The SAME values without sources: Undeclared, typed — the M4
    // PR 5 narrowing (equal bits without shared source stay unglued).
    let r = oriented_plane_eq(&q1, &q2, PlaneIdentity::NONE, &metre_ball(1.0), band);
    assert!(
        matches!(r, Err(topo::PlaneEqError::Undeclared { .. })),
        "unsourced value-equal planes must refuse Undeclared, got {r:?}"
    );
}

// ---- Interval lane spot checks on the review fixtures. ----
mod interval {
    use super::*;
    use geom_core::Interval;

    #[test]
    fn post_through_interval() {
        let a = finished(
            "operand A",
            brick::<Interval>(
                (0.0, 3.0),
                (0.0, 3.0),
                (0.0, 2.0),
                geom_core::Tol::witness(),
            ),
            Tol::witness(),
        );
        let b = finished(
            "operand B",
            brick::<Interval>(
                (1.0, 2.0),
                (1.0, 2.0),
                (-1.0, 3.0),
                geom_core::Tol::witness(),
            ),
            Tol::witness(),
        );
        let red = reduce_ok(BooleanOp::Subtract, &a, &b);
        assert_eq!(red.null_pairs.len(), 8);
    }

    #[test]
    fn wedge_touch_interval() {
        let a = finished(
            "operand A",
            brick::<Interval>(
                (0.0, 1.0),
                (0.0, 1.0),
                (0.0, 1.0),
                geom_core::Tol::witness(),
            ),
            Tol::witness(),
        );
        let b = finished(
            "operand B",
            brick::<Interval>(
                (1.0, 2.0),
                (0.0, 1.0),
                (1.0, 2.0),
                geom_core::Tol::witness(),
            ),
            Tol::witness(),
        );
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
    let a = finished(
        "operand A",
        brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), Tol::witness()),
        Tol::witness(),
    );
    // Crossing twin: wedge contains A's −y bound.
    let b_cross = finished(
        "the crossing twin",
        prism_z::<f64>(
            &[(1.0, 1.0), (0.7, 0.0), (1.5, 0.2)],
            -0.5,
            1.5,
            Tol::witness(),
        )
        .body,
        Tol::witness(),
    );
    // Touching twin: wedge strictly inside A's exterior quadrant.
    let b_touch = finished(
        "the touching twin",
        prism_z::<f64>(
            &[(1.0, 1.0), (2.0, 1.2), (1.2, 2.0)],
            -0.5,
            1.5,
            Tol::witness(),
        )
        .body,
        Tol::witness(),
    );
    for op in ALL_OPS {
        // FINDING R-1 (fixed in the review fix-pass): the mixed-order
        // (interleaved-wedge) collinear overlap — TOG Fig. 19-left's
        // analogue — used to refuse with an ODD surviving-germ count:
        // `resolve_bisector_graze`'s reference-sector fallback matched
        // ANY On record on the wide-sector twins, conflating the
        // along-line overlap event with the transverse bisector-graze
        // crossing and keying it against the wrong face (fix:
        // `find_ref_sector` requires the start-holder match). The
        // interleaved configuration intersects per TOG's mixed-order
        // criterion. Derived census: on each z-cap of A (z=0, z=1) the
        // section polyline (1,1)→(0.7,0)→(1,0.075) crosses A's
        // boundary at three sites — the mixed-order corner vv pair at
        // (1,1) (edge-edge germ along the shared line + transverse
        // bisector-graze germ), the transverse edge-edge vv pair at
        // (0.7,0) (B's wall edge in A's y=0 plane crossing A's cap
        // edge), and the vertex-on-face pierce at (1,0.075) — six
        // null-edge pairs total (4 vv + 2 vf), op-independently.
        let red = reduce_ok(op, &a, &b_cross);
        assert_eq!(
            red.null_pairs.len(),
            6,
            "op {op:?}: mixed-order crossing census (4 vv + 2 vf seam pairs)"
        );
        let vv_pairs = red
            .null_pairs
            .iter()
            .filter(|p| matches!(p.site, topo::PairSite::VertexVertex(_)))
            .count();
        assert_eq!(vv_pairs, 4, "op {op:?}: vertex-vertex seam-pair census");
        let red = reduce_ok(op, &a, &b_touch);
        assert_eq!(red.contacts.vv.len(), 2, "op {op:?}");
        assert!(
            red.null_pairs.is_empty() && red.null_edges.is_empty(),
            "op {op:?}: non-interleaved touch minted a seam"
        );
    }
}

/// A brick over `x` with one face relabelled a cone whose apex sits
/// one unit before the brick on the `x` axis, the face's boundary left
/// on the brick's lines. Its box is read off the face's own boundary,
/// so where the BRICK sits decides whether the box reaches `[0, 1]^3`.
#[cfg(test)]
fn brick_with_cone_face_at(x: (f64, f64)) -> (topo::Body<f64>, topo::FaceKey) {
    use geom_core::Vec3;
    let apex = geom_core::Point3::new(x.0 - 1.0, 0.5, 0.5);
    let mut b = brick::<f64>(x, (0.0, 1.0), (0.0, 1.0), Tol::witness());
    let (face, _) = b.faces().next().unwrap();
    // Lifts RechartStrandsDescriptions: the relabelled cone is the operand gate's input, its box read off the brick.
    b.set_face_surface_unvouched_for_tests(
        face,
        topo::FaceSurface::New {
            surface: geom::Surface::Cone {
                apex,
                axis: Vec3::new(1.0, 0.0, 0.0),
                half_angle: 0.25,
                u_ref: Vec3::new(0.0, 0.0, 1.0),
            },
            sense: true,
        },
    )
    .unwrap();
    (b, face)
}

/// The edges bounding `face`'s outer loop, and its half-edges there.
fn face_boundary(
    b: &Body<f64>,
    face: topo::FaceKey,
) -> (Vec<topo::EdgeKey>, Vec<topo::HalfEdgeKey>) {
    let outer = b.get_face(face).unwrap().outer;
    let topo::LoopBoundary::Cycle { first } = b.get_loop(outer).unwrap().boundary else {
        panic!("the relabelled face's outer loop is a cycle")
    };
    let hes = b.loop_cycle(first).unwrap();
    let mut edges: Vec<_> = hes
        .iter()
        .map(|&he| b.get_half_edge(he).unwrap().edge)
        .collect();
    edges.sort();
    (edges, hes)
}

/// The `DescriptionNotAdjacent` edges among `errors`, sorted.
fn not_adjacent_edges(errors: &[topo::ValidationError]) -> Vec<topo::EdgeKey> {
    let mut edges: Vec<_> = errors
        .iter()
        .filter_map(|e| match e {
            topo::ValidationError::DescriptionNotAdjacent { edge } => Some(*edge),
            _ => None,
        })
        .collect();
    edges.sort();
    edges
}

/// The cone-relabelled brick does not finish: the at-rest gate refuses
/// it on the relabelled face — one `DescriptionNotAdjacent` for each of
/// the face's four boundary edges, whose lines do not lie on the cone,
/// and one pcurve finding, a certification refusal on one of the face's
/// own half-edges (its chart image does not map back onto its line, check
/// 4's map residual) — and on nothing else. One, though all four lines
/// are off the cone: the relabelled face stores no rows, so tier 3
/// re-derives it whole and reports the derivation's refusal, which is the
/// face's first owed one (`topo::pcurves::validate_pcurves`, step 1).
fn assert_cone_face_refuses_at_rest(b: Body<f64>, face: topo::FaceKey) {
    assert!(
        matches!(
            b.get_surface(b.get_face(face).unwrap().surface),
            Some(geom::Surface::Cone { .. })
        ),
        "the relabelled face carries the cone"
    );
    let (edges, hes) = face_boundary(&b, face);
    assert_eq!(edges.len(), 4);
    let errors = AtRestBody::validate(b, Tol::witness())
        .expect_err("a cone face over a brick's lines does not finish");
    assert_eq!(errors.len(), 5, "four edges and one pcurve: {errors:?}");
    assert_eq!(not_adjacent_edges(&errors), edges, "{errors:?}");
    let pcurve: Vec<_> = errors
        .iter()
        .filter_map(|e| match e {
            topo::ValidationError::Pcurve {
                finding:
                    topo::PcurveMintError::Certify {
                        half_edge,
                        error:
                            geom_brep::PcurveCertifyError::ResidualExceeded {
                                check: geom_brep::PcurveCheck::MapResidual,
                                ..
                            },
                    },
            } => Some(*half_edge),
            _ => None,
        })
        .collect();
    assert!(
        matches!(pcurve[..], [he] if hes.contains(&he)),
        "one map-residual refusal, on the cone face's loop: {errors:?}"
    );
}

/// A relabelled cone face posed so its box reaches the other operand:
/// the brick is refused at rest on the relabelled face
/// (`assert_cone_face_refuses_at_rest`), so it never reaches the
/// boolean.
#[test]
fn curved_face_gate_witness() {
    // The brick overlaps `[0, 1]^3`, so the cone face's box would reach it.
    let (b, face) = brick_with_cone_face_at((0.5, 1.5));
    assert_cone_face_refuses_at_rest(b, face);
}

/// **The other side of the same question**: the SAME body with the
/// cone face's box moved clear of `[0, 1]^3` is refused at rest just
/// the same, on the relabelled face — where the brick sits does not
/// make a stranded relabel a finished body.
#[test]
fn a_cone_relabelled_brick_clear_of_the_other_operand_is_refused_at_rest() {
    // Ten units out along x: the face's box cannot reach x ∈ [0, 1].
    let (b, face) = brick_with_cone_face_at((10.0, 11.0));
    assert_cone_face_refuses_at_rest(b, face);
}

/// Shape (iii) READINESS at the operand: a brick with one face
/// relabelled to the placeholder NURBS net does not finish. The
/// at-rest gate refuses it on that face and nothing else: one
/// `UncertifiableSurface` naming the face (the placeholder net), and
/// one `DescriptionNotAdjacent` for each of its four boundary edges,
/// whose lines lie on no surface the net describes.
#[test]
fn a_placeholder_nurbs_wall_is_refused_at_rest() {
    let mut b = brick::<f64>((0.5, 1.5), (0.0, 1.0), (0.0, 1.0), Tol::witness());
    let (face, _) = b.faces().next().unwrap();
    // Lifts RechartStrandsDescriptions: the NURBS wall is the at-rest gate's input; its edges are not the row.
    b.set_face_surface_unvouched_for_tests(
        face,
        topo::FaceSurface::New {
            surface: geom::Surface::Nurbs(std::sync::Arc::new(geom::NurbsSurface::placeholder())),
            sense: true,
        },
    )
    .unwrap();
    let (edges, _) = face_boundary(&b, face);
    assert_eq!(edges.len(), 4);
    let errors = AtRestBody::validate(b, Tol::witness())
        .expect_err("a placeholder NURBS wall does not finish");
    assert_eq!(errors.len(), 5, "one surface and four edges: {errors:?}");
    assert_eq!(
        errors[0],
        topo::ValidationError::UncertifiableSurface { face },
        "the placeholder net is named on its face: {errors:?}"
    );
    assert_eq!(not_adjacent_edges(&errors), edges, "{errors:?}");
}

/// The two DERIVATION-CORRECTED ∖-column cells, executed geometrically
/// (the A-versus-B rows where the shipped table diverges from TOG's
/// print). Small operand-A resting on B (AonB⁻, opposite): the printed
/// table would seam A∖B; the corrected rule must not. Small A embedded
/// floor-to-floor in B (AonB⁺, identical): printed would seam A∖B;
/// corrected must not (A∖B needs no crossings — A's boundary
/// classifies uniformly). ∪ rows (both tables agree) must still seam.
#[test]
fn corrected_subtract_cells_geometric() {
    // AonB⁻ (opposite): A rests on B's top face.
    let a = finished(
        "operand A",
        brick::<f64>((1.0, 2.0), (1.0, 2.0), (2.0, 4.0), Tol::witness()),
        Tol::witness(),
    );
    let b = finished(
        "operand B",
        brick::<f64>((0.0, 3.0), (0.0, 3.0), (0.0, 2.0), Tol::witness()),
        Tol::witness(),
    );
    let red = reduce_ok(BooleanOp::Subtract, &a, &b);
    assert_eq!(red.contacts.a_on_b.len(), 4);
    assert!(
        red.null_edges.is_empty() && red.pierce_rings.is_empty(),
        "corrected (∖, A-vs-B, opposite) = No-intersect violated"
    );
    let red = reduce_ok(BooleanOp::Union, &a, &b);
    assert_eq!(red.pierce_rings.len(), 4, "∪ must still seam the rim");

    // AonB⁺ (identical): A embedded floor-to-floor inside B.
    let a = finished(
        "operand A",
        brick::<f64>((1.0, 2.0), (1.0, 2.0), (0.0, 1.0), Tol::witness()),
        Tol::witness(),
    );
    let b = finished(
        "operand B",
        brick::<f64>((0.0, 3.0), (0.0, 3.0), (0.0, 2.0), Tol::witness()),
        Tol::witness(),
    );
    let red = reduce_ok(BooleanOp::Subtract, &a, &b);
    assert_eq!(red.contacts.a_on_b.len(), 4);
    assert!(
        red.null_edges.is_empty() && red.pierce_rings.is_empty(),
        "corrected (∖, A-vs-B, identical) = No-intersect violated"
    );
    // ∩ keeps A whole (A∩B = A): uniform In, no seam either.
    let red = reduce_ok(BooleanOp::Intersect, &a, &b);
    assert!(red.null_edges.is_empty());
    // ∪ = B with A's coincident floor patch surviving: seams needed.
    let red = reduce_ok(BooleanOp::Union, &a, &b);
    assert_eq!(red.pierce_rings.len(), 4);
}

/// A ball of radius `arm` about the origin, no point of either face
/// known: the extent a bare arm names.
fn metre_ball(arm: f64) -> topo::ConsumedExtent<'static, f64> {
    topo::ConsumedExtent::unwitnessed(geom_brep::ExtentBall::new(geom_core::Point3::origin(), arm))
}

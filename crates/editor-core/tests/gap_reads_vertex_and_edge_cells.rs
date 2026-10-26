//! **`Gap` reads vertex and edge cells** (DESIGN `[assertions.quiet]`:
//! a contact's two cells "at whatever dimension they are").
//!
//! A triangular prism rests apex down on a block, the two placed as two
//! arrivals of the world: once with its apex edge lying on the block's
//! top, once tilted so only an end of that edge, a vertex, touches it.
//! A `Gap = 0` over the two cells, read where each copy holds them,
//! holds at rest, and the gap reads the prism's lift when it is moved
//! apart and its depth, negative, when moved into the block.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;
use crate::fixture::{ename, fname, in_world, insert, len, on_frame, place, vname, vpiece};

use editor_core::{
    AssertionRelation, AssertionVerdict, CapEnd, Frame, MeasurePrimitive, Node, NodeResult,
    ProfileDoc, RecipeNodeId, RoleSeg, SitedRef, StableName, ValuePayload,
};
use geom_core::{Tol, Turns};

/// An extrude of `loop_` on the profile's own xy, `height` up its z.
fn extrude(doc: ProfileDoc, loop_: Vec<(f64, f64)>, height: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![loop_],
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(height),
        },
    )
}

/// Which of the prism's cells rests on the block.
#[derive(Clone, Copy, Debug)]
enum Rest {
    /// The apex edge, lying along the block's top: the prism turned a
    /// quarter turn about x, so its profile's y is the world's z and
    /// its extrusion runs along −y.
    Edge,
    /// The apex edge's end on the prism's end cap, alone: the prism
    /// turned a third of a turn about x, which tips that end lowest,
    /// 0.2 below the apex's start.
    Vertex,
}

/// **The block, the prism resting on it `lift` above contact, and the
/// two cells in contact**, as the copies hold them: `(doc, the block's
/// top, the prism's resting cell)`.
fn resting(rest: Rest, lift: f64) -> (ProfileDoc, SitedRef, SitedRef) {
    let doc = ProfileDoc::empty_derived("gap-reads-vertex-and-edge-cells", Tol::witness());
    let (doc, block) = extrude(
        doc,
        vec![(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)],
        1.0,
    );
    // The apex first, so it is the profile's canonical vertex 0.
    let (doc, prism) = extrude(doc, vec![(0.0, 0.0), (0.2, 0.3), (-0.2, 0.3)], 0.4);
    let (turns, at) = match rest {
        Rest::Edge => (Turns::new(1, 4), [0.0, 0.2, 1.0 + lift]),
        Rest::Vertex => (Turns::new(1, 3), [0.0, 0.3, 1.2 + lift]),
    };
    let frame = Frame::rotate_by_turns_then_translate(
        [1.0, 0.0, 0.0],
        turns.expect("a fraction of a turn"),
        at,
        crate::fixture::band(),
    )
    .expect("the x axis is a direction");
    let (doc, p) = place(doc, block);
    let (doc, q) = in_world(doc, prism, &frame);
    let apex = vpiece(&doc, prism, 0, 0);
    let cell: StableName = match rest {
        Rest::Edge => ename(prism, RoleSeg::LateralEdge(apex)),
        Rest::Vertex => vname(prism, RoleSeg::CapVertex(CapEnd::End, apex)),
    };
    let top = fname(block, RoleSeg::Cap(CapEnd::End));
    (
        doc,
        SitedRef::new(p, top.in_copy(p)),
        SitedRef::new(q, cell.in_copy(q)),
    )
}

/// A `Gap(outer, inner)` and a `Gap = 0` over it: the document after
/// them, the measure and the assertion.
fn asserted_contact(
    doc: &ProfileDoc,
    outer: &SitedRef,
    inner: &SitedRef,
) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let (doc, measure) = crate::fixture::measure_node(
        doc,
        MeasurePrimitive::Gap { outer: 0, inner: 1 },
        vec![outer.clone(), inner.clone()],
    );
    let value = crate::fixture::value_of(&doc, measure);
    let (doc, assertion) = insert(
        doc,
        Node::Assertion {
            value,
            bound: len(0.0),
            relation: AssertionRelation::Equal,
        },
    );
    (doc, measure, assertion)
}

/// **(The unit's acceptance) A vertex resting on a face and an edge
/// lying on one, between two arrivals, each hold a `Gap = 0` over their
/// two cells, and the gap reads the prism's lift**: `+0.01` moved
/// apart, `−0.01` moved into the block, in either seat. Red if `Gap`
/// still admits faces only (the measure is refused at its door), if a
/// seat flips the sign, or if the edge reads at its higher end.
#[test]
fn a_vertex_and_an_edge_resting_on_a_face_read_their_gap() {
    for rest in [Rest::Edge, Rest::Vertex] {
        for lift in [0.0, 0.01, -0.01] {
            let (doc, top, cell) = resting(rest, lift);
            for (outer, inner, seat) in [(&top, &cell, "face outer"), (&cell, &top, "cell outer")] {
                let (doc, measure, assertion) = asserted_contact(&doc, outer, inner);
                let run = corpus::eval::<f64>(&doc);
                let value = match run.nodes.get(&measure) {
                    Some(NodeResult::Ok(v)) => match &v.payload {
                        ValuePayload::Measure { value, .. } => *value,
                        other => panic!("{rest:?} {seat}: a measure, not {}", other.kind_name()),
                    },
                    other => panic!("{rest:?} {seat}, lift {lift}: the gap reads: {other:?}"),
                };
                assert!(
                    (value - lift).abs() < 1e-9,
                    "{rest:?} {seat}: the gap at lift {lift} reads {value}"
                );
                let holds = matches!(
                    run.value(assertion).map(|v| &v.payload),
                    Some(ValuePayload::Assertion(AssertionVerdict::Holds { .. }))
                );
                assert_eq!(
                    holds,
                    lift == 0.0,
                    "{rest:?} {seat}: `Gap = 0` holds exactly at rest, lift {lift}"
                );
            }
        }
    }
}

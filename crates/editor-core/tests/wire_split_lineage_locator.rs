//! **A cycling split lineage names the edge it cycles on.**
//!
//! `emit_topo`'s `chase_edge_to_table` walks `SplitEdge` birth records
//! to the root the operand's table names. A walk that never reaches one
//! is a corrupt provenance record, and `Body::split_root` says so with
//! the one fact a repair can start from: WHICH edge. The emitter used
//! to `map_err` that key away and raise a sentence, so the refusal's
//! KIND stayed honest — an emission inconsistency is what happened —
//! while its only locator died at the conversion.
//!
//! WHAT THIS SUITE CAN AND CANNOT REACH. The refusal is defensive:
//! `Provenance` is written only by `topo`'s crate-internal `add_edge`,
//! `Body` has no `Deserialize`, and a lineage built by `split_edge`
//! runs to strictly older edges, so a chain is bounded by the body's
//! own edge count and the guard's `edges.len() + 1` budget cannot be
//! spent from outside the kernel. No door reachable from this crate
//! constructs a cycling body, so no row here evaluates one. What the
//! rows DO pin is everything between the caught record and the human:
//! the `From` the chase site now spends, the rendering it lands in, and
//! the property `pncad_py::errors::reads_as_prose` asserts live on
//! every raise. A key hard-coded or dropped anywhere on that path turns
//! a row red.
//!
//! The keys are real ones, taken out of an evaluated body rather than
//! fabricated: a slotmap key's rendering is an index and a version, and
//! a suite that invents them is asserting over its own arithmetic.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    EvalOptions, Evaluation, NamingError, Node, NodeErrorKind, ProfileDoc, RecipeNodeId,
    ValuePayload,
};
use geom_core::Tol;
use topo::{Body, EdgeKey, SplitLineageCycle};

use fixture::{insert, len, square};

/// Two distinct edges of one real evaluated body.
fn two_edges() -> (EdgeKey, EdgeKey) {
    let doc = ProfileDoc::empty_derived("wire_split_lineage_locator", Tol::witness());
    let (doc, plane) = insert(doc, fixture::xy_frame());
    let (doc, profile) = insert(
        doc,
        Node::Profile(fixture::desc(plane, vec![square(0.0, 0.0, 1.0)])),
    );
    let (doc, block) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(2.0),
        },
    );
    let ev = fixture::run(&doc, &EvalOptions::default());
    let body = body_of(&ev, block);
    let mut keys = body.edges().map(|(k, _)| k);
    let a = keys.next().expect("a prism has edges");
    let b = keys.next().expect("a prism has more than one edge");
    assert_ne!(a, b, "two DISTINCT keys, or the rows below prove nothing");
    (a, b)
}

fn body_of(ev: &Evaluation<f64>, n: RecipeNodeId) -> &Body<f64> {
    match &ev.value(n).expect("the extrude evaluates").payload {
        ValuePayload::Body(b) => b,
        other => panic!("an extrude's payload is a body, got {}", other.kind_name()),
    }
}

/// The node-level prose is the ONLY route by which an emitter refusal
/// reaches a human (Python's typed exception message is exactly this
/// string), so the locator is pinned there rather than at
/// `NamingError`'s own `Display`.
fn carried(edge: EdgeKey) -> String {
    NodeErrorKind::Naming(NamingError::from(SplitLineageCycle { edge })).to_string()
}

/// The caught record's edge survives the conversion the chase site
/// spends and the node boundary above it — and it is THIS edge, not a
/// constant: two different keys must give two different sentences.
#[test]
fn the_cycling_edge_reaches_the_node_level_prose() {
    let (a, b) = two_edges();

    let sa = carried(a);
    assert!(
        sa.contains(&format!("{a:?}")),
        "the cycling edge is the only locator this failure has: {sa}"
    );

    let sb = carried(b);
    assert!(sb.contains(&format!("{b:?}")), "{sb}");
    assert_ne!(
        sa, sb,
        "a refusal that reads the same for two different edges has no locator"
    );
    assert!(
        !sa.contains(&format!("{b:?}")),
        "the refusal names the edge it caught, not another: {sa}"
    );
}

/// The KIND stays honest. An emission inconsistency is what a corrupt
/// birth record is, and the framing sentence that says so is the one
/// `NamingError::Emission` uses; what the variant adds is the locator,
/// not a new category.
#[test]
fn the_refusal_keeps_the_emission_framing() {
    let (a, _) = two_edges();
    let s = carried(a);
    assert!(
        s.contains("a mint-time emission fact was inconsistent with the result body"),
        "category kept: {s}"
    );
    assert!(
        s.contains("name emission failed"),
        "node category kept: {s}"
    );
}

/// `pncad_py::py::typed_err` asserts `reads_as_prose` on every raise,
/// live under release, and its fingerprint is the field brace `" { "`.
/// A payload rendered through a derived `Debug` is how that assertion
/// gets broken, and this refusal carries one.
#[test]
fn the_refusal_reads_as_prose() {
    let (a, b) = two_edges();
    for s in [carried(a), carried(b)] {
        assert!(
            !s.contains(" { "),
            "a braced payload panics the Python binding at the arm meant to \
             refuse gracefully: {s}"
        );
    }
}

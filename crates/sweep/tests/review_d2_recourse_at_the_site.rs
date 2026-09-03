//! **D2 review probes — the recourse contract checked AT THE SITE, and
//! the entity vocabulary checked against `topo`'s.**
//!
//! An independent derivation of what PR #740 claims, not a re-reading
//! of its diff. The unit's own guard
//! (`fillet::recourse_tests::a_recourse_is_appended_only_where_the_table_allows_it`)
//! builds `BlendError` values BY HAND and renders them, so it pins the
//! `Display` impl against a table. What it cannot see is the half S19
//! actually complained about: whether a REFUSAL SITE picks the right
//! class. The wrong the finding named was a user being told to
//! "fillet a set of edges whose open chains are single convex
//! plane–plane links…" after a refusal that had nothing to do with
//! chain shape — and that is a property of the sites, reachable only
//! through the front door.
//!
//! So these rows drive `fillet_edges` and read what a caller would
//! actually see.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use sweep::blend::build::fillet_edges;
use sweep::blend::{
    BlendError, FILLET3_ASSEMBLY_RECOURSE, FILLET3_BODY_RECOURSE, FILLET3_CHAIN_RECOURSE,
    FILLET3_CLEARANCE_RECOURSE, FILLET3_CONVEXITY_RECOURSE, FILLET3_CORNER_RECOURSE,
    FILLET3_GEOMETRY_RECOURSE, FILLET3_RADIUS_RECOURSE, FILLET3_RING_RECOURSE,
    FILLET3_SPINE_KIND_RECOURSE, FILLET3_SPINE_RECOURSE, FILLET3_TANGENTIAL_RECOURSE,
};
use sweep::test_support::cube;
use topo::query;

const L: f64 = 1.0;
const R: f64 = 0.1;

/// Every recourse sentence `fillet` can append, with a short name for
/// the assertion message. Deliberately restated here rather than
/// imported as the module's own `ALL` array: this suite is an
/// independent derivation, and a constant dropped from that private
/// array would silently weaken the "no foreign recourse" half.
const ALL: [(&str, &str); 12] = [
    ("radius", FILLET3_RADIUS_RECOURSE),
    ("clearance", FILLET3_CLEARANCE_RECOURSE),
    ("tangential", FILLET3_TANGENTIAL_RECOURSE),
    ("spine", FILLET3_SPINE_RECOURSE),
    ("chain", FILLET3_CHAIN_RECOURSE),
    ("convexity", FILLET3_CONVEXITY_RECOURSE),
    ("corner", FILLET3_CORNER_RECOURSE),
    ("assembly", FILLET3_ASSEMBLY_RECOURSE),
    ("body", FILLET3_BODY_RECOURSE),
    ("geometry", FILLET3_GEOMETRY_RECOURSE),
    ("ring", FILLET3_RING_RECOURSE),
    ("spine-kind", FILLET3_SPINE_KIND_RECOURSE),
];

/// Assert that the rendered refusal carries `expect` (or nothing, when
/// `expect` is `None`) and NO other recourse sentence.
fn only_recourse(err: &BlendError, expect: Option<&str>, what: &str) {
    let text = err.to_string();
    if let Some(e) = expect {
        assert!(
            text.contains(e),
            "{what}: the refusal must carry its own recourse.\n  got: {text}"
        );
    }
    for (name, other) in ALL {
        if Some(other) == expect {
            continue;
        }
        assert!(
            !text.contains(other),
            "{what}: the refusal carries the `{name}` recourse, which is not its own.\n  got: {text}"
        );
    }
}

/// **The finding's own complaint, through the front door.** A cube
/// with one edge requested leaves its two corners partly requested —
/// a run-out. Before #740 the user was handed the ASSEMBLY recourse
/// ("…single plane–plane links ending at fully-requested
/// trivalent corners…") for it. This row goes red if any site on that
/// path ever re-attaches it.
#[test]
fn a_run_out_refusal_gives_corner_advice_and_no_assembly_advice() {
    let body = cube(L, Tol::witness());
    let edges = query::all_edges(&body);
    let err = fillet_edges(&body, &edges[..1], R, Tol::witness())
        .expect_err("one edge of a box leaves its corners partly requested");
    assert!(
        matches!(err.error, BlendError::UnsupportedRunOut { .. }),
        "expected a corner frontier, got {err:?}"
    );
    only_recourse(
        &err.error,
        Some(FILLET3_CORNER_RECOURSE),
        "one-edge run-out",
    );
}

/// **A refusal that reports invalid input gives no fillet advice.**
/// A repeated edge is malformed for the chain walk; there is no
/// "instead, do this" to offer, and the pre-#740 variant offered one
/// anyway.
#[test]
fn a_repeated_edge_refusal_gives_no_recourse_at_all() {
    let body = cube(L, Tol::witness());
    let edges = query::all_edges(&body);
    let mut req = edges.clone();
    req.push(edges[0]);
    let err = fillet_edges(&body, &req, R, Tol::witness()).expect_err("a repeated edge");
    assert!(
        matches!(err.error, BlendError::RepeatedEdge { edge } if edge == edges[0]),
        "expected the repeated-edge refusal naming the key, got {err:?}"
    );
    only_recourse(&err.error, None, "repeated edge");
}

/// **The body frontier, reached through the public graft door.** Two
/// disjoint cubes in one body are valid input the in-place surgery has
/// not been built for; the advice must be the BODY one, not the chain
/// one.
#[test]
fn a_multi_solid_body_gives_body_advice_and_no_chain_advice() {
    let mut body = cube(L, Tol::witness());
    let other = cube(L, Tol::witness());
    topo::instance::graft_disjoint_all(&mut body, &other, Tol::witness())
        .expect("a disjoint graft");
    let edges = query::all_edges(&body);
    let err = fillet_edges(&body, &edges[..1], R, Tol::witness())
        .expect_err("the in-place surgery is built for one solid");
    assert!(
        matches!(err.error, BlendError::UnsupportedBody { solids, .. } if solids == 2),
        "expected the body frontier carrying the solid count, got {err:?}"
    );
    only_recourse(&err.error, Some(FILLET3_BODY_RECOURSE), "two-solid body");
}

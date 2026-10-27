//! **An overlap the census decides through an aligned face pair can be
//! quieted** (D10 `[assertions]`, `[assertions.quiet]`): the touches of
//! the pair's boundaries in the closure of its shared area are the
//! pair's, so its one `SameSideInterference` is the finding's whole
//! evidence, and a holding negative `Gap` over the opposed pair that
//! bounds the overlap quiets it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;
use crate::fixture::{fname, insert, len, on_frame, place, square, wall};

use editor_core::{AssertionRelation, MeasurePrimitive, Node, ProfileDoc, RecipeNodeId, SitedRef};
use geom_core::Tol;

fn block(doc: ProfileDoc, (cx, cy): (f64, f64), h: f64, height: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(cx, cy, h)],
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(height),
        },
    )
}

/// Two unit blocks overlapping in a 0.05 slab across x, flush in y and
/// z: the slab is bounded by `a`'s +x wall and `b`'s −x wall (opposed),
/// and its other faces lie on the two blocks' aligned walls and caps. A
/// holding `Gap ≤ −0.01` over the opposed pair quiets the one finding,
/// whose evidence is the census's `SameSideInterference` alone. Red if
/// an aligned pair's boundary touches are reported beside it, which
/// keeps the finding loud.
#[test]
fn a_flush_sided_slab_is_quiet_under_its_gap() {
    let doc = ProfileDoc::empty_derived("aligned-slab-quiet", Tol::witness());
    let (doc, a) = block(doc, (0.0, 0.0), 0.5, 1.0);
    let (doc, b) = block(doc, (0.95, 0.0), 0.5, 1.0);
    let (doc, p) = place(doc, a);
    let (doc, q) = place(doc, b);
    let (doc, measure) = crate::fixture::measure_node(
        &doc,
        MeasurePrimitive::Gap { outer: 0, inner: 1 },
        vec![
            SitedRef::new(p, fname(a, wall(&doc, a, 1))),
            SitedRef::new(q, fname(b, wall(&doc, b, 3))),
        ],
    );
    let value = crate::fixture::value_of(&doc, measure);
    let (doc, assertion) = insert(
        doc,
        Node::Assertion {
            value,
            bound: len(-0.01),
            relation: AssertionRelation::AtMost,
        },
    );
    let run = corpus::eval::<f64>(&doc);
    let findings = crate::fixture::at_rest(&doc, &run)
        .unwrap_or_else(|e| panic!("an overlap is a finding: {e}"));
    let interference = crate::fixture::interference(&findings);
    let [finding] = &interference[..] else {
        panic!("one interference: {interference:?}");
    };
    assert!(
        matches!(
            finding.evidence[..],
            [topo::ValidationError::SameSideInterference { .. }]
        ),
        "the aligned pair is the finding's whole evidence: {:?}",
        finding.evidence
    );
    assert_eq!(
        finding.quiet,
        Some(assertion),
        "the holding gap quiets it: {finding:?}"
    );
}

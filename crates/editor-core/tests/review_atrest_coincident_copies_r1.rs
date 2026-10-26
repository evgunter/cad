//! Reviewer probe for PR #5745: an overlap the census decides through a
//! same-side face pair, bounded by an opposed pair a holding negative
//! `Gap` reads, against `[assertions.quiet]`.

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
/// and every other face bounding it lies between their carriers. A
/// holding `Gap ≤ −0.01` over that opposed pair is what
/// `[assertions.quiet]` admits. Printed: the finding's census evidence
/// and whether it is quiet.
#[test]
fn review_a_flush_sided_slab_under_its_gap() {
    for (label, h_b, z_b) in [("flush-sided", 0.5, 0.0), ("strictly inside", 0.3, 0.25)] {
        let doc = ProfileDoc::empty_derived("review-flush-slab", Tol::witness());
        let (doc, a) = block(doc, (0.0, 0.0), 0.5, 1.0);
        let (doc, b) = block(doc, (0.45 + h_b, 0.0), h_b, 2.0 * h_b);
        let (doc, p) = place(doc, a);
        let (doc, q) =
            crate::fixture::in_world(doc, b, &editor_core::Frame::translation([0.0, 0.0, z_b]));
        let (doc, measure) = crate::fixture::measure_node(
            &doc,
            MeasurePrimitive::Gap { outer: 0, inner: 1 },
            vec![
                SitedRef::new(p, fname(a, wall(&doc, a, 1)).in_copy(p)),
                SitedRef::new(q, fname(b, wall(&doc, b, 3)).in_copy(q)),
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
        let verdict = run.value(assertion).map(|v| format!("{:?}", v.payload));
        let assembly = editor_core::assemble(&doc, &run, Tol::witness())
            .unwrap_or_else(|e| panic!("{label}: the gate refused: {e}"));
        for f in &assembly.interference {
            let kinds: Vec<String> = f
                .evidence
                .iter()
                .map(|e| {
                    format!("{e:?}")
                        .split([' ', '{'])
                        .next()
                        .unwrap()
                        .to_string()
                })
                .collect();
            eprintln!(
                "PROBE {label}: assertion={verdict:?} quiet={:?} overlap={:?} evidence={kinds:?}",
                f.quiet,
                std::mem::discriminant(&f.overlap)
            );
        }
        eprintln!("PROBE {label}: findings={}", assembly.interference.len());
    }
}

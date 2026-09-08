//! **The nominal is an input to the key** — the rows for the rule at
//! `eval::tag::slot`: a frame node's slot values are fixed at the
//! evaluation scalar AND at the document's nominal f64, because the
//! profile pre-pass and the pinned op read the nominal whatever the
//! evaluation scalar is.
//!
//! At `Interval` the bounds do not determine the nominal — a value
//! edit under a compensating box leaves `(lo, hi, dec)` equal — so
//! without the nominal in the key the second evaluation below hits the
//! memo and serves a profile placed on the FIRST nominal's plane.
//! Every row here is about that: the edit recomputes, the served
//! plane is the cold run's, and the two neighbouring cases (same
//! nominal under a different box; same nominal under the same box)
//! still key the way a box being a real input says they should.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;
use std::sync::Arc;

use crate::fixture;

use editor_core::analysis::{BoxAxis, ParamBox};
use editor_core::{
    CancelToken, ContentKey, Datum, Dimension, DocEdit, DocParam, DocParamValue, EvalOptions,
    Evaluation, Expr, Node, ParamName, ProfileDoc, RecipeNodeId, ValuePayload, evaluate,
};
use geom_core::{Interval, Tol};

/// The frame is node 0 and the profile drawn on it node 1, in that
/// insertion order.
const FRAME: RecipeNodeId = RecipeNodeId(0);
const PROFILE: RecipeNodeId = RecipeNodeId(1);

fn p() -> ParamName {
    ParamName::new("p")
}

/// The probe's document: `p` a scalar parameter at `nominal`, a frame
/// whose first in-plane direction is `u = [1, p, 0]`, and a square
/// profile drawn on it. The profile's own program holds no parameter,
/// so everything that moves in these rows moves through the frame.
fn doc_at(nominal: f64) -> ProfileDoc {
    let doc = ProfileDoc::empty_derived("eval9_nominal_in_the_key", Tol::witness());
    let doc = doc
        .apply(
            &DocEdit::SetDocParam {
                name: p(),
                value: DocParam::continuous(Dimension::Scalar, nominal),
            },
            Tol::witness(),
        )
        .expect("the parameter declares")
        .doc;
    let doc = doc
        .apply(
            &DocEdit::InsertNode {
                node: Node::Datum(Datum::Frame {
                    origin: [fixture::len(0.0), fixture::len(0.0), fixture::len(0.0)],
                    u: [
                        fixture::scl(1.0),
                        Expr::param(p(), Dimension::Scalar),
                        fixture::scl(0.0),
                    ],
                    v: [fixture::scl(0.0), fixture::scl(1.0), fixture::scl(0.0)],
                }),
            },
            Tol::witness(),
        )
        .expect("the frame inserts")
        .doc;
    doc.apply(
        &DocEdit::InsertNode {
            node: Node::Profile(fixture::desc(FRAME, vec![fixture::square(0.0, 0.0, 1.0)])),
        },
        Tol::witness(),
    )
    .expect("the profile inserts")
    .doc
}

/// The same document with `p`'s nominal moved — the edit under test.
fn set_p(doc: &ProfileDoc, value: f64) -> ProfileDoc {
    doc.apply(
        &DocEdit::SetDocParamValue {
            name: p(),
            value: DocParamValue::Continuous(value),
        },
        Tol::witness(),
    )
    .expect("the value edit applies")
    .doc
}

/// A one-axis box: `p ∈ nominal + [lo, hi]`.
fn box_of(lo: f64, hi: f64) -> Arc<ParamBox> {
    let mut axes = BTreeMap::new();
    axes.insert(p(), BoxAxis::Varying { lo, hi });
    Arc::new(ParamBox::from_axes(axes))
}

fn run(
    doc: &ProfileDoc,
    prior: Option<&Evaluation<Interval>>,
    box_: &Arc<ParamBox>,
) -> Evaluation<Interval> {
    let opts = EvalOptions {
        param_box: Some(Arc::clone(box_)),
        ..EvalOptions::default()
    };
    evaluate::<Interval>(doc, prior, &CancelToken::new(), &opts, Tol::witness())
}

/// The profile's placement, as the value carries it: the pinned lift
/// embeds the f64 plane whole, so this IS what the pre-pass read.
/// Rendered, because the comparison wanted here is "the same twelve
/// numbers", which is what `Debug` on the placement prints.
fn plane_of(ev: &Evaluation<Interval>) -> String {
    let ValuePayload::Profile(pv) = &ev.value(PROFILE).expect("the profile evaluates").payload
    else {
        panic!("a profile value");
    };
    format!("{:?}", pv.validated.plane().placement)
}

fn keys(ev: &Evaluation<Interval>) -> (ContentKey, ContentKey) {
    (
        ev.value(FRAME).expect("the frame evaluates").content_key,
        ev.value(PROFILE).expect("the profile evaluates").content_key,
    )
}

/// **The hole, closed** (EVAL-7's review probe).
///
/// `p = 0` over `[-0.25, 0.25]`, then `p = 0.5` over `[-0.75, -0.25]`:
/// the two boxes bind the SAME interval for `p`, so every bit the lane
/// feed writes is equal, while the nominal — the value the pre-pass
/// resolves the plane at and the pinned op embeds — moved from `0` to
/// `0.5`. The prior must not be served: the frame's key moves, the
/// profile's moves with it, the second run recomputes both, and the
/// plane it reports is the plane a cold run of the edited document
/// reports rather than the identity the prior held.
#[test]
fn a_nominal_edit_under_a_compensating_box_does_not_hit() {
    let first = doc_at(0.0);
    let prior = run(&first, None, &box_of(-0.25, 0.25));

    let edited = set_p(&first, 0.5);
    let narrow = box_of(-0.75, -0.25);
    let hit = run(&edited, Some(&prior), &narrow);
    let cold = run(&edited, None, &narrow);

    // The plane really does move between the two nominals - `u` is
    // the x axis at `p = 0` and normalized (1, 0.5, 0) at `p = 0.5` -
    // so the row below is not vacuous.
    assert_ne!(
        plane_of(&prior),
        plane_of(&cold),
        "the two nominals are two planes"
    );
    // The harm: what the second evaluation reports as the
    // profile's placement.
    assert_eq!(
        plane_of(&hit),
        plane_of(&cold),
        "the served profile is placed on the edited nominal's plane"
    );
    // And the mechanism: the nominal is in the key, so nothing of the
    // edited document is served from the prior at all.
    assert_eq!(
        hit.recomputed, cold.recomputed,
        "the edited document recomputes as much with the prior as without it"
    );
    assert_ne!(
        keys(&prior),
        keys(&hit),
        "the keys must move with the nominal"
    );
}

/// **The box is a real input, and the nominal did not swallow it**:
/// the same document under a DIFFERENT box does not hit, and under the
/// same box it does.
#[test]
fn the_box_still_decides_a_hit_at_one_nominal() {
    let doc = doc_at(0.25);
    let prior = run(&doc, None, &box_of(-0.1, 0.1));

    let widened = run(&doc, Some(&prior), &box_of(-0.2, 0.2));
    assert_ne!(
        keys(&prior),
        keys(&widened),
        "a widened box is a different evaluation of the same nominal"
    );
    assert_eq!(widened.reused, 0, "nothing may be served across a box edit");

    let again = run(&doc, Some(&prior), &box_of(-0.1, 0.1));
    assert_eq!(keys(&prior), keys(&again));
    assert_eq!(
        again.recomputed, 0,
        "the same nominal under the same box is the memo's own case"
    );
    assert_eq!(again.reused, doc.len());
}

/// The document's own shape, so the rows above are about the key and
/// not about a fixture that quietly stopped carrying `p`: the frame
/// reads the parameter, the profile's program does not.
#[test]
fn the_probe_document_carries_the_parameter_only_in_the_frame() {
    let doc = doc_at(0.0);
    let mut refs = Vec::new();
    let Some(Node::Datum(Datum::Frame { u, .. })) = doc.node(FRAME) else {
        panic!("a frame at node 0");
    };
    for e in u {
        e.param_refs(&mut refs);
    }
    assert_eq!(refs, vec![(p(), Dimension::Scalar)]);
    let Some(Node::Profile(program)) = doc.node(PROFILE) else {
        panic!("a profile at node 1");
    };
    assert_eq!(program.plane, FRAME);
    assert!(
        !program.references(&p()),
        "the program must hold no parameter"
    );
}

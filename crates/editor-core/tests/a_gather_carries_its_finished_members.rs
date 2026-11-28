//! **A product of finished members carries their verdicts**
//! (`Framed::gathered`, `topo::boolean::gather_finished`): each member finished in its own
//! frame and placed by a joint, the aggregate finished by carrying, at
//! every scalar alike.
//!
//! The member is a union of two overlapping blocks (a Boolean's result,
//! a finished body at every scalar), copied twice into the world: once
//! turned about `z` by a joint (`theta`, `±0.03` rad), a few centimetres
//! off the axis, and once at rest. Finishing that product again from
//! values asks whether each turned edge's carrier starts at its turned
//! vertex, two enclosures of one point as wide as the joint, which plain
//! `Interval` cannot decide. A product of sweeps, whose leaves the switch
//! keeps at tier 2 over a box (`topo::AtRestPolicy::leaf`), is still
//! gathered as data and finished whole.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{self, Recorder, len, square};
use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use editor_core::{
    Bodies, CancelToken, Dimension, DocEdit, EvalOptions, Evaluation, Formula, FreeVar, Held,
    LoopProgram, Node, ProfileDoc, ProfileProgram, RecipeNodeId, TargetAxis, UnitSym, VarName,
    evaluate,
};
use geom_core::{Interval, Sym, SymBudget, Tol};

/// A block: a square of half-side 5 mm about `(cx, 0)`, extruded 1 cm.
fn block(r: &mut Recorder, cx: f64) -> RecipeNodeId {
    let p = r.insert(Node::Profile(ProfileProgram {
        loops: vec![LoopProgram::polygon(square(cx, 0.0, 0.005)).expect("finite corners")],
        ids: Vec::new(),
        on: None,
    }));
    r.insert(Node::Extrude {
        profile: p.into(),
        distance: len(0.01),
    })
}

/// The product: `member` copied turned by the joint at `x = 3 cm` and at
/// rest at `x = at_rest`, gathered. Returns the document and its
/// product's `⊔`.
fn product(finished: bool, at_rest: f64) -> (ProfileDoc, RecipeNodeId) {
    let mut r = Recorder::new();
    r.push(DocEdit::DeclareVar {
        name: VarName::from_static("theta"),
        def: editor_core::VarDecl::Free(FreeVar::Continuous {
            dim: Dimension::Angle,
            value: 0.0,
            display_unit: UnitSym::canonical_for(Dimension::Angle),
            distribution: Some(editor_core::Distribution::Uniform {
                lo: -0.03,
                hi: 0.03,
            }),
        }),
    });
    let a = block(&mut r, 0.0);
    let member = if finished {
        let b = block(&mut r, 0.004);
        let b = r.against(a, b, &editor_core::Frame::IDENTITY);
        r.insert(Node::Union {
            members: Bodies::spelled(vec![a.into(), b.into()]),
        })
    } else {
        a
    };
    r.in_world_by(
        member,
        fixture::turned_to(
            [0.03, 0.0, 0.0],
            &[(
                TargetAxis::Z,
                Formula::named(VarName::from_static("theta"), Dimension::Angle),
            )],
        ),
    );
    r.in_world_by(member, fixture::turned_to([at_rest, 0.0, 0.0], &[]));
    r.assemble_world_copies();
    let doc = r.doc;
    let gather = doc
        .product()
        .and_then(|var| doc.operation_of(var))
        .expect("the product is a gather");
    (doc, gather)
}

/// Options over the document's whole analyzed box.
fn over_the_box(doc: &ProfileDoc) -> EvalOptions {
    let analyzed = analyzed_box(doc, &AnalysisPolicy::default());
    EvalOptions {
        param_box: Some(std::sync::Arc::new(ParamBox::of(&analyzed))),
        ..EvalOptions::default()
    }
}

/// The gather's body as its value holds it.
fn held<T: topo::AtRestPolicy + geom_core::Decide>(
    ev: &Evaluation<T>,
    gather: RecipeNodeId,
) -> Held<T> {
    if let Some(e) = ev.node_error(gather) {
        panic!("the gather builds: {:?}", e.kind);
    }
    ev.value(gather)
        .and_then(|v| v.payload.one_body().flatten())
        .expect("the gather holds one body")
        .holding()
        .clone()
}

/// **A product of finished members under a boxed joint is a finished
/// body at both box scalars**, and the product's census gathers it too.
/// Before the gather carried, its graft was a leaf there and its taker
/// finished it from values, refusing on the turned endpoints at plain
/// `Interval`.
#[test]
fn a_product_of_finished_members_under_a_boxed_joint_finishes_at_the_box_scalars() {
    let tol = Tol::witness();
    let (doc, gather) = product(true, -0.03);
    let opts = over_the_box(&doc);
    let ev: Evaluation<Interval> = evaluate(&doc, None, &CancelToken::new(), &opts, tol);
    let body = held(&ev, gather);
    assert!(
        matches!(body, Held::Finished(_)),
        "Interval: the gather holds a finished body"
    );
    body.finished(tol)
        .expect("Interval: the taker takes it as it is");
    // The census's own questions between the copies are its own, read
    // over the box; what is pinned is that its gather finishes.
    editor_core::product_recorded(&doc, &ev, tol)
        .unwrap_or_else(|e| panic!("Interval: the product's census gathers: {e:?}"));
    let budget = SymBudget {
        max_terms: 4096,
        max_degree: 128,
    };
    let (finished, _) = geom_core::sym::with_session(budget, || {
        let ev: Evaluation<Sym<Interval>> = evaluate(&doc, None, &CancelToken::new(), &opts, tol);
        let body = held(&ev, gather);
        matches!(body, Held::Finished(_)) && body.finished(tol).is_ok()
    });
    assert!(finished, "Sym<Interval>: the gather holds a finished body");
}

/// **A product of sweeps is gathered as data while the switch lives**:
/// a sweep's leaf is tier 2 alone over a box, so no lane carries it, and
/// at `f64` its aggregate is finished whole at the gather, once.
#[test]
fn a_product_of_sweeps_is_finished_whole_while_their_leaves_are_not_finished() {
    let tol = Tol::witness();
    let (doc, gather) = product(false, -0.03);
    let ev: Evaluation<Interval> =
        evaluate(&doc, None, &CancelToken::new(), &over_the_box(&doc), tol);
    let body = held(&ev, gather);
    // The switch's state, read off its own type: whether a box scalar's
    // taker finishes its leaf.
    let tier_2 = <Interval as topo::AtRestPolicy>::TAKE_FINISHES;
    assert_eq!(
        matches!(body, Held::Leaf(_)),
        tier_2,
        "Interval: a gather of sweeps is a leaf exactly while the box scalars' leaf is tier 2"
    );
    #[cfg(debug_assertions)]
    if tier_2 {
        let before = topo::finishes_on_this_thread();
        let ev: Evaluation<f64> = evaluate(
            &doc,
            None,
            &CancelToken::new(),
            &EvalOptions::default(),
            tol,
        );
        let _ = held(&ev, gather);
        // The block's own finish, and the aggregate's, from values.
        let finishes = topo::finishes_on_this_thread() - before;
        assert_eq!(
            finishes, 2,
            "f64: the block finishes once and the sweeps' aggregate once more, from values"
        );
    }
}

/// **Every lane logs the same gather**: what the product's node logs,
/// then what a door that takes its body logs taking it, is the same at
/// `f64` and at `Interval` on the nominal, so a drive's witness and its
/// leaf compare equal there. Red if one lane carried what the other
/// derived: the derived lane's body is finished again where it is taken.
#[test]
fn a_gather_of_finished_members_logs_one_vector_at_every_lane() {
    let tol = Tol::witness();
    let (doc, gather) = product(true, -0.03);
    macro_rules! logged {
        ($t:ty) => {{
            let ev: Evaluation<$t> = evaluate(
                &doc,
                None,
                &CancelToken::new(),
                &EvalOptions::default(),
                tol,
            );
            let mut log = ev
                .value(gather)
                .map(|v| v.verdicts.to_vec())
                .expect("the gather builds");
            let bracket = geom_core::k_stats::Bracket::open();
            held(&ev, gather).finished(tol).expect("its body is taken");
            log.extend(bracket.finish().verdicts);
            log
        }};
    }
    assert_eq!(
        logged!(f64),
        logged!(Interval),
        "the gather's and its taker's verdict log at f64 and at Interval"
    );
}

/// **What lies between members is still asked**: two finished copies
/// that overlap are carried into one finished body, and the product's
/// census, which reads that body, still finds them overlapping.
#[test]
fn carrying_the_members_verdicts_does_not_certify_them_apart() {
    let tol = Tol::witness();
    let (doc, _) = product(true, 0.03);
    let opts = over_the_box(&doc);
    let ev: Evaluation<Interval> = evaluate(&doc, None, &CancelToken::new(), &opts, tol);
    let census = editor_core::product_recorded(&doc, &ev, tol).map(|p| p.census);
    assert!(
        !matches!(&census, Ok(found) if found.is_empty()),
        "two copies standing in one place are not certified apart: {census:?}"
    );
}

/// Reviewer row: the between-members row above passes on ANY refusal of
/// the census (`!matches!(Ok(empty))`), and reads plain `Interval`, where
/// the census escalates over the turned box. At `f64` (the joint at its
/// value) two copies in one place must be FOUND: an `Ok` census naming
/// a finding, not an `Err`.
#[test]
fn review_two_copies_in_one_place_are_found_overlapping_at_f64() {
    let tol = Tol::witness();
    let (doc, _) = product(true, 0.03);
    let ev: Evaluation<f64> = evaluate(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        tol,
    );
    let census = editor_core::product_recorded(&doc, &ev, tol).map(|p| p.census);
    eprintln!("REVIEW f64 census: {census:?}");
    let ev: Evaluation<Interval> =
        evaluate(&doc, None, &CancelToken::new(), &over_the_box(&doc), tol);
    let boxed = editor_core::product_recorded(&doc, &ev, tol).map(|p| p.census);
    eprintln!("REVIEW Interval census: {boxed:?}");
    assert!(
        matches!(&census, Ok(found) if !found.is_empty()),
        "f64: the census finds the two copies overlapping: {census:?}"
    );
}

//! **The nominal is an input to the key** — the rows for the rule at
//! `eval::tag::slot`: a node's slot values are fixed at the evaluation
//! scalar AND at the document's nominal f64, because the profile
//! pre-pass reads the nominal whatever the evaluation scalar is.
//!
//! At `Interval` the bounds do not determine the nominal — a value
//! edit under a compensating box leaves `(lo, hi, dec)` equal — so
//! without the nominal in the key the second evaluation below hits the
//! memo and serves a profile whose structure the FIRST nominal
//! selected. Every interval row here is about that: the edit
//! recomputes, the served profile is the cold run's, and the two
//! neighbouring cases
//! (same nominal under a different box; the same value re-applied
//! under the same box) still key the way a box being a real input says
//! they should.
//!
//! The last row is lane-free and runs in every build: a slot that
//! refuses at the nominal refuses its NODE, typed, which is why the
//! key has no word for an unreadable nominal.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;
use std::sync::Arc;

use crate::fixture;

use editor_core::analysis::{BoxAxis, ParamBox};
use editor_core::{
    CancelToken, Dimension, DocEdit, EvalOptions, Formula, FreeVar, Node, NodeErrorKind,
    NodeResult, ProfileDoc, RecipeNodeId, SlotId, VarName, evaluate,
};
use geom_core::Tol;

/// The profile, the document's first node.
fn profile(doc: &ProfileDoc) -> RecipeNodeId {
    doc.ids()[0]
}

/// Its extrude, the second.
fn extrude(doc: &ProfileDoc) -> RecipeNodeId {
    doc.ids()[1]
}

fn p() -> VarName {
    VarName::from_static("p")
}

/// The probe's one variable in `doc`.
fn var(doc: &ProfileDoc) -> editor_core::VarId {
    doc.var_named("p").expect("the probe declares p")
}

/// The probe's document: `p` a scalar parameter at `nominal`, a
/// triangle profile whose apex is slid `u` along its own x, `u` built
/// from `p` by `u_of`, and its extrude. The extrude holds no
/// parameter, so everything that moves in these rows moves through the
/// profile. Only the apex reads `p`: a coordinate two vertices share
/// widens each on its own at `Interval`, and the edge between them
/// (exactly axis-aligned at the nominal) then reads an enclosure about
/// zero that a box this wide cannot decide (`chord_side`).
fn doc_with(nominal: f64, u_of: fn(Formula) -> Formula) -> ProfileDoc {
    use editor_core::{LoopProgram, ProfileProgram, ProgramStep, ProgramTarget};
    let doc = ProfileDoc::empty_derived("eval9_nominal_in_the_key", Tol::witness());
    let doc = doc
        .apply(
            &DocEdit::DeclareVar {
                name: p(),
                def: editor_core::VarDecl::Free(FreeVar::continuous(Dimension::Scalar, nominal)),
            },
            Tol::witness(),
        )
        .expect("the parameter declares")
        .doc;
    let len = fixture::len;
    let x = |at: f64| {
        Formula::add(
            len(at),
            Formula::mul(u_of(Formula::named(p(), Dimension::Scalar)), len(1.0))
                .expect("a scalar times a length"),
        )
        .expect("two lengths")
    };
    let to = |x: Formula, y: Formula| ProgramStep::LineTo(ProgramTarget::Point([x, y]));
    let program = ProfileProgram {
        loops: vec![LoopProgram::Chain(vec![
            ProgramStep::At([len(-1.0), len(-1.0)]),
            to(len(1.0), len(-1.0)),
            to(x(0.0), len(1.0)),
            ProgramStep::LineTo(ProgramTarget::Start),
        ])],
        ids: Vec::new(),
        on: None,
    };
    let doc = doc
        .apply(
            &DocEdit::InsertNode {
                node: Box::new(Node::Profile(program)),
                fresh: Vec::new(),
            },
            Tol::witness(),
        )
        .expect("the profile inserts")
        .doc;
    doc.apply(
        &DocEdit::InsertNode {
            node: Box::new(Node::Extrude {
                profile: profile(&doc).into(),
                distance: len(1.0),
            }),
            fresh: Vec::new(),
        },
        Tol::witness(),
    )
    .expect("the extrude inserts")
    .doc
}

/// `u = p` — the probe's slide.
fn doc_at(nominal: f64) -> ProfileDoc {
    doc_with(nominal, |param| param)
}

/// A one-axis box: `p ∈ nominal + [lo, hi]`.
fn box_of(doc: &ProfileDoc, lo: f64, hi: f64) -> Arc<ParamBox> {
    let mut axes = BTreeMap::new();
    axes.insert(var(doc), BoxAxis::Varying { lo, hi });
    Arc::new(ParamBox::from_axes(axes))
}

fn boxed(box_: &Arc<ParamBox>) -> EvalOptions {
    EvalOptions {
        param_box: Some(Arc::clone(box_)),
        ..EvalOptions::default()
    }
}

/// **The rows that need a widened lane**, whose whole point is that
/// the interval bits agree while the nominals do not.
mod over_a_param_box {
    use super::{box_of, boxed, doc_at, extrude, p, profile};

    /// **The scale of every value and box below**: the largest power
    /// of two at most half the process ε, so each `nominal + offset` is
    /// exact and the two boxes of the first row bind one interval by
    /// bits. `p` slides the triangle's apex, so the edge from the apex
    /// back to the start reads, against its neighbour's chord, a
    /// `chord_side` enclosure about zero about as wide as the box: a box
    /// this small stays inside the zero band at every ε row, so the
    /// profile certifies, and what these rows hold is the key.
    fn s() -> f64 {
        let half_eps = Tol::witness().eps() / 2.0;
        2f64.powi(half_eps.log2().floor() as i32)
    }

    use std::sync::Arc;

    use editor_core::analysis::ParamBox;
    use editor_core::{
        CancelToken, ContentKey, DocEdit, Evaluation, FreeValue, ProfileDoc, ValuePayload, evaluate,
    };
    use geom_core::{Bounds, Interval, Tol};

    /// The same document with `p`'s value re-applied — the edit under
    /// test. `SetVarValue` is a value-only edit, so applying the
    /// value the document already carries is a real edit that moves no
    /// nominal.
    fn set_p(doc: &editor_core::ProfileDoc, value: f64) -> ProfileDoc {
        doc.apply(
            &DocEdit::SetVarValue {
                var: p().into(),
                value: FreeValue::Continuous(value),
            },
            Tol::witness(),
        )
        .expect("the value edit applies")
        .doc
    }

    fn run(
        doc: &ProfileDoc,
        prior: Option<&Evaluation<Interval>>,
        box_: &Arc<ParamBox>,
    ) -> Evaluation<Interval> {
        evaluate::<Interval>(
            doc,
            prior,
            &CancelToken::new(),
            &boxed(box_),
            Tol::witness(),
        )
    }

    /// The profile's vertices, as BITS: the profile at the lane. Read
    /// through the public accessors and compared by bits, which is
    /// stricter than `==` on an interval and is the comparison these
    /// rows mean.
    fn vertex_bits(doc: &ProfileDoc, ev: &Evaluation<Interval>) -> Vec<(u64, u64)> {
        let ValuePayload::Profile(pv) = &ev
            .value(profile(doc))
            .expect("the profile evaluates")
            .payload
        else {
            panic!("a profile value");
        };
        pv.validated
            .loops()
            .iter()
            .flat_map(|l| l.vertices().iter().flat_map(|q| [q.x, q.y]))
            .map(|v: Interval| (v.lo().to_bits(), v.hi().to_bits()))
            .collect()
    }

    fn keys(doc: &ProfileDoc, ev: &Evaluation<Interval>) -> (ContentKey, ContentKey) {
        (
            ev.value(profile(doc))
                .expect("the profile evaluates")
                .content_key,
            ev.value(extrude(doc))
                .expect("the extrude evaluates")
                .content_key,
        )
    }

    /// **The hole, closed.**
    ///
    /// `p = 0` over `[-0.25, 0.25]·s`, then `p = 0.5·s` over
    /// `[-0.75, -0.25]·s` ([`s`]):
    /// the two boxes bind the SAME interval for `p`, so every bit the lane
    /// feed writes is equal, while the nominal — the value the pre-pass
    /// resolves at — moved from `0` to `0.5·s`. The prior must not be
    /// served: the profile's key moves, the extrude's moves with it, the
    /// second run recomputes both, and the profile it reports is the
    /// profile a cold run of the edited document reports.
    #[test]
    fn a_nominal_edit_under_a_compensating_box_does_not_hit() {
        let first = doc_at(0.0);
        let prior = run(&first, None, &box_of(&first, -0.25 * s(), 0.25 * s()));

        let edited = set_p(&first, 0.5 * s());
        let narrow = box_of(&first, -0.75 * s(), -0.25 * s());
        let hit = run(&edited, Some(&prior), &narrow);
        let cold = run(&edited, None, &narrow);

        // Each run resolves the profile at its LANE, over the box, and
        // the two boxes bind the same interval for `p`: the two runs
        // report one profile. What the nominal moves is the first pass's
        // structure, which the key must carry.
        assert_eq!(
            vertex_bits(&first, &prior),
            vertex_bits(&first, &cold),
            "both boxes bind one interval for p, so one profile"
        );
        // And the mechanism: the nominal is in the key, so nothing of the
        // edited document is served from the prior at all.
        assert_eq!(
            hit.recomputed, cold.recomputed,
            "the edited document recomputes as much with the prior as without it"
        );
        assert_ne!(
            keys(&first, &prior),
            keys(&first, &hit),
            "the keys must move with the nominal"
        );
    }

    /// **The box is a real input, and the nominal did not swallow it.**
    /// The document is EDITED in both halves — `SetVarValue` applying
    /// the value it already carries — so each half is a prior served (or
    /// refused) across a real edit rather than a re-run of one document.
    #[test]
    fn the_box_still_decides_a_hit_at_one_nominal() {
        let doc = doc_at(0.25 * s());
        let narrow = box_of(&doc, -0.1 * s(), 0.1 * s());
        let prior = run(&doc, None, &narrow);
        let edited = set_p(&doc, 0.25 * s());

        let widened = run(&edited, Some(&prior), &box_of(&doc, -0.2 * s(), 0.2 * s()));
        assert_ne!(
            keys(&doc, &prior),
            keys(&doc, &widened),
            "a widened box is a different evaluation of the same nominal"
        );
        assert_eq!(widened.reused, 0, "nothing may be served across a box edit");

        let again = run(&edited, Some(&prior), &narrow);
        assert_eq!(keys(&doc, &prior), keys(&doc, &again));
        assert_eq!(
            again.recomputed, 0,
            "the same nominal under the same box is the memo's own case"
        );
        assert_eq!(again.reused, edited.len());
    }
}

/// **A slot that refuses at the nominal refuses its node, typed** —
/// which is why `tag::slot` has no word for an unreadable nominal.
///
/// The box is DEGENERATE at a non-zero offset, the one shape `f64`
/// admits (`f64::axis` accepts `lo == hi` by bits and binds
/// `nominal + c`), so the lane and the nominal genuinely disagree at
/// f64: `u = 1 / p` with `p` at nominal `0` evaluates to `0.5` at
/// the lane's `p = 2` and refuses non-finite at the nominal. The profile
/// names the slot it refused at, and the extrude downstream is poisoned
/// through it. The profile is inserted at `p = 1`, where its program
/// resolves, and `p` is then set to `0`.
#[test]
fn a_slot_that_refuses_at_the_nominal_refuses_its_node() {
    let doc = doc_with(1.0, |param| {
        Formula::div(fixture::scl(1.0), param).expect("scalar over scalar")
    });
    let doc = doc
        .apply(
            &DocEdit::SetVarValue {
                var: p().into(),
                value: editor_core::FreeValue::Continuous(0.0),
            },
            Tol::witness(),
        )
        .expect("the value edit applies")
        .doc;
    let ev = evaluate::<f64>(
        &doc,
        None,
        &CancelToken::new(),
        &boxed(&box_of(&doc, 2.0, 2.0)),
        Tol::witness(),
    );
    match ev.nodes.get(&profile(&doc)) {
        Some(NodeResult::Failed(e)) => match &e.kind {
            NodeErrorKind::Expr { slot, .. } => {
                assert!(
                    matches!(slot, SlotId::Profile { .. }),
                    "the refusing slot is named: {slot:?}"
                );
            }
            other => panic!("expected a typed expression refusal, got {other:?}"),
        },
        other => panic!("expected the profile to refuse, got {other:?}"),
    }
    assert!(matches!(
        ev.nodes.get(&extrude(&doc)),
        Some(NodeResult::Poisoned { through }) if *through == profile(&doc)
    ));
}

/// The document's own shape, so the rows above are about the key and
/// not about a fixture that quietly stopped carrying `p`: the profile's
/// program reads the parameter, the extrude does not.
#[test]
fn the_probe_document_carries_the_parameter_only_in_the_profile() {
    let doc = doc_at(0.0);
    let var = doc.var_named(p().as_str()).expect("the profile's variable");
    let Some(Node::Profile(program)) = doc.node(profile(&doc)).map(|n| n.written(&doc)) else {
        panic!("a profile first");
    };
    let mut program_refs = Vec::new();
    let _ = program.try_map_slots(
        &mut |slot| {
            slot.var_reads(&mut program_refs);
            Ok::<_, ()>(slot.clone())
        },
        &mut |_, read| Ok(read.clone()),
    );
    assert!(
        program_refs.iter().any(|(v, _)| *v == var),
        "the program holds the parameter"
    );
    let Some(Node::Extrude { distance, .. }) = doc.node(extrude(&doc)) else {
        panic!("an extrude second");
    };
    let mut refs = Vec::new();
    doc.written(&editor_core::Expr::var(*distance, Dimension::Length))
        .var_reads(&mut refs);
    assert!(
        refs.iter().all(|(v, _)| *v != var),
        "the extrude must hold no parameter: {refs:?}"
    );
}

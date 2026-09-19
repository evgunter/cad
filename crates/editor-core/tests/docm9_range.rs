//! **The certified locally-valid range** — the on-demand query that
//! proves instead of sampling (`editor_core::range`).
//!
//! Everything here goes through the public doors a consumer has:
//! `derive` for the query's own document, `certified_range` for the
//! answer, the answer's accessors for reading it. Nothing reaches past
//! them and nothing edits the driver.
//!
//! # The probe this is compared against
//!
//! The sampling probe itself lives above this crate (`viewer::bounds`),
//! so what is reproduced here is its QUESTION — the failing-node set,
//! tested for growth against the value the field has now
//! ([`no_new_failure`]) — which is what "a certified range is a subset
//! of the LOCALLY-VALID RANGE" is a claim about. It is not a claim
//! about the probe's reported bracket, and the row that runs the real
//! probe beside the query is `viewer::docm9_range_vs_probe`.
//!
//! # What is NOT here
//!
//! The walk and the arm-separating predicate over leaf lists no
//! document produces are `range::tests`, in the crate: a
//! `NewFailure` needs a flip whose evidence carries a standing change,
//! and no fixture reaches one
//! (`work/props/coincidence-zone-priced-budget-at-the-floor`).
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::collections::{BTreeMap, BTreeSet};

use editor_core::analysis::{AnalysisPolicy, analyzed_box};
use editor_core::drive::{BudgetKind, DriveConfig, RefusalReason};
use editor_core::range::{
    CertifiedRange, RangeField, RangeRefusal, RangeSeed, RangeSide, certified_range, derive,
};
use editor_core::{
    CancelToken, Dimension, Distribution, DocEdit, DocParam, EvalOptions, Evaluation, Expr,
    LoopProgram, Node, NodeResult, ParamName, PatternKind, ProfileDoc, ProfileProgram,
    RecipeNodeId, SlotId, StableName, evaluate,
};

use fixture::{Recorder, tol};

fn name(n: &str) -> ParamName {
    ParamName::new(n)
}

fn lit(v: f64) -> Expr {
    Expr::literal(v, Dimension::Length).expect("finite length literal")
}

fn scalar(v: f64) -> Expr {
    Expr::literal(v, Dimension::Scalar).expect("finite scalar literal")
}

fn param(n: &str) -> Expr {
    Expr::param(name(n), Dimension::Length)
}

/// The drive's budgets, both of them the caller's: the query is on
/// demand, so what it spends is what it was asked to spend.
fn budget(max_depth: u32, max_leaves: usize) -> DriveConfig {
    DriveConfig {
        max_depth,
        max_leaves,
        ..DriveConfig::default()
    }
}

fn frame(r: &mut Recorder) -> RecipeNodeId {
    r.insert(Node::Datum(editor_core::Datum::Frame {
        origin: [lit(0.0), lit(0.0), lit(0.0)],
        u: [scalar(1.0), scalar(0.0), scalar(0.0)],
        v: [scalar(0.0), scalar(1.0), scalar(0.0)],
    }))
}

fn declare(r: &mut Recorder, n: &str, value: f64) {
    r.push(DocEdit::SetDocParam {
        name: name(n),
        value: DocParam::continuous(Dimension::Length, value),
    });
}

fn unit_square() -> LoopProgram {
    LoopProgram::polygon([(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)])
        .expect("finite square corners")
}

/// **The branch fixture**: a unit square extruded by a document
/// parameter.
///
/// Through zero the extrusion runs the other way: every node still
/// builds and the body is a perfectly good solid, on a branch the
/// witness's recorded decisions do not describe. That is the
/// difference between "nothing new fails", which the probe asks, and
/// "nothing decides differently", which a certificate asserts.
fn slab(depth: f64) -> ProfileDoc {
    let mut r = Recorder::new();
    declare(&mut r, "depth", depth);
    let f = frame(&mut r);
    let p = r.insert(Node::Profile(ProfileProgram {
        plane: f,
        loops: vec![unit_square()],
    }));
    r.insert(Node::Extrude {
        profile: p,
        distance: param("depth"),
    });
    r.doc
}

/// The same document with the extrusion distance left as a bare
/// LITERAL — a field with no name at all until the query derives one.
fn slab_slot(depth: f64) -> (ProfileDoc, RecipeNodeId) {
    let mut r = Recorder::new();
    let f = frame(&mut r);
    let p = r.insert(Node::Profile(ProfileProgram {
        plane: f,
        loops: vec![unit_square()],
    }));
    let e = r.insert(Node::Extrude {
        profile: p,
        distance: lit(depth),
    });
    (r.doc, e)
}

/// A slab with a SECOND, annotated parameter that the query is not
/// about — the document the one-field contract has to pin.
fn two_param_slab() -> ProfileDoc {
    let mut r = Recorder::new();
    declare(&mut r, "depth", 1.0);
    r.push(DocEdit::SetDocParam {
        name: name("side"),
        value: DocParam::continuous_with(
            Dimension::Length,
            1.0,
            Distribution::Uniform { lo: -0.1, hi: 0.1 },
        ),
    });
    let f = frame(&mut r);
    let p = r.insert(Node::Profile(ProfileProgram {
        plane: f,
        loops: vec![LoopProgram::polygon_expr([
            [lit(0.0), lit(0.0)],
            [param("side"), lit(0.0)],
            [param("side"), param("side")],
            [lit(0.0), param("side")],
        ])],
    }));
    r.insert(Node::Extrude {
        profile: p,
        distance: param("depth"),
    });
    r.doc
}

/// A document whose one interesting slot is STRUCTURAL.
fn patterned() -> (ProfileDoc, RecipeNodeId) {
    let mut r = Recorder::new();
    let f = frame(&mut r);
    let p = r.insert(Node::Profile(ProfileProgram {
        plane: f,
        loops: vec![unit_square()],
    }));
    let e = r.insert(Node::Extrude {
        profile: p,
        distance: lit(0.5),
    });
    let pat = r.insert(Node::Pattern {
        input: e,
        count: Expr::count(3),
        kind: PatternKind::Linear {
            direction: [scalar(1.0), scalar(0.0), scalar(0.0)],
            spacing: lit(2.0),
        },
    });
    (r.doc, pat)
}

fn f64_run(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        tol(),
    )
}

fn failing(ev: &Evaluation<f64>) -> BTreeSet<RecipeNodeId> {
    ev.nodes
        .iter()
        .filter(|(_, r)| matches!(r, NodeResult::Failed(_)))
        .map(|(id, _)| *id)
        .collect()
}

/// The PROBE's question at one value of one parameter: does this
/// document fail anywhere the document at the nominal did not?
fn no_new_failure(doc: &ProfileDoc, p: &str, value: f64) -> bool {
    let baseline = failing(&f64_run(doc));
    let moved = editor_core::apply(
        doc,
        &DocEdit::SetDocParamValue {
            name: name(p),
            value: editor_core::DocParamValue::Continuous(value),
        },
        tol(),
        &editor_core::RefusingReach,
    )
    .expect("a value edit applies")
    .doc;
    failing(&f64_run(&moved)).is_subset(&baseline)
}

/// Every node's standing and every published name of one f64 run —
/// the two halves of "the derived witness is the document's own".
fn standings_and_names(
    ev: &Evaluation<f64>,
) -> (
    BTreeMap<RecipeNodeId, &'static str>,
    BTreeMap<RecipeNodeId, BTreeSet<StableName>>,
) {
    let standings = ev
        .nodes
        .iter()
        .map(|(id, r)| {
            (
                *id,
                match r {
                    NodeResult::Ok(_) => "ok",
                    NodeResult::Failed(_) => "failed",
                    NodeResult::Poisoned { .. } => "poisoned",
                },
            )
        })
        .collect();
    let names = ev
        .nodes
        .iter()
        .filter_map(|(id, r)| {
            r.value()
                .map(|v| (*id, v.name_table.iter().map(|(n, _)| n.clone()).collect()))
        })
        .collect();
    (standings, names)
}

fn range_of(doc: &ProfileDoc, p: &str, seed: RangeSeed, config: &DriveConfig) -> CertifiedRange {
    certified_range(doc, &RangeField::Param(name(p)), seed, config, tol())
        .expect("the fixture has an axis and a witness that builds")
}

// -------------------------------------------------- the certified arm

/// A side with nothing to find stops at the SEED's edge, never at
/// "unbounded": outside the seed nothing was replayed.
#[test]
fn a_certified_side_stops_at_the_seeds_edge() {
    let doc = slab(1.0);
    let r = range_of(&doc, "depth", RangeSeed::symmetric(0.25), &budget(24, 2048));
    assert_eq!(*r.lo(), RangeSide::Certified { to: -0.25 });
    assert_eq!(*r.hi(), RangeSide::Certified { to: 0.25 });
    assert_eq!(r.certified_interval(), (0.75, 1.25));
    assert_eq!(r.nominal(), 1.0);
}

// ------------------------------------------------ the four-arm contract

/// **A2.** A branch change whose flipped leaf still builds every node
/// is a [`RangeSide::DecisionFlip`] and never a
/// [`RangeSide::NewFailure`]: the evidence names the predicate that
/// decided otherwise, and no node's standing moved IN THE LEAF THE
/// ARM WAS CHOSEN BY.
///
/// It pins the fold in one direction — a document that reaches
/// `DecisionFlip` cannot be answered `NewFailure`. The PREDICATE
/// itself, both directions, is pinned at the function in
/// `range::tests::a_standing_change_is_what_makes_a_flip_a_new_failure`,
/// because no document reaches `NewFailure` at all
/// (`work/props/coincidence-zone-priced-budget-at-the-floor`).
#[test]
fn a_branch_change_is_a_decision_flip_and_names_the_predicate() {
    let doc = slab(1.0);
    let r = range_of(
        &doc,
        "depth",
        RangeSeed { lo: -1.05, hi: 0.5 },
        &budget(24, 2048),
    );
    assert_eq!(*r.hi(), RangeSide::Certified { to: 0.5 });
    let RangeSide::DecisionFlip {
        certified_to,
        within,
        evidence,
    } = r.lo()
    else {
        panic!("the branch change is a decision flip, got {:?}", r.lo());
    };
    // The certificate stops short of the crossing at depth zero, and
    // the bracket holds it.
    assert!(
        -1.0 < *certified_to && *certified_to < 0.0,
        "certified to {certified_to}"
    );
    assert!(
        within.0 <= -1.0 && -1.0 <= within.1,
        "the crossing at offset -1 is inside {within:?}"
    );
    assert_eq!(within.1, *certified_to);
    // NAMED, by the driver's own engine: the extrusion's normal
    // component decided the other way.
    let named: Vec<_> = evidence
        .verdicts
        .nodes
        .values()
        .flat_map(|d| &d.flips)
        .map(|f| (f.predicate, f.from, f.to))
        .collect();
    assert!(
        named
            .iter()
            .any(|(p, from, to)| *p == "extrusion_normal_component"
                && *from == geom_core::Sign::Positive
                && *to == geom_core::Sign::Negative),
        "the flip was not named: {named:?}"
    );
    // And it is a decision flip BECAUSE no standing moved — the
    // predicate this arm is chosen by, asserted rather than implied.
    assert!(
        evidence
            .verdicts
            .nodes
            .values()
            .all(|d| d.old_status == d.new_status),
        "a standing changed, so this should have been a new failure"
    );
}

/// **A3.** A budget too small to reach the boundary answers
/// [`RangeSide::Indeterminate`], which is NOT a bound; the same
/// document with the budget raised answers the boundary.
#[test]
fn a_budget_too_small_is_indeterminate_and_never_a_bound() {
    let doc = slab(1.0);
    let seed = RangeSeed { lo: -1.05, hi: 0.5 };
    let starved = range_of(&doc, "depth", seed, &budget(1, 2048));
    let RangeSide::Indeterminate {
        certified_to,
        reason,
        ..
    } = starved.lo()
    else {
        panic!("a starved drive is indeterminate, got {:?}", starved.lo());
    };
    assert!(matches!(
        **reason,
        RefusalReason::Budget(BudgetKind::Depth { max_depth: 1 })
    ));
    // Proven ground, short of the seed's edge and short of the
    // crossing: an indeterminate side still carries what it proved.
    assert!(
        seed.lo < *certified_to && *certified_to < 0.0,
        "certified to {certified_to}"
    );
    assert!(!starved.lo().is_bound(), "indeterminate is not a bound");

    let funded = range_of(&doc, "depth", seed, &budget(24, 2048));
    assert!(funded.lo().is_bound(), "got {:?}", funded.lo());
    assert!(
        funded.lo().certified_to() < starved.lo().certified_to(),
        "more budget proves more ground"
    );
}

/// **A MEASUREMENT, and a defect's cost.** `within` is a bracket
/// around a boundary whose interior the driver did not decide, and
/// today that interior CONTAINS values at which the document does not
/// build at all: `drive::classify_replay` bisects a leaf whose node
/// definitely fails to the budget floor and prices it `Budget`
/// (`work/props/coincidence-zone-priced-budget-at-the-floor`) rather
/// than naming the failure, so the failure never reaches a leaf class
/// the walk can report.
///
/// The row exists so that the arm's documentation and the behaviour
/// cannot drift apart: a consumer rendering `within` as valid would
/// be confidently wrong, and when the PROPS row lands this assertion
/// is the one that will turn over and say so.
#[test]
fn a_decision_flips_within_contains_a_value_that_does_not_build() {
    let doc = slab(1.0);
    let r = range_of(
        &doc,
        "depth",
        RangeSeed { lo: -1.05, hi: 0.5 },
        &budget(24, 2048),
    );
    let RangeSide::DecisionFlip { within, .. } = r.lo() else {
        panic!(
            "the A2 fixture's lo side is a decision flip, got {:?}",
            r.lo()
        );
    };
    // The crossing itself is one of the sampled offsets, by
    // construction rather than by a grid's luck: at a tight eps the
    // window that refuses is narrower than any ten-way split of the
    // bracket, and a row that relied on the split would be a row about
    // the eps the gate happened to run.
    let broken: Vec<f64> = (0..=10)
        .map(|k| within.0 + (within.1 - within.0) * f64::from(k) / 10.0)
        .chain(std::iter::once(-1.0))
        .filter(|&off| !no_new_failure(&doc, "depth", r.absolute(off)))
        .collect();
    assert!(
        !broken.is_empty(),
        "`within` {within:?} is expected to hold values that do not build, and none of the \
         eleven sampled offsets did — if the driver now names such a leaf, this arm can say \
         more than it does and the docs must be re-cut"
    );
    // And the crossing itself, the value the extrude refuses, is one
    // of them: named rather than left to a sweep's luck.
    assert!(
        !no_new_failure(&doc, "depth", 0.0),
        "depth zero is a degenerate extrusion"
    );
    assert!(
        within.0 <= -1.0 && -1.0 <= within.1,
        "and it is inside the bracket: {within:?}"
    );
}

// ----------------------------------------------- against the probe

/// **The certificate is inside the LOCALLY-VALID RANGE**, on a
/// fixture with a real boundary: every value it covers passes the
/// probe's own validity test, and the frontier stops orders short of
/// the crossing rather than at it.
///
/// It is NOT a claim about the sampling probe's reported bracket —
/// those are different questions, and the row that runs the real
/// probe beside the query is
/// `viewer::docm9_range_vs_probe::the_certificate_is_inside_the_locally_valid_range_not_the_probes_bracket`.
/// This one reproduces the probe's QUESTION, which is all a crate
/// below the viewer can do.
#[test]
fn the_certificate_is_inside_the_locally_valid_range() {
    let doc = slab(1.0);
    let r = range_of(
        &doc,
        "depth",
        RangeSeed { lo: -1.05, hi: 0.5 },
        &budget(24, 2048),
    );
    let (lo, hi) = r.certified_interval();
    assert!(lo < hi);
    for i in 0..=32 {
        let v = lo + (hi - lo) * f64::from(i) / 32.0;
        assert!(
            no_new_failure(&doc, "depth", v),
            "the certificate covers {v}, which the probe's test calls invalid"
        );
    }
    // The crossing is at depth zero and the proof stops above it, at
    // the DRIVER's own floor rather than at the boundary. The bound is
    // DERIVED from the two mechanisms that set that floor — the run's
    // ambiguity band and the drive's resolution over this seed —
    // because a constant here is a claim about one eps row and the
    // gate runs three. It caught exactly that: a literal `1e-6` passed
    // at the shipped eps and red at 1e-6 and 1e-12.
    let resolution = (1.05 + 0.5) / 2f64.powi(24);
    assert!(
        lo > 0.0,
        "the certificate must not reach the crossing: {lo}"
    );
    assert!(
        lo < 100.0 * resolution.max(tol().eps()),
        "the frontier is the driver's stopping point, not the boundary: {lo}"
    );
}

/// **A4.** Validity is not monotone, and the query says so by
/// stopping: the far branch is valid to the probe and is NOT in the
/// certificate. A seed placed on that branch certifies it.
#[test]
fn the_first_boundary_stops_the_walk_and_a_seed_on_the_island_certifies_it() {
    let doc = slab(1.0);
    let r = range_of(
        &doc,
        "depth",
        RangeSeed { lo: -2.0, hi: 0.5 },
        &budget(24, 2048),
    );
    assert!(r.lo().is_bound(), "got {:?}", r.lo());
    assert!(
        r.certified_interval().0 > 0.0,
        "the certificate must stop at the crossing, not leap it"
    );
    assert!(
        no_new_failure(&doc, "depth", -1.0),
        "the far branch is valid to the probe"
    );

    let island = slab(-1.0);
    let on_it = range_of(
        &island,
        "depth",
        RangeSeed::symmetric(0.25),
        &budget(24, 2048),
    );
    assert_eq!(*on_it.lo(), RangeSide::Certified { to: -0.25 });
    assert_eq!(*on_it.hi(), RangeSide::Certified { to: 0.25 });
}

// ------------------------------------------------- purity and identity

/// **A5.** The query writes nothing: the input document is
/// bit-identical and serializes identically after a range is taken.
#[test]
fn the_query_does_not_touch_the_input_document() {
    let (doc, node) = slab_slot(1.0);
    let before = doc.clone();
    let before_text = editor_core::persist::save(&doc, &[], tol()).expect("the fixture saves");
    let _ = certified_range(
        &doc,
        &RangeField::Slot {
            node,
            slot: SlotId::Distance,
        },
        RangeSeed::symmetric(0.25),
        &budget(24, 2048),
        tol(),
    )
    .expect("the slot widens");
    assert!(doc.bit_eq(&before));
    assert_eq!(
        editor_core::persist::save(&doc, &[], tol()).expect("the fixture saves"),
        before_text
    );
}

/// **A5.** The derived document's f64 build IS the input's: the
/// synthetic parameter enters the environment through the same
/// `from_f64` the literal did, so every node's standing and every
/// published name match.
#[test]
fn the_derived_witness_is_the_documents_own_build() {
    let (doc, node) = slab_slot(1.0);
    let derived = derive(
        &doc,
        &RangeField::Slot {
            node,
            slot: SlotId::Distance,
        },
        RangeSeed::symmetric(0.25),
        tol(),
    )
    .expect("the slot widens");
    let (mine, theirs) = (
        standings_and_names(&f64_run(&doc)),
        standings_and_names(&f64_run(&derived.doc)),
    );
    assert_eq!(mine.0, theirs.0, "a node's standing moved");
    assert_eq!(mine.1, theirs.1, "a published name moved");
    assert!(!mine.0.is_empty());
}

/// **C3.** The synthetic parameter's nominal is the literal's bits,
/// and the slot names it.
#[test]
fn the_slot_rewrite_is_exact() {
    let literal = 1.0_f64 + 2.0_f64.powi(-40);
    let (doc, node) = slab_slot(literal);
    let derived = derive(
        &doc,
        &RangeField::Slot {
            node,
            slot: SlotId::Distance,
        },
        RangeSeed::symmetric(0.25),
        tol(),
    )
    .expect("the slot widens");
    assert_eq!(derived.nominal.to_bits(), literal.to_bits());
    let Some(DocParam::Continuous { value, dim, .. }) = derived.doc.params().get(&derived.axis)
    else {
        panic!("the derived document declares the synthetic parameter");
    };
    assert_eq!(value.to_bits(), literal.to_bits());
    assert_eq!(*dim, Dimension::Length);
    assert_eq!(
        derived
            .doc
            .node(node)
            .and_then(|n| n.expr(SlotId::Distance)),
        Some(&Expr::param(derived.axis.clone(), Dimension::Length))
    );
    // The input document declared no parameter at all; the derived one
    // declares exactly the query's.
    assert!(doc.params().is_empty());
    assert_eq!(derived.doc.params().len(), 1);
}

/// **A6.** A parameter field boxes directly: no rewrite, no synthetic
/// name, and the same four arms.
#[test]
fn a_parameter_field_boxes_directly() {
    let doc = slab(1.0);
    let derived = derive(
        &doc,
        &RangeField::Param(name("depth")),
        RangeSeed::symmetric(0.25),
        tol(),
    )
    .expect("the parameter boxes");
    assert_eq!(derived.axis, name("depth"));
    assert_eq!(derived.doc.params().len(), doc.params().len());
    assert_eq!(derived.doc.order(), doc.order());
    assert_eq!(derived.nominal, 1.0);
    let r = range_of(&doc, "depth", RangeSeed::symmetric(0.25), &budget(24, 2048));
    assert!(matches!(r.lo(), RangeSide::Certified { .. }));
}

/// **A6.** A slot the rewrite cannot name refuses TYPED, at the door,
/// before anything is driven.
#[test]
fn a_slot_the_rewrite_cannot_name_refuses_typed() {
    let seed = RangeSeed::symmetric(0.25);
    let (doc, pattern) = patterned();
    assert_eq!(
        derive(
            &doc,
            &RangeField::Slot {
                node: pattern,
                slot: SlotId::Count,
            },
            seed,
            tol()
        ),
        Err(RangeRefusal::StructuralSlot {
            node: pattern,
            slot: SlotId::Count
        })
    );
    // A slot already driven by an expression: naming it would shadow
    // the expression, so the query refuses instead.
    let driven = slab(1.0);
    let extrude = *driven.order().last().expect("the extrude is last");
    assert_eq!(
        derive(
            &driven,
            &RangeField::Slot {
                node: extrude,
                slot: SlotId::Distance
            },
            seed,
            tol()
        ),
        Err(RangeRefusal::SlotIsNotALiteral {
            node: extrude,
            slot: SlotId::Distance
        })
    );
    // A slot the node does not carry, and a node the document does not
    // carry.
    assert_eq!(
        derive(
            &driven,
            &RangeField::Slot {
                node: extrude,
                slot: SlotId::Radius
            },
            seed,
            tol()
        ),
        Err(RangeRefusal::UnknownSlot {
            node: extrude,
            slot: SlotId::Radius
        })
    );
    let ghost = RecipeNodeId(9999);
    assert_eq!(
        derive(
            &driven,
            &RangeField::Slot {
                node: ghost,
                slot: SlotId::Distance
            },
            seed,
            tol()
        ),
        Err(RangeRefusal::UnknownNode { node: ghost })
    );
    assert_eq!(
        derive(&driven, &RangeField::Param(name("nope")), seed, tol()),
        Err(RangeRefusal::NotAContinuousParam {
            param: name("nope")
        })
    );
}

/// **The door's read order**: whether THIS node carries the slot is
/// asked before what the slot id means in the vocabulary.
///
/// `SlotId::Count` is structural everywhere, so a door that tested
/// that first told a caller "the count slot of node N is structural"
/// about a node with no count slot at all — the vocabulary's fact
/// where the caller asked about this document's.
#[test]
fn a_structural_slot_on_a_node_that_has_none_is_an_unknown_slot() {
    let (doc, node) = slab_slot(1.0);
    assert!(
        doc.node(node).and_then(|n| n.expr(SlotId::Count)).is_none(),
        "an extrude carries no Count slot"
    );
    assert_eq!(
        derive(
            &doc,
            &RangeField::Slot {
                node,
                slot: SlotId::Count
            },
            RangeSeed::symmetric(0.25),
            tol()
        ),
        Err(RangeRefusal::UnknownSlot {
            node,
            slot: SlotId::Count
        })
    );
    // And a node that DOES carry it still refuses structural — the
    // reorder narrows the first door, it does not open the second.
    let (patterned_doc, pattern) = patterned();
    assert!(
        patterned_doc
            .node(pattern)
            .and_then(|n| n.expr(SlotId::Count))
            .is_some()
    );
    assert_eq!(
        derive(
            &patterned_doc,
            &RangeField::Slot {
                node: pattern,
                slot: SlotId::Count
            },
            RangeSeed::symmetric(0.25),
            tol()
        ),
        Err(RangeRefusal::StructuralSlot {
            node: pattern,
            slot: SlotId::Count
        })
    );
}

/// The synthetic parameter's name is the query's, and a document that
/// has already taken it is refused rather than quietly widened
/// through somebody else's parameter.
#[test]
fn a_taken_synthetic_name_refuses() {
    let (doc, node) = slab_slot(1.0);
    let taken = format!(
        "query:certified-range:{}:{}",
        node.0,
        SlotId::Distance.label()
    );
    let doc = editor_core::apply(
        &doc,
        &DocEdit::SetDocParam {
            name: name(&taken),
            value: DocParam::continuous(Dimension::Length, 3.0),
        },
        tol(),
        &editor_core::RefusingReach,
    )
    .expect("the parameter declares")
    .doc;
    assert_eq!(
        derive(
            &doc,
            &RangeField::Slot {
                node,
                slot: SlotId::Distance
            },
            RangeSeed::symmetric(0.25),
            tol()
        ),
        Err(RangeRefusal::SyntheticNameTaken {
            param: name(&taken)
        })
    );
}

/// **The three guard arms**, which the doors cannot reach and which
/// stay anyway: what a reader would be told if a driver or analysis
/// change ever fired one.
///
/// `SeedIsNotTheAnalyzedAxis` and `MoreThanOneAxisVaries` are
/// unreachable through [`derive`] (whose `Band` states the seed's own
/// offsets, and which clears every other spread), and
/// `LeavesAreNotAPartition` is unreachable through the driver
/// (`ParamBox::split` refuses a midpoint on an endpoint, so the halves
/// tile). The walk-side behaviour is pinned at the function in
/// `range::tests`; what is pinned here is that each refusal RENDERS
/// both of the facts its payload carries, since a guard that fires
/// with half a message costs exactly the debugging it exists to save.
#[test]
fn the_guard_refusals_render_what_they_carry() {
    let axis = RangeRefusal::SeedIsNotTheAnalyzedAxis {
        analyzed: (-0.5, 0.5),
        asked: (-0.25, 0.75),
    }
    .to_string();
    for fragment in ["-0.5", "0.5", "-0.25", "0.75"] {
        assert!(axis.contains(fragment), "{axis:?} omits {fragment}");
    }
    let axes = RangeRefusal::MoreThanOneAxisVaries { varying: 3 }.to_string();
    assert!(axes.contains('3'), "{axes:?} omits the count");
    let gap = RangeRefusal::LeavesAreNotAPartition { at: (1.5, 2.5) }.to_string();
    for fragment in ["1.5", "2.5"] {
        assert!(gap.contains(fragment), "{gap:?} omits {fragment}");
    }
}

/// **The answer carries its condition.** The derivation holds every
/// other annotated parameter at its nominal, so the names it pinned
/// ride on the certificate for a consumer to state.
#[test]
fn the_answer_names_the_parameters_it_pinned() {
    let doc = two_param_slab();
    let r = range_of(&doc, "depth", RangeSeed::symmetric(0.1), &budget(24, 64));
    assert_eq!(r.pinned(), [name("side")]);
    // The axis itself is never pinned: it did not lose a spread, it
    // was given one.
    assert!(!r.pinned().contains(&name("depth")));
    // A document with no other spread to drop pins nothing, which is
    // not the same claim as "nothing was checked".
    let plain = range_of(
        &slab(1.0),
        "depth",
        RangeSeed::symmetric(0.1),
        &budget(24, 64),
    );
    assert!(plain.pinned().is_empty());
}

/// A seed that does not bracket the field's current value refuses: the
/// witness is taken there, so a range around some other point is a
/// range of a different document.
#[test]
fn a_seed_that_does_not_bracket_the_nominal_refuses() {
    let doc = slab(1.0);
    for (lo, hi) in [
        (0.1, 0.5),
        (-0.5, -0.1),
        (0.0, 0.0),
        (f64::NAN, 1.0),
        (-1.0, f64::INFINITY),
    ] {
        // Compared on BITS, because a NaN seed is one of the cases and
        // `NaN != NaN` would make the equality pass for the wrong
        // reason — or, here, fail for one.
        let Err(RangeRefusal::SeedNotABracket { lo: glo, hi: ghi }) = derive(
            &doc,
            &RangeField::Param(name("depth")),
            RangeSeed { lo, hi },
            tol(),
        ) else {
            panic!("seed [{lo}, {hi}] must refuse");
        };
        assert_eq!((glo.to_bits(), ghi.to_bits()), (lo.to_bits(), hi.to_bits()));
    }
}

/// **`RangeSeed::symmetric` is infallible, and the invariant still
/// holds at one door.** A negative, zero or non-finite half-width
/// builds a seed the derivation refuses, naming the offsets it was
/// handed — which is why the constructor does not refuse a second
/// time under a second spelling.
#[test]
fn a_symmetric_seed_of_a_bad_half_width_refuses_at_the_derivation() {
    let doc = slab(1.0);
    for w in [-1.0, 0.0, f64::NAN, f64::INFINITY] {
        let seed = RangeSeed::symmetric(w);
        let out = derive(&doc, &RangeField::Param(name("depth")), seed, tol());
        assert!(
            matches!(out, Err(RangeRefusal::SeedNotABracket { .. })),
            "symmetric({w}) must refuse at the door, got {out:?}"
        );
    }
    assert_eq!(
        RangeSeed::symmetric(0.25),
        RangeSeed {
            lo: -0.25,
            hi: 0.25
        }
    );
}

// ------------------------------------------- the seed reaches the axis

/// The seed IS the analyzed axis, bit for bit — the stop-clause check
/// the query makes before it drives anything.
#[test]
fn the_seed_reaches_the_driver_as_the_analyzed_axis() {
    let doc = slab(1.0);
    let seed = RangeSeed { lo: -0.3, hi: 0.7 };
    let derived =
        derive(&doc, &RangeField::Param(name("depth")), seed, tol()).expect("the parameter boxes");
    let analyzed = analyzed_box(&derived.doc, &AnalysisPolicy::default());
    let axis = analyzed.get(&derived.axis).expect("the axis is there");
    assert_eq!(axis.offsets.lo.to_bits(), seed.lo.to_bits());
    assert_eq!(axis.offsets.hi.to_bits(), seed.hi.to_bits());
    assert_eq!(
        axis.distribution,
        Some(Distribution::Band {
            lo: seed.lo,
            hi: seed.hi
        })
    );
}

/// The query's contract is ONE field: every other parameter of the
/// derived document is pinned at its nominal, whatever the input
/// declared.
#[test]
fn every_other_parameter_is_pinned() {
    let doc = two_param_slab();
    assert_eq!(
        analyzed_box(&doc, &AnalysisPolicy::default())
            .varying()
            .count(),
        1,
        "the fixture's OTHER parameter is the annotated one"
    );
    let derived = derive(
        &doc,
        &RangeField::Param(name("depth")),
        RangeSeed::symmetric(0.1),
        tol(),
    )
    .expect("the parameter boxes");
    let analyzed = analyzed_box(&derived.doc, &AnalysisPolicy::default());
    let varying: Vec<&ParamName> = analyzed.varying().map(|(n, _)| n).collect();
    assert_eq!(varying, vec![&name("depth")]);
    assert!(
        matches!(
            derived.doc.params().get(&name("side")),
            Some(DocParam::Continuous {
                distribution: None,
                ..
            })
        ),
        "the other parameter keeps its value and loses its spread"
    );
}

/// The stop clause's third condition, checked on a real drive: the
/// one-axis leaves tile the seed end to end, with no gap and no
/// overlap, so the walk has something to walk.
#[test]
fn the_one_axis_leaves_tile_the_seed() {
    let doc = slab(1.0);
    let seed = RangeSeed { lo: -1.05, hi: 0.5 };
    let derived =
        derive(&doc, &RangeField::Param(name("depth")), seed, tol()).expect("the parameter boxes");
    let analyzed = analyzed_box(&derived.doc, &AnalysisPolicy::default());
    let verdict = editor_core::drive::drive(&derived.doc, &analyzed, &budget(24, 512), tol())
        .expect("the nominal builds");
    let mut spans: Vec<(f64, f64)> = verdict
        .certified()
        .iter()
        .map(|l| l.box_.clone())
        .chain(verdict.refused().iter().map(|l| l.box_.clone()))
        .map(|b| {
            let varying: Vec<_> = b.varying().map(|(n, lo, hi)| (n.clone(), lo, hi)).collect();
            assert_eq!(varying.len(), 1, "a one-axis drive varies one axis");
            assert_eq!(varying[0].0, derived.axis);
            (varying[0].1, varying[0].2)
        })
        .collect();
    spans.sort_by(|a, b| a.0.total_cmp(&b.0));
    assert_eq!(spans.first().expect("some leaf").0, seed.lo);
    assert_eq!(spans.last().expect("some leaf").1, seed.hi);
    for pair in spans.windows(2) {
        assert_eq!(pair[0].1, pair[1].0, "a gap or an overlap at {pair:?}");
    }
    assert!(verdict.receipt().holds());
}

//! **DOCM-2 under the Interval lane** — A7's lane rows: the
//! `part_select` corpus document evaluates at `Interval` with a
//! WIDENED parameter the split's body reads, and the read member's
//! body is the member's own, read off the family's value at the same
//! lane;
//! and the amendment's `Sym<Interval>` pin of the exact document.
//!
//! The widening is scaled to the row's ε because the hosted matrix
//! runs this row at every ε row and the widened split must certify at
//! each. The floor is MEASURED by the ladder row below, not inferred
//! from DOCM-1's extrude floor: at the default ε the split of a
//! widened box escalates at ε/10, and every rung from ε/16 down
//! certifies; the assertion row sits two rungs under that floor.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use crate::corpus;

use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use editor_core::drive::{DEFAULT_SYM_MAX_DEGREE, DEFAULT_SYM_MAX_TERMS};
use editor_core::{
    CancelToken, Dimension, Distribution, DocEdit, EvalOptions, Evaluation, FreeVar, Node,
    ProfileDoc, RecipeNodeId, SplitSide, UnitSym, ValuePayload, VarName, apply, evaluate,
};
use geom_core::{Bounds, Decide, Interval, SymBudget, Tol};
use topo::Body;

fn run(
    doc: &ProfileDoc,
    prior: Option<&Evaluation<Interval>>,
    opts: &EvalOptions,
) -> Evaluation<Interval> {
    evaluate::<Interval>(doc, prior, &CancelToken::new(), opts, Tol::witness())
}

/// Every description of a body as sorted text (`docm2_part`'s
/// instrument, at the lane scalar).
fn bits<T: Decide + core::fmt::Debug>(b: &Body<T>) -> Vec<String> {
    let mut v: Vec<String> = vec![format!(
        "counts f{} e{} v{}",
        b.faces().count(),
        b.edges().count(),
        b.vertices().count()
    )];
    let mut s: Vec<String> = b.surfaces().map(|(_, s)| format!("S {s:?}")).collect();
    let mut c: Vec<String> = b.curves().map(|(_, c)| format!("C {c:?}")).collect();
    let mut p: Vec<String> = b.points().map(|(_, p)| format!("P {p:?}")).collect();
    s.sort();
    c.sort();
    p.sort();
    v.extend(s);
    v.extend(c);
    v.extend(p);
    v
}

/// The corpus document with its height parameter WIDENED to ±`width`.
fn widened_document(width: f64) -> ProfileDoc {
    let cd = corpus::part_select::document();
    apply(
        &cd.doc,
        &DocEdit::DefineVar {
            var: VarName::from_static(corpus::part_select::H).into(),
            def: editor_core::VarDecl::Free(FreeVar::Continuous {
                dim: Dimension::Length,
                value: 1.0,
                display_unit: UnitSym::canonical_for(Dimension::Length),
                distribution: Some(Distribution::Uniform {
                    lo: -width,
                    hi: width,
                }),
            }),
            fresh: Vec::new(),
        },
        Tol::witness(),
    )
    .expect("the parameter re-declares with a distribution")
    .doc
}

/// Every placement of `doc` whose source is an indexed read, with the
/// family it reads.
fn parts(doc: &ProfileDoc) -> Vec<(RecipeNodeId, RecipeNodeId)> {
    doc.ids()
        .iter()
        .filter_map(|&id| match doc.node(id) {
            Some(Node::Place { source, .. }) if source.is_member() => {
                Some((id, doc.operation_of(source.read)?))
            }
            _ => None,
        })
        .collect()
}

/// **The read member's body IS the body read off the value at the
/// lane** — the same `Arc`, and description for description.
fn assert_parts_are_their_bodies(ev: &Evaluation<Interval>, doc: &ProfileDoc, label: &str) {
    let found = parts(doc);
    assert_eq!(found.len(), 1, "{label}: one member read");
    for (id, of) in found {
        let ValuePayload::Body(part) = &ev.value(id).expect("the copy").payload else {
            panic!("{label}: a body value");
        };
        let read: &Arc<Body<Interval>> = match &ev.value(of).expect("the input").payload {
            ValuePayload::Instances(v) => v[1].shared(),
            other => panic!("{label}: {}", other.kind_name()),
        };
        assert!(
            Arc::ptr_eq(part.shared(), read),
            "{label}: node {}'s Arc",
            id.0
        );
        assert_eq!(bits(part.body()), bits(read), "{label}: node {}", id.0);
    }
}

/// The widened evaluation at `width`: the nominal run, then the
/// widened run threaded through it.
fn widened_runs(width: f64) -> (ProfileDoc, Evaluation<Interval>, Evaluation<Interval>) {
    let doc = widened_document(width);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    // The prior is the drive's witness: the box collapsed to the
    // nominal, so it elaborates under the widened run's (guided) lift
    // and the memo compares like with like (`ParamBox::at_nominal`).
    let nominal = run(
        &doc,
        None,
        &EvalOptions {
            param_box: Some(Arc::new(ParamBox::of(&analyzed).at_nominal())),
            ..EvalOptions::default()
        },
    );
    let opts = EvalOptions {
        param_box: Some(Arc::new(ParamBox::of(&analyzed))),
        ..EvalOptions::default()
    };
    let widened = run(&doc, Some(&nominal), &opts);
    (doc, nominal, widened)
}

/// The node of `doc` matching `pick`, by evaluation order.
fn node_where(
    doc: &ProfileDoc,
    pick: impl Fn(&editor_core::Node<editor_core::ProfileProgram>) -> bool,
) -> RecipeNodeId {
    *doc.ids()
        .iter()
        .find(|id| doc.node(**id).is_some_and(&pick))
        .expect("the document carries the node")
}

/// What one rung of the width ladder must show.
enum Rung {
    /// Every node green.
    Green,
    /// Exactly this node fails, escalating on this predicate (the
    /// rest poisoned through it or green).
    Escalates {
        node: RecipeNodeId,
        predicate: &'static str,
    },
}

/// **The width ladder, asserted rung by rung** — the measurement
/// behind the row below, held at every ε row the matrix runs
/// (enclosures scale with ε, so the rungs are the same at each). A
/// plain-`Interval` extrude of a widened height certifies at ε/10
/// (DOCM-1's measurement of record); the SPLIT of that box does not:
/// at ε/10 its section-edge carrier certification against the mapped
/// source escalates (`carrier_matches_mapped_source`, an enclosure a
/// few parts per million over the band's zero). At ε/16 the split
/// certifies and the union of its halves does not: the glue door reads
/// the cut faces' plane offset in band (`bool_plane_offset`, an
/// enclosure across zero), so the pair is not glued, and the corner that
/// reads it one carrier refuses with that reading, the one a narrower
/// width clears: ε/32 and every narrower rung certify. Each
/// rung is asserted, so a floor that moves — up or down — reds this
/// row naming the rung.
#[test]
fn a7_the_width_ladder_of_the_split_of_a_widened_box() {
    let e = Tol::witness().eps();
    let doc = widened_document(e);
    let split = node_where(&doc, |n| matches!(n, Node::Split { .. }));
    let union = node_where(&doc, |n| matches!(n, Node::Union { .. }));
    let ladder: [(u32, Rung); 8] = [
        (
            10,
            Rung::Escalates {
                node: split,
                predicate: "carrier_matches_mapped_source",
            },
        ),
        (
            16,
            Rung::Escalates {
                node: union,
                predicate: "bool_plane_offset",
            },
        ),
        (32, Rung::Green),
        (64, Rung::Green),
        (128, Rung::Green),
        (256, Rung::Green),
        (1024, Rung::Green),
        (4096, Rung::Green),
    ];
    for (divisor, want) in ladder {
        let width = e / f64::from(divisor);
        let (_, _, widened) = widened_runs(width);
        let bad = corpus::failures(&widened);
        println!(
            "DOCM2-A7-LADDER eps/{divisor}: {}",
            if bad.is_empty() {
                "green".to_owned()
            } else {
                bad.join(" | ")
            }
        );
        match want {
            Rung::Green => assert!(bad.is_empty(), "eps/{divisor} certifies: {bad:?}"),
            Rung::Escalates { node, predicate } => {
                let failed: Vec<&String> = bad.iter().filter(|s| s.contains("FAILED")).collect();
                assert_eq!(
                    failed.len(),
                    1,
                    "eps/{divisor}: exactly one node fails, the rest poisoned: {bad:?}"
                );
                assert!(
                    failed[0].starts_with(&format!("{node:?} FAILED"))
                        && failed[0].contains("Escalated")
                        && failed[0].contains(predicate),
                    "eps/{divisor}: node {} escalates on {predicate}: {}",
                    node.0,
                    failed[0]
                );
            }
        }
    }
}

/// The divisor of ε the assertion row is pinned at — two rungs under
/// the floor the ladder measures at the default ε.
const A7_DIVISOR: f64 = 128.0;

/// **A7 — the corpus document at `Interval`, nominal and widened.** The
/// height the split's body reads is widened by ε/`A7_DIVISOR`; the
/// split and the union of its halves recompute through the upstream
/// key; the read member's body is the member's own at the lane.
#[test]
fn a7_the_corpus_document_evaluates_at_interval_with_a_widened_height() {
    let width = Tol::witness().eps() / A7_DIVISOR;
    let doc = widened_document(width);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    // The prior is the drive's witness: the box collapsed to the
    // nominal, so it elaborates under the widened run's (guided) lift
    // and the memo compares like with like (`ParamBox::at_nominal`).
    let nominal = run(
        &doc,
        None,
        &EvalOptions {
            param_box: Some(Arc::new(ParamBox::of(&analyzed).at_nominal())),
            ..EvalOptions::default()
        },
    );
    assert!(
        corpus::failures(&nominal).is_empty(),
        "nominal: {:?}",
        corpus::failures(&nominal)
    );
    assert_parts_are_their_bodies(&nominal, &doc, "nominal");

    let opts = EvalOptions {
        param_box: Some(Arc::new(ParamBox::of(&analyzed))),
        ..EvalOptions::default()
    };
    let widened = run(&doc, Some(&nominal), &opts);
    assert!(
        corpus::failures(&widened).is_empty(),
        "widened by eps/{A7_DIVISOR}: {:?}",
        corpus::failures(&widened)
    );
    assert_parts_are_their_bodies(&widened, &doc, "widened");
    // The profile reads no parameter and is served from the memo; the
    // box and everything that reads it — the split (its tool plane is
    // read off the box), the union of its halves, the family, the
    // lifted member's placement — recompute.
    assert_eq!(widened.reused, 1, "the one parameter-free leaf");
    assert_eq!(widened.recomputed, doc.len() - 1);
    // The widened box reaches the halves: the above half's top cap
    // carries the width.
    let split = *doc
        .ids()
        .iter()
        .find(|id| matches!(doc.node(**id), Some(Node::Split { .. })))
        .expect("the split");
    let ValuePayload::Split {
        above: SplitSide::Body(above),
        ..
    } = &widened.value(split).expect("the split").payload
    else {
        panic!("a split with an above half");
    };
    let top = above
        .body()
        .points()
        .map(|(_, p)| p.z)
        .max_by(|a, b| a.hi().partial_cmp(&b.hi()).expect("finite"))
        .expect("points");
    assert!(
        top.hi() - top.lo() >= 1.9 * width,
        "the above half's top carries the widened height: {top:?}"
    );
}

/// **The `Sym<Interval>` pin** (the amendment, item 2): the exact
/// corpus document — whose union rejoins the two halves of one split
/// — evaluates green at the symbolic scalar, which has no bit channel
/// either.
#[test]
fn the_part_select_document_evaluates_at_sym_interval() {
    let cd = corpus::part_select::document();
    let budget = SymBudget {
        max_terms: DEFAULT_SYM_MAX_TERMS,
        max_degree: DEFAULT_SYM_MAX_DEGREE,
    };
    let (bad, _) = geom_core::sym::with_session(budget, || {
        let ev: Evaluation<geom_core::Sym<Interval>> = evaluate(
            &cd.doc,
            None,
            &CancelToken::new(),
            &EvalOptions::default(),
            Tol::witness(),
        );
        corpus::failures(&ev)
    });
    assert!(bad.is_empty(), "{bad:?}");
}

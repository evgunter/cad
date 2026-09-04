//! **DOCM-2 under the Interval lane** — A7's lane rows: the
//! `part_select` corpus document evaluates at `Interval` with a
//! WIDENED parameter the split's body reads, and each Part's body is
//! the half's or the instance's own, read off the split's or the
//! pattern's value at the same lane; and the amendment's `Sym<Interval>`
//! pin of the relaxed same-source assertions on the exact document.
//!
//! The widening is scaled to the row's ε because the hosted matrix
//! runs this row at every ε row and the widened split must certify at
//! each. The floor is MEASURED by the ladder row below, not inferred
//! from DOCM-1's extrude floor: at the default ε the split of a
//! widened box escalates at ε/10 and the union of its halves at ε/16,
//! and every rung from ε/32 down certifies; the assertion row sits two
//! rungs under that floor.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use crate::corpus;

use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use editor_core::drive::{DEFAULT_SYM_MAX_DEGREE, DEFAULT_SYM_MAX_TERMS};
use editor_core::{
    CancelToken, Dimension, Distribution, DocEdit, DocParam, EvalOptions, Evaluation, Node,
    ParamName, PartSelect, ProfileDoc, RecipeNodeId, SplitHalf, SplitSide, UnitSym, ValuePayload,
    apply, evaluate,
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
        &DocEdit::SetDocParam {
            name: ParamName::new(corpus::part_select::H),
            value: DocParam::Continuous {
                dim: Dimension::Length,
                value: 1.0,
                display_unit: UnitSym::canonical_for(Dimension::Length),
                distribution: Some(Distribution::Uniform {
                    lo: -width,
                    hi: width,
                }),
            },
        },
        Tol::witness(),
    )
    .expect("the parameter re-declares with a distribution")
    .doc
}

/// Every Part of `doc` with the node it reads and its selector.
fn parts(doc: &ProfileDoc) -> Vec<(RecipeNodeId, RecipeNodeId, PartSelect)> {
    doc.order()
        .iter()
        .filter_map(|&id| match doc.node(id) {
            Some(Node::Part { of, select }) => Some((id, *of, select.clone())),
            _ => None,
        })
        .collect()
}

/// **Each Part's body IS the body read off the value at the lane** —
/// the same `Arc`, and description for description.
fn assert_parts_are_their_bodies(ev: &Evaluation<Interval>, doc: &ProfileDoc, label: &str) {
    let found = parts(doc);
    assert_eq!(found.len(), 3, "{label}: two halves and one instance");
    for (id, of, select) in found {
        let ValuePayload::Body(part) = &ev.value(id).expect("the Part").payload else {
            panic!("{label}: a body value");
        };
        let read: &Arc<Body<Interval>> = match (&select, &ev.value(of).expect("the input").payload)
        {
            (PartSelect::SplitHalf(h), ValuePayload::Split { above, below }) => {
                let side = match h {
                    SplitHalf::Above => above,
                    SplitHalf::Below => below,
                };
                match side {
                    SplitSide::Body(b) => b,
                    SplitSide::Empty => panic!("{label}: the half holds material"),
                }
            }
            (PartSelect::Instance(_), ValuePayload::Instances(v)) => &v[1],
            other => panic!("{label}: {other:?}"),
        };
        assert!(Arc::ptr_eq(part, read), "{label}: node {}'s Arc", id.0);
        assert_eq!(bits(part), bits(read), "{label}: node {}", id.0);
    }
}

/// The widened evaluation at `width`: the nominal run, then the
/// widened run threaded through it.
fn widened_runs(width: f64) -> (ProfileDoc, Evaluation<Interval>, Evaluation<Interval>) {
    let doc = widened_document(width);
    let nominal = run(&doc, None, &EvalOptions::default());
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
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
    pick: impl Fn(&Node<editor_core::ProfileProgram>) -> bool,
) -> RecipeNodeId {
    *doc.order()
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
/// few parts per million over the band's zero); at ε/16 the split
/// certifies and the UNION of the two halves escalates
/// (`point_in_loop_arm`); ε/32 and every narrower rung certify. Each
/// rung is asserted, so a floor that moves — up or down — reds this
/// row naming the rung.
#[test]
fn a7_the_width_ladder_of_the_split_of_a_widened_box() {
    let e = Tol::witness().eps();
    let doc = widened_document(e);
    let split = node_where(&doc, |n| matches!(n, Node::Split { .. }));
    let union = node_where(&doc, |n| matches!(n, Node::Boolean { .. }));
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
                predicate: "point_in_loop_arm",
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
const A7_DIVISOR: f64 = 64.0;

/// **A7 — the corpus document at `Interval`, nominal and widened.** The
/// height the split's body reads is widened by ε/`A7_DIVISOR`; the
/// split, both halves and the union recompute through the upstream
/// key; every Part's body is the half's or the instance's own at the
/// lane.
#[test]
fn a7_the_corpus_document_evaluates_at_interval_with_a_widened_height() {
    let width = Tol::witness().eps() / A7_DIVISOR;
    let doc = widened_document(width);
    let nominal = run(&doc, None, &EvalOptions::default());
    assert!(
        corpus::failures(&nominal).is_empty(),
        "nominal: {:?}",
        corpus::failures(&nominal)
    );
    assert_parts_are_their_bodies(&nominal, &doc, "nominal");

    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
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
    // The frame, the profile, the tool plane and the declaration read
    // no parameter and are served from the memo; the box and everything
    // that reads it — the split, the halves, the union, the pattern,
    // the instance, its placement — recompute.
    assert_eq!(widened.reused, 4, "the four parameter-free leaves");
    assert_eq!(widened.recomputed, doc.len() - 4);
    // The widened box reaches the halves: the above half's top cap
    // carries the width.
    let split = *doc
        .order()
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
/// corpus document — whose union rejoins two pieces carrying one
/// pass-through source — evaluates green at the symbolic scalar, which
/// has no bit channel either.
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

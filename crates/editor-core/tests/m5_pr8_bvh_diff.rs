//! M5 PR 8 — the idealized/realized differential suite over the FULL
//! Band 4 corpus (PERF-PLAN §4.4; the demo-body twin lives in
//! `topo/tests/m5_pr8_bvh_diff.rs`). `EvalOptions::boolean_sweep` is
//! the runtime switch (the `parallel` mold): the same document
//! evaluates through the tree-backed and the brute-force sweep, and
//! the pins are
//!
//! 1. **Superset**: for every boolean node's operand pair, realized
//!    candidates ⊇ idealized accepted pairs (per direction);
//! 2. **Bit-equal**: the ENTIRE evaluation — every node result, keys,
//!    names, bodies — is `Debug`-byte-identical through either
//!    strategy (same bodies, same bytes: the D9 pin). The single
//!    scrubbed field is the k-telemetry verdict LOG, which records
//!    which predicates ran — pruning exists to run fewer (see
//!    [`scrubbed`]); every decision that produced the result is
//!    identical;
//! 3. **Planted degradation** (in the topo twin — the seam lives at
//!    the sweep door): a too-small box loses its pairs and the
//!    comparator catches it.
//!
//! Rides the existing corpus lanes (`cargo test --workspace` at every
//! ε row; the interval feature adds the `Interval` twin below) — no
//! new hosted CI row.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod corpus;
mod fixture;

use std::collections::BTreeSet;

use corpus::{body_of, documents};
use editor_core::{CancelToken, EvalOptions, Evaluation, Node, evaluate};
use geom_core::Tol;
use topo::{SweepStrategy, SweepTrace, sweep_traces};

fn eval_with<T>(doc: &editor_core::ProfileDoc, strategy: SweepStrategy) -> Evaluation<T>
where
    T: editor_core::EvalScalar,
{
    let opts = EvalOptions {
        boolean_sweep: strategy,
        ..EvalOptions::default()
    };
    evaluate::<T>(doc, None, &CancelToken::new(), &opts, Tol::witness())
}

type Pair = (topo::EdgeKey, topo::FaceKey);

/// A node result's `Debug` form MINUS the verdict log. The log is the
/// one field that legitimately differs between sweep strategies: it
/// records every predicate decision the op MADE, and pruning exists
/// precisely to make fewer of them (the idealized path decides
/// `bool_vertex_face_side` on pairs the tree never examines). The
/// RESULT — bodies, contacts, names, keys — is what D9 pins
/// bit-equal; the log is telemetry about the evaluation path.
fn scrubbed<T: geom_core::Decide>(nr: Option<&editor_core::NodeResult<T>>) -> String {
    match nr {
        Some(editor_core::NodeResult::Ok(v)) => format!(
            "Ok(payload: {:?}, table: {:?}, witness: {:?}, content: {:?}, naming: {:?})",
            v.payload, v.name_table, v.witness, v.content_key, v.naming_key
        ),
        other => format!("{other:?}"),
    }
}

/// Idealized-accepted pairs missing from the realized candidate set.
fn missing_pairs(realized: &SweepTrace, idealized: &SweepTrace) -> Vec<Pair> {
    let examined: BTreeSet<Pair> = realized.examined.iter().copied().collect();
    idealized
        .accepted
        .iter()
        .copied()
        .filter(|p| !examined.contains(p))
        .collect()
}

/// Pin 2 over the whole corpus: tree-backed and brute-force
/// evaluations of every document are byte-identical, node for node.
#[test]
fn corpus_evaluations_bit_equal_realized_vs_idealized() {
    for d in documents() {
        let real: Evaluation<f64> = eval_with(&d.doc, SweepStrategy::Realized);
        let ideal: Evaluation<f64> = eval_with(&d.doc, SweepStrategy::Idealized);
        // Evaluation-level fields (fix-pass item 5): everything but
        // the per-run identity token (epoch, minted per options) and
        // the per-node results (scrubbed-compared below).
        assert_eq!(real.order, ideal.order, "{}: evaluation order", d.name);
        assert_eq!(real.outcome, ideal.outcome, "{}: outcome", d.name);
        assert_eq!(real.recomputed, ideal.recomputed, "{}: recomputed", d.name);
        assert_eq!(real.reused, ideal.reused, "{}: reused", d.name);
        assert_eq!(
            format!("{:?}", real.appearance),
            format!("{:?}", ideal.appearance),
            "{}: appearance resolution",
            d.name
        );
        // Node-by-node so a failure names the first divergent node
        // (and the first divergent byte region) instead of dumping
        // two whole evaluations.
        for &id in &real.order {
            let (r, i) = (
                scrubbed(real.nodes.get(&id)),
                scrubbed(ideal.nodes.get(&id)),
            );
            if r != i {
                let at = r
                    .bytes()
                    .zip(i.bytes())
                    .position(|(x, y)| x != y)
                    .unwrap_or(r.len().min(i.len()));
                let lo = at.saturating_sub(120);
                panic!(
                    "{}: node {id:?} ({:?}-kind) diverged between sweep strategies at byte {at}:\n\
                     realized  …{}…\nidealized …{}…",
                    d.name,
                    d.doc.node(id).map(corpus::sub_kinds),
                    &r[lo..(at + 120).min(r.len())],
                    &i[lo..(at + 120).min(i.len())],
                );
            }
        }
    }
}

/// Pin 1 over every boolean node in the corpus: realized candidate
/// sets are supersets of the idealized accepted sets, both directions.
#[test]
fn corpus_boolean_operands_superset_pin() {
    let (mut booleans, mut examined, mut accepted) = (0usize, 0usize, 0usize);
    for d in documents() {
        let ev: Evaluation<f64> = eval_with(&d.doc, SweepStrategy::Realized);
        for &id in &ev.order {
            let Some(Node::Boolean { a, b, .. }) = d.doc.node(id) else {
                continue;
            };
            let (body_a, body_b) = (body_of(&ev, *a), body_of(&ev, *b));
            let (r_ab, r_ba) = sweep_traces(
                body_a,
                body_b,
                SweepStrategy::Realized,
                None,
                Tol::witness(),
            )
            .expect("realized sweep runs on green corpus operands");
            let (i_ab, i_ba) = sweep_traces(
                body_a,
                body_b,
                SweepStrategy::Idealized,
                None,
                Tol::witness(),
            )
            .expect("idealized sweep runs on green corpus operands");
            assert_eq!(
                missing_pairs(&r_ab, &i_ab),
                vec![],
                "{} node {id:?}: A→B lost accepted pairs",
                d.name
            );
            assert_eq!(
                missing_pairs(&r_ba, &i_ba),
                vec![],
                "{} node {id:?}: B→A lost accepted pairs",
                d.name
            );
            booleans += 1;
            examined += r_ab.examined.len() + r_ba.examined.len();
            accepted += i_ab.accepted.len() + i_ba.accepted.len();
        }
    }
    assert!(booleans > 0, "the corpus exercises boolean nodes");
    println!(
        "superset pin: {booleans} boolean node(s), {examined} realized candidate pair(s) \
         ⊇ {accepted} idealized accepted pair(s)"
    );
}

/// The `Interval` twin of pin 2 (the interval lane's row — PERF-PLAN
/// §4.5: differential suites run the ε rows AND the interval lane).
#[cfg(feature = "interval")]
#[test]
fn corpus_evaluations_bit_equal_at_interval() {
    use geom_core::Interval;
    for d in documents() {
        let real: Evaluation<Interval> = eval_with(&d.doc, SweepStrategy::Realized);
        let ideal: Evaluation<Interval> = eval_with(&d.doc, SweepStrategy::Idealized);
        for &id in &real.order {
            assert_eq!(
                scrubbed(real.nodes.get(&id)),
                scrubbed(ideal.nodes.get(&id)),
                "{}: interval node {id:?} diverged between sweep strategies",
                d.name
            );
        }
    }
}

//! Review lane R2 probes for DOCM-4. NOT part of the unit — these rows
//! exist to falsify the PR's claims by execution. Rows named
//! `red_*` are EXPECTED TO FAIL on this head: each one asserts the
//! behaviour a claim implies and shows the tree does not have it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;
use crate::fixture;

use std::collections::BTreeMap;

use editor_core::{
    CancelToken, ChecksConfig, DocEdit, DocumentId, EvalOptions, Evaluation,
    Node, NodeResult, ProfileDoc, RecipeNodeId, Severity, evaluate, run_checks,
};
use fixture::{insert, len, on_frame, square};
use geom_core::Tol;

// ---- fixtures, deliberately NOT the implementer's shapes ----

/// A two-root document: two disjoint squares extruded, each its own
/// product root. `gap` decides whether the two solids are separated.
fn two_root(id: DocumentId, gap: f64) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let doc = ProfileDoc::empty(id, Tol::witness());
    let (doc, p0) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 0.5)],
    );
    let (doc, e0) = insert(
        doc,
        Node::Extrude {
            profile: p0,
            distance: len(1.0),
        },
    );
    let (doc, p1) = on_frame(
        doc,
        [gap, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 0.5)],
    );
    let (doc, e1) = insert(
        doc,
        Node::Extrude {
            profile: p1,
            distance: len(1.0),
        },
    );
    let (doc, _) = fixture::step(doc, DocEdit::SetRoots { roots: vec![e0, e1] });
    (doc, vec![e0, e1])
}

fn plain(doc: &ProfileDoc, prior: Option<&Evaluation<f64>>) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        prior,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

// ---- C2, re-built independently ----

/// C2 (green expected) — MY colliding pair: a two-root document, not
/// the implementer's one-root part, built through `SetRoots` so the
/// recipe carries a doc edit as well as nodes. Same ids, same content
/// keys, same naming keys; still zero reuse.
#[test]
fn r2_c2_my_own_colliding_pair_reuses_nothing() {
    let (a, _) = two_root(DocumentId::derive("r2-c2-a"), 3.0);
    let (b, _) = two_root(DocumentId::derive("r2-c2-b"), 3.0);
    assert_ne!(a.id(), b.id());

    let ev_a = plain(&a, None);
    let cold_b = plain(&b, None);
    assert_eq!(ev_a.order, cold_b.order, "the same node ids");
    for &id in &ev_a.order {
        let (Some(NodeResult::Ok(x)), Some(NodeResult::Ok(y))) =
            (ev_a.result(id), cold_b.result(id))
        else {
            panic!("both evaluate {id:?} green");
        };
        assert_eq!(x.content_key, y.content_key, "{id:?} content key");
        assert_eq!(x.naming_key, y.naming_key, "{id:?} naming key");
    }
    // The control: A's own prior DOES serve every node, so the memo
    // would in fact have hit on every one of these ids.
    let warm_a = plain(&a, Some(&ev_a));
    assert_eq!(warm_a.reused, a.len(), "the memo hits on its own document");

    let warm_b = plain(&b, Some(&ev_a));
    assert_eq!(warm_b.reused, 0, "a foreign prior serves nothing");
    assert_eq!(warm_b.recomputed, b.len());
    assert!(warm_b.prior_ignored.is_some());
}

/// The pairing is IDENTITY only, never content (DI3 says so). Two
/// documents sharing one id but holding different geometry pass every
/// door and the memo serves across them by content key. Recorded as
/// the boundary the design chose, not as a defect.
#[test]
fn r2_same_id_different_content_is_admitted_everywhere() {
    let id = DocumentId::derive("r2-shared-id");
    let (a, _) = two_root(id, 3.0);
    let (b, _) = two_root(id, 9.0);
    let ev_a = plain(&a, None);
    let ev_b = plain(&b, Some(&ev_a));
    assert_eq!(ev_b.prior_ignored, None, "same id, admitted");
    assert!(ev_b.reused > 0, "and mined: {} nodes", ev_b.reused);
    editor_core::product(&b, &ev_a, Tol::witness()).expect("the door admits a's evaluation for b");
}

// ---- C3: the doors the sweep left ----

/// RED — `run_checks` with the separation resident `Off` answers about
/// ANOTHER document's geometry. `connectedness` runs first and reads
/// `ev.value(root)` with no pairing check (`checks.rs:567`), so the
/// report is computed from the foreign evaluation and returned `Ok`.
///
/// The two documents mint the same ids and hold DIFFERENT geometry
/// (a 0.5-half square against a 2.0-half one). The resident's counted
/// quantity is `classify_shells` over the value it read out of the
/// evaluation it was handed, so with `separation: Off` every finding
/// this returns is about `a`'s solid while the report is addressed to
/// `b`. The row asserts a refusal.
#[test]
fn red_run_checks_with_separation_off_answers_about_another_document() {
    let (a, roots) = two_root(DocumentId::derive("r2-checks-a"), 3.0);
    let (b, _) = two_root(DocumentId::derive("r2-checks-b"), 9.0);
    let ev_a = plain(&a, None);

    let cfg = ChecksConfig {
        connectedness: Severity::Warn,
        separation: editor_core::Advisory::Off,
        expected_components: BTreeMap::from([((roots[0], 0), 7)]),
    };
    let report = run_checks(&b, &ev_a, &cfg, Tol::witness());
    assert!(
        report.is_err(),
        "run_checks accepted an evaluation of another document and answered \
         {:?} — read off a's node values, not b's",
        report.map(|r| r
            .findings
            .iter()
            .map(|f| format!("{:?}", f.evidence))
            .collect::<Vec<_>>())
    );
}

/// RED — `resolve::apply_with_names` validates an edit's names against
/// a FOREIGN evaluation's tables (`resolve/mod.rs:1132`). With two
/// documents of one recipe the node ids collide, so the carve-out
/// (`eval.value(name.node).is_some()`) is satisfied and the name is
/// checked against the wrong document's table.
#[test]
fn red_apply_with_names_admits_a_foreign_evaluation() {
    let (a, _) = two_root(DocumentId::derive("r2-names-a"), 3.0);
    let (b, _) = two_root(DocumentId::derive("r2-names-b"), 3.0);
    let ev_a = plain(&a, None);
    // Any edit at all: the door reads `eval` before it reads `edit`.
    let edit = DocEdit::SetRoots {
        roots: b.roots().to_vec(),
    };
    let out = editor_core::resolve::apply_with_names(&b, &edit, &ev_a, Tol::witness());
    assert!(
        out.is_err(),
        "apply_with_names took an evaluation of document {} to validate an \
         edit to document {}",
        ev_a.document,
        b.id()
    );
}

// ---- C4: the corpus reuse figure, re-measured ----

/// The bench corpus's total `reused`, printed. Run on this head and on
/// the merge base; the two numbers must agree.
#[test]
fn r2_c4_total_reused_on_the_corpus() {
    let mut total = 0usize;
    let mut rows = Vec::new();
    for d in corpus::documents() {
        let full = corpus::eval::<f64>(&d.doc);
        let bumped = d.bumped();
        let after = evaluate::<f64>(
            &bumped,
            Some(&full),
            &CancelToken::new(),
            &EvalOptions::default(),
            Tol::witness(),
        );
        total += after.reused;
        rows.push(format!("{} {}", d.name, after.reused));
    }
    println!("R2-CORPUS-REUSED-TOTAL {total}");
    for r in &rows {
        println!("R2-CORPUS-ROW {r}");
    }
}

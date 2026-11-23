//! M4 PR 3 naming tests, part 3 (spec D5): the N4 CI invariant — the
//! name table is a function of (recipe structure, structural params,
//! predicate verdict vector) ONLY. The die's tables replay identically
//! and its names-only digest is golden, continuous-parameter motion without flips
//! leaves every table identical, and memo reuse transfers tables
//! bit-identically.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{CancelToken, EvalOptions, Evaluation, ProfileDoc, VarName, evaluate};
use fixture::{DEPTH, die, step};
use geom_core::Tol;

fn run(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

/// FNV-1a 64 over the tables' deterministic Debug encoding: node ids
/// in order, each table's rows in name order, arena keys included —
/// same recipe + same verdicts ⇒ same kill history ⇒ same keys (D9).
fn digest(ev: &Evaluation<f64>) -> u64 {
    let mut h = crate::fixture::digest::Fnv::new();
    let mut feed = |s: &str| h.feed(s.as_bytes());
    for id in &ev.order {
        feed(&format!("#{id:?}"));
        for t in crate::fixture::tables(ev, *id) {
            for (n, e) in t.iter() {
                feed(&format!("{n:?}={e:?};"));
            }
        }
    }
    h.finish()
}

/// Names-only digest (review R11): each name with its node ids spelled
/// by their place and its entry's shape (`fixture::digest::name_shapes`)
/// — no arena key, no mint id. It moves only where what the names SAY
/// moves: the role vocabulary, the entry shapes, the recipe's structure.
fn digest_names(ev: &Evaluation<f64>) -> u64 {
    let mut h = crate::fixture::digest::Fnv::new();
    for (place, id) in ev.order.iter().enumerate() {
        h.feed(format!("@node{place}").as_bytes());
        for row in crate::fixture::digest::name_shapes(ev, *id) {
            h.feed(row.as_bytes());
            h.feed(b";");
        }
    }
    h.finish()
}

/// The pinned names-only die digest ([`digest_names`]); update only on a
/// ratified naming change.
const DIE_NAMES_DIGEST: u64 = 0xb543_5c1d_c3e1_8fac;

/// **The die's tables are a function of its recipe and verdicts, and
/// its names are the ratified ones.** Two fresh evaluations of the die
/// give byte-identical tables, arena keys and ids included (D9: same
/// recipe + same verdicts ⇒ same kill history ⇒ same keys), and what
/// the names say is [`DIE_NAMES_DIGEST`].
#[test]
fn die_name_tables_are_golden() {
    let d = die();
    let ev = run(&d.doc);
    // Every node evaluated (the die is fully green).
    for id in &ev.order {
        assert!(ev.value(*id).is_some(), "die node {id:?} failed");
    }
    assert_eq!(
        digest(&ev),
        digest(&run(&d.doc)),
        "two evaluations of the die named its entities differently: a replay-identity break"
    );
    assert_eq!(
        digest_names(&ev),
        DIE_NAMES_DIGEST,
        "die NAMES-ONLY digest drifted (naming semantics changed): got {:#018x}",
        digest_names(&ev)
    );
}

#[test]
fn pip_depth_motion_without_flips_leaves_every_table_identical() {
    let d = die();
    let ev1 = run(&d.doc);
    // A dyadic, still-shallow depth: no verdict flips anywhere.
    let (doc2, _) = step(
        d.doc,
        editor_core::DocEdit::SetVarValue {
            var: VarName::from_static("pip_depth").into(),
            // 0.1875, dyadic.
            value: editor_core::FreeValue::Continuous(DEPTH * 1.5),
        },
    );
    let ev2 = evaluate::<f64>(
        &doc2,
        Some(&ev1),
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    assert!(ev2.recomputed > 0, "the edit must recompute its cone");
    let mut changed = 0usize;
    for id in &ev1.order {
        let (a, b) = (ev1.value(*id), ev2.value(*id));
        assert!(a.is_some() && b.is_some(), "both runs value {id:?}");
        if crate::fixture::tables(&ev1, *id) != crate::fixture::tables(&ev2, *id) {
            changed += 1;
        }
    }
    assert_eq!(changed, 0, "no-flip motion changed {changed} node tables");
}

#[test]
fn memo_reuse_transfers_tables_bit_identically() {
    let d = die();
    let ev1 = run(&d.doc);
    let ev2 = evaluate::<f64>(
        &d.doc,
        Some(&ev1),
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    assert_eq!(ev2.recomputed, 0, "unchanged doc must be a full memo hit");
    for id in &ev1.order {
        let (a, b) = (
            crate::fixture::tables(&ev1, *id),
            crate::fixture::tables(&ev2, *id),
        );
        // The table rides the value: reuse is the SAME Arc.
        assert!(
            a.len() == b.len() && a.iter().zip(&b).all(|(x, y)| std::ptr::eq(*x, *y)),
            "memo reuse did not transfer the table for {id:?}"
        );
    }
}

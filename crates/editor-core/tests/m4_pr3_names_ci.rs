//! M4 PR 3 naming tests, part 3 (spec D5): the N4 CI invariant — the
//! name table is a function of (recipe structure, structural params,
//! predicate verdict vector) ONLY. Golden die table digest (joins the
//! replay-identity family), continuous-parameter motion without flips
//! leaves every table identical, and memo reuse transfers tables
//! bit-identically.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use editor_core::{
    CancelToken, DocParam, EvalOptions, Evaluation, ParamName, ProfileDoc, evaluate,
};
use fixture::{DEPTH, die, step};
use geom_core::Tol;

fn run(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(doc, None, &CancelToken::new(), &EvalOptions::default(), Tol::witness())
}

/// FNV-1a 64 over the tables' deterministic Debug encoding: node ids
/// in order, each table's rows in name order. Arena keys are included
/// deliberately — same recipe + same verdicts ⇒ same kill history ⇒
/// same keys (D9); a digest drift is a replay-identity break.
fn digest(ev: &Evaluation<f64>) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    let mut feed = |s: &str| {
        for b in s.bytes() {
            h ^= u64::from(b);
            h = h.wrapping_mul(0x1000_0000_01b3);
        }
    };
    for id in &ev.order {
        feed(&format!("#{id:?}"));
        if let Some(v) = ev.value(*id) {
            for (n, e) in v.name_table.iter() {
                feed(&format!("{n:?}={e:?};"));
            }
        }
    }
    h
}

/// Names-only companion digest (review R11): entry keys excluded —
/// only node ids, names, and entry shape (unique vs tie width). A
/// drift HERE is a naming-semantics change; a drift in the full
/// digest alone is an allocation/replay change. Triage reads both.
fn digest_names(ev: &Evaluation<f64>) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    let mut feed = |s: &str| {
        for b in s.bytes() {
            h ^= u64::from(b);
            h = h.wrapping_mul(0x1000_0000_01b3);
        }
    };
    for id in &ev.order {
        feed(&format!("#{id:?}"));
        if let Some(v) = ev.value(*id) {
            for (n, e) in v.name_table.iter() {
                let shape = match e {
                    editor_core::Entry::Unique(r) => format!("u{}", r.body),
                    editor_core::Entry::Tied(c) => format!("t{}", c.len()),
                };
                feed(&format!("{n:?}={shape};"));
            }
        }
    }
    h
}

/// The pinned die digest (update ONLY on a ratified naming change —
/// this is the replay-identity family's naming member).
///
/// RE-PINNED at M4 PR 5: the die document now DECLARES its 21
/// flush pip contacts (F5 — Declare nodes between each
/// Transform/Subtract pair), so every node id downstream of the
/// first Declare shifted and the digests moved. The naming
/// VOCABULARY did not change; the fixture's authoring did (21 new
/// Declare nodes, 56 → 77).
const DIE_TABLE_DIGEST: u64 = 0x8d2e_4c61_3057_071e;

/// The pinned names-only die digest (R11 companion; see
/// [`digest_names`]). Re-pinned with `DIE_TABLE_DIGEST` (above).
const DIE_NAMES_DIGEST: u64 = 0xf154_2d1e_3b36_a26e;

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
        DIE_TABLE_DIGEST,
        "die name-table digest drifted: got {:#018x}",
        digest(&ev)
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
        editor_core::DocEdit::SetDocParam {
            name: ParamName::new("pip_depth"),
            value: DocParam::Continuous {
                dim: editor_core::Dimension::Length,
                value: DEPTH * 1.5, // 0.1875, dyadic
            },
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
        let (a, b) = (a.expect("ev1 value"), b.expect("ev2 value"));
        if a.name_table != b.name_table {
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
        let (a, b) = (ev1.value(*id).unwrap(), ev2.value(*id).unwrap());
        // The table rides the value: reuse is the SAME Arc.
        assert!(
            std::sync::Arc::ptr_eq(&a.name_table, &b.name_table),
            "memo reuse did not transfer the table for {id:?}"
        );
    }
}

//! **The face-identity census: its one home.**
//!
//! `lib.rs`'s module docs say what rule 4's precondition cannot see —
//! two faces of one scene agreeing on every [`IDENTITY_COLUMNS`] entry
//! and swapping ordinals — and deliberately do not say how many there
//! are. This file says how many, by counting them, because the count
//! is a reading of a committed artefact that a re-baseline moves and a
//! number transcribed into prose is a number nothing can check.
//!
//! Three copies of that paragraph — the module docs and two rows in
//! `work/code-quality/` — drifted apart from the file they describe.
//! The cure is the one this tree's own CI comment states for the rule
//! roster next door: a pointer cannot go stale, so there is one home
//! and everything else points at it.
//!
//! # The sweep's definition, beside its result
//!
//! Over `docs/tess-budget-data/tess-budget-baseline.csv`, read through
//! `tess_lint::parse` — so this census counts exactly what the gate
//! parses, not what a separate reader thinks the columns mean:
//!
//! * a **row** is one parsed [`Row`]: one face of one scene;
//! * a row is **sized** when it carries the Hessian-sized block
//!   (`Row::nurbs` is `Some`) — every column from `u0` on is filled;
//! * a row's **identity** is `IDENTITY_COLUMNS` as rule 4 compares it:
//!   `chart` and the sizing block's PRESENCE as text, then `u0`, `u1`,
//!   `v0`, `v1`, `nu`, `nv` as numbers;
//! * a **pair** is two rows OF ONE SCENE with equal identities. They
//!   necessarily have different ordinals (`parse` refuses a repeated
//!   `(scene, face)`), so each pair is a swap rule 4 would wave
//!   through. Counted unordered: a group of `k` equal rows is
//!   `k·(k−1)/2` pairs.
//!
//! The census is taken over the SIZED rows, because an unsized swap
//! costs rule 2 nothing — that restriction is the load-bearing one,
//! and the corpus-wide figure below is here to show how much work it
//! does rather than because anything gates on it.
//!
//! # When this test fails
//!
//! It is not a threshold and no baseline here is a target to preserve.
//! A re-cut that moves these numbers means the corpus moved: read the
//! new number, decide whether the new corpus is what you meant, and
//! write it in. The failure exists so that the paragraph in `lib.rs`
//! cannot go on describing a file it no longer describes.
//!
//! [`Row`]: tess_lint::Row
//! [`IDENTITY_COLUMNS`]: tess_lint

use std::collections::HashMap;

use tess_lint::{Nurbs, Row, parse};

/// The committed baseline, by path relative to this crate's manifest.
///
/// `include_str!` rather than a runtime read: the crate is a cargo
/// root of its own with no dependencies (`Cargo.toml`), and a missing
/// or moved baseline should be a compile error naming the path, not a
/// test that silently reads nothing. It costs this crate a build-time
/// dependency on a path outside itself, which is the price of the
/// census being checkable at all.
const BASELINE: &str = include_str!("../../../docs/tess-budget-data/tess-budget-baseline.csv");

/// One row's reading of the identity columns, in a form that can be
/// grouped: `chart` and the sizing block's presence as text, the six
/// measured entries as bit patterns.
///
/// `to_bits` and not the float, because floats are not `Hash`. It is
/// the same comparison rule 4 runs with ONE exception, and the
/// exception cannot bite here: `-0e0` and `0e0` compare equal as
/// numbers and unequal as bits, and `parse` admits no `NaN` into these
/// columns — so a sign-bit split would show up as a pair MISSING, and
/// the sweep's own assertion that the trim box is the unit square on
/// every sized row rules it out for this baseline.
type Identity = (String, bool, [u64; 6]);

fn identity(r: &Row) -> Identity {
    let n = r.nurbs;
    (
        r.chart.clone(),
        n.is_some(),
        n.map_or([0; 6], |n: Nurbs| {
            [n.u0, n.u1, n.v0, n.v1, n.nu, n.nv].map(f64::to_bits)
        }),
    )
}

/// `(pairs, rows in a group of two or more, scenes carrying a pair)`,
/// with the scene names.
fn census(rows: &[&Row]) -> (usize, usize, Vec<String>) {
    let mut groups: HashMap<(&str, Identity), usize> = HashMap::new();
    for r in rows {
        *groups.entry((r.scene.as_str(), identity(r))).or_default() += 1;
    }
    let pairs = groups.values().map(|k| k * (k - 1) / 2).sum();
    let in_group = groups.values().filter(|k| **k > 1).sum();
    let mut scenes: Vec<String> = groups
        .iter()
        .filter(|(_, k)| **k > 1)
        .map(|((s, _), _)| (*s).to_string())
        .collect();
    scenes.sort();
    scenes.dedup();
    (pairs, in_group, scenes)
}

/// The census itself. Every quantity `lib.rs` used to transcribe, and
/// the corpus it is over.
#[test]
fn the_committed_baseline_carries_this_many_indistinguishable_pairs() {
    let rows = parse(BASELINE).expect("the committed baseline parses");
    let all: Vec<&Row> = rows.iter().collect();
    let sized: Vec<&Row> = rows.iter().filter(|r| r.nurbs.is_some()).collect();

    // The corpus the census is over.
    assert_eq!(all.len(), 1306, "rows in the committed baseline");
    assert_eq!(sized.len(), 64, "of them sized");
    let sized_scenes = {
        let mut s: Vec<&str> = sized.iter().map(|r| r.scene.as_str()).collect();
        s.sort_unstable();
        s.dedup();
        s
    };
    assert_eq!(sized_scenes.len(), 12, "scenes carrying a sized face");

    // The census over the SIZED rows — the one that matters, because
    // an unsized swap costs rule 2 nothing.
    let (pairs, in_pairs, scenes) = census(&sized);
    assert_eq!(pairs, 7, "indistinguishable pairs among the sized rows");
    assert_eq!(in_pairs, 14, "sized rows sitting in such a pair");
    assert_eq!(
        scenes,
        [
            "lily/lily_leaf_b",
            "lily/lily_leaf_c",
            "lofts/loft_prism",
            "lofts/nonuniform_loft",
            "s_duct/s_duct",
        ],
        "scenes carrying at least one such pair"
    );

    // …and the same count over every row, which is what the
    // restriction to sized rows is worth: three orders of magnitude,
    // and not one of them reaches a rule.
    let (all_pairs, _, all_scenes) = census(&all);
    assert_eq!(all_pairs, 22_143, "pairs across every row");
    assert_eq!(all_scenes.len(), 70, "scenes carrying one, corpus-wide");
}

/// The other half of the paragraph: WHICH identity entries actually
/// discriminate among the sized rows, which is the honest reading of
/// how big the hole is. Five of the eight are constant there, so the
/// live pair is `nu`/`nv` alone.
#[test]
fn five_of_the_eight_identity_entries_discriminate_nothing_among_the_sized_rows() {
    let rows = parse(BASELINE).expect("the committed baseline parses");
    let sized: Vec<&Row> = rows.iter().filter(|r| r.nurbs.is_some()).collect();
    assert!(!sized.is_empty(), "the census needs sized rows to be over");

    for r in &sized {
        let n = r.nurbs.expect("filtered to sized rows");
        assert_eq!(r.chart, "nurbs", "{} face {}", r.scene, r.face);
        assert_eq!(
            [n.u0, n.u1, n.v0, n.v1],
            [0.0, 1.0, 0.0, 1.0],
            "the trim box, {} face {}",
            r.scene,
            r.face
        );
    }

    // The positive control the four constants above are worth nothing
    // without: `nu`/`nv` DO separate rows here, so the pair count
    // above is a reading of those two columns and of nothing else.
    let distinct = {
        let mut v: Vec<[u64; 2]> = sized
            .iter()
            .map(|r| {
                let n = r.nurbs.expect("filtered to sized rows");
                [n.nu.to_bits(), n.nv.to_bits()]
            })
            .collect();
        v.sort_unstable();
        v.dedup();
        v.len()
    };
    assert!(
        distinct > 1,
        "nu/nv would discriminate nothing either: {distinct} distinct"
    );
}

//! **The face-identity census: its one home.**
//!
//! `lib.rs`'s module docs say what rule 4's precondition cannot see —
//! two faces of one scene agreeing on every [`IDENTITY_COLUMNS`] entry
//! and swapping ordinals — and deliberately do not say how many there
//! are. This file says how many, by counting them, because the count
//! is a reading of a committed artefact that a re-baseline moves and a
//! number transcribed into prose is a number nothing can check.
//!
//! # The transcription sweep, and its hit list
//!
//! The pattern swept for was that paragraph's own quantities — the
//! literals `8 pairs` / `16 of` / `22,545` / `1327` / `five of the
//! eight` and their spellings with and without the thousands comma —
//! over every `*.md` and `*.rs` in the tree. **What it could not
//! match**: a copy that paraphrases the figures without repeating a
//! literal, and a copy in a non-text artefact. It also matches text
//! about other subjects entirely — a `topo` review probe's "8 pairs",
//! `SMELL-E-LOG`'s "five of the eight edited files", a GitHub issue
//! numbered 1327 — which are read out rather than counted. Four hits
//! on this paragraph:
//!
//! * `lib.rs`'s module docs — FIXED, they now point here.
//! * `work/code-quality/C15.md` — FIXED, points here.
//! * `work/code-quality/D201.md` — FIXED, points here.
//! * `work/code-quality/logs/SMELL-KPW-LOG.md` — **NOT FIXED, and
//!   deliberately.** It is a dated unit record in `logs/`, and a log
//!   entry is what the unit reported on the day it reported it. Its
//!   figures were correct against the tree it closed on; editing them
//!   now would make the record say something the unit did not say,
//!   which is a worse defect than the stale number. Frozen, not
//!   propagated: nothing cites the log for a current count.
//!
//! The cure for the three live copies is the one this tree's own CI
//! comment states for the rule roster next door: a pointer cannot go
//! stale, so there is one home and everything else points at it.
//!
//! # The sweep's definition, beside its result
//!
//! Over `docs/tess-budget-data/tess-budget-baseline.csv`, read through
//! [`parse`] — so this census counts exactly what the gate parses, not
//! what a separate reader thinks the columns mean:
//!
//! * a **row** is one parsed [`Row`]: one face of one scene;
//! * a row is **sized** when it carries the Hessian-sized block
//!   (`Row::nurbs` is `Some`) — every column from `u0` on is filled;
//! * a row's **identity** is [`identity_readings`], which is
//!   [`IDENTITY_COLUMNS`] as rule 4 compares it. It is the crate's own
//!   definition and not a copy of it: an eighth entry in that list
//!   lengthens the key here, so the census cannot go on grouping on
//!   seven columns while the gate parses eight;
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
//! # What an undetected swap COSTS, and which half is a theorem
//!
//! The pairs counted above are what the gate cannot tell apart. What
//! that is WORTH is a separate question, and its two halves are not
//! the same kind of claim:
//!
//! * **Rule 1 cannot move it — a theorem about the rule's shape.** It
//!   compares per-SCENE triangle totals, and a swap within one scene
//!   permutes the summands of one sum. No reading of any corpus can
//!   make that false, so nothing below asserts it.
//! * **Rule 2 does not move — a READING of this baseline.** It
//!   compares [`Row::recoverable`], and within every pair the
//!   committed corpus carries today the two members' `grid_cells /
//!   span_opt_cells` are bit-identical. That is a fact about a
//!   committed artefact, and a re-cut can end it, so it is asserted
//!   here rather than written down.
//!
//! `an_undetected_swap_costs_the_gate_nothing_on_the_committed_baseline`
//! puts the consequence the way the gate puts it — the swap is handed
//! to [`compare`] and the [`Report`] must come back empty — and then
//! the equality that is the margin behind it.
//!
//! **The reported side is deliberately NOT pinned.** A swap does move
//! what the report prints: `total` = `delta / worst_dev`, and
//! `worst_cert` in its last digits. Neither is gated —
//! [`tess_lint::Kind`] has five variants and not one of them reads
//! `worst_dev` — so an assertion on that movement would be a
//! threshold on an ungated column, and it would fire on a re-cut that
//! made a pair's two members AGREE, which is not a defect. The
//! asymmetry is the finding; the magnitude is a reading, and
//! `work/meter/C15.md` carries the method that re-derives it.
//!
//! # When this test fails
//!
//! It is not a threshold and no baseline here is a target to preserve.
//! A re-cut that moves these numbers means the corpus moved: read the
//! new number, decide whether the new corpus is what you meant, and
//! write it in. The failure exists so that the paragraph in `lib.rs`
//! cannot go on describing a file it no longer describes.

use std::collections::{BTreeSet, HashMap};

use tess_lint::{IDENTITY_COLUMNS, Report, Row, compare, identity_readings, parse, totals};

/// The committed baseline, by path relative to this crate's manifest.
///
/// `include_str!` rather than a runtime read: the crate is a cargo
/// root of its own with no dependencies (`Cargo.toml`), and a missing
/// or moved baseline should be a compile error naming the path, not a
/// test that silently reads nothing. It costs this crate a build-time
/// dependency on a path outside itself, which is the price of the
/// census being checkable at all.
///
/// `k-lint`'s `tests/threshold_provenance.rs` reaches out of its own
/// crate root for the same kind of artefact and does it the other way,
/// with `CARGO_MANIFEST_DIR` and a runtime read. That is not a second
/// convention: its payload is deflate, `include_str!` cannot hold it
/// and that crate has no inflater, so it must stream the file through
/// `gzip -dc` at run time. Text that fits in the binary is embedded;
/// anything else is read. The rule is the payload, not the taste.
const BASELINE: &str = include_str!("../../../docs/tess-budget-data/tess-budget-baseline.csv");

/// One row's identity, keyed for grouping.
///
/// Nothing is transcribed here: the array comes from the crate, and
/// its length is `IDENTITY_COLUMNS.len()`. What this adds is only the
/// scene, because a pair is two rows OF ONE SCENE.
fn key(r: &Row) -> (&str, [String; IDENTITY_COLUMNS.len()]) {
    (r.scene.as_str(), identity_readings(r))
}

/// The given rows grouped by [`key`] — the ONE spelling in this file
/// of "rows this CSV cannot tell apart".
///
/// The census below counts these groups; the swap tests read their
/// members. Sharing the grouping is what stops the two from ever
/// disagreeing about which rows those are.
type Groups<'a> = HashMap<(&'a str, [String; IDENTITY_COLUMNS.len()]), Vec<&'a Row>>;

fn groups<'a>(rows: &[&'a Row]) -> Groups<'a> {
    let mut out = Groups::new();
    for &r in rows {
        out.entry(key(r)).or_default().push(r);
    }
    out
}

/// `(pairs, rows in a group of two or more, scenes carrying a pair)`,
/// with the scene names.
fn census(rows: &[&Row]) -> (usize, usize, Vec<String>) {
    let groups = groups(rows);
    let pairs = groups.values().map(|g| g.len() * (g.len() - 1) / 2).sum();
    let in_group = groups.values().filter(|g| g.len() > 1).map(Vec::len).sum();
    let mut scenes: Vec<String> = groups
        .iter()
        .filter(|(_, g)| g.len() > 1)
        .map(|((s, _), _)| (*s).to_string())
        .collect();
    scenes.sort();
    scenes.dedup();
    (pairs, in_group, scenes)
}

/// Every indistinguishable pair among `rows`, each `(lower ordinal,
/// higher ordinal)`, in a stable order.
///
/// The MEMBERS of what [`census`] counts: a group of `k` contributes
/// its `k·(k−1)/2` pairs, off the same [`groups`], so a corpus that
/// moves moves both readings together.
fn indistinguishable_pairs<'a>(rows: &[&'a Row]) -> Vec<(&'a Row, &'a Row)> {
    let mut pairs: Vec<(&Row, &Row)> = Vec::new();
    for g in groups(rows).values() {
        for (i, &a) in g.iter().enumerate() {
            for &b in &g[i + 1..] {
                pairs.push(if a.face < b.face { (a, b) } else { (b, a) });
            }
        }
    }
    pairs.sort_by(|x, y| {
        (x.0.scene.as_str(), x.0.face, x.1.face).cmp(&(y.0.scene.as_str(), y.0.face, y.1.face))
    });
    // Every claim below ranges over this list, so an empty one would
    // pass them all while saying nothing. On this corpus it cannot be
    // empty — the census above pins the count, and no number is
    // repeated here — and the guard is what makes that dependence a
    // failure rather than a silence.
    assert!(
        !pairs.is_empty(),
        "no indistinguishable pair for the swap-cost claims to be over: they would pass vacuously"
    );
    pairs
}

/// The committed corpus with ONE pair's two ordinals exchanged: the
/// undetected swap, in the shape a fresh sweep would hand the gate.
///
/// A transposition WITHIN one scene, so `(scene, face)` stays unique
/// across the result. That is the property [`parse`] guarantees and
/// [`compare`]'s per-face index needs, and it is why a permutation
/// that did not itself come from a parse may be handed to the gate.
fn with_pair_swapped(rows: &[Row], scene: &str, a: usize, b: usize) -> Vec<Row> {
    rows.iter()
        .map(|r| {
            let mut r = r.clone();
            if r.scene == scene {
                if r.face == a {
                    r.face = b;
                } else if r.face == b {
                    r.face = a;
                }
            }
            r
        })
        .collect()
}

/// The census itself. Every quantity `lib.rs` used to transcribe, and
/// the corpus it is over.
///
/// **One of these is not independently exercisable and it is said
/// rather than hidden**: `all_scenes.len()` is a second reading of the
/// same corpus-wide grouping as `all_pairs`, and `all_pairs` asserts
/// first, so no perturbation reaches the scene count without moving
/// the pair count. It is kept because it is the figure a reader wants
/// beside the pair count, not because it discriminates on its own.
#[test]
fn the_committed_baseline_carries_this_many_indistinguishable_pairs() {
    let rows = parse(BASELINE).expect("the committed baseline parses");
    let all: Vec<&Row> = rows.iter().collect();
    let sized: Vec<&Row> = rows.iter().filter(|r| r.is_sized()).collect();

    // The corpus the census is over.
    assert_eq!(all.len(), 1353, "rows in the committed baseline");
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
    assert_eq!(all_pairs, 22_352, "pairs across every row");
    assert_eq!(all_scenes.len(), 72, "scenes carrying one, corpus-wide");
}

/// The other half of the paragraph: WHICH identity entries actually
/// discriminate among the sized rows, which is the honest reading of
/// how big the hole is. Five of the seven are constant there —
/// `chart` and the four trim-box edges — so the live pair is
/// `nu`/`nv` alone.
///
/// The split is DERIVED, column by column, from
/// [`tess_lint::identity_readings`] rather than spot-checked: the
/// constant set and the discriminating set are both named in full and
/// must partition [`IDENTITY_COLUMNS`], so a column that changes side
/// and an eighth column both land here rather than going uncounted.
#[test]
fn five_of_the_seven_identity_entries_discriminate_nothing_among_the_sized_rows() {
    let rows = parse(BASELINE).expect("the committed baseline parses");
    let sized: Vec<&Row> = rows.iter().filter(|r| r.is_sized()).collect();
    assert!(!sized.is_empty(), "the census needs sized rows to be over");

    // Distinct readings per identity column, over the sized rows.
    let distinct: Vec<usize> = (0..IDENTITY_COLUMNS.len())
        .map(|i| {
            let mut v: Vec<String> = sized
                .iter()
                .map(|r| identity_readings(r)[i].clone())
                .collect();
            v.sort_unstable();
            v.dedup();
            v.len()
        })
        .collect();

    let constant: Vec<&str> = IDENTITY_COLUMNS
        .iter()
        .zip(&distinct)
        .filter(|(_, d)| **d == 1)
        .map(|(c, _)| *c)
        .collect();
    let discriminating: Vec<&str> = IDENTITY_COLUMNS
        .iter()
        .zip(&distinct)
        .filter(|(_, d)| **d > 1)
        .map(|(c, _)| *c)
        .collect();

    assert_eq!(
        constant,
        ["chart", "u0", "u1", "v0", "v1"],
        "the identity entries that discriminate NOTHING among the \
         sized rows; readings per column {distinct:?}"
    );
    assert_eq!(
        discriminating,
        ["nu", "nv"],
        "the identity entries that do the separating among the sized \
         rows; readings per column {distinct:?}"
    );
    // The arithmetic the prose states, so the prose cannot drift from
    // it: five constant plus the live pair is the whole list.
    assert_eq!(
        constant.len() + discriminating.len(),
        IDENTITY_COLUMNS.len(),
        "every identity entry is either constant or discriminating"
    );

    // What the two constant halves ARE, which is why they are
    // constant.  `chart` is the trivial member: `parse` admits the
    // sizing block only under the charts that owe it, so every sized
    // row of this corpus is one of those tags, and here it is the
    // same one.
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
}

/// The other quantity `lib.rs` used to transcribe: WHICH scenes of
/// the committed corpus gate a re-key, and so how much of it is a
/// scene where one is a NOTE rather than a finding.
///
/// Rule 4's judgement is per SCENE and reads BOTH sides of a
/// comparison — does EITHER carry a sized face. A census is over ONE
/// corpus, so the most it can pin is the baseline's side of that
/// question, and the names below are exactly it: the scenes a fresh
/// sweep re-keys at a FINDING however the fresh side reads, because
/// the committed side alone already gates them. The remaining 60 are
/// notes only while the fresh sweep leaves them unsized too — that
/// half is a two-sided reading this file cannot take, and the
/// conversion the day one of them gains a sized face is pinned in
/// `lib.rs` by
/// `a_scene_that_gains_its_first_sized_face_reds_rather_than_notes`.
///
/// **Asserted by NAME rather than by count**, for the reason the pair
/// census asserts its scene list: a count is the weaker pin, and this
/// one is already asserted 130 lines up over an identical predicate
/// over the identical corpus, so a second count here would exercise
/// nothing. The note count is `72 − 12` by construction — every scene
/// is one or the other — so it is arithmetic and is stated in this
/// sentence rather than asserted. An assertion no perturbation can
/// reach is the defect this file exists to keep out of its own
/// numbers, and `scenes.difference(&gating).len()` would have been
/// one.
#[test]
fn the_committed_baseline_gates_a_re_key_in_exactly_these_scenes() {
    let rows = parse(BASELINE).expect("the committed baseline parses");
    let scenes: BTreeSet<&str> = rows.iter().map(|r| r.scene.as_str()).collect();
    let gating: Vec<&str> = rows
        .iter()
        .filter(|r| r.is_sized())
        .map(|r| r.scene.as_str())
        .collect::<BTreeSet<&str>>()
        .into_iter()
        .collect();

    assert_eq!(scenes.len(), 72, "scenes in the committed baseline");
    assert_eq!(
        gating,
        [
            "lily/lily_leaf_a",
            "lily/lily_leaf_b",
            "lily/lily_leaf_c",
            "lily/lily_sepal_a",
            "lily/lily_sepal_b",
            "lily/lily_sepal_c",
            "lofts/loft_prism",
            "lofts/nonuniform_loft",
            "s_duct/s_duct",
            "twisted_duct/twisted_duct",
            "twisted_duct_shadow_y/twisted_duct_shadow_y",
            "twisted_duct_shadow_z/twisted_duct_shadow_z",
        ],
        "the scenes carrying a sized face, where a re-key is a FINDING; \
         in every other scene of the 72 it is a NOTE"
    );

    // The SCENE-level spelling of "carries a sized face", which cannot
    // share [`Row::is_sized`]'s body: `SceneTotals::recoverable` reads
    // summed cell counts, and those are above zero exactly when some
    // row of the scene is sized only because `parse` admits no cell
    // count below one. That is the crate's argument for having no
    // second counter; it holds by the floor rather than by
    // construction, so it is exercised here over the whole committed
    // corpus rather than left written down.
    let per_scene = totals(&rows);
    let by_totals: Vec<&str> = {
        // `totals` is in tour order; `gating` came out of a `BTreeSet`.
        let mut v: Vec<&str> = per_scene
            .iter()
            .filter(|(_, t)| t.recoverable().is_some())
            .map(|(s, _)| s.as_str())
            .collect();
        v.sort_unstable();
        v
    };
    assert_eq!(
        by_totals, gating,
        "the scene-level and row-level readings of \"carries a sized \
         face\" name the same scenes"
    );
}

/// What an undetected swap costs the GATE, executed rather than
/// argued: each pair the CSV cannot tell apart is swapped in a copy of
/// the committed corpus, that copy is handed to [`compare`] as the
/// fresh side, and the [`Report`] must come back empty.
///
/// **Two assertions, and both are reachable** — which is the whole
/// reason they are in this order:
///
/// * the [`Report`] is red exactly when some pair's two recoverable
///   slacks differ by more than [`tess_lint::GROWTH_TOLERANCE`]. A
///   swap presents each member's ratio at the other's ordinal, so the
///   larger of the two always arrives where the smaller was and the
///   one-sided growth test sees it. That is the alarm: a swap has
///   become gate-visible, and `C15`'s cost is no longer zero.
/// * the equality is red as soon as the two ratios differ AT ALL. It
///   is the margin behind the first, and a re-cut that moves a pair
///   sub-tolerance reds this one alone — an early warning that the
///   reading the item rests on has started to go.
///
/// Rule 1 is not asserted anywhere here: it compares per-SCENE
/// triangle totals and a swap permutes the summands of one sum, so no
/// corpus can make it fire and an assertion on it could not fail.
#[test]
fn an_undetected_swap_costs_the_gate_nothing_on_the_committed_baseline() {
    let rows = parse(BASELINE).expect("the committed baseline parses");
    let sized: Vec<&Row> = rows.iter().filter(|r| r.is_sized()).collect();

    for (a, b) in indistinguishable_pairs(&sized) {
        let swapped = with_pair_swapped(&rows, &a.scene, a.face, b.face);
        let report = compare(&rows, &swapped);
        assert_eq!(
            report,
            Report::default(),
            "swapping {} faces {} and {} — two rows no IDENTITY_COLUMNS entry \
             separates — is no longer invisible to the gate",
            a.scene,
            a.face,
            b.face
        );

        let (na, nb) = (
            a.nurbs.expect("filtered to sized rows"),
            b.nurbs.expect("filtered to sized rows"),
        );
        assert_eq!(
            a.recoverable(),
            b.recoverable(),
            "{} faces {} and {} no longer read one recoverable slack: \
             grid_cells/span_opt_cells is {}/{} against {}/{}. The swap still \
             costs the gate nothing, but the margin that made it free has gone",
            a.scene,
            a.face,
            b.face,
            na.grid_cells,
            na.span_opt_cells,
            nb.grid_cells,
            nb.span_opt_cells
        );
    }
}

/// What the `name` column contributes to these pairs, which today is
/// nothing.
///
/// The join-relevant claim, and it is deliberately NOT "no sized row
/// carries a name": a named row changes nothing until the name tells
/// two rows APART, and it is the pairs that a name would rescue. The
/// producer-side mechanism is already in place — `tess_meter::face_rows`
/// takes a name table and refuses one that misses a face — so this
/// fires the day a scene carrying a pair becomes document-built, and
/// its message is the handover.
///
/// It cannot be satisfied by an absence either way: an empty name on
/// both members is an agreement, and so is one shared name; only a
/// name that SEPARATES them reds it, which is exactly the condition
/// under which `C15` becomes dischargeable for that pair.
#[test]
fn no_indistinguishable_pair_is_separated_by_the_name_column() {
    let rows = parse(BASELINE).expect("the committed baseline parses");
    let sized: Vec<&Row> = rows.iter().filter(|r| r.is_sized()).collect();

    for (a, b) in indistinguishable_pairs(&sized) {
        assert_eq!(
            a.name, b.name,
            "`name` now separates {} faces {} and {} ({:?} against {:?}), and no \
             other identity column does. C15 is dischargeable for this pair: the \
             sweep hands the gate a durable per-face identity here, so the join \
             has something to key on besides the ordinal. Re-key rule 4 over the \
             rows that carry a name and re-cut this census",
            a.scene, a.face, b.face, a.name, b.name
        );
    }
}

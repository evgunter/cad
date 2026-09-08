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
//! * `work/meter/C15.md` — FIXED, points here.
//! * `work/meter/D201.md` — FIXED, points here.
//! * `work/code-quality/logs/SMELL-KPW-LOG.md` — **NOT FIXED, and
//!   deliberately.** It is a dated unit record in `logs/`, and a log
//!   entry is what the unit reported on the day it reported it. Its
//!   figures were correct against the tree it closed on; editing them
//!   now would make the record say something the unit did not say,
//!   which is a worse defect than the stale number. Frozen, not
//!   propagated: nothing cites the log for a current count.
//!
//! **The FIGURES in that list are frozen; the PATHS are not.** The
//! first two rows were re-homed from `work/code-quality/` to
//! `work/meter/` in the tracker-wide cut of 2026-09-06 and the
//! pointers here followed them. A dated record may keep the numbers
//! it reported on the day it reported them — that is what makes it a
//! record — but a pointer that no longer resolves has stopped being a
//! record of anything, which is the same one-home doctrine this
//! paragraph is about, applied to the pointer rather than the count.
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
//! * **Rule 1 cannot move it — a theorem, on a premise worth
//!   naming.** It compares per-SCENE triangle totals, and a swap
//!   within one scene permutes the summands of one sum. Permutation
//!   -invariance of a sum is FALSE for `f64`, so the theorem rests
//!   entirely on `SceneTotals::triangles` being `usize`: an integer
//!   sum is exactly invariant, a floating one is not. Its neighbour
//!   in the same struct, `extrapolated_triangles`, is `f64` and does
//!   move in its last bit under this very permutation. Nothing below
//!   asserts rule 1 — an assertion on it could not fail — so the
//!   premise has to be carried by this sentence.
//! * **Rule 2 does not move — a READING of this baseline.** It
//!   compares [`Row::recoverable`], and within every pair the
//!   committed corpus carries the two members' quotients are the same
//!   `f64`. The QUOTIENT is what agrees and what is asserted; the two
//!   cell counts behind it are printed in the failure message to
//!   locate a drift, not asserted, because a corpus where they
//!   differed and the quotient did not would still be a swap rule 2
//!   cannot see. That is a fact about a committed artefact, and a
//!   re-cut can end it, so it is asserted here rather than written
//!   down.
//!
//! `an_undetected_swap_costs_the_gate_nothing_on_the_committed_baseline`
//! puts the consequence the way the gate puts it — the swap is handed
//! to [`compare`] and the [`Report`] must come back empty — and then
//! the equality that is the margin behind it.
//!
//! **The report does not move either, and there is no reported-side
//! figure to threshold** — but the two halves of that are not one
//! argument, for the reason the rule-1 bullet above gives. A swap
//! exchanges two whole rows of one scene, so every aggregate the
//! report folds sees the same multiset: EXACT for the integer sums,
//! and no more than suggestive for the floating ones, where
//! `extrapolated_triangles` is the standing counterexample. That the
//! PRINTED report comes back byte-identical for all seven pairs is a
//! separate thing — a measurement over one corpus, which survives in
//! part because that last-bit move dies at the `{:.1}x` the column
//! prints at. `work/meter/C15.md` carries the method.
//!
//! `worst_dev` does reach a reader, through `SceneTotals::total_slack`
//! — which is `measured_triangles / extrapolated_triangles`,
//! triangle-weighted, and NOT the per-row `delta / worst_dev` that
//! shares its name — and `worst_cert` reaches none at all. So what an
//! undetected swap costs is a wrong-face ATTRIBUTION in columns
//! nothing compares, not a moved number anywhere.
//!
//! **Nothing guards that, and it is a choice rather than a
//! constraint.** One report line does read a face ordinal — the
//! worst-realized-aspect line — and a test here could reach it:
//! `tests/cli_contract.rs` runs the binary through
//! `CARGO_BIN_EXE_tess-lint` and reads its stdout, with nothing
//! twinned. What such a guard would pin is WHICH face happens to
//! carry the corpus's worst aspect, a reading that moves on any
//! re-cut for reasons that have nothing to do with this row; and the
//! wrong-face attribution it would be guarding is the defect the pair
//! claims below already carry. So it is left unwritten deliberately,
//! not because it cannot be written.
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

/// Rows bucketed under [`key`]: one entry per distinct
/// `(scene, identity)`, holding every row that reads that way.
type Groups<'a> = HashMap<(&'a str, [String; IDENTITY_COLUMNS.len()]), Vec<&'a Row>>;

/// The given rows grouped by [`key`] — the ONE spelling in this file
/// of "rows this CSV cannot tell apart".
///
/// The census below counts these groups; the swap tests read their
/// members. Sharing the grouping is what stops the two from ever
/// disagreeing about which rows those are.
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
///
/// # Panics
///
/// If there is no such pair. Every claim in this file ranges over the
/// list this returns, so an empty one would pass them all while
/// saying nothing; the refusal is what makes that dependence a
/// failure rather than a silence. It lives here rather than at each
/// caller because one home is the point, and it is advertised here
/// because a caller for whom emptiness were legitimate would
/// otherwise meet it as a surprise.
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
    // On this corpus it cannot be empty — the census above pins the
    // count, and no number is repeated here.
    assert!(
        !pairs.is_empty(),
        "no indistinguishable pair for the swap-cost claims to be over: they would pass vacuously"
    );
    pairs
}

/// The committed baseline with ONE pair's two MEASUREMENTS exchanged,
/// re-parsed: the undetected swap, in the text a fresh sweep would
/// have written.
///
/// The exchange is made on the CSV text and the result goes back
/// through [`parse`], which is what [`compare`] documents as its
/// input — so nothing here has to reconcile a hand-built `Vec<Row>`
/// against that precondition, and no `Row` is built by hand at all.
/// Face `a` comes back carrying what face `b` measured, with the
/// ordinals left in the ascending order a sweep writes them in — the
/// text a producer would have emitted. That the rows arrive through
/// the parser is the whole reason for the shape; it buys no stronger
/// permutation than relabelling two rows in place would, because
/// `totals` folds a scene through a `BTreeMap` keyed by ordinal and
/// so is permuted identically either way.
fn with_pair_swapped(scene: &str, a: usize, b: usize) -> Vec<Row> {
    let (pa, pb) = (format!("{scene},{a},"), format!("{scene},{b},"));
    let mut lines: Vec<String> = BASELINE.lines().map(str::to_string).collect();
    // The FIRST line under each prefix is the only one: [`parse`]
    // refuses a repeated `(scene, face)`, so re-checking here would be
    // a second reading of a guarantee the corpus already carries — and
    // an assertion nothing could make fail.
    let only = |lines: &[String], p: &str| {
        lines
            .iter()
            .position(|l| l.starts_with(p))
            .expect("the pair's rows are in the text they were parsed from")
    };
    let (ia, ib) = (only(&lines, &pa), only(&lines, &pb));
    let (ma, mb) = (
        lines[ia][pa.len()..].to_string(),
        lines[ib][pb.len()..].to_string(),
    );
    lines[ia] = pa + &mb;
    lines[ib] = pb + &ma;
    parse(&lines.join("\n")).expect("two rows' measurements exchanged still parses")
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
/// argued: each pair the CSV cannot tell apart is swapped in the
/// baseline's own text, re-parsed, handed to [`compare`] as the fresh
/// side, and the [`Report`] must come back empty.
///
/// **Two assertions, over every pair rather than pair by pair**, so a
/// re-cut that moves five of them names five. **Both are reachable**,
/// which is the whole reason they are in this order:
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
/// The premise that buys that — `SceneTotals::triangles` is `usize`,
/// and an integer sum is exactly permutation-invariant where an
/// `f64` one is not — is stated in this file's module docs, because
/// declining to assert a claim puts the whole weight on the argument
/// for it.
#[test]
fn an_undetected_swap_costs_the_gate_nothing_on_the_committed_baseline() {
    let rows = parse(BASELINE).expect("the committed baseline parses");
    let sized: Vec<&Row> = rows.iter().filter(|r| r.is_sized()).collect();

    // Both readings are collected over EVERY pair before either is
    // asserted, so a re-cut that moves five of them names five. The
    // order of the two asserts is what keeps both reachable: the gate
    // one can only fire past `GROWTH_TOLERANCE`, so a sub-tolerance
    // drift reaches the second and nothing else.
    let (mut visible, mut drifted) = (Vec::new(), Vec::new());
    for (a, b) in indistinguishable_pairs(&sized) {
        let report = compare(&rows, &with_pair_swapped(&a.scene, a.face, b.face));
        if report != Report::default() {
            visible.push(format!(
                "{} faces {}/{}: {report:?}",
                a.scene, a.face, b.face
            ));
        }
        if a.recoverable() != b.recoverable() {
            let (na, nb) = (
                a.nurbs.expect("filtered to sized rows"),
                b.nurbs.expect("filtered to sized rows"),
            );
            drifted.push(format!(
                "{} faces {}/{}: grid_cells/span_opt_cells {}/{} against {}/{}",
                a.scene,
                a.face,
                b.face,
                na.grid_cells,
                na.span_opt_cells,
                nb.grid_cells,
                nb.span_opt_cells
            ));
        }
    }

    assert!(
        visible.is_empty(),
        "swapping these pairs — rows no IDENTITY_COLUMNS entry separates — is no \
         longer invisible to the gate: {visible:#?}"
    );
    assert!(
        drifted.is_empty(),
        "these pairs no longer read one recoverable slack: {drifted:#?}. The swap \
         still costs the gate nothing, but the margin that made it free has gone"
    );
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

    let separated: Vec<String> = indistinguishable_pairs(&sized)
        .into_iter()
        .filter(|(a, b)| a.name != b.name)
        .map(|(a, b)| {
            format!(
                "{} faces {}/{}: {:?} against {:?}",
                a.scene, a.face, b.face, a.name, b.name
            )
        })
        .collect();

    assert!(
        separated.is_empty(),
        "`name` now tells these pairs apart, and no IDENTITY_COLUMNS entry does — \
         `name` is not one of them, which is what makes this the C15 case and not \
         a re-key: {separated:#?}. C15 is dischargeable for them: the sweep hands \
         the gate a durable per-face identity here, so the join has something to \
         key on besides the ordinal. Re-key rule 4 over the rows that carry a name \
         and re-cut this census"
    );
}

/// Why reading the `name` column discharges nothing today: the rows it
/// covers and the rows the defect lives in do not intersect.
///
/// Only a scene whose body arrived from an evaluated document can hand
/// `tess_meter::face_rows` a name table; on this corpus no scene
/// carrying a sized row is one of them. That is a READING of a
/// committed artefact, so it belongs here and not in prose — the same
/// reason every other quantity in this file does.
///
/// **Weaker in its trigger than the pair claim above, deliberately,
/// and its failure set strictly contains that one — which is said
/// rather than hidden.** This reds the moment ANY sized scene becomes
/// document-built, which is the first moment `D201`'s column reaches
/// the ground the defect lives on; the pair claim reds only when a
/// name actually separates two rows nothing else separates, which is
/// the moment `C15` becomes dischargeable. A sized scene carrying no
/// pair can red this one alone, so it is not a restatement; a pair
/// gaining separating names reds both, and both messages are wanted
/// because they ask for different work.
#[test]
fn no_scene_carrying_a_sized_row_carries_a_name() {
    let rows = parse(BASELINE).expect("the committed baseline parses");
    let sized_scenes: BTreeSet<&str> = rows
        .iter()
        .filter(|r| r.is_sized())
        .map(|r| r.scene.as_str())
        .collect();
    let named_scenes: BTreeSet<&str> = rows
        .iter()
        .filter(|r| !r.name.is_empty())
        .map(|r| r.scene.as_str())
        .collect();
    assert!(
        !sized_scenes.is_empty() && !named_scenes.is_empty(),
        "both sides of the disjointness must be non-empty for it to say anything"
    );

    let both: Vec<&str> = sized_scenes.intersection(&named_scenes).copied().collect();
    assert_eq!(
        both,
        [] as [&str; 0],
        "these scenes now carry both a sized row and a name, so the coverage \
         D201 added has reached the rows C15 is about. Read whether the join can \
         now key on `name` for them and re-cut this census"
    );
}

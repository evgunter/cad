//! **The baseline censuses: their one home.**
//!
//! Two censuses over one committed artefact,
//! `docs/tess-budget-data/tess-budget-baseline.csv`: the FACE-IDENTITY
//! census — how many rows a rule-4 swap could wave through, and how
//! much of the corpus is a scene where a re-key costs no comparison —
//! and the SIZING census, what the tour's mesh and its sizing come to
//! folded through the gate's own accumulator. Both are readings of a
//! committed file that a re-baseline moves, and a number transcribed
//! into prose is a number nothing can check, so both are counted here
//! rather than written down. **A census has one executable home and
//! every other site points at it**, because a pointer cannot go
//! stale — the cure this tree's own CI comment states for the rule
//! roster next door.
//!
//! **That rule is the standard here, not a description of the tree.**
//! `lib.rs`'s module docs meet it, for the face-identity count they
//! used to transcribe. `docs/TESS-BUDGET.md` does not: it still
//! carries four of the sizing figures asserted below in present-tense
//! prose, and neither census can edit it. The row is
//! `work/meter/baseline-sizing-census-second-copy`, and it is not the
//! only site — this file's own sizing section transcribes the same
//! four, which that row covers too.
//!
//! **No figure is asserted twice, and the split is which function
//! asserts what.** The corpus counts — rows, sized rows, the scenes
//! holding them — are the face-identity census's; the sizing sums
//! start where those leave off and restate none of them, and the two
//! percentages the report prints are quotients of the one pair against
//! the other.
//!
//! # The sweep's definition, beside its result
//!
//! Over `docs/tess-budget-data/tess-budget-baseline.csv`, read through
//! [`parse`] — so these censuses count exactly what the gate parses,
//! not what a separate reader thinks the columns mean:
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
//! The face-identity census is taken over the SIZED rows, because an
//! unsized swap costs rule 2 nothing — that restriction is the
//! load-bearing one, and the corpus-wide figure below is here to show
//! how much work it does rather than because anything gates on it.
//!
//! The sizing census folds the parsed rows through [`SceneTotals`],
//! the same accumulator the CLI's report header and the gate's
//! per-scene table use, so it counts what the gate counts. Its cell
//! sums are over the sized rows alone, because [`SceneTotals::add`]
//! adds a cell count only where there is one.
//!
//! # The face-identity census
//!
//! `lib.rs`'s module docs say what rule 4's precondition cannot see —
//! two faces of one scene agreeing on every [`IDENTITY_COLUMNS`] entry
//! and swapping ordinals — and deliberately do not say how many there
//! are. This census says how many, by counting them.
//!
//! ## The transcription sweep, and its hit list
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
//! The cure for the three live copies is the one-home rule above.
//!
//! ## What an undetected swap COSTS, and which half is a theorem
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
//! # The sizing census
//!
//! ## What it is for
//!
//! **A re-cut alarm, and the citation it guards is still live.**
//! `docs/TESS-BUDGET.md` carries `grid_cells`, `patch_cells`,
//! `opt_cells` and `span_opt_cells` — every one of them asserted
//! below — in present-tense prose. So this census does the job a
//! pointer cannot: go RED when a re-cut moves the tour's mesh or its
//! sizing, so the move is read rather than folded in silently. **What
//! it cannot do is finish the job.** A re-cut reds here and leaves
//! those sentences standing, wrong, until somebody reads them; the
//! alarm is the prompt to go and read them, not a guarantee they were
//! read.
//!
//! It does not fire on everything a re-cut can move.
//! `docs/TESS-BUDGET.md`'s "Re-cutting the baseline" says what the two
//! censuses between them do and do not read; the short version is that
//! the descriptive per-face columns are read by neither.
//!
//! ## Why the pre-fix block reads as stale when it is not
//!
//! Three readers in a row have compared `docs/TESS-BUDGET.md`'s
//! pre-fix block against this census column by column and read the
//! difference as drift. It is the fix landing, and the two halves
//! separate cleanly:
//!
//! * the columns that describe a SHIPPED SCHEDULE moved by the factors
//!   TESS-SPAN and TESS-SPLIT were built to move them — the tour's
//!   grid went from a whole-patch AM-GM product to a per-knot-span
//!   cell grid at the aspect-capped cell minimizer. That is 154,129 to
//!   46,019, **3.35x**, on the grid the lane actually builds. The
//!   whole-patch column moved 390,100 to 110,811, **3.52x**, and that
//!   one is a different point selection rather than a smaller grid
//!   (the document's decoder table says why);
//! * the columns that are pure OPTIMA over the certified ellipse —
//!   `opt_cells` and `span_opt_cells` — are schedule-independent, and
//!   they sit within 2.2% and 0.7% of the pre-fix figures because the
//!   sized faces are the same 64 faces. Both gaps are wider than they
//!   were, by the amount the split scan's own resolution moved when
//!   `tess_meter::SPLIT_SCAN_SAMPLES` was raised: a finer scan finds
//!   cheaper splits, so an optimum column falls without a face moving.
//!
//! **What that separates is a change of SIZING RULE from everything
//! else, and no more than that.** Corpus growth, certificate changes
//! and the meter's own resolution all move the optima too, so two
//! columns still within a few percent says the faces and their bounds
//! are still the block's. It does not by itself
//! say which sizing rule changed: a re-cut taken after a schedule
//! change and the schedule change landing are one event. The dated
//! record settles that — `docs/MODEL-AB-LOG.md`'s TESS-SPLIT row reads
//! *"tour NURBS cells 163,182 -> 46,102"*, and 46,102 is what the
//! committed file carried from that cut on.
//!
//! Everything the corpus has grown by since is analytic, so it adds
//! rows and triangles and no cells. **Cells have moved anyway, once**:
//! `grid_cells` read 46,102 from TESS-SPLIT's cut through six re-cuts
//! until CERT-10's (`a4eb03ae`) moved four faces' certified bounds and
//! 83 cells with them — `lily/lily_sepal_a` faces 3 and 7 and the two
//! `twisted_duct_shadow_*` face 4s. Neither growth nor a schedule
//! change; a certificate change, which is the third thing a re-cut
//! can be.
//!
//! **The fourth thing moves the OPTIMA and nothing else, and it is not
//! a reading about geometry at all**: the meter's own split scan. Its
//! resolution sets how close `opt_cells` and `span_opt_cells` get to
//! the cheapest grid the same certificates admit, so raising
//! `tess_meter::SPLIT_SCAN_SAMPLES` lowers both columns over a corpus
//! that did not move — 94,154 to 93,066 and 44,446 to 44,162 over the
//! whole sweep, with `grid_cells`, `patch_cells` and every triangle
//! count identical. A re-cut whose only movers are those two columns
//! is that event and is never a schedule regression.
//!
//! **"Only movers" is a condition this census cannot check, and the
//! re-cut that produced the figures above did not meet it.** Nine
//! columns moved on it, not two: `muv`, `mvv`, `mu1`, `mv1`,
//! `worst_cert`, `worst_dev` and `realized_aspect` moved on 8-16 rows
//! each, four days of `crates/` drift folded in by the same cut. This
//! census reads totals — triangles, the four cell columns and the two
//! factors — so all seven are invisible to it, and so is any future
//! set like them. `work/meter/tess-lint-ungated-columns-fold-silently`
//! is the row for that, and until it lands the fourth category is a
//! thing a reader has to verify by diffing the file, not a thing this
//! census can certify.
//!
//! ## The retired vocabulary, which is what actually mis-reads
//!
//! The block predates the columns it is read against, and two of its
//! phrases name something else now. **The decoder is one table, in
//! `docs/TESS-BUDGET.md` under "The finding", beside the block it
//! decodes; `tess_meter`'s field docs are the definitions of record
//! for every column in it.** Neither is restated here.
//!
//! What is worth carrying at this site is the trap: **an unqualified
//! "cheapest split" names `opt_cells` in one place and
//! `span_opt_cells` in another**, and dropping the qualifier is the
//! mis-read that put the block's `span_cells` line against
//! `span_opt_cells`.
//!
//! # When these tests fail
//!
//! Neither census is a threshold and no baseline here is a target to
//! preserve; the failure is the product. A re-cut that moves these
//! numbers means the corpus, the schedule or a certified bound moved:
//! read the new number, decide whether it is what you meant, and write
//! it in. The failures exist so that no prose anywhere — `lib.rs`'s
//! paragraph on what rule 4's precondition cannot see,
//! `docs/TESS-BUDGET.md`'s account of the sweep — can go on describing
//! a file it no longer describes.

use std::collections::{BTreeSet, HashMap};

use tess_lint::{
    IDENTITY_COLUMNS, Report, Row, SceneTotals, compare, identity_readings, parse, totals,
};

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
    assert_eq!(all.len(), 1599, "rows in the committed baseline");
    assert_eq!(sized.len(), 72, "of them sized");
    let sized_scenes = {
        let mut s: Vec<&str> = sized.iter().map(|r| r.scene.as_str()).collect();
        s.sort_unstable();
        s.dedup();
        s
    };
    assert_eq!(sized_scenes.len(), 13, "scenes carrying a sized face");

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
    //
    // This figure is dominated by ARITHMETIC rather than by geometry,
    // and `impeller` is the clearest case of why. A planar row carries
    // no trim box and no divisions, so every identity column but
    // `chart` reads empty on it — which makes ALL of one scene's
    // planar rows one group, contributing `C(n, 2)` by construction.
    // The impeller's three stops are planar throughout (56, 66 and 86
    // faces), and 1540 + 2145 + 3655 = 7340 is exactly what they added
    // when they landed. A number that grows quadratically in a scene's
    // face count is not a budget reading and was never used as one;
    // what it measures is the size of the hole the sized-row census
    // above sits inside.
    let (all_pairs, _, all_scenes) = census(&all);
    assert_eq!(all_pairs, 29_723, "pairs across every row");
    assert_eq!(all_scenes.len(), 78, "scenes carrying one, corpus-wide");
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

    // 75 twice in this file, and NOT one figure asserted twice: this
    // is every scene of the corpus, where the pair census's 75 is the
    // scenes carrying an indistinguishable pair among ALL rows. They
    // agree only because every scene currently carries one, and a
    // re-cut can end that without either assertion being wrong.
    assert_eq!(scenes.len(), 78, "scenes in the committed baseline");
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
            "twisted_tube/twisted_tube",
        ],
        "the scenes carrying a sized face, where a re-key is a FINDING; \
         in every other scene of the 75 it is a NOTE"
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

/// The whole sweep folded as the CLI folds it: one [`SceneTotals`]
/// over every row.
fn sweep(rows: &[Row]) -> SceneTotals {
    let mut t = SceneTotals::default();
    for r in rows {
        t.add(r);
    }
    t
}

/// What the report header prints over the committed baseline, less
/// the two face counts the face-identity census above already pins,
/// plus `opt_cells`, which the header does not print.
///
/// **The two factors are undiscriminating against a change in the
/// DATA, and it is said rather than hidden**: each is a quotient of
/// sums asserted above it, so no perturbation of the baseline reaches
/// a factor without moving a sum first, and the sum reds first. They
/// are not inert — they are taken through [`SceneTotals`]'s own
/// methods, so inverting either method reds its own assertion and no
/// sum (executed, both directions). That is what they guard: the
/// CLI's arithmetic for the two figures the report prints and prose
/// would otherwise copy. They do not add coverage over the CSV.
#[test]
fn the_committed_baseline_sizes_this_much() {
    let rows = parse(BASELINE).expect("the committed baseline parses");
    let t = sweep(&rows);

    // What the Hessian-sized lane carries. The corpus these are over —
    // rows, sized rows, the scenes holding them — is pinned by
    // `the_committed_baseline_carries_this_many_indistinguishable_pairs`
    // above and is deliberately not restated here; the report prints
    // its two percentages from that pair against this one.
    assert_eq!(t.triangles, 1_647_626, "triangles over the whole sweep");
    assert_eq!(
        t.nurbs_triangles, 188_908,
        "triangles the Hessian-sized faces carry"
    );

    // The grid-cell totals, over the sized rows. `grid_cells` is what
    // the lane built; `patch_cells` is the whole-patch bound as a
    // counterfactual, at the SHIPPED point selection rather than the
    // retired schedule's own (`NurbsColumns::nu` says so); the other
    // two are the optima the same certificates still admit
    // (whole-patch bound / per cell).
    assert_eq!(t.grid_cells, 56_517.0, "grid cells the lane built");
    assert_eq!(t.patch_cells, 126_331.0, "the whole-patch counterfactual");
    assert_eq!(
        t.opt_cells, 105_301.0,
        "cheapest split under the whole-patch bound"
    );
    assert_eq!(
        t.span_opt_cells, 54_282.0,
        "per-cell sizing at the cheapest split in each cell"
    );

    // The two factors the report header prints beside them.
    let held = t.span_held().expect("the sweep has Hessian-sized faces");
    let recoverable = t.recoverable().expect("the sweep has Hessian-sized faces");
    assert!(
        (held - 2.2353).abs() < 5e-4,
        "the held span gain, patch_cells / grid_cells; got {held}"
    );
    assert!(
        (recoverable - 1.0412).abs() < 5e-4,
        "slack still recoverable, grid_cells / span_opt_cells; got {recoverable}"
    );
}

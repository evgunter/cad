//! **The sizing census: its one home, and the re-cut alarm over it.**
//!
//! What the committed baseline
//! (`docs/tess-budget-data/tess-budget-baseline.csv`) says TODAY about
//! the tour's mesh and its sizing, folded through the gate's own
//! accumulator. `docs/TESS-BUDGET.md` points here rather than carrying
//! a second copy: a census has one executable home and every other
//! site points at it.
//!
//! # What this file is for
//!
//! **A re-cut alarm.** It was written to guard the document's
//! citation, and there is no longer a citation to guard: after METER
//! unit 2 the document holds no live figure from this file, and it
//! argues in its own text that a pointer cannot go stale. What is left
//! is the job a pointer cannot do — go RED when a re-cut moves the
//! tour's mesh or its sizing, so the move is read and written down
//! rather than folded in silently. No number here is a target to
//! preserve; the failure is the product.
//!
//! It does not fire on everything a re-cut can move.
//! `docs/TESS-BUDGET.md`'s "Re-cutting the baseline" says what the two
//! census files between them do and do not read; the short version is
//! that the descriptive per-face columns are read by neither.
//!
//! # The sibling next door
//!
//! `baseline_census.rs` is the FACE-IDENTITY census over the same
//! file — how many rows a rule-4 swap could wave through, and how much
//! of the corpus is a scene where a re-key costs no comparison. **No
//! figure is asserted twice**: the row count and the sized-row count
//! are the neighbour's and are not restated here, and the sums below
//! start where those leave off.
//!
//! **Whether the two should be one file is open, and filed** —
//! `work/meter/fold-the-two-baseline-census-files.md`. The reason
//! previously written here, that folding would put one file's failure
//! under two unrelated headings, does not survive: a failure is named
//! by its `#[test]`, not by its file. What is true either way is that
//! both read the same `include_str!` path, duplicated for want of a
//! shared `tests/` module, and a reader moving the baseline has two
//! sites to change — both compile errors.
//!
//! # Why the pre-fix block reads as stale when it is not
//!
//! Three readers in a row have compared `docs/TESS-BUDGET.md`'s
//! pre-fix block against this file column by column and read the
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
//! # The retired vocabulary, which is what actually mis-reads
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
//! # The sweep's definition, beside its result
//!
//! Over `docs/tess-budget-data/tess-budget-baseline.csv`, read through
//! [`parse`] and folded through [`SceneTotals`] — the same accumulator
//! the CLI's report header and the gate's per-scene table use, so this
//! census counts what the gate counts and not what a second reader
//! thinks the columns mean. A row is **sized** when it carries the
//! Hessian-sized block (`Row::nurbs` is `Some`); the cell sums are
//! over the sized rows alone, because [`SceneTotals::add`] adds a
//! cell count only where there is one.
//!
//! # When this test fails
//!
//! No baseline here is a target to preserve. A re-cut that moves these
//! numbers means the corpus, the schedule or a certified bound moved:
//! read the new number, decide whether it is what you meant, and write
//! it in. The
//! failure exists so that no prose anywhere can go on describing a
//! file it no longer describes — which is the defect this file was
//! written for, one document over.

use tess_lint::{Row, SceneTotals, parse};

/// The committed baseline, by path relative to this crate's manifest.
///
/// `include_str!` for the reason `baseline_census.rs` gives at its own
/// copy: a missing or moved baseline is a compile error naming the
/// path, not a test that silently reads nothing.
const BASELINE: &str = include_str!("../../../docs/tess-budget-data/tess-budget-baseline.csv");

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
/// the two face counts `baseline_census.rs` already pins, plus
/// `opt_cells`, which the header does not print.
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
    // rows, sized rows, the scenes holding them — is pinned in
    // `baseline_census.rs` and is deliberately not restated here; the
    // report prints its two percentages from that pair against this
    // one.
    assert_eq!(t.triangles, 1_552_822, "triangles over the whole sweep");
    assert_eq!(
        t.nurbs_triangles, 164_710,
        "triangles the Hessian-sized faces carry"
    );

    // The grid-cell totals, over the sized rows. `grid_cells` is what
    // the lane built; `patch_cells` is the whole-patch bound as a
    // counterfactual, at the SHIPPED point selection rather than the
    // retired schedule's own (`NurbsColumns::nu` says so); the other
    // two are the optima the same certificates still admit
    // (whole-patch bound / per cell).
    assert_eq!(t.grid_cells, 46_019.0, "grid cells the lane built");
    assert_eq!(t.patch_cells, 110_811.0, "the whole-patch counterfactual");
    assert_eq!(
        t.opt_cells, 93_066.0,
        "cheapest split under the whole-patch bound"
    );
    assert_eq!(
        t.span_opt_cells, 44_162.0,
        "per-cell sizing at the cheapest split in each cell"
    );

    // The two factors the report header prints beside them.
    let held = t.span_held().expect("the sweep has Hessian-sized faces");
    let recoverable = t.recoverable().expect("the sweep has Hessian-sized faces");
    assert!(
        (held - 2.408).abs() < 5e-4,
        "the held span gain, patch_cells / grid_cells; got {held}"
    );
    assert!(
        (recoverable - 1.0420).abs() < 5e-4,
        "slack still recoverable, grid_cells / span_opt_cells; got {recoverable}"
    );
}

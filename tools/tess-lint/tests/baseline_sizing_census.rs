//! **The sizing census: its one home.**
//!
//! `docs/TESS-BUDGET.md` is the #547 measurement, taken before
//! TESS-SPAN and TESS-SPLIT shipped and kept as the pre-fix record.
//! Its headline block is therefore NOT a reading of the committed
//! baseline and cannot be re-derived from one — the tree it was swept
//! from no longer exists and no artefact holds its numbers. What the
//! committed baseline says TODAY is this file, and the document points
//! here rather than carrying a second copy: a census has one
//! executable home and every other site points at it.
//!
//! The sibling next door, `baseline_census.rs`, is the FACE-IDENTITY
//! census over the same file — how many rows a rule-4 swap could wave
//! through. Two censuses, two homes, one artefact: they are different
//! questions and neither restates the other's numbers.
//!
//! # Why the pre-fix block reads as stale when it is not
//!
//! Three readers in a row have compared the document's block against
//! this file column by column and read the difference as drift. It is
//! the fix landing, and the two halves separate cleanly:
//!
//! * the columns that describe a SHIPPED SCHEDULE moved by the factors
//!   TESS-SPAN and TESS-SPLIT were built to move them — the tour's
//!   grid went from a whole-patch AM-GM product to a per-knot-span
//!   cell grid at the aspect-capped cell minimizer;
//! * the columns that are pure OPTIMA over the certified ellipse —
//!   `opt_cells` and `span_opt_cells` — are schedule-independent, and
//!   they sit within 1% of the pre-fix figures because the sized faces
//!   are the same faces. Everything the corpus has grown by since is
//!   analytic, so it adds rows and triangles and no cells at all.
//!
//! A re-cut moves all four together. Two columns still at 1% while two
//! moved by 3–8x is not a re-cut; it is the measurement's own subject.
//!
//! # The retired vocabulary, which is what actually mis-reads
//!
//! The block predates the columns it is read against, and two of its
//! phrases name something else now:
//!
//! | the block's phrase | then | now |
//! |---|---|---|
//! | *grid cells used* | `uniform_cells`, the shipped whole-patch-sup grid | `patch_cells`, a counterfactual — `grid_cells` now names the shipped PER-CELL grid |
//! | *at the cheapest split* | `opt_cells` | `opt_cells`, unchanged |
//! | *sized per knot-span cell* | `span_cells` | REMOVED (it was identically `grid_cells`) |
//! | *with both* | `span_opt_cells` | `span_opt_cells`, unchanged |
//!
//! **"The cheapest split" names two different columns across this
//! tree, and the qualifier is the whole of the difference.**
//! `opt_cells` is the cheapest split under the WHOLE-PATCH bound;
//! `span_opt_cells` is per-cell sizing AND the cheapest split in each
//! cell, which is what the report header prints as *at the cheapest
//! split per cell*. `tess_meter`'s field docs are the definitions of
//! record for both. An unqualified "cheapest split" is the mis-read
//! that put the block's `span_cells` line against `span_opt_cells`.
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
//! numbers means the corpus or the schedule moved: read the new
//! number, decide whether it is what you meant, and write it in. The
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

/// The census `docs/TESS-BUDGET.md` cites instead of transcribing:
/// every figure the report header prints over the committed baseline,
/// plus `opt_cells`, which the header does not print and the document
/// needs to name the pre-fix block's split column.
///
/// **Two of these are not independently exercisable and it is said
/// rather than hidden**: the two factors are quotients of sums
/// asserted above them, so no perturbation reaches a factor without
/// moving a sum first. They are asserted because they are the figures
/// the report prints and prose would otherwise copy — through
/// [`SceneTotals`]'s own methods, so this file cannot re-derive a
/// factor by hand and disagree with the CLI about what one is.
#[test]
fn the_committed_baseline_sizes_this_much() {
    let rows = parse(BASELINE).expect("the committed baseline parses");
    let t = sweep(&rows);

    // The corpus, and what of it the Hessian-sized lane carries. The
    // percentages the report prints are these four numbers.
    assert_eq!(t.faces, 1353, "faces in the committed baseline");
    assert_eq!(t.triangles, 1_552_822, "triangles over all of them");
    let sized = rows.iter().filter(|r| r.nurbs.is_some()).count();
    assert_eq!(sized, 64, "of them Hessian-sized");
    assert_eq!(
        t.nurbs_triangles, 164_710,
        "triangles the Hessian-sized faces carry"
    );

    // The grid-cell totals, over the sized rows. `grid_cells` is what
    // the lane built; `patch_cells` is the retired whole-patch
    // schedule as a counterfactual; the other two are the optima the
    // same certificates still admit (whole-patch bound / per cell).
    assert_eq!(t.grid_cells, 46_019.0, "grid cells the lane built");
    assert_eq!(t.patch_cells, 110_811.0, "the whole-patch counterfactual");
    assert_eq!(
        t.opt_cells, 94_154.0,
        "cheapest split under the whole-patch bound"
    );
    assert_eq!(
        t.span_opt_cells, 44_446.0,
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
        (recoverable - 1.035).abs() < 5e-4,
        "slack still recoverable, grid_cells / span_opt_cells; got {recoverable}"
    );
}

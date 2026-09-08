//! **The report header names its columns, and the names are the
//! file's.**
//!
//! The CLI's cell-total block prints four figures. Until this file
//! each was introduced by a prose phrase — *"grid cells used"*,
//! *"whole-patch counterfactual"*, *"at the cheapest split per cell"*
//! — every one of them TRUE of the column it stood for, and none of
//! them saying which column that was. A reader joining the report to
//! `docs/tess-budget-data/tess-budget-baseline.csv`, or to a document
//! quoting either, had to resolve the phrase in `tess_meter` first.
//!
//! **"The cheapest split" is why that is a defect and not a style
//! preference**: unqualified it names `opt_cells`, and with a trailing
//! *"per cell"* it names `span_opt_cells`. One qualifier separates two
//! columns whose sums differ by a factor of two on the committed
//! baseline, and dropping it has been the mis-read three times running.
//!
//! So the block now prints the COLUMN NAME beside each figure, and
//! this file is the gate on that pair — the names against the schema,
//! and each name against the figure it is printed beside. Three
//! claims, and they fail differently on purpose:
//!
//! * **The names are real columns** ([`the_report_names_only_real_columns`]).
//!   One direction plus its converse over the cell columns: a rename
//!   in [`EXPECTED_HEADER`] leaves the report spelling a column that
//!   no longer exists, and a cell column ADDED to the schema is one
//!   the report would otherwise omit in silence — which is exactly
//!   how `opt_cells` came to be summed and never printed.
//! * **Each figure is its own column's sum**
//!   ([`each_printed_figure_is_that_columns_sum`]). A name beside the
//!   wrong total is worse than no name at all: it invites the join and
//!   then gets it wrong. The fixture gives the four columns sums that
//!   are pairwise distinct, so any transposition reds.
//! * **The two factors name their operands**
//!   ([`the_two_factors_are_printed_as_their_formulas`]). *"held"* and
//!   *"recoverable"* are phrases with the same problem, and the fix is
//!   the same one.
//!
//! **This is not the sizing census and does not overlap it.**
//! `baseline_sizing_census.rs` asserts what the COMMITTED baseline's
//! four cell sums are, and moves when a re-cut moves them. This file
//! asserts how the report RENDERS whatever sums it was handed, over a
//! synthetic fixture that no re-cut touches. Deleting either leaves
//! the other's question unasked.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::process::Command;

use tess_lint::{EXPECTED_HEADER, cell_count_columns};

/// The columns the report's cell-total block prints, in the order it
/// prints them.
///
/// **Written out, and held against the schema's own answer.** The
/// list is authored here because what a report prints is a choice;
/// [`the_report_names_only_real_columns`] then asserts it equals
/// [`tess_lint::cell_count_columns`], which derives the sizing
/// block's cell counts from the admissibility `parse` polices them
/// with. So a cell column added to that block reds this file until
/// someone DECIDES about it, and an exclusion — there are none today
/// — arrives as an edit to that assertion carrying its reason, which
/// is what makes it a choice rather than an omission.
///
/// **Not derived from the `_cells` suffix**, which was the first
/// spelling of this pin and is wrong in both directions: `cells`, the
/// analysis-cell column, is a cell count not named that way, and a
/// future non-total could be named that way. A suffix rule also does
/// not merely miss such a column — asserted as an equality it would
/// FORCE one into the block for its spelling alone. Under the schema
/// derivation `cells` needs no exemption at all: it sits outside the
/// sizing block, so it is not in the table being filtered.
///
/// **The ORDER is load-bearing and is asserted.** `opt_cells` and
/// `span_opt_cells` are the two the report exists to keep apart, so
/// they are printed adjacent, with their figures in one eyeline. A
/// reorder that separates them gives back the mis-read this block
/// was built to prevent, and reds here.
const PRINTED: [&str; 4] = ["grid_cells", "patch_cells", "opt_cells", "span_opt_cells"];

/// What a reader is told when this file reds.
///
/// The suite lives in a cargo root OUTSIDE the workspace, so a change
/// to the schema or to the report meets it as a CI failure rather
/// than under `cargo test` at the repo root. A pin whose alarm names
/// neither the rule nor the remedy is a pin that gets deleted by
/// whoever it ambushes.
const RULE: &str = "\n  the report header names its columns: every name printed in \
     tess-lint's cell-total block is a column of EXPECTED_HEADER, and the figure beside \
     a name is that column's sum. Change the block in tools/tess-lint/src/main.rs \
     (CELL_TOTALS) and PRINTED here together. To run this suite: \
     cargo test --manifest-path tools/tess-lint/Cargo.toml";

/// One CSV row, with every column of [`EXPECTED_HEADER`] given a
/// value BY NAME.
///
/// Built from the header rather than typed positionally, for the
/// reason `cli_contract.rs` counts its empty tail from it: a column
/// inserted into the schema must not turn a fixture into a row that
/// fails for the wrong reason. Here it does better than survive — an
/// unlisted column is refused by name, so whoever adds one is told to
/// give it a value rather than left to discover a mis-parse.
fn row(fields: &[(&str, &str)]) -> String {
    let cells: Vec<&str> = EXPECTED_HEADER
        .split(',')
        .map(|col| {
            fields
                .iter()
                .find(|(name, _)| *name == col)
                .unwrap_or_else(|| {
                    panic!("this fixture gives no value for the column {col:?}{RULE}")
                })
                .1
        })
        .collect();
    format!("{}\n", cells.join(","))
}

/// A sized row: the sizing and indicator blocks filled, on the
/// `nurbs` lane.
///
/// The four cell counts are the arguments because they are what the
/// block under test prints; everything else is the least a row can
/// carry and still parse.
fn sized(face: &str, nu: &str, nv: &str, patch: &str, grid: &str, opt: &str, span: &str) -> String {
    row(&[
        ("scene", "fixture/one"),
        ("face", face),
        ("name", ""),
        ("chart", "nurbs"),
        ("delta", "1e-3"),
        ("triangles", "100"),
        ("u0", "0e0"),
        ("u1", "1e0"),
        ("v0", "0e0"),
        ("v1", "1e0"),
        ("nu", nu),
        ("nv", nv),
        ("muu", "1e0"),
        ("muv", "1e0"),
        ("mvv", "1e0"),
        ("mu1", "1e0"),
        ("mv1", "1e0"),
        ("cells", "4"),
        ("grid_cells", grid),
        ("patch_cells", patch),
        ("opt_cells", opt),
        ("span_opt_cells", span),
        ("worst_cert", "1e-4"),
        ("worst_dev", "5e-5"),
        ("dev_samples", "99"),
        ("bands", "2"),
        ("cap_bands", "1"),
        ("snap_bands", "0"),
        ("realized_aspect", "3e0"),
    ])
}

/// A row on a lane that sizes nothing, so that the block's own count
/// of Hessian-sized faces is not the sweep's face count.
fn unsized_row() -> String {
    let mut fields = vec![
        ("scene", "fixture/one"),
        ("face", "0"),
        ("name", ""),
        ("chart", "plane"),
        ("delta", "1e-3"),
        ("triangles", "7"),
    ];
    let head = fields.len();
    fields.extend(EXPECTED_HEADER.split(',').skip(head).map(|col| (col, "")));
    row(&fields)
}

/// Two sized faces whose four cell sums are 30, 300, 120 and 12.
///
/// **Pairwise distinct, and distinct from every single row's value**,
/// which is what makes the figure assertions falsifiable: a report
/// that transposed two columns, or that printed one row instead of
/// the fold, prints a number this file can name. The per-row values
/// also satisfy what `parse` refuses across columns — `patch_cells`
/// is exactly `nu · nv`, and `opt_cells` never exceeds it — so the
/// fixture reaches the report rather than the harness voice.
fn fixture() -> String {
    format!(
        "{EXPECTED_HEADER}\n{}{}{}",
        unsized_row(),
        sized("1", "1e1", "2e1", "2e2", "1e1", "4e1", "5e0"),
        sized("2", "1e1", "1e1", "1e2", "2e1", "8e1", "7e0"),
    )
}

/// The four sums [`fixture`] folds to, by column.
const SUMS: [(&str, &str); 4] = [
    ("grid_cells", "30"),
    ("patch_cells", "300"),
    ("opt_cells", "120"),
    ("span_opt_cells", "12"),
];

/// The report the CLI prints over [`fixture`], as stdout.
fn report() -> String {
    // A path per CALL, not per process. The tests in this binary run
    // as parallel THREADS of one process, so a `{pid}` path is one
    // path shared by all of them: `fs::write` truncates before it
    // writes, and another thread's CLI can read the file in that
    // window and fail to parse an empty sweep. Identical content is
    // no defence — the hazard is the truncation, not disagreement.
    // Observed as three of four tests failing on one run and none on
    // the next.
    static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let nth = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!("tess-lint-report-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join(format!("fixture-{nth}.csv"));
    std::fs::write(&path, fixture()).unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_tess-lint"))
        .arg(&path)
        .output()
        .expect("the CLI under test builds");
    let stdout = String::from_utf8(out.stdout).unwrap();
    assert!(
        out.status.success(),
        "the report exits 0 — a report is not a verdict; got {:?} on\n{stdout}{}{RULE}",
        out.status.code(),
        String::from_utf8_lossy(&out.stderr),
    );
    stdout
}

/// The block's lines, as `(column, figure, gloss)`, in printed order.
///
/// Located by the line the block opens with rather than by an
/// absolute offset, so the lines above it stay free to change, and
/// ENDED by the indent rather than by a count: the column rows are
/// the four-space lines and everything else in the report — the
/// factor line that follows them included — sits at two. Reading a
/// fixed number of lines instead would swallow whatever came next
/// and report a missing column as a malformed one.
fn printed_pairs(stdout: &str) -> Vec<(String, String, String)> {
    let mut lines = stdout.lines();
    let opener = lines
        .find(|l| l.trim_start().starts_with("cell totals over "))
        .unwrap_or_else(|| panic!("no cell-total block in the report:\n{stdout}{RULE}"));
    assert!(
        opener.contains(" 2 Hessian-sized faces"),
        "the block counts the SIZED faces, not the sweep's three rows: {opener}{RULE}"
    );
    lines
        .take_while(|l| l.starts_with("    "))
        .map(|l| {
            let mut w = l.split_whitespace();
            let column = w
                .next()
                .unwrap_or_else(|| panic!("a blank line inside the block:\n{stdout}{RULE}"));
            let figure = w.next().unwrap_or_else(|| {
                panic!("no figure beside {column:?} in the block:\n{stdout}{RULE}")
            });
            // The REST of the line, not the next token: a gloss is a
            // clause, and taking one word of it would pin whichever
            // word happened to come first.
            let gloss = w.collect::<Vec<_>>().join(" ");
            assert!(
                !gloss.is_empty(),
                "{column} is printed with a figure and no gloss{RULE}"
            );
            (column.to_string(), figure.to_string(), gloss)
        })
        .collect()
}

/// Every name the block prints is a column of the file it is a report
/// about, and every cell column of that file is one the block prints.
///
/// The first direction is the one a rename breaks; the second is the
/// one an ADDITION breaks, and it is the direction `opt_cells` was
/// lost in — summed by `SceneTotals`, asserted by the sizing census,
/// and printed by nothing.
#[test]
fn the_report_names_only_real_columns() {
    let schema: Vec<&str> = EXPECTED_HEADER.split(',').collect();
    for name in PRINTED {
        assert!(
            schema.contains(&name),
            "the report prints {name:?}, which is not a column of the sweep{RULE}"
        );
    }
    assert_eq!(
        PRINTED.as_slice(),
        cell_count_columns().as_slice(),
        "the block's roster against the sizing block's own cell-count columns — either a \
         cell column was added to the schema and the report has not decided whether to \
         print it, or the report prints one that is not a sizing total{RULE}"
    );
    assert_eq!(
        printed_pairs(&report())
            .iter()
            .map(|(c, _, _)| c.as_str())
            .collect::<Vec<_>>(),
        PRINTED,
        "the block's columns, in printed order{RULE}"
    );
}

/// The figure printed beside a column name is that column's sum.
///
/// The claim that actually protects a citing document. `SUMS` is
/// distinct in every entry, so a transposed accessor in `CELL_TOTALS`
/// — the one edit a positional format list used to make invisible —
/// reds naming both the column and the number it was given.
#[test]
fn each_printed_figure_is_that_columns_sum() {
    let printed = printed_pairs(&report());
    for (column, sum) in SUMS {
        let (_, figure, _) = printed
            .iter()
            .find(|(c, _, _)| c == column)
            .unwrap_or_else(|| panic!("the block does not print {column:?}{RULE}"));
        assert_eq!(
            figure, sum,
            "the block prints {figure} beside {column}, whose sum over the fixture is \
             {sum}{RULE}"
        );
    }
}

/// The qualifier that separates the twins is printed on the twin it
/// belongs to, and on no other line.
///
/// **The gloss is the block's third field and nothing read it before
/// this test.** Names and figures could all be right while
/// `opt_cells` carried "PER CELL" and `span_opt_cells` carried
/// "WHOLE-PATCH" — the qualifier on the wrong figure, which is the
/// exact mis-read this file exists to close and which the module doc
/// above already claimed to gate.
///
/// **The two words, not the two sentences.** Asserting the glosses
/// verbatim would hand-twin `main.rs`'s prose into this file, which
/// this crate has a standing row about; what is load-bearing is which
/// column each qualifier lands on. So that is pinned and the wording
/// stays free to change.
#[test]
fn each_twins_qualifier_is_printed_on_its_own_column() {
    let printed = printed_pairs(&report());
    for (qualifier, owner) in [("WHOLE-PATCH", "opt_cells"), ("PER CELL", "span_opt_cells")] {
        let carriers: Vec<&str> = printed
            .iter()
            .filter(|(_, _, gloss)| gloss.contains(qualifier))
            .map(|(c, _, _)| c.as_str())
            .collect();
        assert_eq!(
            carriers,
            [owner],
            "{qualifier:?} is what tells {owner} from its twin, so it belongs on that \
             line and on no other{RULE}"
        );
    }
}

/// The two factors are printed as their formulas, in the columns'
/// own names — in the totals block, and again in the per-scene
/// table's legend, which glosses the same two quotients.
///
/// *"held"* and *"still recoverable"* are the same kind of phrase as
/// *"the cheapest split"* — true, and silent about which pair of
/// columns they are the quotient of. Over [`fixture`] the two are
/// `300 / 30` and `30 / 12`, which are different numbers, so a
/// swapped pair in the block reds rather than printing a plausible
/// ratio; in the legend, where no figure appears, the formula is the
/// whole of the claim and a swap reds on the text alone.
///
/// **The legend's third factor is deliberately not here.** `total` is
/// triangles against an extrapolation, not a quotient of two columns,
/// so there is no formula for it to carry and asserting one would be
/// asking the report to invent a join it does not have.
#[test]
fn the_two_factors_are_printed_as_their_formulas() {
    let stdout = report();
    for phrase in [
        "patch_cells / grid_cells = 10.0x held",
        "grid_cells / span_opt_cells = 2.5x still recoverable",
        "held = patch_cells / grid_cells",
        "split = grid_cells / span_opt_cells",
    ] {
        assert!(
            stdout.contains(phrase),
            "the report does not state {phrase:?}:\n{stdout}{RULE}"
        );
    }
}

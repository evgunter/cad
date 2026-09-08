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

use tess_lint::EXPECTED_HEADER;

/// The columns the report's cell-total block prints, in the order it
/// prints them.
///
/// **Written out rather than derived, because each entry is a choice.**
/// What the report prints is not "every column that happens to be a
/// cell count": `cells` is one too — the analysis cells the per-cell
/// bound reported — and it is deliberately not here, because
/// [`tess_lint::SceneTotals`] does not sum it and a total of it would
/// answer no question the block asks. A derivation would have quietly
/// made that choice for whoever wrote the next column.
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
    let dir = std::env::temp_dir().join(format!("tess-lint-report-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("fixture.csv");
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

/// The block's lines, as `(column, figure)`, in printed order.
///
/// Located by the line the block opens with rather than by an
/// absolute offset, so the lines above it stay free to change, and
/// ENDED by the indent rather than by a count: the column rows are
/// the four-space lines and everything else in the report — the
/// factor line that follows them included — sits at two. Reading a
/// fixed number of lines instead would swallow whatever came next
/// and report a missing column as a malformed one.
fn printed_pairs(stdout: &str) -> Vec<(String, String)> {
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
            (column.to_string(), figure.to_string())
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
    for col in &schema {
        assert_eq!(
            col.ends_with("_cells"),
            PRINTED.contains(col),
            "the column {col:?} and the report's block disagree about whether it is one of \
             the cell totals{RULE}"
        );
    }
    assert_eq!(
        printed_pairs(&report())
            .iter()
            .map(|(c, _)| c.as_str())
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
        let (_, figure) = printed
            .iter()
            .find(|(c, _)| c == column)
            .unwrap_or_else(|| panic!("the block does not print {column:?}{RULE}"));
        assert_eq!(
            figure, sum,
            "the block prints {figure} beside {column}, whose sum over the fixture is \
             {sum}{RULE}"
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

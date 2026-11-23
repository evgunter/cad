//! The two-face CSV fixture, with ONE definition. This crate's own
//! `#[cfg(test)]` module owns it as `tests::csv_fixture` and
//! `tests/cli_contract.rs` mounts the same file by path: the two are
//! separate compilation units and an integration test cannot see a
//! `#[cfg(test)]` item, so they cannot SHARE an item — but they can
//! mount one, which is the difference between two copies kept in step
//! by a comment and a fixture with one home.
//!
//! Two things a mounting site owes:
//!
//! * `EXPECTED_HEADER` nameable as `super::EXPECTED_HEADER` — `use
//!   super::*` inside the crate, `use tess_lint::EXPECTED_HEADER`
//!   outside it. Nothing else from the crate is used here: the column
//!   indices are read out of the header, which both cargo roots can
//!   see, rather than out of the private `NAME` and `IDENTITY_FIRST`,
//!   which the crate's own header test pins against this same header.
//! * a MOUNT and not an `include!` — one thing to the compiler and
//!   two to `crates/test-utils/tests/reader_census.rs`, which reads
//!   `#[path = "` as a mount and anything else naming a `.rs` file as
//!   a source reader, a class this file does not belong to and has no
//!   honest ledger line for. **A pointer, not a pin**: nothing here
//!   reds if that rule moves, and the census is the authority over it.
//!   The mount is also why the file sits under `src/` — `#[path]`
//!   inside an inline `mod tests` resolves against `src/tests/`, which
//!   has to exist for a relative path to open.
//!
//! [`the_fixture_fills_the_head_block_the_header_declares`] lives here
//! rather than beside one mounting site, so every binary that builds a
//! row also checks it. `name` is read by no rule and printed by no
//! report, so **its token has exactly two readers and they divide the
//! way the mount does**: this file's test says the FIXTURE writes the
//! token at the column the header names, in both binaries, and
//! `tess_lint`'s `parses_both_chart_shapes` says `parse` READS it back
//! out of that column into `Row::name`, on the crate side only.
//! Neither substitutes for the other — one never calls `parse`, the
//! other never inspects the text.

use super::EXPECTED_HEADER;

/// A `name` token of the shape the tour writes: structural, flat, and
/// carrying no `,`. The sized row carries it and the unsized rows go
/// unnamed, so both spellings of the column — a token and the honest
/// absence — reach every reader of this fixture.
pub(crate) const FIXTURE_NAME: &str = "{\"kind\":\"Face\";\"node\":3;\"path\":[\"OutputBody\"]}";

/// A two-face scene: one plane (empty NURBS columns) at ordinal 0, one
/// NURBS wall at ordinal 1.
///
/// `tris` is the wall's triangle count and `span_opt` the cheapest
/// per-cell grid; the two move the gate's two rules independently, and
/// `span_opt = 0` makes the slack ratio's denominator unreadable — a
/// harness-breakage input rather than a measurement.
pub(crate) fn scene(tris: usize, span_opt: f64) -> String {
    format!(
        "{EXPECTED_HEADER}\n{}\
         s/b,1,{FIXTURE_NAME},nurbs,2e-3,{tris},0e0,1e0,0e0,1e0,1e1,2e1,1e0,1e0,1e0,\
         2e0,3e0,4,1e2,2e2,5e1,{span_opt:e},1e-4,5e-5,99,2,1,0,3e0\n",
        unsized_row(0, "plane", 4)
    )
}

/// A row on a lane that sizes nothing, at a chosen ordinal and chart —
/// enough to move a scene's roster without moving a triangle.
///
/// The empty tail is COUNTED from the header, never typed, and **the
/// head is counted too**: a column inserted before `u0` moves the
/// tail's start and the two moves cancel, so a row that counted only
/// its tail would keep the header's width and lose a field in fact.
/// The head is typed out rather than built from the header — a fixture
/// that built it from the header would be asserting the header against
/// itself — so its width is checked against where the tail begins.
pub(crate) fn unsized_row(face: usize, chart: &str, tris: usize) -> String {
    let first = column("u0");
    let head = format!("s/b,{face},,{chart},2e-3,{tris}");
    assert_eq!(
        head.split(',').count(),
        first,
        "the fixture's head is {} fields and the header's is {first}: a head column was \
         added and this row would go in short",
        head.split(',').count()
    );
    let blanks = ",".repeat(EXPECTED_HEADER.split(',').count() - first);
    format!("{head}{blanks}\n")
}

/// Where the header puts the column called `name` — the one index both
/// cargo roots can compute, since the crate's own index constants are
/// private.
fn column(name: &str) -> usize {
    EXPECTED_HEADER
        .split(',')
        .position(|c| c == name)
        .unwrap_or_else(|| panic!("the header names no `{name}` column"))
}

/// The fixture's head block carries the values it declares, at the
/// columns the header puts them in.
///
/// **Width is loud without this; content is heard in two places and
/// only one of them is here.** A row that loses a field fails the
/// parse in whichever test reaches it first. A row that keeps its
/// width and blanks a head token reaches exactly the two assertions
/// that read the token: this one, and the crate-side
/// `parses_both_chart_shapes`, which round-trips it through `parse`.
/// This is the fixture-side half, and the only half the integration
/// binary has: it reads each head field by the header's index for its
/// column — never by counting the literal, which is the thing under
/// test — and says the sized row is named and the unsized row is not.
#[test]
fn the_fixture_fills_the_head_block_the_header_declares() {
    let text = scene(100, 2.5e1);
    // The first line is not checked, and deliberately: `scene`
    // interpolates `EXPECTED_HEADER` to build it, so asserting the two
    // are equal would be asserting one expression against itself and
    // could not fail — the crate's header can be rewritten wholesale
    // and such a row stays green. What is typed out, and therefore
    // what is under test, is everything BELOW that line: the width and
    // head-field checks read the crate's header as the oracle and the
    // row literals as the subject.
    let rows: Vec<&str> = text.lines().skip(1).collect();

    // A name carrying a comma would split into two fields and every
    // measurement after it would be read one column to the left.
    assert!(
        !FIXTURE_NAME.is_empty() && !FIXTURE_NAME.contains(','),
        "the fixture's name is a non-empty, comma-free token: {FIXTURE_NAME:?}"
    );

    let want = [
        [
            ("scene", "s/b"),
            ("face", "0"),
            ("name", ""),
            ("chart", "plane"),
        ],
        [
            ("scene", "s/b"),
            ("face", "1"),
            ("name", FIXTURE_NAME),
            ("chart", "nurbs"),
        ],
    ];
    assert_eq!(rows.len(), want.len(), "the fixture is a two-face scene");
    for (row, head) in rows.iter().zip(want) {
        let field: Vec<&str> = row.split(',').collect();
        assert_eq!(
            field.len(),
            EXPECTED_HEADER.split(',').count(),
            "the row is not the header's width: {row}"
        );
        for (name, value) in head {
            assert_eq!(field[column(name)], value, "the fixture's `{name}` field");
        }
    }
}

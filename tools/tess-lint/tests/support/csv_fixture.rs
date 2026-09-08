// The two-face CSV fixture, with ONE definition. This crate's own
// `#[cfg(test)]` module and `tests/cli_contract.rs` `include!` this
// text: they are separate compilation units and an integration test
// cannot see a `#[cfg(test)]` item, so the two cannot SHARE an item —
// but they can include one, which is the difference between two
// copies kept in step by a comment and a fixture with one home.
//
// Not a module and not a target: Cargo builds a test from `tests/*.rs`
// and `tests/*/main.rs` only, so nothing here compiles on its own and
// the text lands in whatever scope includes it. What an includer owes:
//
// * `EXPECTED_HEADER` in scope — `use super::*` inside the crate,
//   `use tess_lint::EXPECTED_HEADER` outside it. Nothing else from the
//   crate is used here, deliberately: the column indices are read out
//   of the header, which both cargo roots can see, rather than out of
//   `NAME` and `IDENTITY_FIRST`, which are private. The crate's own
//   header test pins those constants against this same header, so
//   reading the header costs no pin.
// * hand formatting. `rustfmt` does not follow `include!` and builds
//   no target from this directory, so `cargo fmt --check` is silent
//   about this file in both roots.
//
// [`the_fixture_fills_the_head_block_the_header_declares`] is included
// with the fixture rather than written beside one includer, so every
// binary that builds a row also checks it. That is the point: `name`
// is read by no rule and printed by no report, so the fixture's own
// test is the only thing in either root that can see its token.

/// A `name` token of the shape the tour writes: structural, flat, and
/// carrying no `,`. The sized row carries it and the unsized rows go
/// unnamed, so both spellings of the column — a token and the honest
/// absence — reach every reader of this fixture.
const FIXTURE_NAME: &str = "{\"kind\":\"Face\";\"node\":3;\"path\":[\"OutputBody\"]}";

/// A two-face scene: one plane (empty NURBS columns) at ordinal 0, one
/// NURBS wall at ordinal 1.
///
/// `tris` is the wall's triangle count and `span_opt` the cheapest
/// per-cell grid, which together move the two gate rules independently
/// — and, at `span_opt = 0`, produce the unreadable denominator the
/// CLI's harness-voice row needs.
fn scene(tris: usize, span_opt: f64) -> String {
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
fn unsized_row(face: usize, chart: &str, tris: usize) -> String {
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
/// **Width was already loud and content was silent.** A row that loses
/// a field fails the parse in whichever test reaches it first; a row
/// that keeps its width and blanks a head token failed nothing, on
/// either side, because `name` reaches no rule and no report and the
/// only assertion over it compared a fixture's constant with itself.
/// So this reads each head field by the header's index for its column
/// — never by counting the literal, which is the thing under test —
/// and says the sized row is named and the unsized row is not.
#[test]
fn the_fixture_fills_the_head_block_the_header_declares() {
    let text = scene(100, 2.5e1);
    let (header, body) = text
        .split_once('\n')
        .expect("the fixture has a header line");
    assert_eq!(
        header, EXPECTED_HEADER,
        "the fixture's header is the crate's"
    );

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
    let rows: Vec<&str> = body.lines().collect();
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

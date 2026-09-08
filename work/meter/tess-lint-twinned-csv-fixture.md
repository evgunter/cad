---
id: tess-lint-twinned-csv-fixture
kind: unit
title: tess-lint's test CSV fixture is hand-twinned across the crate/integration boundary, in three literals with a silent half
status: closed
branch: meter/12-twinned-csv-fixture
opened: 2026-09-07
closed: 2026-09-08
pr: 2179
---

Filed by the style-review fix pass on `meter/join-gated-voice` (PR 2111)
as an issue. **Re-filed as a unit by METER unit 6's fix pass**: unit 6
WIDENED the duplication this row predicted, and the measurements below
show the twinning has a half that reds nothing at all — which makes it
a defect with a scoped fix rather than an observation.

## The three copies

`tools/tess-lint/src/lib.rs`'s `tests::csv` and
`tools/tess-lint/tests/cli_contract.rs`'s `scene` build the SAME
two-face fixture — one plane at ordinal 0, one NURBS wall at ordinal 1,
parameterised by `tris` and `span_opt` — from two copies of one
29-column format string. Both comment sites say so and both end with
**"Keep them in step."**, which is an invariant nothing enforces.

`unsized_row` is a second twinned pair, spelled differently on each
side: the lib copy reads `IDENTITY_FIRST` from the crate, the
integration copy re-derives the same index by
`position(|c| c == "u0")`.

**`FIXTURE_NAME` is a third**, added by METER unit 6 — the same
`{"kind":"Face";"node":3;"path":["OutputBody"]}` literal declared once
per side, each with its own doc comment. Unit 6's PR body said the
`name` column's twinning "would have reddened nothing"; that is half
false and the half that is true is the worse half.

## What actually reds, measured

At `meter/split-scan-and-face-name` after its fix pass, one side edited
at a time:

| edit | lib tests | `cli_contract` |
|---|---|---|
| drop the whole `name` column (width changes) | 42 red | 9 red |
| keep the width, blank the token only | **1 red** | **0 red** |

The width half is loud on both sides because a short row fails the
field count. **The TOKEN half is where the twinning actually hides**:
before unit 6's fix pass it reddened nothing on either side, because
nothing in the crate read `Row::name` after `parse` wrote it. The one
red in that cell today is the assertion that fix pass added
(`parses_both_chart_shapes`, `src/lib.rs`), and it exists on the lib
side only — the integration copy's token can still be silently changed
to anything comma-free.

So the twinning's cost is not "the two could drift"; it is that one of
them has no reader, and adding a column to one side and not the other
is caught by arithmetic rather than by intent.

## Cures, cheapest first

- an `include!`d fixture file both sides pull in — one literal, both
  spellings of `unsized_row` gone with it;
- a `#[doc(hidden)] pub mod fixtures` behind a `test-support` feature;
- one test that asserts the two strings are equal for a couple of
  `(tris, span_opt)` points, which enforces "in step" without moving
  either copy. **Weakest of the three**, and the measurements above say
  why: it makes drift loud but leaves three literals in the tree.

The stated reason for the split is real — an integration test cannot
see a `#[cfg(test)]` item — but it is a reason not to share THAT item,
not a reason to have no shared one.

## Sweep note

The `Keep them in step` / `The twin of` vocabulary turns up nowhere
else under `tools/tess-lint`; what that grep cannot match is an
undisclosed copy, and the constants sweep for one is the row literal
itself, which has exactly these two occurrences, plus `FIXTURE_NAME`'s
two. What neither sweep can match is a fixture built by a DIFFERENT
route to the same shape, which is what a third consumer would look
like.

Fence: `tools/tess-lint/*`, METER's.

## Re-measured on `main` after unit 6 (2026-09-08)

The table above was measured at `meter/split-scan-and-face-name`. Both
mutations were reproduced on `8c6770a` (unit 6 merged), one side edited
at a time, each edit confirmed in `git diff` and each run confirmed to
have recompiled. `tools/tess-lint` from its own root, its own
`CARGO_TARGET_DIR`. Green baseline: 54 lib, 14 `cli_contract`.

**One edit at a time, and the whole grid measured** — three mutations
x two sides x {lib tests, `cli_contract` tests, `cargo clippy
--all-targets -- -D warnings`}. Nothing here is inferred from a
neighbouring cell.

| edit, made on the LIB side only | lib | `cli_contract` | clippy |
|---|---|---|---|
| drop the `name` field from the sized row (width changes) | 42 red | 0 red | green |
| keep the width, blank the token (`{FIXTURE_NAME}` -> nothing) | 1 red | 0 red | green |
| keep the width, change the const's VALUE | **0 red** | **0 red** | **green** |

| edit, made on the `cli_contract` side only | lib | `cli_contract` | clippy |
|---|---|---|---|
| drop the `name` field from the sized row (width changes) | 0 red | 9 red | red |
| keep the width, blank the token (`{FIXTURE_NAME}` -> nothing) | 0 red | **0 red** | red |
| keep the width, change the const's VALUE | **0 red** | **0 red** | **green** |

The `42 red / 9 red` and `1 red / 0 red` cells confirm the filing. The
third row of each is the one the filing did not measure and it is the
twinning's actual failure mode: with two constants, one side's token
can be respelled to anything and no test and no lint anywhere in the
tree sees it.

**The clippy column is side-specific, and it is reachability rather
than content.** Every red in it is `error: constant FIXTURE_NAME is
never used` at `tests/cli_contract.rs:75:7` (`-D dead-code` implied by
`-D warnings`): on that side the token in the row literal was the
const's ONLY use, so removing it either way makes the const dead. The
same two edits on the lib side leave the const used by unit 6's own
assertion and clippy stays green — and the value-drift row keeps the
use on both sides, so clippy is green there whatever the value says.
A lint that tracks reachability cannot see a wrong value.

## Closed

Landed as METER unit 12. `tools/tess-lint/src/tests/csv_fixture.rs` is
the fixture's one home: `FIXTURE_NAME`, `unsized_row` and the two-face
`scene` are one file, owned by the crate's test module as
`tests::csv_fixture` and mounted by `tests/cli_contract.rs` with
`#[path]`. The cure is the item's first, and it took the second
`unsized_row` spelling with it — the header-derived index is the one
both cargo roots can compute, since `NAME` and `IDENTITY_FIRST` are
private and making them public is non-test code.

`the_fixture_fills_the_head_block_the_header_declares` lives in that
file rather than beside one mounting site, so it runs in both binaries:
it reads each head field by the header's index for that column and
asserts the sized row is named and the unsized row is not. After the
fold, blanking the token reds **2 lib / 1 `cli_contract`** and dropping
the field reds **44 lib / 10 `cli_contract`**; the value-drift mutation
cannot be written, because there is one constant.

**The token has two readers and the tree now says so in both places.**
`csv_fixture.rs`'s test says the FIXTURE writes the token at the column
the header names, in both binaries; `parses_both_chart_shapes`
(`src/lib.rs`) says `parse` READS it back out of that column into
`Row::name`, on the crate side only. Blanking the token reds both, so
neither is the sole reader and neither comment claims to be.

**What the fixture's test does NOT assert, and why.** It does not
compare the fixture's header line with `EXPECTED_HEADER`: `scene`
interpolates that constant to build the line, so the two sides are one
expression and the assertion could not fail — the crate's header can be
rewritten wholesale with such a row staying green. What is typed out is
everything below that line, and the width and head-field checks read
the crate's header as the oracle against those row literals. A column
appended to `EXPECTED_HEADER` reds the width check.

**The blind spot the sweep named had a live instance inside the fence,
and it is folded.** `a_half_filled_sizing_row_is_harness_breakage`
typed its own 29-column sized row — sharing no literal with the fixture
and living inside one file, which is exactly the pair of blind spots
the sweep disclosed. It now blanks one sizing column of the fixture
through `with_column`, and names the column it blanks. Measured both
ways: append a column to `EXPECTED_HEADER`, update the ONE fixture
literal, and the folded tree reds **one test in the lib binary** —
`the_policed_block_is_the_headers_sizing_block`, the schema pin, which
is the right reason. Before the fold the same drill also red
`a_half_filled_sizing_row_is_harness_breakage` with `29 fields,
expected 30`: right test, wrong reason.

**Crate-wide that drill reds five, and the other four are correct.**
`cargo test --no-fail-fast` after the fold: the one lib red above, plus
three in `baseline_census.rs` and one in `baseline_sizing_census.rs`,
each reading the committed baseline — whose header is the old one, so a
schema change is exactly what they exist to report. `cli_contract` and
`cut_line_pin` stay green. The lib-binary figure is quoted above
because it is the one the fold moves; it is scoped and not a
crate-wide count, and `cargo test` fail-fasts at the lib binary, so a
drill run without `--no-fail-fast` never sees the other four.

The two other hand-typed rows in the module (`a_short_row_is_harness_breakage`
and `a_cut_line_shifts_the_reported_line_numbers`) stay typed: both are
deliberately SHORTER than the header, that shortness is their subject,
and a column added to the schema leaves them short and reporting the
same thing.

**Why a mounted module and not an `include!`.** The first attempt used
`include!`, and `crates/test-utils/tests/reader_census.rs` reddened
CI for it: its detector counts a file as a source reader when it *names
more `.rs` files than it MOUNTS*, and mounting is spelled `#[path = "`,
which an `include!` is not. The census's ledger has no honest
disposition for a compile-time mount — the file reads no source, it IS
source — so the fix is the spelling the detector understands, not a
ledger line. That is also why the file sits under `src/`: a `#[path]`
on a module inside an inline `mod tests` resolves against `src/tests/`,
and a relative path cannot open through a directory that does not
exist.

## Residue

`work/meter/tess-lint-recourse-quote-half-pinned` — the half-pin on
`docs/TESS-BUDGET.md`'s quoted recourse sentence, disclosed in
`tools/tess-lint/tests/cli_contract.rs`'s own header since
`d829ddfee` and never given a file. This unit's sweep dispositioned it
as *"a pin, not a twin"*, which rules on the twinning class and says
nothing about the unfiled residue; the file is that.

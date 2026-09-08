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

| edit | lib tests | `cli_contract` | `cargo clippy -D warnings` |
|---|---|---|---|
| drop the `name` field from the sized row (width changes) | 42 red | 9 red | red |
| keep the width, blank the token (`{FIXTURE_NAME}` -> nothing) | 1 red | 0 red | **red** |
| keep the width, change the const's VALUE on one side | **0 red** | **0 red** | **green** |

The first two rows confirm the filing. The third is the one the filing
did not measure and it is the twinning's actual failure mode: with two
constants, one side's token can be respelled to anything and no test
and no lint anywhere in the tree sees it. The blank-token row's clippy
red is not a content check either — it is `dead_code` firing because
that particular spelling of the mutation removes the const's last use.

## Closed

Landed as METER unit 12. `tools/tess-lint/tests/support/csv_fixture.rs`
is the fixture's one home: `FIXTURE_NAME`, `unsized_row` and the
two-face `scene` are one text, `include!`d by the crate's test module
and by `tests/cli_contract.rs`. The cure is the item's first, and it
took the second `unsized_row` spelling with it — the header-derived
index is the one both cargo roots can compute, since `NAME` and
`IDENTITY_FIRST` are private and making them public is non-test code.

`the_fixture_fills_the_head_block_the_header_declares` is included with
the fixture rather than written beside one includer, so it runs in both
binaries: it reads each head field by the header's index for that
column and asserts the sized row is named and the unsized row is not.
After the fold, blanking the token reds **2 lib / 1 `cli_contract`**
(`tests/support/csv_fixture.rs`, "the fixture's `name` field"), and
dropping the field reds **43 lib / 10 `cli_contract`** — the extra red
in each is the fixture's own test, which names the fixture and prints
the offending row instead of surfacing as a `ParseError` inside a test
about something else. The third mutation no longer exists: there is one
constant.

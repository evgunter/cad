---
id: reader-census-shared-disposition-survives-partial-reversion
kind: issue
title: A Shared ledger row is checked by one substring, so a site that keeps the import and re-adds hand-rolled helpers stays green
status: open
opened: 2026-09-07
priority: P4
cost: E
---


## What

`crates/test-utils/tests/reader_census.rs:22-24` states what a ledger
row is for: *"it is a new hand-rolled Rust reader — do not add the
line. Use the shared lexer. That is the whole point of the row."*
`every_shared_entry_actually_reaches_the_shared_lexer` (`:584-603`) is
the check that a `Shared` row is honest, and it is one substring:

```rust
!test_utils::source::code_only(&text).contains("test_utils::source")
```

A bare `use test_utils::source;` import satisfies it with **no call at
all**. Its own doc names the intended failure as a site that "reverts to
a hand-rolled reader"; **partial** reversion — keep the import, re-add
hand-rolled helpers beside it — is what slips through, and the check
cannot see the difference.

## Finding

`meter/klint-roster-pin` is the instance. `tools/k-lint/tests/predicate_roster.rs`
reached the shared lexer for its two blanked views AND hand-rolled
`plain_string_literal`, `initializers` and `sole_initializer`
byte-for-byte from `tools/tess-meter/tests/derivations.rs` — the ledger
row two lines beneath it. It was dispositioned `Shared`, which was
literally true and one level too coarse, and the check was green
throughout. The fix pass hoisted the three helpers into
`crates/test-utils/src/source.rs`, so that row is now honest; the check
that was supposed to notice still cannot.

**This is a class, not an instance.** Every `Shared` row in `LEDGER`
(there are dozens) makes the same claim and gets the same one-substring
audit, so any of them could be carrying its own hand-rolled helpers on
top of the lexer. Nobody has looked.

## What a unit here does

The hard half is defining *reaches the shared lexer for everything it
needs* mechanically. Candidate shapes, cheapest first:

- **require a CALL, not an import**: `source::` followed by an
  identifier and `(`. Catches the bare-import case; does not catch a
  file that calls one view and hand-rolls the rest.
- **red on a second lexer beside the import**: the population is
  recognisable — a `match` over `b'/'` / `b'"'`, a `chars()` walk
  tracking `in_comment`, a `depth` counter over brackets. A needle set
  over the code view, with each `Shared` row's hits listed and
  dispositioned once, is the shape the census itself already uses.
- **or accept the coarseness and say so at the row's doc**, which is
  the cheap honest option and which the current doc does not do — it
  promises the strong reading.

**Confidence:** sure the substring is satisfiable by an import alone
(demonstrated by this PR); sure the doc promises more than the check
delivers; unsure how many other `Shared` rows are in the same state,
because auditing them is the unit.

## Was

Raised by the style review of `meter/klint-roster-pin` re-run against
its ledger commit (STYLE-9, `sure`), on the orchestrator's own ledger
fix. Filed rather than fixed by that PR's fix pass: strengthening the
check requires ruling on every `Shared` row, which is wider than the
unit.

## Re-homed to S-TCOST (2026-09-08)

Moved from `work/meter/` to `work/tcost/` by `git mv` as METER's exit walk
disposed of its residue (`docs/METER-EXIT-WALK.md` §5, ratified by Ev on
2026-09-08 at PR #2212). Id and body unchanged.

`work/tcost/program.md`'s `paths` carries `crates/test-utils/*`, and the
whole fix is in `crates/test-utils/tests/reader_census.rs` and
`crates/test-utils/src/source.rs`. The walk's alternative candidate was
CIW, whose `paths` names no `crates/test-utils` path and whose `keep_out`
cedes test mechanism. METER found the instance and cannot rule on the
class: strengthening the check means dispositioning every `Shared` row in
`LEDGER`, which is this program's ruling to make.

## Moved to S-TINT (2026-09-11)

Moved by `git mv` from `work/tcost/` at S-TINT's opening. Id, title and
body are unchanged; the directory is the claim (`work/README.md`).

**Why it moved.** S-TCOST's board was re-sorted on 2026-09-11 against the
repository going public on 2026-09-03 (`work/tcost/log.md`, the
2026-09-11 seam). That sort found this row is not a cost lever in either
currency — it neither shortens the gate's critical path nor saves a
billed minute, and it was never argued on one. It reached S-TCOST by the
tracker-wide re-home of 2026-09-04, which routed rows by PATH GLOB
(`crates/*/tests/*`, `crates/test-utils/*`) rather than by question.
This program is the question it was always about: whether the suite
asserts what it claims to assert.

## Re-derived (2026-09-15, lane C)

**VERDICT: REPRODUCES** — the check is byte-for-byte the one this row
describes, the `Shared` population has grown, and the audit the row said
nobody had done turns up a live partial reversion.

**The check.** `every_shared_entry_actually_reaches_the_shared_lexer` in
`crates/test-utils/tests/reader_census.rs` still filters on the single
substring

```rust
!test_utils::source::code_only(&text).contains("test_utils::source")
```

and its rustdoc still promises the strong reading (*"a converted site
that reverts to a hand-rolled reader keeps its `Shared` line … and the
census stays green over exactly the change it was built to catch"*).
None of the three candidate shapes in `## What a unit here does` has
been taken: there is no call-shaped needle, no second-lexer needle set,
and the doc does not accept the coarseness. The stricter row the D261
review wrote (`every_shared_entry_reads_through_a_view_not_only_a_traversal`,
quoted in the sibling row `source-lacks-an-item-body-carve-…`) is not in
the file — `grep -n "reads_through_a_view"` over the tree returns
nothing.

**The population, re-derived.** Parsing `LEDGER` (`python3`, regex over
`path:`/`disposition:` pairs) gives **63 entries: 57 `Shared`, 4
`Unconverted`, 1 `NotRust`, 1 `Home`**. `UNCONVERTED_TODAY` is `4` and
matches. "Dozens" in the body is right and is now 57.

**The row's own fix is in the tree.** `plain_string_literal`,
`initializers` and `sole_initializer` are `pub fn`s of
`crates/test-utils/src/source.rs`, and `tools/k-lint/tests/predicate_roster.rs`
and `tools/tess-meter/tests/derivations.rs` both call
`source::plain_string_literal` rather than carrying copies. So the
INSTANCE is repaired and the CLASS check is not — exactly the state the
row predicted.

**The audit, done here for the first time (the part "nobody has looked"
covered).** Over all 57 `Shared` files, with `//`-comments stripped:

- *Bare import, no call at all*: **none**. Every one of the 57 reaches at
  least one `test_utils::source` item by name, so the demonstrated
  satisfiability has no current instance.
- *No VIEW call, only a traversal or a carve*: **one** —
  `crates/topo/src/live.rs`, which imports `{ItemBody, balanced_end,
  item_body}` and takes its blanked text from `source_walk::CodeOnly::of`
  (topo's adapter). The reviewer's proposed stricter row would go RED on
  it today; it was green at PR 1919's head. Whoever takes this owes
  `live.rs` a disposition before writing that row.
- *A second lexer beside the import (the partial reversion this row is
  named for)*: **one live instance — `crates/pncad/tests/all.rs`.** It is
  dispositioned `Shared`, imports `{ItemBody, balanced_end,
  code_and_literals, code_only, item_body}`, and beside that hand-rolls
  `string_literals` (a byte walk over `b'"'` honouring `\` escapes — the
  lexer arm `source.rs` documents as the one "this tree has got wrong
  three times"), `token_starts_at` (an identifier-boundary predicate that
  duplicates `source::boundary_before`), `line_of` (duplicates
  `source::line`) and a `depth`-counting argument splitter (duplicates
  `source::top_level_split`). Every one of the row's three needle shapes
  fires on it, and the check is green.

  Needle set used: `b'/'`/`b'"'`/`b'\''` byte matches, and the
  identifiers `in_comment`/`in_string`/`in_literal`/`in_block`, over each
  `Shared` file with line comments stripped.
  **What that sweep could not match**: a hand-rolled reader written over
  `chars()` with differently-named state, a copy behind a macro, one in a
  module the file `#[path]`-mounts, and a lexer whose only tell is a
  `depth` counter (searched for separately and too noisy to be a needle —
  `top_level_split`-shaped code is everywhere). The three `in_*` hits on
  `wire_operand_door.rs`, `predicate_roster.rs` and `derivations.rs` were
  false positives: all three are the substring `plain_string_literal`.

**Recommendation (orchestrator's call).** Keep open. The unit is
unchanged in shape but now has two named subjects to disposition —
`crates/pncad/tests/all.rs` (partial reversion, live) and
`crates/topo/src/live.rs` (reaches a view only through an adapter, and is
dispositioned `Shared` while `crates/topo/src/boolean/boxes.rs` is
`Unconverted` with a reason that names exactly that situation). That
asymmetry is new since filing and is reported separately.

## TINT-3 narrowed this row's options (2026-09-15)

`tint/3-aggregation-guard` (PR #2680) collapsed the fifteen copies of
`every_suite_file_is_aggregated` onto one macro, and in doing so **added
a second, weaker substring to exactly the check this row is about** and
**handed the unit a fifteen-file exception to disposition**. Disclosed
here at the moment it was done, not after.

### What changed in the check

`every_shared_entry_actually_reaches_the_shared_lexer` no longer tests
one substring. It reads a two-entry `SHARED_LEXER_DOORS`:

```rust
const SHARED_LEXER_DOORS: [&str; 2] = [
    "test_utils::source",
    concat!("test_utils::", aggregation_row_macro!()),
];
```

An aggregating `tests/all.rs` reaches the lexer through
`test_utils::every_suite_file_is_aggregated!()`, whose expansion — not
its text — holds the `source::` path. So the second door is satisfied
without the file naming `test_utils::source` at all.

### The cost to this row's cheapest candidate fix

`## What a unit here does` lists, first and cheapest: *"require a CALL,
not an import: `source::` followed by an identifier and `(`"*. **That
needle now goes RED on fifteen honest files** — every `tests/all.rs` in
`bvh`, `editor-core`, `geom`, `geom-brep`, `geom-core`, `mesh`,
`profile`, `step-export`, `step-import`, `stl`, `sweep`, `test-utils`,
`topo`, `verbs` and `viewer`. None of them reverted to anything; the
call moved into a macro body one crate away. Whoever takes this unit
either widens the call-shaped needle to admit the macro invocation
(which is the second door again, so the needle stops being strictly
call-shaped) or dispositions the fifteen as a named exception. **That
choice is now part of the unit and was not before.**

The second candidate (*"red on a second lexer beside the import"*) is
unaffected: no `all.rs` hand-rolls anything.

### What the census gained, so the trade is on the record

The same PR added
`the_aggregation_row_macro_reaches_the_shared_lexer`, which reads the
macro body out of `crates/test-utils/src/source.rs` and asserts it still
calls `source::crate_dir` and `source::aggregation_violations`. That is
needed because `source.rs` is dispositioned `Home` and this row's check
filters `Home` out, so rewriting the macro body to a hand-rolled
`read_dir` walk left **all fifteen `Shared` lines green and every other
row in the file passing** — measured, on that PR's branch. It is the
partial-reversion hole this row is named for, arriving one level up: the
new row closes it for this one macro, and says nothing about the other
56 `Shared` entries.

### Evidence for the population count

The audit above says 57 `Shared`. Fifteen of them are now `all.rs` files
that satisfy the check through the macro door rather than the original
substring — a subset worth knowing when the unit re-derives the
population, because they answer to a different door and always will.

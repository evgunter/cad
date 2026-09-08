---
id: reader-census-shared-disposition-survives-partial-reversion
kind: issue
title: A Shared ledger row is checked by one substring, so a site that keeps the import and re-adds hand-rolled helpers stays green
status: open
opened: 2026-09-07
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

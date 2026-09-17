---
id: recorded-notation-makes-a-rust-author-count-step-indices
kind: issue
title: a RecordedNotation entry is keyed by an index the path algebra never hands its caller
status: open
opened: 2026-09-16
---


Found by the style review of PR #2779 (lane `notation-rv`), reviewing
`recorded-program-arguments-carry-no-notation` at `1c7b7340d`. Outside
that unit's fence — it is a door this unit could not have built without
touching `profile` — so it is filed rather than fixed.

## What happens

`RecordedNotation::set` takes an authored step index
(`crates/editor-core/src/program.rs:2020`), and the recording side —
`profile`'s `PartialPath`, whose `record` calls are inside the
transition table (`crates/profile/src/path/program.rs:945`) — hands
the author no index at all. A caller writing

```rust
let path = Open.at(p0).line_to(p1, t)?.line_to(Start, t)?;
```

must count `At` = 0, `LineTo` = 1 by reading the table's `record` calls
to write `n.set(1, StepArg::TargetX, MM.def())`. `crates/editor-core/tests/edit_recorded_notation.rs:45`
hard-codes that count as a `const LEG: u32 = 1;` with a comment, which
is the suite doing by hand what a caller would have to.

The refusal that catches a miscount —
`RecordedProgramError::NotationOffProgram` — only fires when the role
does not exist at the index named. A miscount that lands on a step
carrying the SAME role (the `TargetX` of leg 2 instead of leg 1) is
accepted and puts the unit on the wrong argument, silently. That is the
sharp edge, not the counting.

## Why it is not LIB's existing finding

`work/lib/path-legs-erase-the-authored-notation-one-layer-down` is
about VALUES — "Rust's own path API is `f64` throughout" — and its
`B-PATH-NOTATION` charter
(`crates/pncad-py/tests/test_binding_census.py:1329`) scopes the work to
the Python builder recording an entry per typed quantity it lowers. A
Python path builder threading the entries itself never exposes the
index, so closing that row leaves the RUST author with the count. The
gap is this side of the seam and has no row.

## What closing it would decide

Whether the recorder hands back what it recorded — a
`record_with_notation`-shaped door, or a `RecordedNotation` builder
that takes `(unit, StepArg)` and derives the index from the step it was
called after — and where it can live, given D6 ¶1 keeps `quantity` and
`UnitSym` out of `profile`. A wrapper in `editor-core` over
`PartialPath`'s program is the shape that stays inside the layering.

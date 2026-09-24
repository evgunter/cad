---
id: the-exhaustive-argument-is-restated-in-the-viewer-suites
kind: issue
title: The exhaustive-match argument is restated in four viewer suites, two of them beside a variant count that has gone stale
status: open
opened: 2026-09-24
priority: P4
cost: E
---


## Finding

`chrome/subset-policy` gave the argument for an exhaustive `match` one
home, `crates/viewer/README.md`'s *A policy over an enum names every
variant*, and cut every restatement under `crates/viewer/src`. The
suites were outside that lane's fence and still carry their own:

- `crates/viewer/tests/panel_edits.rs`, the `f6_variants!` roster
  doc (*"The `match` the macro writes is exhaustive, so an arm added
  to `Refusal` stops this file compiling…"*) and the note further down
  (*"`Display for Refusal` and `Refusal::rank` are exhaustive matches
  with no wildcard, so a nineteenth arm reds both…"*). `Refusal` has
  24 arms, so "nineteenth" is stale.
- `crates/viewer/tests/gesture_table.rs`, the module doc (*"Its match
  is exhaustive: a forty-third `SessionOp` does not compile…"*;
  `SessionOp` has 44 variants) and `expected`'s doc (*"Exhaustive on
  purpose. A new `SessionOp` fails to compile here…"*).
- `crates/viewer/tests/combine_ops.rs`, `open_flags`' doc (*"stated by
  an exhaustive match rather than left to the reader's eye — so a tool
  added to the set … has to be answered for here"*).

Each keeps its local half, the part worth reading: what the suite's
second copy is for. The generic half goes, and the two ordinal counts
go with it rather than being bumped, because a count beside a
compiler-held roster is a second claim with nothing holding it.

## Sweep

`grep -rniE 'exhaustive|wildcard' crates/viewer/tests`, read. The
other hits there are about an exhaustive WALK (`pick3_acceptance.rs`,
`index_memo.rs`), a different sense of the word.

---
id: tip-mark-doc-duplicates-its-own-first-sentence
kind: issue
title: tip_mark's doc duplicates its first sentence inline, rendering a literal /// in the docs
status: open
opened: 2026-09-04
---


Found by the VIEW-1c app lane while moving `tip_mark` out of `app.rs`,
and deliberately NOT fixed there: 1c's whole safety property is that it
is a mechanical move the compiler checks, so it moved the line verbatim
and reported it instead. Filed rather than folded in for the same
reason.

## What it is

`crates/viewer/src/sketch.rs:1002` (was `crates/viewer/src/app.rs:247`):

```rust
/// **How big the tip marks in a profile preview are**/// **How big the tip marks in a profile preview are**, in sketch-plane
/// metres: a fraction of the whole preview's extent.
```

The first sentence appears twice on one line, with a second `///`
embedded mid-line. Rustdoc renders that inner `///` literally, so the
published doc reads as a doubled sentence with stray slashes in it.

Almost certainly a merge or editor artefact rather than anything
anyone typed. The fix is to delete everything from the first `**` run's
close up to the second `**`, leaving one sentence.

## Why it is worth a file rather than a silent fix

Not for its size — it is one line. It is the one instance found of a
shape nothing in this repo checks: **`doc-gate.sh` gates rustdoc's
`broken_intra_doc_links` and friends, which are about link RESOLUTION,
and says nothing about whether the prose renders as intended.** A
doubled sentence with an inline `///` is valid markdown to rustdoc; it
produces no warning at any level. The same is true of the class this
program hit twice today from the other direction — a README naming a
symbol that does not exist, which no gate opens the file to see.

So the sweep question worth someone's time is not "are there other
doubled sentences" but whether anything at all reads this project's
rendered prose. `crates/viewer/README.md`'s own accuracy has now failed
twice in one day with every gate green.

## Home

VIEW's: `crates/viewer/src/sketch.rs`.

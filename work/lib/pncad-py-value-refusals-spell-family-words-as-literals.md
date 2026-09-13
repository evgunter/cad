---
id: pncad-py-value-refusals-spell-family-words-as-literals
kind: issue
title: pncad-py's Value refusals build a sentence with kind_name() on one side and a bare family word as a literal on the other, five times, on the public Python surface
status: open
opened: 2026-09-11
refs: [2376]
---


## Finding

Found by the style review of WIRE's PR 2376, outside that unit's fence;
filed here by the WIRE orchestrator because `crates/pncad-py/src/*` is
LIB's glob. Confidence `sure`, the reviewer's. Accurate at `af8bbca`.

`crates/pncad-py/src/py/value.rs:737, 773, 848, 887, 900` each build a
refusal as

```rust
format!("a `{}` value is not a body", other.kind_name())
```

— and the same shape for `split`, `datum`, `measure`, `assertion`. One
side is **const-derived** (`kind_name()` answers `eval::family`'s consts),
the other is **the same vocabulary spelled as a literal**, in the same
sentence. That is exactly the defect
`wire-expected-phrases-spell-family-words-as-literals` named one crate
down, five times, on the **user-visible Python surface**.

It escaped WIRE's unit for a stated reason and not by accident: it hits
two of that sweep's disclosed blind spots at once — the word is inside a
`format!` template, and the file is outside `editor-core/src/`.

## The stability obligation nobody has written down

`crates/pncad-py/src/py/value.rs:712-716` exposes `kind_name()` as
Python's `Value.kind`, with the comment *"so the Python tag set cannot
drift from the document layer's"*. So `eval::family`'s ten strings are a
**public API contract**, not only a refusal vocabulary — and nothing at
the const site says so. Whatever this row does about the five literals,
the second consumer is worth a sentence at the home, which is
`crates/editor-core/src/eval/mod.rs`'s `family` module (WIRE's file —
announce, do not land there).

## Where else to look

`crates/viewer/src/` takes the same scoping exclusion and was not swept
by either lane.

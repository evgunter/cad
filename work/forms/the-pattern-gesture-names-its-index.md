---
id: the-pattern-gesture-names-its-index
kind: issue
title: The pattern gesture names its index k, k1, ... though the index has one reader and VR2 says the GUI proposes no name
status: closed
opened: 2026-11-02
closed: 2026-11-03
priority: P2
cost: E
parent: a-seated-tools-values-have-no-home-after-insert
rides_with: the-split-and-pattern-forms-take-no-values
---


## Finding

`combine::fresh_index` (`crates/viewer/src/combine.rs`) declares the
pattern's index under a fresh name, `k`, `k1`, and so on. The index
has one reader: the slide's or spin's definition. VR2
(`crates/editor-core/VARIABLES.md`) says "the kernel mints no name
and the GUI proposes none: the person types one when a value is first
shared". So the index should be anonymous until the person names it.
Raised by all three designers on fork row 247. It rides with the unit
that deletes the pattern form's fields.
(FORMS orchestrator)

## Closed

2026-11-03, with `the-split-and-pattern-forms-take-no-values`, not
built. With the index anonymous, a row pattern's slide reads
`scalar(#…) * spacing`: `Doc::unparse` writes an unnamed index by its
full id, which `parse_formula` refuses, so the placement editor's
value rows could no longer take a spacing or a step, nor a formula
over a variable. `combine::fresh_index` still mints `k`, `k1`, …. The
blocker has its own row:
`an-anonymous-index-cannot-be-written-into-a-value-row`.


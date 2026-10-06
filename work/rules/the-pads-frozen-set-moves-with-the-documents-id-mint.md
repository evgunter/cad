---
id: the-pads-frozen-set-moves-with-the-documents-id-mint
kind: issue
title: The pad's frozen set moves when the document's ids are minted differently, with no arithmetic changed
status: open
opened: 2026-10-06
priority: P3
cost: M
refs: [sym-tier-reach-depends-on-symbol-order, SYM-16]
---


## What was measured (SYM-16's bisect, 2026-10-06)

`SymCounts::frozen` says it is a set, "a claim about the subject" and
the same under every schedule (`geom-core/src/sym.rs`, the field's
doc). On R2's rounded pad it moves with how the document mints its
ids, at two `main` merges that change no geometry and no arithmetic.
The replay is `sym11_the_exact_channel_never_contradicts_past_the_ceiling`'s
(`editor-core/tests/sym11_exact_channel_rows.rs`), taken at the pad's
`refuses_at` scale, release build, ε = 1e-9:

| merge | what it changes | the pad's `frozen` (parent → merge) |
| --- | --- | --- |
| #3455 `6552c509ad` | step ids mint from a digest chain | 2592 → 2577 |
| #3594 `c28f9aba17` | node ids mint from the document's one mint chain | 2529 → 2544 |

The parents are `40bfe407bf` and `8247568db8`. Neither merge changes a
file under `geom-core`, `geom-brep`, `sweep` or `profile`. No decision
column moves on any of the five measured documents at either merge.
The other four documents' `frozen` does not move. The pad's rule-F row
(`m10_9_the_pad_at_both_rule_f_dials`, at the pad's certifying scale)
moves by the same amount at both dials.

## What is not measured

Why it moves. One route is that evaluation visits independent nodes in
an order keyed on `RecipeNodeId` (`editor-core/src/eval/schedule.rs`),
so a re-mint reorders which node the session builds first, and with it
which compound reaches the term budget. This is a hypothesis: no probe
has re-keyed the ids on a fixed tree.

## Why it matters

A count that moves under relabelling cannot be pinned exactly without
pinning the labels too, and the two SYM receipt rows pin it. The
dependence is the same kind as `sym-tier-reach-depends-on-symbol-order`,
which reaches the same kind of result for the parameter symbols'
order. Either make the frozen set order-free, or say on
`SymCounts::frozen` what it depends on, and stop pinning it where that
dependence can move it.

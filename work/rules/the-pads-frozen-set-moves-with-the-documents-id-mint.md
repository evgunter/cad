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

The parent column was read at the nearest first-parent predecessors
that change library source, `40bfe407bf` and `8247568db8`; the true
first parents, `59431c457a` and `8e0bca253b`, read the same (2592 and
2529, the review's re-run). Neither merge changes a
file under `geom-core`, `geom-brep`, `sweep` or `profile`. No decision
column moves on any of the five measured documents at either merge.
The other four documents' `frozen` does not move. The pad's rule-F row
(`m10_9_the_pad_at_both_rule_f_dials`, at the pad's certifying scale)
moves by the same amount at both dials.

## What is not measured

Why it moves. Two routes are ruled out:
- **Not the schedule, at #3594.** `editor-core/src/eval/schedule.rs` breaks ties
  between ready nodes by their position in `Doc::order`, the author's
  order, and `Doc::positions` (`editor-core/src/doc.rs`) says an id is
  a digest and says nothing of seniority. That has held since #3593
  (`3bc4df6595`, 2026-09-30), so a re-mint does not reorder the
  schedule at #3594. At #3455 the tie-break was still the id. #3593's
  own re-keying of the ties did not move the pad's `frozen`: the
  stretch that holds it, `6552c509ad` to `0eccbbff6b`, reads 2577 at
  both ends (SYM-16's bisect). That is weaker than a probe, since
  nothing inside the stretch was bisected and a cancelling move would
  read the same.
- **Not the parameters' symbol order.** At both merges a parameter's
  symbol is keyed on its name (`analysis::param_env_over` hands
  `T::axis_named` the name at `c28f9aba17`), so the parameters sort the
  same before and after either re-mint.

What is left is something keyed on a minted id inside what the session
builds: atoms, or the order in which a map keyed on a node or step id
is walked, which decides which compound is built first and so which
one reaches the term budget. That is `sym-tier-reach-depends-on-symbol-order`'s
class, order and not content deciding what the tier reaches. This
is a hypothesis: no probe has re-keyed the ids on a fixed tree.

## Why it matters

A count that moves under relabelling cannot be pinned exactly without
pinning the labels too, and the two SYM receipt rows pin it. The
dependence is the same kind as `sym-tier-reach-depends-on-symbol-order`,
which reaches the same kind of result for the parameter symbols'
order. Either make the frozen set order-free, or say on
`SymCounts::frozen` what it depends on, and stop pinning it where that
dependence can move it.

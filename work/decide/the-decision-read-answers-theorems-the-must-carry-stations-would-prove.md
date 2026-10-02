---
id: the-decision-read-answers-theorems-the-must-carry-stations-would-prove
kind: issue
title: The decision read answers 32 of the pad's and 16 of the bracket's decisions that are theorems with it shut; 28 of the pad's are the must-carry stations
status: dispatched
opened: 2026-10-01
priority: P2
cost: M
refs: [rule-g-trades-sixteen-of-the-links-carrier-on-surface-2, 2468]
---

## What was measured (LINALG's merge of `main` into `props/sign-hull`, 2026-10-01)

`m10_9_pins_interval`'s replay (`replay_counts`, one whole-box replay at
`Sym<Interval>` over each document's `certifies_at · ε` box, ε = 1e-9,
release), receipts as `symbolic_zero / sign_gated / registered / numeric`:

| document | tree | shipped | `without_the_reads` |
| --- | --- | --- | --- |
| R2's rounded pad | `props/sign-hull` before the merge (`3dc3becc0`) | 890 / 6 / 150 / 907 | 894 / 0 / 150 / 909 |
| R2's rounded pad | merged with `main` | 893 / 34 / 150 / 1002 | 925 / 0 / 150 / 1004 |
| R2's filleted bracket | `props/sign-hull` before the merge | 1104 / 7 / 144 / 766 | 1106 / 0 / 144 / 771 |
| R2's filleted bracket | merged with `main` | 1105 / 21 / 144 / 783 | 1121 / 0 / 144 / 788 |

With the read shut, every theorem is still a theorem: the merged pad's
925 is the pre-merge 894 plus `main`'s 28 must-carry station theorems
(`geom_brep::must_carry_over_edge`'s `classify_dihedral` gate, seven
stations on each of the pad's sixteen rule-reached edges) and its three
fillet run-out theorems (`path_run_out_carrier`). With the read on, 32
of the pad's and 16 of the bracket's decisions that reduce to the zero
form are answered by the read first, and count `sign_gated`. The
decision stays discharged; the claim on it is the read's bracket, not a
theorem — the weakening `m10_9_no_registrant_lies_on_any_measured_document`
names.

`sym.rs`'s rule table says the read fires on "the frame's conditioning
comparisons, which no form settles: `sign_gated` where it fires and
never `symbolic_zero`", and the module header orders it BEHIND every
value-free fold for exactly this reason. These rows are forms that DO
settle, so either the ordering does not hold at the site these
decisions reach (a `min`/`max` or `Select` read at its own node before
the residual above it reduces), or the statement is narrower than it
reads. The pre-merge tree already shows the class at 4 and 2; `main`'s
station gate multiplied the pad's by eight.

## Not established

Which predicates the 32 and 16 are. The per-predicate split
(`m10_10_splits_at_the_nominal_under_a_rule_set`, `CAD_M10_10_DOCS=r2_rounded_pad`)
was killed at 388 s on a 15 GB box before printing — the shape report's
rendering of the pad's blocked residuals. That the pad's 28 are the
`dihedral_wedge` stations is read off the arithmetic (925 − 894 = 31 =
28 + 3), not off a split.

## Home

DECIDE: the read's ordering is `geom_core::sym::signed`'s and the
session's. The pins re-baselined to the shipped numbers with this cause
stated (`m10_9_pins_interval`, `sym_9_retry_interval`).

---
id: rule-g-trades-sixteen-of-the-links-carrier-on-surface-2
kind: issue
title: rule G costs the link's carrier_on_surface_2 sixteen theorems - ten to the companion rewrite opening abs node squares, six to sqrt(R2)=|R| and the registrant's axiom
status: open
opened: 2026-09-21
priority: P1
cost: H
---


## What was measured (DECIDE-3's fix pass, 2026-09-21)

The acceptance clause DECIDE-3 is held to is "no decision LOST", per
PREDICATE. On R2's link at the nominal it fails on exactly one
predicate, and this row is the whole of that failure with its cause
measured apart.

| tier | `carrier_on_surface_2` | document total |
| --- | --- | --- |
| rule G and the read off (the base) | `[98, 0, 0, 10]` | `[515, 0, 90, 497]` |
| shipped | `[82, 0, 6, 20]` | `[541, 0, 96, 465]` |
| `without_the_reads` | `[82, 0, 6, 20]` | — (so the READ moves nothing here) |

Sixteen theorems, and they go two ways:

- **TEN to rule A's companion rewrite** (`abs(X)² = X²`, `sym/algebra.rs`,
  behind rule G's dial). With the rewrite shut and rule G on, the same
  predicate reads `[36, 0, 0, 72]` — so the rewrite buys 52 of the 88
  it leaves and costs 10. It opens the square of an `abs` NODE the
  document wrote, and the expansion does not cancel where the closed
  atom did.
- **SIX to `sqrt(R²) = |R|`**, which the rim registrant's axiom then
  closes: `registered` 0 → 6 at the predicate and 90 → 96 on the
  document. With that step alone shut the predicate reads
  `[88, 0, 0, 20]` and the document `[547, 0, 90, 465]` — six more
  theorems and the door not needed.

**What it is NOT.** Re-enabling the retired side-condition source (the
session's atom table) moves nothing here, so this is not the
consequence of that removal. The decision read moves nothing here.

**The class** is `work/sym/coefficient-ring-width-is-not-monotone-in-reach`'s:
opening an atom the walk was cancelling OVER costs the walk a theorem.
Both halves are net gains taken whole — the link's document total rises
by 26 theorems and 6 registered while `numeric` falls by 32, and the
companion rewrite is what carries the boss's `carrier_on_surface_1`
and `_2` from 81/0/0/9 to 90/0/0/0 — but the per-predicate clause is
the clause, and at this predicate the sum falls 98 → 88.

## What would answer it

The likely shape, in order of cheapness:

1. **A size guard on the companion rewrite**: take `abs(X)² → X²` only
   where the substituted form is no larger than the one it replaces,
   the way a reduction that can only fail to find a cancellation
   should. The ten are an expansion that does not cancel.
2. **Provenance on the magnitude**: rule G's own `sqrt(R²) → |R|` is
   the step that must keep reducing; an `abs` node's square never
   reduced before this unit and need not now.
3. For the six: render the residual the registrant closes under both
   dial sets and read the two atom keys it carries. The content split
   of the magnitude key (`|c·R| = c·|R|`, added in the same fix pass)
   was tried and does not move them.

## Home

`crates/geom-core/src/sym/algebra.rs` (`find_square`'s `Abs` arm),
`crates/geom-core/src/sym/root.rs` (`magnitude_of_root`),
`crates/editor-core/tests/decide_3_split_rows_interval.rs` (the row
that pins it, with the numbers asserted so a drift reds). Filed by
DECIDE-3's fix pass with the measurement above.

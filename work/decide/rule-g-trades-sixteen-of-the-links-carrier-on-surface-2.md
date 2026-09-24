---
id: rule-g-trades-sixteen-of-the-links-carrier-on-surface-2
kind: issue
title: rule G costs the link's carrier_on_surface_2 sixteen theorems - SYM-9's kept-atom retry recovers the ten when installed, and the six weakened to the registrant's axiom are what is left
status: open
opened: 2026-09-21
priority: P2
cost: H
---


## What was measured (DECIDE-3's fix pass, 2026-09-21)

On R2's link at the nominal, rule G costs one predicate sixteen
theorems while the document's totals rise. This row is the whole of
that trade with its cause measured apart.

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

## The remedy that was tried, and what it measured

**Provenance on the magnitude** — apply `abs(X)² = X²` only to the
`Abs` atoms rule G minted as the magnitude of a ROOT, never to an
`abs` node the document wrote — was implemented (a provenance set on
the session, filled at `sqrt(R²) → |R|` and read by `find_square`) and
measured on the link:

| tier | `carrier_on_surface_2` | document total |
| --- | --- | --- |
| G and the read off (the base) | `[98, 0, 0, 10]` | `[515, 0, 90, 497]` |
| shipped, the rewrite unrestricted | `[82, 0, 6, 20]` | `[541, 0, 96, 465]` |
| shipped, the rewrite on rule G's magnitudes only | `[86, 0, 0, 22]` | `[501, 0, 90, 511]` |
| shipped, the rewrite shut | `[36, 0, 0, 72]` | `[445, 0, 90, 567]` |

It recovers the six (the door is not needed again) and four of the
ten, and it costs the DOCUMENT forty theorems — 541 → 501, below the
base's 515. **The reason it cannot work as stated**: after rule G a
`sqrt(R²)` and a document's `abs(R)` are THE SAME ATOM — that is what
the one door is for — so "rule G's magnitudes" and "the document's
`abs` nodes" are not disjoint sets, and which of the two minted an id
first is a fact about the walk's traversal. The restriction therefore
narrows by traversal order, not by provenance, and what it withholds
is cancellations rule G's own respelling needs.

The other shapes, untried:

1. **A size guard on the rewrite** — take `abs(X)² → X²` only where
   the substituted form is no larger. The ten are an expansion that
   does not cancel. (Narrowness of this kind is what the unit's
   ratified section rules out as a remedy, so it is recorded and not
   taken.)
2. For the six: render the residual the registrant closes under both
   dial sets and read the two atom keys it carries. The content split
   of the magnitude key (`|c·R| = c·|R|`) was tried and does not move
   them.

**RE-BASELINED, and pinned** (Ev, 02:03Z on #3039). The "no decision
LOST" acceptance was the orchestrator's spec text, not Ev's ruling;
Ev's ruling was shape 1 plus *"never skip out on a change that would
make the code better because it would require rebaselining"*. So
DECIDE-3 lands with this loss said against the line:
`editor-core/tests/decide_3_split_rows_interval::decide_3_no_predicate_loses_a_decision`
keeps the clause on every other predicate and pins THIS one at its
measured numbers on both sides — `[98, 0, 0, 10]` with rule G shut
against `[82, 0, 6, 20]` shipped — with the reason and this row's name
in the comment, so any further drift reds and says which.

## What SYM-9 measured, and what it recovered (2026-09-22, fix pass 2026-09-24)

**The ten are RECOVERED by the retry ladder, and the trade this row
records still stands at one attempt per rung.** SYM-9's retry ladder
(`geom_core::SymRetry::kept_atom`; the drive's default is no ladder,
`editor_core::drive::DEFAULT_SYM_RETRY` argues why) re-asks
a decision every rung of the first attempt refused, with rule G shut for
that attempt. The first attempt is untouched. Measured on the link at
the nominal (`editor-core/tests/sym_9_retry_interval.rs`):

| tier | `carrier_on_surface_2` | document total | retried |
| --- | --- | --- | --- |
| rule G and the read off (the base) | `[98, 0, 0, 10]` | `[515, 0, 90, 497]` | — |
| shipped, one attempt per rung | `[82, 0, 6, 20]` | `[541, 0, 96, 465]` | 0 |
| **shipped, with the ladder** | **`[92, 0, 6, 10]`** | **`[553, 0, 96, 453]`** | **12** |

Ten theorems back on this predicate and two more on
`witness_on_surface_2` (`[14, 0, 0, 2]` -> `[16, 0, 0, 0]`), every other
predicate bit-identical. The predicate's pin in
`decide_3_split_rows_interval` keeps THIS row's trade —
`([98, 0, 0, 10], [82, 0, 6, 20])`, one attempt per rung on both sides,
because it is a rules differential and a ladder on one side of it would
read rule G's cost as recovered (SYM-9's first cut did exactly that and
both of its reviews caught it). What the ladder recovers is pinned
where the ladder is:
`sym_9_retry_interval::sym_9_the_kept_atom_ladder_recovers_what_phase_1_measured`.

**The two narrow shapes this row left untried are measured, as retries,
and recover NOTHING** on any of the five documents SYM-9 measured: the
companion rewrite alone shut, 0; rule G's magnitude door alone shut, 0.
Only the whole of rule G shut for one attempt reaches the ten. WHY the
rewrite alone does not is not executed on the link: at the scalar,
`geom-core`'s `a_decision_that_closes_only_with_rule_g_off` builds the
row's shape (a document `abs` whose square the rewrite opens past the
term budget) and there the rewrite alone DOES close it, so the link's
ten are a different residual from that one, and shape 2 below is what
would say which. The size guard (shape 1) stays untaken.

**What is left is the SIX, and in the gate's own words they are claims
WEAKENED from a theorem to an axiom** — `registered` where the base has
them `symbolic_zero`, because rule G's `sqrt(R²) = |R|` hands them to
the rim registrant. No retry moves them back, and that is by
construction: they are discharged on the first attempt (by the door), so
the ladder is never entered for them.

This row therefore stays **open at P2** for exactly one thing, shape 2
of the untried list: render the residuals under both dial sets and read
the atom keys off — the six the registrant closes, and the ten the
rule-G attempt recovers — so that both are a described mechanism rather
than an observed count.

## Home

`crates/geom-core/src/sym/algebra.rs` (`find_square`'s `Abs` arm),
`crates/geom-core/src/sym/root.rs` (`magnitude_of_root`),
`crates/editor-core/tests/decide_3_split_rows_interval.rs` (the row
that pins it, with the numbers asserted so a drift reds). Filed by
DECIDE-3's fix pass with the measurement above.

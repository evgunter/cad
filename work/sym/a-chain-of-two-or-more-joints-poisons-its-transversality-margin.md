---
id: a-chain-of-two-or-more-joints-poisons-its-transversality-margin
kind: issue
title: the wedge's transversality margin POISONS at two joints and at three, and it is what bounds the chain's certifiable box: just above the wall it is the first refusal at every link count
status: open
opened: 2026-09-22
priority: P1
cost: D
refs: [SYM-14]
---



## What

**Specifically requested by Ev, 2026-09-22** — found by SYM-14, the
chain demo Ev asked for, measuring the certified lane on it
(`demos/tour/src/chaintol.rs`, the cell's own CI rows).

One leaf over the declared box, `Sym<Interval>`, on the chain of
`demos/tour/src/chain.rs`. At TWO links over the whole study:

```
node 14 — the transform op refused: transform: mapped edge EdgeKey(1v1)
failed re-certification: certification: the transversality margin at
sample 4 escalated: predicate 'dihedral_wedge' indeterminate: margin is
invalid (NaN or a poisoned enclosure)
```

A straddle is the tier working and running out of width. A POISONED
margin is not: a NaN says some upstream quantity went to poison, and
the refusal reports the branch it could not take rather than the thing
that poisoned.

**This is what bounds `chain::CERTIFIABLE_FRACTION`.** MEASURED at
`1.02×` and `1.10×` of each link count's certifiable fraction, at the
default ε and at `1e-6`: the first refusal is this wedge, poisoned, at
two links (node 14), three (node 21) and four (node 30) alike —
`EdgeKey(1v1)`, sample 4, every time. Pinned by
`chaintol.rs`'s `the_wall_is_the_wedge_not_the_arm`.

That it is a BAND comparison on a poisoned enclosure is also why the
certifiable box moves with ε (`1.110e-1` at the default, `1.083e-1` at
`1e-6`) while the arm's straddling bracket does not move at all. The
two facts reconcile here and nowhere else.

**It is not exclusive to two joints.** The three-link leaf carries the
SAME poison, on the same edge and the same sample (`EdgeKey(1v1)`,
sample 4, nodes 18 and 20) — it is simply not the FIRST refusal
reported there, because `dihedral_arm`'s straddle at node 12 comes
earlier in evaluation order. An earlier reading of this said "the
shorter chain is the poisoned one"; that was evaluation order read as
a fact about the chain, and it is wrong.

## What answers it

The upstream quantity that goes to NaN, named —
`memories/refusal-text-is-not-cause.md` applies, so the wedge
predicate's message is where the branch was refused and not where the
poison was minted. The reviewer's reading, not yet confirmed by
execution and recorded here as the first thing to check: the pin's
cylinder gradient straddles zero once the positional box exceeds the
pin radius (`crates/topo/src/dihedral.rs`, the transversality arm).
That would also explain why the certified tip half-width sits at half
`chain::PIN_RADIUS` at every link count and doubles when the radius is
doubled (`chain::CERTIFIED_TIP_OVER_PIN_RADIUS`) — but it is a
hypothesis about the mechanism until someone reads the margin's own
inputs at the wall. Then either the poison is a real defect and is
fixed, or it is a legitimate empty enclosure and the refusal says
which.

## Home

SYM — the tier's reach on a construction Ev asked for; if the poison
turns out to be minted in the certification lane rather than in the
tier, it re-homes there.

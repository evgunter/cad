---
id: a-two-joint-chain-poisons-its-transversality-margin
kind: issue
title: a two-joint chain's mapped edge re-certification gets a POISONED transversality margin (dihedral_wedge, NaN), where three and four joints get a clean straddling enclosure
status: open
opened: 2026-09-22
priority: P1
cost: D
refs: [SYM-14]
---



## What

**Specifically requested by Ev, 2026-09-22** — found by SYM-14, the
chain demo Ev asked for, measuring the certified lane on it
(`demos/tour/src/chaintol.rs`, the cell's own CI row).

One leaf over the whole declared box, `Sym<Interval>`, on the chain of
`demos/tour/src/chain.rs` at σ = 0.01 rad per joint. At TWO links:

```
node 14 — the transform op refused: transform: mapped edge EdgeKey(1v1)
failed re-certification: certification: the transversality margin at
sample 4 escalated: predicate 'dihedral_wedge' indeterminate: margin is
invalid (NaN or a poisoned enclosure)
```

At three and four links the first refusal is a DIFFERENT predicate with
a well-formed answer — `dihedral_arm` with the enclosure
`[0e0, 7.338367397126071e-3]`, straddling the band
(`work/sym/a-chain-of-three-joints-straddles-dihedral-arm`). A
straddle is the tier working and running out of width. A POISONED
margin is not: a NaN says some upstream quantity went to poison, and
the refusal reports the branch it could not take rather than the thing
that poisoned.

That the *shorter* chain is the poisoned one is what makes this worth
its own row: whatever poisons at two joints is evidently not present,
or not reached first, at three and four.

## What answers it

The upstream quantity that goes to NaN in the two-link leaf, named —
`refusal-text-is-not-cause` applies, so the wedge predicate's message
is where the branch was refused and not where the poison was minted.
Then either the poison is a real defect and is fixed, or it is a
legitimate empty enclosure and the refusal says which.

## Home

SYM — the tier's reach on a construction Ev asked for; if the poison
turns out to be minted in the certification lane rather than in the
tier, it re-homes there.

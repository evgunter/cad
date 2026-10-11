---
id: a-mate-reads-face-variables
kind: issue
title: D10 stage 2 PR F: a mate's sides read Face variables; A12's reading edges, the mates-are-not-edges carve-out and A5's minting lift through consumers retire
status: closed
closed: 2026-10-10
pr: 4566
opened: 2026-10-07
priority: P0
cost: H
refs: [split-and-inline-over-a-mate-read-at-a-union-are-unmeasured]
branch: intent/s2-f-mate-faces
---

INTENT stage 2, PR F. Spec: `docs/INTENT-STAGE2-SPEC.md` §7.

Mate sides become `Face` slots read through selects. `mate::reading_edges` (A12) is
deleted, and A9 runs over `Doc::upstream` while keeping the gauge edges (Q3). The
at-rest gate maps the select's resolution instead of re-resolving against the product
table (Q8). `split-and-inline-over-a-mate-read-at-a-union-are-unmeasured` gets its
rows here, because this PR rewrites the four refactor sites it names.

## A5's lift retires here (2026-10-07, #4220)

A mate's declaration is minted on the world copies of its members, read through those copies' placements. A mate whose members are not both placed mints nothing. `names::lift`'s consumer walk and the `MovedAbove`/`Vanished` arms go (spec §7, Q8).

## Closed

By PR 4566. A mate's sides are `Face` selections; A12's reading edges are deleted and A9 runs over reads plus the gauge edges; the at-rest gate mints on the world copies of what a mate reads, and nothing for a side whose body nothing places; `names::lift`'s consumer walk, `MovedAbove`/`Vanished` and `OperandSeveredFromMate` retire. `split-and-inline-over-a-mate-read-at-a-union-are-unmeasured` closed with it (msolve13 A3). Disclosed and filed: `a-gauge-swing-re-keys-the-mates-reading-its-instances` (parked on stage 3 B), `a-mate-read-below-a-placer-is-silent-at-the-gate`, `a-mate-is-poisoned-through-a-member-its-own-solve-faulted`. The fired trigger is pruned from `a-placement-is-the-bundle-of-mates` and `placed-carriers-compare-through-their-frames`; `a-placer-row-states-what-a-poisoned-row-cannot` is re-parked on stage 3 B (its raise site survives F).

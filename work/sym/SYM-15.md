---
id: SYM-15
kind: unit
title: "the chain's wedge margin poisons: name what goes to NaN, and fix it or say it"
status: dispatched
opened: 2026-10-02
priority: P1
cost: M
branch: sym/15-wedge-poison
refs: [a-chain-of-two-or-more-joints-poisons-its-transversality-margin, SYM-14]
---

## What

`a-chain-of-two-or-more-joints-poisons-its-transversality-margin`, on the
chain demo Ev asked for (#3073). Just above the chain's certifiable box,
at every link count, the first refusal is `dihedral_wedge`'s margin
poisoned (NaN) at `EdgeKey(1v1)`, sample 4. That is what bounds
`chain::CERTIFIABLE_FRACTION`.

Phase 1 reads the margin's inputs at the wall, names the operation that
mints the poison and the ε dependence that moves the box, and decides
whether the poison is a defect or a legitimate undefined enclosure.
Phase 2 fixes it at the minting site and re-takes the fractions, or
makes the refusal name its cause.

Spec: `docs/SYM-15-SPEC.md`. Opus implementer. Review tier: single FULL
review (`work/sym/log.md`, 2026-10-02).

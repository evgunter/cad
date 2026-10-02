---
id: SYM-15
kind: unit
title: the chain's wedge margin poisons: name what goes to NaN, and fix it or say it
status: closed
opened: 2026-10-02
priority: P1
cost: M
branch: sym/15-wedge-poison
refs: [a-chain-of-two-or-more-joints-poisons-its-transversality-margin, SYM-14]
closed: 2026-10-02
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

Spec: `docs/SYM-15-SPEC.md`, deleted at close (`docs/doc-ledger/sym-15-spec.md`). Opus implementer. Review tier: single FULL
review (`work/sym/log.md`, 2026-10-02).

## Closed (2026-10-02, PR #3804)

Merged after a single FULL review (APPROVE-WITH-FIXES: the stop was
only partly right), a fix pass and a delta review (MERGEABLE).

- **Phase 1:** the poison is minted by `wedge_decided`'s `sin θ`
  division (`crates/geom-brep/src/dihedral.rs`), on a gradient-magnitude
  product that reaches zero; it is a legitimate undefined enclosure, not
  the tier's.
- **Two levers, both filed and neither fixed here:** (i) the formula
  `|n1×n2|/(|n1||n2|)` loses the shared magnitude and sets the wall at
  ε above the default
  (`work/props/interval-sin-theta-as-cross-over-norms-loses-the-shared-magnitude`);
  (ii) `topo::transform_rigid` encloses the images of one rigid map
  apart and sets the poison point, so the wall at the default ε
  (`work/shell/transform-rigid-recertifies-images-enclosed-apart`).
- **Phase 2 (the spec's legitimate branch):** the certify path names
  the cause (`CertCheck::TangentPlanes`), verdict-neutral; the wall pin
  re-baselined to it. No fraction moved.

---
id: shared-rim-several-is-a-missing-rule-legal-declared-unions-reach
kind: issue
title: SharedRim(Several) — a chord over a fragmented merged face — is the commonest naming refusal a legal declared union reaches
status: closed
opened: 2026-09-23
priority: P0
cost: H
branch: emit/shared-rim-several
closed: 2026-09-24
pr: 3167
---


## What

The boolean edge pass names a derived chord between two faces of one
operand as the single edge those two faces share in that operand
(`crates/editor-core/src/names/emit_topo.rs`, `name_boolean_edges`'
`chord_kind`, the `SameA`/`SameB` arms over `rim_between`). When the
pair shares several edges, the pass refuses `NamingError::SharedRim
{ found: Several }`. That refusal is a missing-rule refusal: WIRE's
`two-emitter-refusals-a-legal-declared-union-reaches` classified it and
closed. No row owns the rule itself, and the rule is what a legal
declared union keeps reaching.

## Evidence

Measured 2026-09-23 on `emit/seam-edge-merged-faces` (the branch that
retires the merged-chord `Emission`). The corpus is PR 3112's review
probe plus that branch's three documents. All are declared unions of
axis-aligned blocks:
- `a` = x∈(0,1), `b` = x∈(0.5,1.5), declared with `flush_pairs`;
- plus one to three slabs.

Every member order was swept.

- **Review probe:** 62 of 304 (document, order) cells refuse
  `SharedRim { found: Several }`.
  - Nearly every document in the probe reaches it, in 2 orders each:
    `abc`, `abg`, `abglow`, `cross`, `nest`, and 18 of the 27 `fam*`
    third-member placements.
  - The multi-slab documents reach it in more orders: `row` and
    `rowids` in 4, `abgg2` in 8.
- **The branch's documents:** a corner slab in 2 of 6 orders, two slabs
  in 8 of 24, three slabs in 20 of 120.

The face pair is always the two MERGED faces of `a` and `b` (the top
cap and a y-wall) or their descendants, read on the A side at a later
step. Those faces share their full rim and also the pieces a slab cut
out of it, so "exactly one shared edge" no longer picks the chord's
edge.

## Why it matters

A declared union of ordinary blocks is refused in about one order in
five of the probe's cells, with a typed refusal that says the rule is
missing. The refusal is honest, but the verb does not work, and a
normal verb broken on normal geometry is P0 whatever the refusal says
(the precedent is `b-arena-edges-skip-the-split-lineage-chase`). A rule
probably exists: the chord's own endpoints lie on exactly one of the
shared edges. Choosing by position is geometry, though, not
combinatorics, so the rule is a design question rather than a patch.

## Found by

The `(unsupported)` sweep for
`seam-edge-between-two-merged-faces-refusal-a-legal-declared-union-reaches`.

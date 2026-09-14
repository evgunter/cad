---
id: a-null-edge-can-be-re-based-onto-a-distinct-point
kind: issue
title: the re-basing gate skips null scaffolding, so a fan mev can leave a null edge whose two ends are distinct points
status: open
opened: 2026-09-14
parent: S93
refs: [S93]
---

## What

Found by S93's R2 reviewer, by execution, and filed by that unit's fix
pass.

`Body::certify_rebased_run` (`crates/topo/src/euler.rs`) skips every
edge whose curve entry is `CurveGeom::NullScaffold`, because a null
edge carries no certificate to invalidate. The skip is unconditional,
so it also skips the QUESTION the gate exists to ask: a null edge in
the moved run is re-based like any other, and where the new vertex's
point differs from the old one's the null edge is left spanning two
DISTINCT points.

Pinned as it stands by
`euler::tests::the_gate_skips_a_null_scaffolded_edge_however_far_the_run_moves`,
which drives the gate over a null-scaffolded run at `(99, 99, 99)` and
gets `Ok`.

## What it costs

Tier 1 accepts it (a null edge is an edge, and nothing in tier 1 reads
points). Tier 2 refuses every null edge at rest already
(`ValidationError::NullEdgeAtRest`), so a body carrying one is refused
whether or not its ends coincide — which is why this is a hole in the
gate's claim rather than a corruption anybody sees today. What is lost
is the null edge's MEANING: `mev_null`'s whole contract is a
zero-length edge at one point (`crates/topo/src/null.rs`, the ratified
F9 shape), and a re-based null edge is no longer that while the type
still says it is.

## Reach

**The splitting and boolean pipelines cannot reach it.** Every
run-moving `MevSite::Fan` in the tree outside `topo`'s own test and
review modules goes through `Body::mev_null`, whose new vertex takes
the old one's point bitwise — measured on this branch's head: 273
`MevSite::Fan` occurrences, 221 struts, 25 runs (every one a test or a
review probe), 17 shorthand/prose, 10 doc lines; the three kernel run
sites in the shorthand group (`splitting/insert.rs`,
`boolean/insert.rs`, `boolean/vtxfac.rs`) all call `mev_null`.

**The public door can**: `Body::mev` at a fan site whose run contains a
null edge, at any point other than the old vertex's. Nothing refuses
it.

## Shapes

- **Refuse a null edge in a moved run**, unless the move is a no-op —
  which needs the same exact `p_new == p_old` question
  `the-re-basing-gate-refuses-m7-8-where-nothing-moves` is about, so
  the two rows want one answer.
- **Refuse a null edge in a moved run, full stop**: a null edge is
  mid-surgery scaffolding and a fan `mev` across it is not a surgery
  any pipeline performs, so the refusal costs nothing measured.
- **Say it is tier 2's**, which is true today and makes the gate's
  claim narrower than its docs read.

The second is smallest; whether it is right depends on the first row's
answer.

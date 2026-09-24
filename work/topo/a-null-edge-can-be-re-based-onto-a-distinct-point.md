---
id: a-null-edge-can-be-re-based-onto-a-distinct-point
kind: issue
title: the re-basing gate skips null scaffolding, so a fan mev can leave a null edge whose two ends are distinct points
status: open
opened: 2026-09-14
parent: S93
refs: [S93]
priority: P0
cost: H
pr: 3148
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

## Brief (TOPO, 2026-09-14) — block TOPO-B5 slot 2, dual at review

**The answer to give.** This row and
`the-re-basing-gate-refuses-m7-8-where-nothing-moves` are one question
— "does this fan `mev` move anything" — asked of
`Body::certify_rebased_run` (`euler.rs`) from two sides: the gate skips
null scaffolding unconditionally (so a null edge can be re-based onto a
distinct point and stop meaning what `mev_null`'s F9 shape says), and
it refuses the plane × NURBS `Unimplemented` class even where the new
vertex takes the old vertex's own point. Both rows agree the exact
question is "is `p_new` the point `p_old`", bitwise, and that
`Point3<T>` at `T: Real` has no door for it by design. Phase 1 decides
between two shapes and says why:

1. **The structural answer inside `topo`**: a moved run refuses a null
   edge, full stop (a fan `mev` across null scaffolding is not a
   surgery any pipeline performs — measured in the row: every kernel
   run site calls `mev_null`, which COPIES the point and never asks),
   with a typed `EulerOpError` arm; and the M7-8 over-refusal is left
   where it is with its rustdoc saying so, because a no-move `mev` is
   already spelled as `mev_null` — the caller who wants "nothing
   moves" has that door. Cheapest; right for every pipeline in the
   tree; narrows the gate's claim honestly.
2. **A structural-identity door** (`Real::is_bitwise`-shaped, or
   `Point3::structurally_identical`), with the Q1 argument
   `Real::is_poison` already carries — structural discrimination, not a
   geometric decision — used by the gate to skip re-certification when
   nothing moved, answering both rows in both directions. Run
   CLAUDE.md's check FIRST: is the `Real` trait's method surface (or
   `Point3`'s) ratified text (`docs/DESIGN.md`, `crates/geom-core`'s
   README, `git log -S` on the sentence that would move)? If it is,
   shape 2 is Ev's — take shape 1 here and write shape 2 up on the
   m7-8 row as the `[ev]` proposal, with the measurement of what it
   would buy. If it is not, and the door's argument survives phase 1
   (SITE's `register_equal` allowlist and the `bit_identity` gate are
   the fences to name — say why a structural-identity predicate is not
   the retired channel), build it, and both rows close.

Either way the null-edge hole closes here, typed, with the gate's
docs re-worded to the claim it now keeps.

**Rows.** Red-first: `the_gate_skips_a_null_scaffolded_edge_however_far_the_run_moves`
pins the hole today — it flips to the typed refusal (or, under shape
2, to `Ok` at the old point and refusal at `(99, 99, 99)`). Control:
`mev_null` unchanged bit for bit; every kernel run site (`splitting/insert.rs`,
`boolean/insert.rs`, `boolean/vtxfac.rs`) green; the S93 `mev` gate's
existing rows green. Under shape 2 the M7-8 no-move row goes from
refusal to `Ok` and the moved M7-8 row stays a refusal.

**Receipt.** Every `MevSite::Fan` run site in production code with
whether it can present a null edge in a moved run (the row's census:
three kernel sites, all `mev_null`); every reader of the gate's
refusal vocabulary.

**Seams.** `euler.rs` and `null.rs` are this program's. Shape 2 touches
`geom-core` (PROPS'/unowned — check territory) — if built, announce on
the owner's log.

Branch `topo/rebasing-gate-null-edges-and-no-move`. PR title: "TOPO:
the re-basing gate refuses a null edge in a moved run, and says what it
cannot ask". Do not close the items; the dual runs at review — the
m7-8 row closes with this one only if shape 2 lands, else it carries
the `[ev]` proposal.

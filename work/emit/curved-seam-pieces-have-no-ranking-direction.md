---
id: curved-seam-pieces-have-no-ranking-direction
kind: issue
title: A seam whose side is curved has no direction to rank its pieces along, so a pair boolean or union with a seam chain on a curved face refuses SplitReference
status: closed
opened: 2026-09-30
priority: P0
cost: M
refs: [edge-pieces-are-named-by-their-ends]
closed: 2026-10-01
pr: 3629
---


## What

Every ranker along a seam line orients it by the pair's `n_a × n_b`
over the two sides' outward plane normals:
- the pair boolean's seam chain (`emit_topo::name_boolean_edges`);
- the rankers that know a seam only by its name (`emit_topo::seam_line_dir`,
  reached from the descent ranker and `resolve_edge_carrier`);
- the union's seam pieces (`emit_union::name_by_parents`).

A side on a curved carrier has no plane, and each ranker refuses a
curved side with `NamingError::SplitReference { curved: true }`, a
missing rule. The recipe is legal and the body sound.

Two details of the refusal as it stands:
- `resolve_edge_carrier` (the seam-vertex carrier) prefers the A
  parent's carrier and falls back to B only when A has none. A curved
  A side refuses; it does not fall back to B.
- In a union the refusal comes from the fold's pair step, which names
  the seam `Seam { a, b }` in member order. The `group` a refusal
  carries therefore depends on member order: every order cites the
  same seam, but which side is `a` changes.

## Evidence

A cylinder of radius 0.3 lying along y across a plate's top (the plate
`[0,3]²×[0,1]`, the cylinder's axis at x = 1.5, z = 1.0, from y = 4.0
for 5.0) unions to a sound body. The pair boolean and the union in both
member orders refuse `SplitReference`, citing the cylinder's wall
(`crates/editor-core/tests/emit_union_borders.rs`,
`a_curved_divider_answers_as_the_pair_boolean_does`). That fixture
reaches the pair boolean's seam chain; in the union it reaches the same
site through the fold's pair step, not `emit_union`'s own ranker.

**Untested: `seam_line_dir`'s curved refusal.** Reaching it needs a
curved seam minted in one step and cut by a later step that mints no
curved seam chain of its own. The seam-chain site runs first in a step,
so a new curved chain refuses there before any descent is ranked. Two
cheap attempts, a vertically sunk cylinder boss on a plate with a slab
subtracted across it, did not reach it:
- a slab from inside the plate (z 0.5 to 1.2) refuses in the kernel
  (`CurvedSectorSideUnsupported`);
- a slab from above, x in [-1, 4] and y in [1.4, 1.6], refuses at the
  new step's seam chain.

Turned 90°, the slab from above refuses `Duplicate` instead
(`a-slot-across-a-sunk-cylinder-boss-refuses-duplicate-merged-face-name`).

## The question

Which direction a curved seam's pieces are ranked along. It has to be
one direction that every ranker along the seam reads, whichever step
cut it (`seam_line_dir`'s contract), and its sign has to move only when
the two faces' material sides do (S10, `emit_topo::carrier_plane`). Some
candidates:
- the planar side's normal crossed with the curved side's normal
  sampled at a point of the seam: one line, but the sample has to be
  a structural choice;
- the seam curve's own oriented parameter (the edge's carrier), as a
  non-seam edge's pieces are ranked (`emit_topo::edge_dir`);
- a rule by carrier kind (a cylinder's axis, a cone's axis), which
  leaves a spline side without one;
- no rank: the pieces tie (N2), which gives up the ordinal.

A chain whose seam is itself a closed curve (a circle on a plate) has
no endpoint order along any single direction, so the answer may need
to say what a closed seam's pieces are ranked by as well.

This row splits out of
`a-seam-chain-along-a-curved-face-refuses-as-an-emission-bug`, which
closed once the refusal was recategorized.

## Ruled (2026-10-01, PR 3553)

Ev took the recommendation of fork-log row 22 ("the recommendation
sounds good, including the change to what was decided in 512!"). The
rule is `crates/editor-core/src/names/README.md` N2. Seam pieces take
`Ends` (the sorted pair of their end vertices' names) instead of a rank
along `n_a × n_b`, so no ranker reads a plane and `SplitReference`
retires. `edge-pieces-are-named-by-their-ends` implements it and closes
this row.

## Closed (2026-10-01, PR 3629)

`edge-pieces-are-named-by-their-ends` built N2's rule: seam pieces take
`Ends`, so no ranker reads a plane and `SplitReference` is gone. The
evidence fixture names in the pair boolean and in the union in both
member orders, with one table between the orders
(`emit_union_borders::a_curved_divider_names_as_the_pair_boolean_does`).

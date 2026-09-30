---
id: curved-seam-pieces-have-no-ranking-direction
kind: issue
title: A seam whose side is curved has no direction to rank its pieces along, so a pair boolean or union with a seam chain on a curved face refuses SplitReference
status: open
opened: 2026-09-30
priority: P0
cost: M
design: true
---


## What

Every ranker along a seam line orients it by the pair's `n_a × n_b`
over the two sides' outward plane normals:
- the pair boolean's seam chain (`emit_topo::name_boolean_edges`);
- the rankers that know a seam only by its name (`emit_topo::seam_line_dir`,
  reached from the descent ranker and `resolve_edge_carrier`);
- the union's seam pieces (`emit_union::name_by_parents`).

A side on a curved carrier has no plane, so each of them refuses
`NamingError::SplitReference { curved: true }`, a missing rule. The
recipe is legal and the body sound.

## Evidence

A cylinder of radius 0.3 lying along y across a plate's top (the plate
`[0,3]²×[0,1]`, the cylinder's axis at x = 1.5, z = 1.0, from y = 4.0
for 5.0) unions to a sound body. The pair boolean and the union in both
member orders refuse `SplitReference`, citing the cylinder's wall
(`crates/editor-core/tests/emit_union_borders.rs`,
`a_curved_divider_answers_as_the_pair_boolean_does`).

## The question

Which direction a curved seam's pieces are ranked along. It has to be
one direction that every ranker along the seam reads, whichever step
cut it (`seam_line_dir`'s contract), and its sign has to move only when
the two faces' material sides do (S10, `emit_topo::face_plane`). Some
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

---
id: a-plane-split-of-a-curved-face-refuses-as-an-emission-bug
kind: issue
title: A plane split that leaves two pieces of a curved face on one side refuses face_plane's emission bug; the split ranker has no rule for a curved parent
status: closed
opened: 2026-09-30
priority: P0
cost: M
refs: [edge-pieces-are-named-by-their-ends]
closed: 2026-10-01
pr: 3629
---


## What

A `Split` whose plane crosses a curved face twice leaves two of its
pieces on one side. The split ranker orders those pieces along
`n_parent × tool_normal`, reading the parent's normal with
`emit_topo::face_plane` (in `emit_topo`'s split fragment naming,
the "Same-side multiplicity" block). The parent is curved, so it
refuses `NamingError::Emission { "face_plane: non-planar carrier in
planar pipeline" }`: a kernel-bug framing for a legal recipe.

## Evidence

Measured with a scratch probe (not committed): the `circle(0, 0, 0.5)`
disc on the xy frame, extruded 1.0, split by the datum plane through
`(0, 0.3, 0)` with normal `(0, 1, 0)`. The split node refuses with the
`Emission` above. The same cylinder split through `(0.3, 0, 0)` with
normal `(1, 0, 0)` evaluates, because there each wall face is crossed
once.

## The question

Two parts:
- the category: this is a missing rule, not an emission bug.
  `NamingError::SplitReference` is the seam rankers' refusal for a
  curved side; the split ranker needs either that variant widened to
  it or a variant of its own;
- the rule: which direction a curved parent's same-side pieces are
  ranked along. It is the split's form of
  `curved-seam-pieces-have-no-ranking-direction`, and may share its
  answer.

This is `face_plane`'s last caller in `crates/editor-core/src/names/`;
the seam rankers read through `emit_topo::seam_side_normal` instead.
`face_plane`'s `Emission` stays right for a caller whose face is planar
by construction.

## Ruled (2026-10-01, PR 3553)

Ev took the recommendation of fork-log row 22 ("the recommendation
sounds good, including the change to what was decided in 512!"). The
rule is `crates/editor-core/src/names/README.md` N2. The Split's
same-side face pieces take `Keeps` (the parent's boundary edges each
holds a stretch of) instead of a rank along `n_parent × n_tool`, so
`face_plane` is no longer read and nothing refuses.
`edge-pieces-are-named-by-their-ends` implements it and closes this row.

## Closed (2026-10-01, PR 3629)

The Split's same-side face pieces take `Keeps`, so `face_plane` and
`carrier_plane` are gone. The probe's cylinder (radius 0.3, split at
y = 0.15) names, pinned by
`emit_split_edge_lineage::a_wall_crossed_twice_names_its_same_side_pieces_by_the_edges_they_keep`.

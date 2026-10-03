---
id: edge-pieces-are-named-by-their-ends
kind: unit
title: Edge pieces are named by their ends; the Split's same-side faces by the edges they keep
status: closed
opened: 2026-10-01
closed: 2026-10-01
priority: P0
cost: H
refs: [curved-seam-pieces-have-no-ranking-direction, a-plane-split-of-a-curved-face-refuses-as-an-emission-bug, a-split-mints-a-twice-crossed-edges-pieces-under-one-name, edge-dir-is-a-chord-so-curved-edge-pieces-misrank]
pr: 3629
branch: emit/edge-pieces-by-ends
---


## What

Ev took the recommendation on PR 3553 (2026-10-01, "the recommendation
sounds good, including the change to what was decided in 512!"; fork-log
row 22). Pieces of an edge are no longer ranked along a direction; each
is named by its ends. The rule is `crates/editor-core/src/names/README.md`
N2. This unit builds it, and closes the four rows in `refs`:
- `curved-seam-pieces-have-no-ranking-direction`;
- `a-plane-split-of-a-curved-face-refuses-as-an-emission-bug`;
- `a-split-mints-a-twice-crossed-edges-pieces-under-one-name`;
- `edge-dir-is-a-chord-so-curved-edge-pieces-misrank`.

## Scope (ruled)

- **`Ends` for every edge piece.** `Fragment(Ends{v0, v1})`, the sorted
  pair of the piece's two end vertices' names as the node publishes them.
  It covers:
  - a pair boolean's seam chain;
  - pieces of an operand edge (`FromA(e)`, the split's edge lane);
  - pieces of an earlier seam;
  - a union's pieces of a member edge, `FromMember(m, e)` + `Ends` over
    the union's published vertex names, read off the finished body;
  - section chords of one face (`SectionEdge{side, face}`), which N2 no
    longer ties (Ev's change to #512's A2).

  Equal pairs are N4's tie. Vertex names cite edges by their heads, never
  by a piece's qualifier, so the mint order is edge bases, then vertex
  names, then edge qualifiers.
- **Crossing vertices.** Where one edge crosses one face several times,
  the crossings share a name and keep an ordinal along the crossed edge,
  by its carrier's own parameter (not by projection onto a chord). The
  orientation is the edge as the operand body stores it; a seam edge
  runs as the loop of the pair's first side runs along it. An equal pair
  ties.
- **`Keeps` for the Split's same-side faces.** Several pieces of one
  parent on one side of the tool plane take `Qualifier::Keeps`, the sorted
  set of the parent's boundary edges each holds a stretch of, cited by
  head. This replaces `n_parent × n_tool` on planar parents too.
  `SplitFragment{side}` stays.
- **What retires.**
  - `Qualifier::OrderAlong` for edges;
  - `emit_topo::seam_line_dir` and the `n_a × n_b` a-first convention;
  - `SeamLineSides`;
  - `NamingError::SplitReference`, and `face_plane`'s last naming caller;
  - the union's member-edge cell count;
  - the union collapse's `of − 1 − rank` reversal.

  No naming rule reads a plane or a direction, so nothing refuses.
- **N5.** The `GroupResized` rung narrows in its edge case: a cut
  elsewhere on a parent no longer changes a piece's name.
- **Not yet designed.** An `Ends`-delta diagnosis rung, the edge
  counterpart of the border delta (`resolve::border_delta`), is not
  designed. Do not invent one here; file it if the unit finds a vanished
  edge piece with no diagnosis richer than `cause_not_in_evidence`.

## Cost

Stored goldens move: seam chains, operand-edge pieces, union member-edge
pieces, section chords and split pieces are renamed. Re-baseline and say
what moved. The union's cell count can land after the seam and split
changes, as its own PR.

## Built (PR 3629)

One PR carries the whole scope: `Ends`, the crossing ordinal and
`Keeps` share the vertex-before-qualifier mint order and the retirement
of `OrderAlong` for edges, and no intermediate state named consistently.

What stays: `names::canonical`'s `RankRule::Reverse` is kept for seam
vertex groups. A crossing ranked along a union seam reads from the other
end where the collapse (or a re-map) puts the seam's pair in the other
order, because the edge is oriented by the loop of the pair's first side.
It no longer applies to any edge.

What is left, as found, each on its own row:
- **The `Ends`-delta diagnosis rung is not designed**, as ruled. A
  vanished edge piece falls to the existing rungs: a flip or an edit on
  its path (its ends cite the cutter, so the cutter is on that path
  now), the group-size rung where its group stopped being divided, then
  the fallback. No fixture in the suite reached the fallback with an
  edge piece, so that rung has no row. Whether the upstream flip and
  recipe-edit rungs are still reachable from emitted names:
  `are-the-upstream-flip-and-recipe-edit-rungs-reachable-from-emitted-names`.
- The crossing ordinal is not local: a second crossing by a face that
  already crosses an edge renames the first and the pieces ending at it
  (`a-second-crossing-by-one-face-renames-the-first-and-its-pieces`,
  design).
- A union's crossings of a seam of several curves tie
  (`crossings-of-a-union-seam-of-several-curves-tie`).
- Crossings of a seam edge whose faces' names do not settle the first
  side tie, where N2 rules only the equal pair
  (`crossings-of-a-seam-edge-whose-sides-names-do-not-settle-tie`).
- Crossings of a NURBS edge tie
  (`a-crossing-of-a-nurbs-edge-ties-for-want-of-its-parameter`).

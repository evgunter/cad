---
id: shared-vertex-crossings-that-tie-or-interleave-are-unprobed
kind: issue
title: "SharedVertexCrossings: a dangling null edge holding another pair's cut is reachable and refuses; the fan-interleave and one-arc arms are reached by no witness"
status: open
opened: 2026-10-03
priority: P0
cost: M
---


## What

`insert::reconcile_shared` (`crates/topo/src/boolean/insert.rs`)
refuses `BooleanError::SharedVertexCrossings` when a pair at a vertex
several crossing pairs cut has no run clear of the other pairs' cuts.

Built on `fuse/shared-vertex-tie`:

- **Cuts that tie in one corner** (a pinch line lying flat in a face of
  the shared corner): placed by the runs (`insert::tied_held`), the
  side each run takes being read off its piece's faces. Witnesses:
  `a_pinch_line_flat_in_the_shared_corners_face_builds_in_every_op`
  and `three_pieces_two_of_whose_cuts_tie_build_in_every_op`.
- **Two dangling null edges tied in one corner**: they splice by their
  lower germs, so `strut_anchor`'s tie is a `ClassificationInvariant`.
  Witness: `two_dangling_null_edges_meeting_on_the_pinch_line_build_in_every_op`.

Still refusing:

- **A dangling null edge whose segment holds another pair's cut:
  reachable.** A piece with a reflex corner at the point (a block's
  pyramidal pit with a spike in it, against a cube's corner) leaves its
  dangling null edge on the Out side, holding the spike's cuts; its
  other way round is the whole orbit. Pinned:
  `a_dangling_null_edge_holding_another_pairs_cut_refuses_typed`.
  Building it needs nested null edges at one vertex whose ends differ
  (In of one, Out of the other), which `mint_directed`'s end guard
  forbids: a design question.
- **A fan both of whose ways round hold a cut (interleave)**: disjoint
  pieces do alternate round a convex corner, but a run that is its own
  piece's In-arc is clear, so this needs a pair paired across its
  Out-arcs (four crossings, the In-arc wrapping the A orbit's first
  sector). That shape refuses `PairingMismatch` first:
  `work/cleave/a-corner-crossing-another-four-times-refuses-pairing-mismatch.md`.
  Unreached.
- **Two pairs whose runs are one arc**: needs the shared corner's whole
  boundary inside two pieces touching along two of its rays, which
  leaves the vertex no sector of its own. No witness.

## Owed

Decide the nested-null-edge structure for the reachable arm and build
it, flipping its pin. Re-probe the interleave arm once the
`PairingMismatch` row is fixed. Find a witness for the one-arc arm or
show it unreachable.

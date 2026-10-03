---
id: shared-vertex-crossings-that-tie-or-interleave-are-unprobed
kind: issue
title: "SharedVertexCrossings: the fan-interleave arm, the one arm left, is reached by no witness"
status: dispatched
opened: 2026-10-03
priority: P0
cost: M
branch: fuse/strut-holding-a-cut
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

Built on `fuse/strut-holding-a-cut`:

- **A dangling null edge whose segment holds another's whole**: the
  inner hangs at the outer's tip (`insert::holds_whole`), strictly
  inside or along either end's direction. Witnesses:
  `a_dangling_null_edge_holding_another_pairs_cut_builds_in_every_op`,
  `a_dangling_null_edge_holding_two_pairs_cuts_builds_in_every_op`,
  `a_dangling_null_edge_inside_another_along_one_end_builds_in_every_op`.
- **Two pairs whose runs are one arc**: reachable as two dangling
  null edges with one segment, which nest the same way. Witness:
  `two_dangling_null_edges_with_one_segment_build_in_every_op`. For
  two fans, the other way round is the complement on the opposite
  side, so one turns: no refusal.

Still refusing:

- **A fan both of whose ways round hold a cut (interleave)**: disjoint
  pieces do alternate round a convex corner, but a run that is its own
  piece's In-arc is clear, so this needs a pair paired across its
  Out-arcs (four crossings, the In-arc wrapping the A orbit's first
  sector). That shape refuses `PairingMismatch` first:
  `work/cleave/a-corner-crossing-another-four-times-refuses-pairing-mismatch.md`.
  Unreached.

## Owed

Re-probe the interleave arm once the `PairingMismatch` row is fixed.

## Landed (FUSE, PR 3943, 2026-10-03)

The tie arms are built, and the strut tie is an invariant. Review tier:
single FULL, with one fix pass that added three things:
- strut cuts take their side from angular ends;
- `reconcile_shared` runs to a bounded fixed point;
- a row that turns red under the old leaving-germ splice.

The row stays open at P0 for the reachable arm. Its pinch-end 3′
failure is now its own row,
`a-carried-row-whose-ends-split-into-null-edge-copies-is-dropped`.


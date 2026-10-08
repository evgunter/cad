---
id: inline-refuses-a-posed-instance-a-placement-pose-could-take
kind: issue
title: Inline refuses UnplaceableFrame for a posed instance of plain geometry, though the part's placement now carries a pose that could take the frame
status: closed
opened: 2026-10-08
closed: 2026-10-08
---


Found by INTENT stage 2 unit C (branch `intent/s2-c-world`).

`InlineError::UnplaceableFrame` (`crates/editor-core/src/refactor.rs`)
refuses an instance off the world's origin whose part places plain
recipe geometry: plain geometry "sits on no gauge", so the frame is not
expressible locally. Spec §4 keeps it, reading the part's placements,
and C built it so (the refusal names the part's placement).

Under the world a `PlaceInWorld` carries a pose (residue 3 of #4220),
so the frame now has a local home: inline could compose the instance's
frame onto the carried placement's pose instead of refusing. Inline
refuses the other half of that today too
(`InlineError::PlacementPoseCrosses`: a posed host placement over a part
world that is not one identity placement), because it computes no
frame. Whether inline should compose poses is a design question (it
changes A4's inline rule); the rows that pin today's refusal are
`asm4_split_inline::row3_further_typed_refusals` and
`node_labels::an_inline_refusal_speaks_host_nodes_from_the_host_and_part_nodes_from_the_part`.

## Closed

Superseded by stage 3 (`[ev]` #4326; orchestrator ruling on PR #4359).
A placement's pose is a transient interim that only the gather and
export read as position, so inline does not compose a frame into it;
stage 3 replaces the pose, and with it the refusal this row asked about.

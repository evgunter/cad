---
id: a-mirrored-split-through-a-pinch-refuses-emission-at-an-on-plane-vertex
kind: issue
title: A split through a pinch whose two runs lie in the LOWER half refuses Naming(Emission) for an on-plane vertex with neither a SplitEdge record nor an operand identity
status: closed
opened: 2026-10-17
priority: P1
cost: M
rides_with: a-split-through-a-pinch-names-both-tip-copies-alike
pr: 5275
closed: 2026-10-18
---


## Finding

Measured by a FORK-PINCHNAME designer (2026-10-17, against `35794d0de7`):
the `MIRRORED` constant of `crates/topo/tests/m3_pr3_split.rs`, extruded 1
and split at `y = 1`, puts the pinch's two runs in the lower half. Where the
`NOTCHED` case (runs above) refuses `Naming(Duplicate)` — the
`a-split-through-a-pinch-names-both-tip-copies-alike` row — this one refuses
earlier with `Naming(Emission("on-plane vertex with neither a SplitEdge record
nor an operand identity"))`, the `else` arm after the `OnToolVertex` arm in
`names/emit_topo.rs` `name_split_edges_vertices`. A refusal that depends on
which side of the plane the runs fall is a bookkeeping defect, not a naming
choice.

Unverified guesses at the cause: a copy the reduction made for a pinch on the
lower side has no row in `naming.vertex_pairs`, or the flat floor at
`(6..7, 1)` in the fixture. Isolate which before fixing.

Rides with the split-pinch row: the unit that builds that rule must pass both
the `NOTCHED` and `MIRRORED` fixtures, or the qualifier lands on one side
only.

Signed (WIRE orchestrator).

## Resolution (2026-10-18)

Closed by PR 5275 with its companion row: the `MIRRORED` fixture now
names, pinned by `a_mirrored_split_through_a_notch_tip_tells_its_copies_apart_by_keeps`.
The flipped-normal revolved refusal is a separate row,
`a-revolved-pinch-split-refuses-with-its-normal-flipped`.

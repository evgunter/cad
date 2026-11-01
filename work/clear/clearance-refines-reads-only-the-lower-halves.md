---
id: clearance-refines-reads-only-the-lower-halves
kind: issue
title: The clearance window door's refines() halves only toward the lower ends, so a loft wall whose enclosure extremes all lie in its lower u- or v-half is refused as a chart that does not refine
status: review
opened: 2026-10-31
priority: P0
cost: E
branch: clear/refines
pr: 6206
---


## What was found (CARVE's embedding-certificate measurement, 2026-10-31)

`refines` in `crates/topo/src/clearance.rs` (the admission rule
`window_of` applies before a face enters the sweep) builds the whole
window's box and the box of the LOWER half on each axis
(`(u.0, mid)` and `(v.0, mid)`), and admits the chart only when each
lower half moves at least one bound. The upper halves are never read.
On a face whose six box extremes are all attained in the lower half of
an axis, that half's box equals the whole box, and the face is refused
`Unsupported { carrier: "a face whose interval chart does not refine
…" }`, although the upper half narrows and the subdivision would
converge.

A loft wall hits this whenever its whole extent is reached on one side
of its `u` or `v` midpoint. The planar arc-spine loft of
`crates/sweep/tests/bool6_per_slab_stacking.rs` (a 0.1 square along a
unit-radius arc, 17 stations) is the clean case: on the wall
`(0.05, ±0.05)`, every extreme of `x` and `z` is reached at the profile
coordinate −0.05, so the lower `u` half spans the whole box.

Refused so, in the scratch `crates/sweep/tests/carve_embed_scratch.rs`
on `carve/embedding-certificate` (each at `Body<f64>` and
`Body<Interval>`):

- the arc spine at 3.5, 6, 9 and 13 rad over 17 stations;
- `m8_14_long_turn_sweep`'s half-turn helix at 17 stations;
- CONTACTHOLD's closed spine (a 0.5 square around a radius-3 circle,
  9 stations);
- CARVETAIL's opposite-turning joints loft;
- the two-section loft whose top faces down (z 0 and 1, normals +z and
  −z). This wall is a bow-tie whose `u` iso collapses at `v = 0.5`, so
  NEITHER `u` half narrows it, and the refusal stands with the fix
  below. The body is not embedded, so refusing is right, but it is
  refused as an unsupported chart rather than as a self-intersection.

## The experiment

The scratch gates an alternative behind
`CARVE_EMBED_SCRATCH_REFINES_BOTH_HALVES`: an axis refines when EITHER
of its halves moves a bound. With it, every row above except the
top-facing-down one is admitted, and the sweep then decides them:

- the 9 and 13 rad arcs are `Violated`, naming the wall against itself
  between sections 1..2 and 12..13 (and 4..5 against 12..13);
- the closed spine is `Violated`, top against bottom at (3, 0, ±0.25);
- the CARVETAIL loft is `Violated`, wall 0 against wall 3 between
  sections 0..1.

All are the right answers. The doc comment's premise (inclusion
isotony: an axis that does not move under a sub-box never moves) holds
of the pair of halves, not of one half alone.

## Home

CLEAR: `refines` in `crates/topo/src/clearance.rs`. Reading both halves
costs two more enclosures per window. Whether an axis needs both halves
to move, or either, is the design question. Either is sound for
admission, because the engine still refuses later on depth or budget.
The CARVE unit `self-overlapping-spines-build-and-validate` cannot
certify the loft and sweep bodies above until this is settled.

## A long helical sweep hits it too (CARVE, PR 6138, 2026-10-31)

`crates/sweep/tests/self_intersection_proves_injectivity.rs`'s helical
row (a 0.16 square swept two turns round a radius-1 helix of pitch 0.4,
65 stations, v-degree 3) used to certify `Holds` within the default
budget. Under the sweep frame of
`work/carve/sweep-frame-is-a-minimal-rotation-from-the-start-tangent.md`,
which carries the section without spinning it, the window door refuses
its wall `FaceKey(3v1)` as `Unsupported { carrier: "a face whose
interval chart does not refine …" }` at `f64`. With
`CARVE_EMBED_SCRATCH_REFINES_BOTH_HALVES` set (the experiment above,
applied locally and not committed), the same body certifies `Holds`
again (64.6 s in a debug build). The row is renamed
`a_long_helical_sweep_waits_on_the_window_doors_halving` and pins the
refusal, citing this row. When this row lands it goes red, and it goes
back to `holds_at_both`.

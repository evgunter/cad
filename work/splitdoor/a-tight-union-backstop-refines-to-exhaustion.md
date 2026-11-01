---
id: a-tight-union-backstop-refines-to-exhaustion
kind: issue
title: The union's volume backstop refines every face to exhaustion where the bound is tight: the tilted plug's union costs 8.7 s in debug against 30 ms for its differences
status: closed
opened: 2026-10-28
priority: P3
cost: M
branch: cleave/volume-relations-a-test-oracle
pr: 5842
closed: 2026-10-31
---


Found by `splitdoor/tilted-plug-rosters`
(`a-tilted-plug-union-refuses-at-the-germ-segment-and-section-copy-rosters`).

## What

`crates/sweep/tests/continuation_pose.rs`
`a_tilted_plug_in_its_hole_reads_alike_at_every_pose`: the tilted plug
(θ = 0.3) back in its hole. Timed at the identity pose, ε = 1e-9, debug:
`topo::union` 8.7 s; `topo::intersect` 27 ms; `topo::subtract` 37 ms;
the union's own mass properties 0.4 ms. The upright twin's whole test
(13 poses × five ops) runs in 6 s; the tilted one's in about 350 s.

Stack samples of the union (gdb, five samples) all sit in
`boolean::ops::bound_holds` → `props::walk_set`, the volume backstop.
The plug and the holed slab fill the slab exactly, so the backstop's
`vol(A ∪ B) ≤ vol A + vol B` holds with equality: the enclosures never
separate the two sides, and the walk refines every face (the walls'
ellipse-bounded quadrature) round after round until the lane stops it,
then decides what is left against the band. With circle rims every face
takes a closed form and the walk ends at once.

## Direction

A tight inequality is the common case for a union of operands that
touch without overlapping (a plug in its hole, two blocks face to
face). Read where the walk could stop early: an inequality that holds
with equality is a Zero, decided against the band at the first round
whose enclosures already lie inside it, rather than after the lane's
last refinement. Not measured beyond the timing and the samples.

Banded P3 by its filer: a union of touching operands is everyday input,
and its cost is a user's latency as well as the suite's; the
orchestrator re-bands.

## Closed (2026-10-31)

The volume backstop left the boolean's operation path (`[ev]` PR 5780: the volume relations are a test oracle the suites run on their own results, `topo::test_support::VolumeOracle`), so nothing refuses or passes on it. The union no longer walks any face to decide a volume inequality, so the tilted plug's union pays only the at-rest gate's own measure, as its differences do. The timing was not re-taken.

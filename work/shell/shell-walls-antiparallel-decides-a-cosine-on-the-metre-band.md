---
id: shell-walls-antiparallel-decides-a-cosine-on-the-metre-band
kind: issue
title: shell_walls_antiparallel decides a bare cosine against the metre band, and the dimension audit has no row for it
status: closed
opened: 2026-10-01
priority: P2
cost: E
rides_with: shell-clearance-footprint-reads-vertices-not-arcs
pr: 4115
branch: shell/planar-gate-misses
closed: 2026-10-06
---


## What

`crates/topo/src/shell.rs`, the wall-pair gate in the thicken path
(`shell_walls_antiparallel`, near :2379), decides

    Margin::of(-(a.normal.dot(b.normal)) - T::one())

against the metre band. Both normals are unit vectors, so the margin is
`−cos θ − 1`, dimensionless: whether two faces "face each other" is
read at an angle of about `√(2ε)`, the same at every model size. The
ray caster's skip test had this shape (ledger row F2, fixed on
`reach/opensign-red`); there a non-length comparand let a wide rod's
wall go unseen at ε = 1e-6.

`docs/predicate-dimension-audit.md` has no row for this predicate, and
`work/tier/symbolic-tier-census.md` lists it as "not in the M10-8
documents".

## Found by

The REACH `opensign-red` lane's sweep for bare-cosine margins
(`Margin::of(… .dot(…))` outside tests), 2026-10-01. Not measured on a
fixture: no row here shows it answering wrong.

## Final state

The gate's margin is a length (the angle levered by the extent over
which the two walls are compared, as the other parallel gates lever
theirs), or the audit carries a row saying why a cosine is the right
comparand here.

## Closed (SHELL orchestrator, 2026-10-06, PR 4115)

The facing test now reads a pair when the levered drift
`2·sin(δ/2)·L` is within one wall, and takes its gap short by that
drift. It also keeps the old cosine window, so the union never narrows
what the gate read before (the review's MINOR 1, pinned by
`the_clearance_gate_reads_a_near_parallel_wall_on_a_tall_part`). The
drift correction is pinned by
`the_clearance_gate_takes_a_tilted_gap_short_by_its_drift`, which goes
red under the review's mutant C. The predicate is in the dimension
audit. A pair outside both windows is the tilted residue,
`shell-clearance-gate-skips-planar-pairs-tilted-off-antiparallel`.

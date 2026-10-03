---
id: a-corner-crossing-another-four-times-refuses-pairing-mismatch
kind: issue
title: "A corner whose boundary crosses another corner's four times refuses PairingMismatch: the B-adjacency guard orders two germs in one B sector by their A sector"
status: open
opened: 2026-10-03
priority: P1
cost: M
---

## What

A thin trihedral corner at the origin (`corner_prism` over the rays
(0.5948, 0.757, −0.2704), (−0.5774, −0.5774, 0.5774),
(0.757, 0.5948, −0.2704), scaled 0.4) against the unit cube's corner
there. The corner's cone holds the cube's z-edge and crosses the
cube's bottom face, so the two corners' boundaries cross four times:
two section-polygon edges at one vertex pair. Every op, in both
operand orders, refuses `PairingMismatch`. Pinned by
`a_corner_crossing_the_cubes_four_times_refuses_pairing_mismatch`
(`crates/topo/tests/union_flush_onto_edge_contact.rs`). The same
holds under the cube's three cyclic axis rotations (measured).

## Cause, as far as traced

F12 guard 1 in `insert::plan_null_pairs`
(`crates/topo/src/boolean/insert.rs`) checks each A-consecutive pair
is cyclically adjacent in B order, and builds B order by sorting the
survivors on `(b, a)`: B's sector index, then A's. Two germs in one B
sector are then ordered by their A sector, not round the B sector.
Measured survivors (a, b): (0,1), (1,2), (2,1), (2,2): the two germs
in B sector 1 and the two in B sector 2 each sort by A, which puts
the B order at [0, 2, 1, 3], and the pair (0, 1) reads as not
adjacent. Ordering germs within one B sector needs the angular
reading `insert::walks_after` makes within an A corner. The A-major
order the pairing itself consumes (`ordered.sort_by_key(|(r, _)| (r.a,
r.b))`, same function) has the same blind spot: germs in one A sector
entry are ordered by their B sector, not round the A sector, so four
germs in one A entry (two struts' worth) could be paired across.
Unprobed.

## Why it matters beyond itself

It is the candidate witness for `SharedVertexCrossings`' fan arm (a
null edge both of whose ways round hold another pair's cut,
`work/fuse/shared-vertex-crossings-that-tie-or-interleave-are-unprobed.md`):
a pair with four crossings, its In-arc wrapping the A orbit's first
sector, pairs across the arcs outside its piece, and a piece in each
of those arcs leaves neither way round clear. With two more corners
in the gaps (the measured probe: j and k of
`three_corners_alternating_round_the_cube_refuse_three_ops`' shape,
rotated), this guard refuses first, so that arm is unreached. Fixing
the guard should re-probe it.

## Owed

Order germs within one B sector round the sector, then flip the pin
to a build in every op at volumes checked outside the kernel, and
re-probe the shared-vertex fan arm.

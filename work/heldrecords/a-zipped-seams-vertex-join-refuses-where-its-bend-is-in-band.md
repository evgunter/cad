---
id: a-zipped-seams-vertex-join-refuses-where-its-bend-is-in-band
kind: issue
title: A junction's zipped seam refuses at the vertex join where its bend lands in band, though its wedge is decided π
status: open
opened: 2026-11-23
priority: P2
cost: M
---


Found by HELDRECORDS' elbow lane (2026-11-23) on the second review of
PR 7432, which deferred it to the seam lane (PR 7471): it is that
lane's to do or to rule out, and this row holds it until then.

## Read

`crates/topo/src/boolean/edge_join.rs:743-756` (`joinable`): at a
vertex between two edges on one tangent locus, the join reads
`geom_brep::tangent_second_order` at the vertex and joins only where
the sign is decided `Positive`. An in-band reading is refused through
`regularity` (`JoinReading::Regularity`, `JoinUndecided`); a decided
`Zero` or `Negative` leaves the vertex unjoined.

A junction (C4's `Junction`) glues a π seam whatever its bend, and the
zip mints it as one seam edge carrying `TangentIntersection`. Where
the result holds a vertex on that seam (an arc ending on the rim, a
rim cut by a crossing), the join asks the seam's second order at the
vertex. At an elbow's neutral point, `κ_rel` passes through zero, so
a vertex there or near one reads in band and refuses, though the
junction owed no second-order margin and tier 3's check 4 reads the
seam's wedge π.

Not measured: no fixture yet leaves a vertex on a junction's seam near
a neutral point (PR 7471's elbow seams carry none at the bend's
zeros). Whether check 11 (maximal edges) owes the join there, or the
join should read a decided-π seam as joinable whatever its bend, is
the question.

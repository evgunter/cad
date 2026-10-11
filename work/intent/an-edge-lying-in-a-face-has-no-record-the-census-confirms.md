---
id: an-edge-lying-in-a-face-has-no-record-the-census-confirms
kind: issue
title: An edge lying in a face between two groups is recorded by its bounds alone: the census confirms an edge-face record on no lane
status: open
opened: 2026-10-11
priority: P3
cost: M
---


D1 says "Every pair whose interiors meet is recorded, an edge lying in a
face … included; only how the census confirms a record depends on its
geometry." `topo::record_at_rest` (`census.rs`) records every touch
between two groups except an edge lying in a face: the census's confirm
pass refuses an edge–face record on every lane (`confirm_declarations`,
the `ContactKind::EdgeFace` arm, "an edge-face record, which the census
confirms on no lane"), so a minted one would refuse the body. The door
records the edge's bounds instead, and those records back the overlap on
the census's next pass, which is D1's own confirmation route for a
straight edge in a plane. The pair itself has no record, so a later
reader of the records (a carry, the lint) sees the bounds and not the
edge in the face.

Found by J0's review (R2, MINOR-2). Closing it takes an edge–face
confirm arm in the census (reconstruct from the bound records, as the
overlap lane already does), after which `record_at_rest` drops its
`EdgeFace` exclusion.

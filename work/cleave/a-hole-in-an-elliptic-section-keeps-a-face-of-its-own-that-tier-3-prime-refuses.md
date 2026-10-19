---
id: a-hole-in-an-elliptic-section-keeps-a-face-of-its-own-that-tier-3-prime-refuses
kind: issue
title: A hole in an elliptic section keeps a face of its own over the outline's, and tier 3′ refuses the side
status: open
opened: 2026-10-19
priority: P2
cost: M
---


Found by the re-review of PR 5350 (`cleave/keel-slit-ring`).

## What

A right cylinder of radius 3 tilted 15° about x, less a square pocket
through the plane `z = 0`, with a square post put back inside the pocket
(`crates/sweep/tests/a_keel_ring_in_an_elliptic_section.rs`, the island
row; measured without its trench too). Split at `z = 0`, the section is
an ellipse with the pocket's square as a hole and the post's square as
an island.

`nest_hole_sections` (`crates/topo/src/splitting/finish.rs`) cannot place
the hole. `section_loops::outlines_disjoint` separates a segment from an
ellipse only where the whole ellipse lies on one side of the segment's
line (`line_clears_conic`), so no hole of straight edges inside an
ellipse is ever decided clear of it. The hole keeps a face of its own
(`sense` false) over the ellipse's face. Its doc calls that "sound by
cancellation", and volumes and tier 3 do pass. But tier 3′ with no
records refuses each side: `UnrecordedContact`, `VertexOnFace` and
`EdgeFaceOverlap` (16 of each on the walls' side with the trench), for
the island's cells against the coplanar faces over them, and 10
`CensusUndecidable` between the hole's face and the cylinder's faces,
read as two parts' curved faces in reach, both normals,
trench or no trench. Untilted, the hole nests as a ring and 3′ passes.
The split's gate reads tier 3 only, so the sides ship
(`the-splits-at-rest-gate-reads-tier-3-where-a-finished-body-owes-tier-3-prime`).

## Owed

A segment-against-ellipse disjointness reading that decides the clear
case, the segment's distance from the ellipse arc rather than the line's
from the whole carrier, so the hole nests as a ring of the outline. Then
lift the tilted arm of that test's 3′ pin.

---
id: a-hole-in-an-elliptic-section-keeps-a-face-of-its-own-that-tier-3-prime-refuses
kind: issue
title: A hole in an elliptic section keeps a face of its own over the outline's, and tier 3′ refuses the side
status: open
opened: 2026-10-19
priority: P1
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

## Slate triage (2026-10-26, CLEAVE)

- Re-banded P2 → P1: segment-against-ellipse disjointness decides only the whole-carrier case, so a tilted split ships sides tier 3′ refuses: verb breadth beyond the everyday shapes (a split or boolean that refuses, or ships a body its own gate fails, on a legal input), P1 by `work/README.md` Priority.
- Moved to SPLITDOOR (`work/splitdoor/`) by `git mv`, id and body unchanged: the P1 track opened for split and boolean shapes that refuse, or ship a side their own gate fails, where a uniform reading would build: the split door's tier-3′ bar, tangent and pinch poses, and the curved-carrier lanes that read lines and circles only. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).

## Measured under a tier-3′ split gate (2026-10-28, SPLITDOOR)

`splitdoor/split-gate-3prime` (fefc1eea99): `a_keel_ring_in_an_elliptic_section::a_keel_ring_on_an_island_in_a_hole_nests_on_the_island` refuses at tier 3′ at all three ε with 18 errors (VertexOnFace 4, EdgeFaceOverlap 4, ConformalPatch 1, MayOverlap, BoundaryCrossesItselfUndecided, SameSide*). Not a records problem: the side is wrong-shaped until the nesting fix (`outlines_disjoint` segment against ellipse).

## Moved (2026-10-28, SPLITDOOR)

- Moved to HELDRECORDS (`work/heldrecords/`) by `git mv`, id and body unchanged: SPLITDOOR's seam cut put the records chain Ev ratified on PR 5967 on a track of its own (`work/README.md` Track size).

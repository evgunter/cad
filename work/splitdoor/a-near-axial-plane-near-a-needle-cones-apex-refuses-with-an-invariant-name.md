---
id: a-near-axial-plane-near-a-needle-cones-apex-refuses-with-an-invariant-name
kind: issue
title: a needle cone cut by a plane off its apex near the seam refuses Join SectionInvariant or Join Euler, identically on main
status: closed
opened: 2026-10-29
priority: P1
cost: M
closed: 2026-10-29
pr: 6002
---


## What

Needle cones of half-angle 1° and 3° (profile revolved about y, apex at
(0, 1, 0)), cut by a plane through the point 1000ε along the seam
generator from the apex, well off the apex. The plane's normal is tilted
from the axis by `tilt` about azimuth `spin`. The refusals:

- tilt 1.5, spin 0: `Join(SectionInvariant ..)`;
- tilt 1.2, spin 2.0, at 1°: `Join(Euler ..)`.

Both refusals carry an invariant's name, on ordinary geometry. Both are
identical on main and on PR 5956: the delta review's differential
`review_5956_delta_needle::delta_needle_cone_near_apex_reads_as_main`
(ignored; run it on both heads and diff its `NEEDLE` lines).

## Where to look

The join on a near-axial section of a needle cone near its apex: the
section conic is tiny and steep there. Read the payload and the raising
site before guessing at the cause.

## Closed (2026-10-29, PR 6002)

The poses are in band, not "well off the apex". At k = 1000 the cone's radius at the seam foot is 17ε (1°) or 52ε (3°), so the wall normal there is as uncertain as the plane-to-wall angle.

The `SectionInvariant` pose was `chord_join::arc_leaving` reading a zero heading. On the split arm that Zero now decides `chord_arc_leave_section_aligned`, the datum's cosine with the conic's tangent levered by its extent:
- a datum off the tangent stays `SectionInvariant`;
- a datum along it with a short lever refuses with the new `SplitJoinError::SectionDepartureGrazes`, whose recourse is to cut farther from the apex or tighten ε.

The "Euler" pose was already a typed `dihedral_wedge` escalation. At k = 1e5 both sides land at the closed form π tan²α h³ / (3 D^{3/2}). A 2240-split sweep per ε showed no `SectionInvariant`.

FULL review: APPROVE WITH FIXES. A fix pass separated the two meanings of the Zero and corrected the refusal text.

Filed:
- `tier-3-prime-cannot-settle-a-ruling-against-an-ellipse-it-ends-at-on-a-needle-cone`;
- `a-boolean-cut-near-a-needle-cones-apex-refuses-normal-at-cone-apex-at-the-sections-far-vertex` (P2).

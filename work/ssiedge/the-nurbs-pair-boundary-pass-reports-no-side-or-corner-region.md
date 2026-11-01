---
id: the-nurbs-pair-boundary-pass-reports-no-side-or-corner-region
kind: issue
title: The NURBS x NURBS boundary pass reports no Side or Corner region: a section along or through a side or corner of either rectangle refuses PairBoundaryUnresolved
status: open
opened: 2026-10-10
priority: P3
cost: H
refs: [a-wall-seam-between-two-fits-has-no-section]
---

Found building the NURBS × NURBS arm (`crates/geom-brep/src/ssi/pair.rs`,
`Section::boundary_pass`). The pass walks the eight faces of the two
knot rectangles' product (`ssi::four::face_roots`) and keeps each
solution Krawczyk isolates. Where the section touches or runs along a
side, or passes through a corner, of either rectangle, no face piece
isolates it and the pass refuses `SsiError::PairBoundaryUnresolved`.
The plane × NURBS pass reports such places as `Side` and `Corner`
regions (C3); this lane reports none.

Measured instance: a wall of the planar prism moved alone by `d = 5e-10`
beside its held spline neighbour (`crates/sweep/tests/offd_r1_probes.rs`,
`seam_refused`): at a right-angle dihedral the section runs along the
moved fit's side, and the seam refuses `PairBoundaryUnresolved` at 1e-9
and 1e-12 (`SeedOffDomain`, `CrossingUnmatched` or a step escalation at
other ε, by rounding). The at-rest loft seam (two walls sharing a row)
is the same shape.

Owed: the side and corner regions on this lane, as the plane pass
reads them (a side of one chart within the band of the other surface,
its cover, and the corner classification), so the accounting banks them
and a section along a shared row is answered rather than refused.

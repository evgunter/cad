---
name: Curved containment fallback
description: The no-crossings boolean fallback classifies curved operands by a certified curved-EXTENT scan (fixed at M5 S13) — the old unsound vertex probe survives only as the witness the scan certifies; NURBS is re-gated typed at the fallback; residual curved classes still refuse typed.
type: finding
---

**Current state (since M5 S13, branch `ev/m5-s13-pips-enablers`).**
When the boolean reduction finds no crossings, the containment
fallback runs a certified curved-EXTENT scan (`sphere_extent_scan`
+ rigid re-chart re-cut + one pipeline re-entry; the `(Plane,
Sphere)` germ arm). The vertex probe — sound for polyhedra, unsound
for curved boundaries (a face can leave the other solid strictly
between its vertices; a half-buried unit ball once metered union
16.0 where the truth is 17.30899693899575) — survives as the
WITNESS only; the scan is its certificate. The flipped test row is
`finding_row_flipped_containment_fallback_now_sees_the_curved_extent`
in the S12 suite. Die pips are LIVE — PR 12 shipped on this.

**NURBS is re-gated typed at the fallback**
(`NurbsExtentUnsupported`, pinned): the extent test cannot be
written for NURBS with what exists (`implicit_residual(Nurbs)` is
poison; `NurbsSurface::project` is f64-only), so the class refuses
rather than silently inheriting the admission. If the projection is
ever lifted to `T: Real`, this gate is the site to revisit.

**Still open behind typed doors:** cyl×sphere chords
(`Pcurve::Fitted`), sphere×sphere seams, cone/torus operands,
tangent/coplanar contacts, non-polar residual sections.

History (executed witness, S12's pin-the-defect posture, the
two-unit dependency for die pips — all resolved): M5-LOG and the
S12/S13 specs and suites.

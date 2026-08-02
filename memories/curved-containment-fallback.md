---
name: Curved containment fallback is vertex-probed
description: FIXED at M5 S13 for the sphere class — the fallback now runs a certified curved-EXTENT scan first (escape ⇒ rigid re-chart re-cut + one pipeline re-entry; uncertifiable ⇒ typed refusal; NURBS re-gated typed). The historical finding: vertex probes let a half-buried ball meter as contained (16.0 for 17.309); flipped to construction 17.30899693899575 in the S12 suite.
type: finding
---

When the boolean reduction finds no crossings, `ops.rs::fallback`
classifies each operand shell with `point_in_solid` on its **vertices**.
That is sound for polyhedra and unsound for curved boundaries: a face
can leave the other solid strictly between its vertices.

**Executed witness** (M5 S12, `crates/sweep/tests/m5_s12_curved_ops.rs`,
`finding_sphere_class_containment_fallback_is_wrong_today`): the unit
ball translated to (2, 2, 0.5) inside a 4 × 4 × 1 slab pokes out of both
faces, but its only two vertices are the revolve poles, which sit inside
the slab. No crossings are found for the sphere class, the fallback
declares the ball contained, and `union` returns **16.0** where the true
answer is **17.30899693899575**. Reproduced by the same call on main at
`3ef715e` (S12's parent), so it predates S12 and no part of the revert
wiring touches that path.

**Scope of the ∪ silence today: SPHERE ONLY.** `reduce::gate_planar`
admits exactly `{Plane, Cylinder, Sphere, Nurbs}`, so **cone and torus
operands refuse typed** (`CurvedBooleanUnsupported`) before reaching the
fallback at all — the S12 review's torus probe confirms this on all
three ops — and **no constructor in this build mints a NURBS-surfaced
solid**, so that admitted class is unreachable. Cylinder operands ARE
admitted and DO reach the fallback (the opened ∖/∩ make that newly
common), but their escape is caught by the curved pierce frontier: the
review's half-buried horizontal log and radial-poke probes refuse typed
rather than answering, and the wholly-contained boss answers exactly.

**The NURBS hazard the fix unit must handle.** The obvious fix — replace
the vertex probe with a curved-EXTENT test (a face's extremum against
the other solid) — **cannot be written for NURBS with what exists**:
`implicit_residual(Nurbs)` is poison and the only foot-point projection
is `NurbsSurface::project`, which is `impl NurbsSurface<f64>` (the same
wall PR 9c deviations 2 and 6 hit). So the day a NURBS body constructor
lands, this silence re-opens for a class the extent test cannot cover.
The fix unit must either lift that projection to `T: Real` or
**explicitly re-gate NURBS out of `gate_planar`** — silently inheriting
the admission is the failure mode.

**Why S12 did not fix it.** Re-cutting the fallback is its own unit.
S12's response was to refuse the class up front for the two ops it was
OPENING (`BooleanError::CurvedOpUnsupported` is now per class:
`Plane`/`Cylinder` operands pass, sphere/cone/torus/NURBS refuse), leave
`union` exactly as it was, and pin the defect at the wrong value so the
eventual fix fails loudly.

**Consequence for planners.** The die-pips class (PR 12) waits on TWO
units, not one: the fitted-chord join lane (PR 9c deviations 1–2) and
this fallback. A join lane alone would still leave the no-crossings path
answering wrongly.


**RESOLUTION (M5 S13).** `sphere_extent_scan` + the rotation re-cut +
the `(Plane, Sphere)` germ arm landed together (branch
`ev/m5-s13-pips-enablers`): the finding row is now the construction row
`finding_row_flipped_containment_fallback_now_sees_the_curved_extent`
(union bracketed, one shell), the die-pips smoke rows are green, and
NURBS is re-gated at the fallback (`NurbsExtentUnsupported`, pinned).
The vertex probe survives as the WITNESS only; the scan is its
certificate. Still open behind typed doors: cyl×sphere chords
(Pcurve::Fitted), sphere×sphere seams, cone/torus, tangent/coplanar
contacts, non-polar residual sections.

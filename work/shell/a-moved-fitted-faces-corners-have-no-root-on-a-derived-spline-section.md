---
id: a-moved-fitted-faces-corners-have-no-root-on-a-derived-spline-section
kind: issue
title: A moved fitted face's corner has no root unless one of the fit's own rows meets it: solve_corners skips rooting on a spline surface and a derived spline section seeks no corner
status: dispatched
opened: 2026-10-09
priority: P2
refs: [a-fitted-wall-has-no-section-with-a-moved-cap]
branch: shell/fitted-corners
---

Found by the unit that routed plane × `Approx` over the fit
(`a-fitted-wall-has-no-section-with-a-moved-cap`). Moving a fitted face
now derives its edges with held planes, but its corners refuse: a corner
is a root of some surface around it along an edge meeting it, and for a
moved fitted face none is sought.

- `solve_corners` (`crates/topo/src/replace_face.rs:3012`) skips every
  `Surface::Nurbs` / `Surface::Approx` as the rooted surface
  (`replace_face.rs:3094`), so the moved fit is never rooted along a held
  edge (a box's vertical line, a loft seam).
- `incident_edges` (`replace_face.rs:3185`) seeds no root on a derived
  spline section ("A derived spline section seeks no corner",
  `replace_face.rs:3230`), so the held plane on the corner's other side
  is never rooted along the new plane × fit section either.

What is left is a corner one of the fit's own rows meets (the iso-row
arm's carrier is seeded at its ends). Everywhere else the door refuses
`CornerSection { verdict: Unsupported { "no edge meeting this corner
crosses a surface the corner can be solved on" } }`, which
`crates/sweep/tests/r1_lane0_e2e.rs`'s
`the_f64_seam_answers_every_public_door` pins on a box whose cap is a
NURBS patch: its fitted cap's four edges derive, and its first corner
refuses.

Owed: seed the plane's root on a derived spline section (from the
section's domain ends, which run with the old carrier), or a surface ×
curve root on the fit along a held edge; either makes a moved fitted
face bounded by planes movable, and the shell of such a body the first
fitted wall to move.

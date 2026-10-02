---
id: a-declared-rest-mate-does-not-license-its-rim-against-the-partner-wall
kind: issue
title: Two solids abutting on an equal-radius rim refuse CurvedPierceUnsupported whatever the corner, even with their end discs declared Rest: a declared planar mate never licenses its rim edge against the partner's curved wall
status: open
opened: 2026-10-01
priority: P0
cost: H
design: true
---


## What

Measured by TANG's π-seam lane (PR 3746,
`crates/sweep/tests/pi_seam_and_kiss_through_the_boolean.rs`,
`a_cap_abutting_on_the_rim_refuses_whatever_the_corner_with_its_discs_declared_rest`).
A cylinder r=1, z∈[0,2], is unioned with a cap standing on its top
disc, with an equal radius at the rim. Both member orders were run,
undeclared and with only the two end discs declared `Rest`:

| cap (corner at the rim) | undeclared | discs `Rest` |
|---|---|---|
| hemisphere (G1, wedge π) | `CurvedPierceUnsupported` | same |
| spherical dome ρ=√2 (transverse, 45°) | `CurvedPierceUnsupported` (`Escalated` at 1e-6) | same |
| stacked cylinder, same carrier | `CurvedPierceUnsupported` | same |
| 45° cone frustum | `CurvedPairUnsupported` at the operand gate (a cone × plane gate, separate) | same |

Raised by `topo::boolean::reduce::curved_face_arm`'s `frontier()`.
The edge is always on the rim: the rim circle, a tube ruling that
ends there, or the cap's meridian seam, each against the other
operand's undeclared wall. The arm covers an edge only when the
edge's OWN face is declared against the face it touches, so the
disc×disc `Rest` declaration never covers a rim edge against the
partner's wall.

A domed or capped end on a cylinder is everyday CAD, which is why
this row is P0. No test on main builds an abutment between distinct
curved carriers through a two-operand boolean.

## The design question

What licenses the coincidence of a declared `Rest` patch's boundary
edge with the partner's curved wall, given D1's rule that coincidence
is "structural or declared, never inferred from values"? This is the
DEV-1 circle-arm pair's seam-licence fork
(`dev1-cylinder-sphere-circle-locus-arm`), widened by this measurement
from π seams to every rim abutment.

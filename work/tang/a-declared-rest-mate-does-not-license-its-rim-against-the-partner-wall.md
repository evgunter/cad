---
id: a-declared-rest-mate-does-not-license-its-rim-against-the-partner-wall
kind: issue
title: Two solids abutting on an equal-radius rim refuse whatever the corner: C4's cover sentence holds back the transverse rim, and two curved incidence events are unbuilt
status: open
opened: 2026-10-01
priority: P0
cost: H
design: true
needs_ev: true
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

## 2026-10-02 — what the designer pair found

The row's first framing, that "a declared mate never licenses its rim",
is the wrong question for two of the three corners:

- **Transverse corner (the dome).** No licence is needed. The rim
  circle lies identically on the partner's wall (a certified
  `Constant`, not a band root). That is an ON event under D1's
  reduction trilean, as `Coincide::EdgeOnPlane` already records a curved
  edge lying in a partner's plane. C4's one-sided-cover sentence
  ("an edge lying on, or touching …") covers it by its letter, and
  `curved_face_arm`'s undeclared `(Zero, Zero)`/`Constant` arm keeps the
  frontier for that reason. The `[ev]` PR on branch
  `tang/ev-rim-licence` narrows the sentence to touches. After that the
  dome needs two unbuilt pieces: the circle × cylinder "lies on" cell
  (in flight on `tang/circle-cylinder-crossing`), and a curved edge-edge
  coincidence event at the rim.
- **Same carrier (the stacked cylinder).** This needs `Continuation`
  declared on its walls, which exists. The measurement declared only
  the discs. Re-measure it with the walls declared as well.
- **π seam (the hemisphere).** The edges leaving the rim graze the
  partner tangentially (double roots), so C4's graze sentence requires a
  declaration or structure. That is
  `pi-seam-between-two-operands-has-no-declaration`.

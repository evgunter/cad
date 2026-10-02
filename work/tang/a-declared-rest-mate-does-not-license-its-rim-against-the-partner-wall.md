---
id: a-declared-rest-mate-does-not-license-its-rim-against-the-partner-wall
kind: issue
title: Two solids abutting on an equal-radius rim refuse whatever the corner: the transverse rim needs the circle-lies-on-cylinder cell and a curved edge-edge event at the rim (C4 narrowed, PR 3756)
status: open
opened: 2026-10-01
priority: P0
cost: H
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

## Ruled (Ev, PR 3756, 2026-10-02)

C4's one-sided-cover sentence now governs touches only. An edge lying
identically on the partner's carrier is an ON event when every surface
of a face it bounds is decided distinct from that carrier by the
carrier ladder. What remains is implementation: the circle × cylinder
"lies on" cell (on `tang/circle-cylinder-crossing`, PR 3752) and a
curved edge-edge coincidence event at the rim. The stacked
same-carrier cylinder still needs `Continuation` on its walls;
re-measure it declared so.

## Outcome (2026-10-02, PR 3823)

The transverse rim builds. A circle the root door decides lies on the
partner's carrier (`SpanVerdict::LiesOn`), with every parent decided
distinct by the carrier ladder, is an ON event. `reduce::lying_on`
records its ends once its interior is certified to cross the face's
boundary nowhere. There are two certificates: the boundary meets the
arc's circle only at the paired end vertices, or the arc runs along a
chain of the partner's own circle arcs. The declared-REST zip matches
the rim's semicircles as arcs (`rest::arcs_along`).

- The dome builds with its discs `Rest`: both orders, seams aligned or
  turned, on an extruded or a revolved tube, at tier 3 and the
  closed-form volume. A lens of two domes builds too.
- Undeclared, the dome refuses on its value-coincident discs
  (`UndeclaredCoincidence`), which is correct.
- A rim in band of the partner's wall escalates.
- The hemisphere still refuses at an edge leaving the rim (the graze,
  `pi-seam-between-two-operands-has-no-declaration`).
- The stacked cylinder still refuses, on its own rim, whose parent shares
  the partner's carrier. `BooleanCoincidence::Continuation` is not in the
  code, so it could not be measured declared so. Declared aligned `Rest`,
  it builds (evidence on REACH's `cosurface-disjoint-curved-walls-refuse`).
- The cone stops at the operand gate.
- Side effects: the torus dumbbell, its cylinder control and the torus
  peg-in-socket build, and their rows are re-pinned.
- A tube ending on a ball, or on a torus latitude, now passes the
  crossing layer and stops in the join. Filed as JOIN's
  `a-tube-ending-on-a-ball-refuses-section-loop-mixed`, with evidence on
  GERM's `c5-plane-torus-cone-cylinder-arms`.

---
id: a-nurbs-face-against-a-cone-or-sphere-has-no-section-arm
kind: issue
title: A NURBS face against a cone or sphere face has no C5 section arm, and no boolean reads a rung-3 section, so klein wall 3 and teapot wall 3 cannot join
status: dispatched
opened: 2026-10-25
priority: P0
cost: H
refs: [c5-plane-torus-cone-cylinder-arms, nurbs-edge-crossing-rung-is-the-ring-composite, joining-a-spline-carrier-is-unbuilt]
pr: 6304
branch: sectarms/nurbs-implicit-arm-build
---


Filed by the SPLICE orchestrator (2026-10-25, agreed in chat). SECT's
plan names this arm only in a sentence of its step 4 ("say so in the
step that builds it"); this row is what keeps it from being lost.

## What

Two demo joins pierce a curved wall with a NURBS wall:

- **klein wall 3** (`demos/tour/src/klein.rs`, `wall3_pinned`): the
  loop's lofted NURBS walls descend through the bulb's cone;
- **teapot wall 3** (`demos/tour/src/teapot.rs`, the "WALL 3" block):
  the spout's walls pass through the belly's sphere.

C5's table (`crates/geom-brep/README.md`, the rung list) has a rung-3
arm for plane×NURBS and NURBS×NURBS, and none for NURBS×cone or
NURBS×sphere ("every other pair refuses"). It also says no boolean
reads a rung-3 arm yet. So each join needs both: the C5 section of a
NURBS face against a cone and against a sphere, and the section
object (`cylinder-sphere-germ-pair-has-no-join-lane`, step 1) reading
a rung-3 section at the join.

Today both joins refuse earlier, at the boolean's operand edge gate
(`topo::boolean::reduce::gate_operand_edges`). SPLICE deletes that gate
(`work/splice/delete-the-boolean-operand-edge-gate.md`), and its PR
re-pins both walls on the refusal they reach next.

## Why it is SECT's

SECT owns the curved section rows outright, and no other track builds a
per-pair stopgap (`work/sect/plan.md`, "Ownership"). SPLICE builds the
operand edge side: the crossing rung for a NURBS edge against a face and
the join along a spline carrier. This row is the face × face section.

## Open

How the section is certified is undecided. It could be a marched fitted
cubic, as the rung-3 carrier text allows, or a per-span bound like the
cone and torus metres conversion (C2 limb 2). This is a design read for
SECT's step 3 or 4. Parked on SECT's step 1 (the section object) and on
SPLICE's gate deletion (until then no NURBS operand reaches the join).

## Correction (2026-10-26, SPLICE orchestrator)

PR 5723 measured where klein wall 3 actually meets. The neck cylinder's
seam line runs onto the loop wall's rim, and the wall leaves that rim
tangent to the neck cylinder. So klein's join pair is **cylinder ×
NURBS, tangent along the rim**: the "tangent section decided at the
door" clause of SECT's design, not a transverse cone × NURBS arm.
Teapot wall 3 is sphere × NURBS, as filed. C5's new curve × face clause
(PR 5727) is the substrate a section's boundary pass will read with.

## Unparked (2026-10-28, SECT step 1b)

Both gates it was parked on have gone: the boolean reads a rung-3
section at the join (`cylinder-sphere-germ-pair-has-no-join-lane`,
closed: the cylinder × sphere chord lane cuts its chords from the
section's fitted loop), and SPLICE's operand edge gate is deleted
(`splice/delete-the-boolean-operand-edge-gate`, closed). What remains is
this row's own open design read, the certification of a NURBS face's
section against a sphere or a cone.

## Decided (2026-10-31, Ev on `[ev]` PR 6121; fork rows 225 and 226)

The section of a NURBS face against any surface with an implicit residual
is one arm, `φ = f∘S` on the face's knot rectangle. The rectangle is the
section value's domain for this pair (C3's domain rule). Components may
end on its sides.

Coincidence and tangency are decided once, before the march, and every
reader reads that verdict. The decision runs in three stages:
- the hull of `φ`;
- the boundary pass;
- one regularity sweep, which is W5 at a plane.

An isolated critical point of `φ` at the zero level with a nondegenerate
Hessian is decided Zero: a touch where the Hessian is definite, a node
where it is indefinite. A side collapsed to a point not clear of the
partner refuses before the sweep.

Plane × NURBS folds into this arm. Its build follows PR 6026 and SPLICE's
W5 row. The curved partners (sphere, cylinder, torus) come first and the
cone after SECTRUNG's cone unit. The build is staged as units on this row.

The row's open question, "fitted cubic or per-span bound", was a false
choice: the arm needs both.

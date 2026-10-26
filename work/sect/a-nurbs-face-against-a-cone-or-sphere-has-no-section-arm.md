---
id: a-nurbs-face-against-a-cone-or-sphere-has-no-section-arm
kind: issue
title: A NURBS face against a cone or sphere face has no C5 section arm, and no boolean reads a rung-3 section, so klein wall 3 and teapot wall 3 cannot join
status: parked
opened: 2026-10-25
priority: P0
cost: H
design: true
blocked_on: [cylinder-sphere-germ-pair-has-no-join-lane]
refs: [c5-plane-torus-cone-cylinder-arms, nurbs-edge-crossing-rung-is-the-ring-composite, joining-a-spline-carrier-is-unbuilt]
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

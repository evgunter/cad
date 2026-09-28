---
id: undeclared-chord-between-two-pierces-refuses-on-the-sibling-face
kind: issue
title: The undeclared (Zero, Zero) chord between two torus pierces refuses on the sibling face, though NoInterior and two certified Elsewhere ends leave it no incidence there
status: open
opened: 2026-09-25
refs: [torus-operand-gate-admission]
priority: P1
cost: D
---

## What

A line edge that crosses a torus twice is split at both roots by the
line×torus lane (`reduce.rs`'s `wall_crossing`, torus arm). The middle
fragment, the chord between the two pierces, has both ends ON the
carrier. Against the face that holds them it records. Against the
torus's OTHER face it reaches the undeclared `(Sign::Zero, Sign::Zero)`
arm of `curved_face_arm` (`crates/topo/src/boolean/reduce.rs`). There
`wall_crossing` answers `NoInterior` and both ends come back
`Placement::Elsewhere` (the chart trim certifies both are outside that
face). The arm's strict rule records only on a `Recorded` end, so it
refuses `CurvedPierceUnsupported`.

**Measured** (`crates/sweep/tests/germ_torus_doors.rs`,
`a_chord_across_the_hole_is_pierced_not_passed`): a donut (`R = 2`,
`r = 0.5` about `y`) against a bar along `z` through the hole with both
ends inside the tube refuses `CurvedPierceUnsupported { operand: B,
face: <the donut's other face>, edge: <the chord fragment> }`, with both
chord ends' `curved_face_containment` = `Out` on that face. The belly
pose (both ends outside the tube, two pierces in the inner half) stops
the same way.

## Why it is torus-reached, though the rule is kind-generic

The rule is written for every curved kind. On a cylinder or sphere
split by a plane through its axis, the chord's box never meets the
other half's box (both pierces lie in one half-space and the chord is
straight), so the sweep never tests the pair. A cylinder-handle control
was measured and never reached the arm. A torus's two revolve faces are
its inner and outer halves, and neither half's box excludes the other
half's chords, so the torus reaches the arm on ordinary transverse
poses.

## The question

`NoInterior` already certifies that no root lies strictly inside the
span within this face's trim. With both ends certified `Elsewhere`, the
chord meets this face nowhere, and `None` looks sound. The strict rule's
comment says the rule was chosen because "the chord's interior is itself
the unanswered question". That is what `NoInterior` answers. The
mixed-sign arm's `Elsewhere` no-event was settled separately. Whether
the `(Zero, Zero)` arm may take the same step is a reduction-rule
change on an undeclared arm. It is not a torus arm, so the GERM
torus-doors unit did not make it.

## Home

GERM: `reduce.rs`'s crossing layer, the lane the torus doors run
through.

---
id: held-reach-keeps-an-unread-pole-with-no-witness
kind: issue
title: census::held_reach_in keeps the carrier's end at a pole it cannot place, and no row holds that branch: no verb is known to build a trimmed sphere face whose region does not read
status: open
priority: P3
cost: M
refs: [census-cross-solid-curved-pairs-undecidable-on-shell-results, check-10-is-silent-where-point-in-solid-refuses]
opened: 2026-10-27
---

Found in review of `restread/curved-pairs-apart` (PR 5817).

## What

`census::held_reach_in` narrows a sphere face's reach along a turned
direction to its boundary's only where `boolean::curved_face_placement`
places the pole at that end definitely `Out`. Every other reading keeps
the carrier's end. Two of them say nothing about whether the face holds
the pole:

- `Trim(None)`: `sphere_region::sphere_face_region` cannot read a
  region with a boundary edge that is not a circle arc (a spline, or the
  quartic a cylinder, cone or torus off the sphere's centre leaves);
- an error: the pole in an arc's band, or a region no ray settles.

A face like that may hold the pole, so narrowing there could clear a
pair that meets. Keeping the carrier's end is the sound choice, but
nothing pins it. A mutant that narrows on every reading but `In`
survives the whole suite, both reviewers' probes included.

## Why there is no row

A pinning row needs a trimmed sphere face whose region does not read
while it holds the pole, beside a partner near that pole. No such face
is known to build:
- the trimmed sphere faces the booleans and revolves build are bounded
  by circle arcs, so their regions read;
- a round bore through a ball, which would leave a different edge, is
  refused by the boolean (`CurvedBooleanUnsupported`, measured by the
  PR 5817 reviewer);
- the only sphere face with spline edges in the tree is in a wild STEP
  file (`crates/step-import/tests/fixtures/wild/stepcode/TAIL_TURBINE.stp`).

## Do

When a verb or an import fixture builds such a face, land the row. Put
the face's pole along a turned direction, with a box face at
`reach + dx` beside it. It refuses at `dx ≤ 0`, and is red under the
mutant above. `cross_solid_apart::a_trimmed_sphere_face_holding_its_pole_clears_only_past_it`
is the shape to copy. A hand-written STEP sphere face bounded by a
B-spline curve may be the cheapest witness.

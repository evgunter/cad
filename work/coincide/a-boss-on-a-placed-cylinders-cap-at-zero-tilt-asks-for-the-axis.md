---
id: a-boss-on-a-placed-cylinders-cap-at-zero-tilt-asks-for-the-axis
kind: issue
title: A boss on a placed cylinder's cap proves its heading at a tilt of 0.2 but asks for the axis at a tilt of 0
status: closed
closed: 2026-11-23
opened: 2026-11-22
priority: P2
cost: M
---


Found by `doctail/the-tangency-prover-does-not-see-through-a-placement`'s fix, while it wrote the twin for `intent_s6_e_traced_on_a_face::a_hole_parallel_to_the_face_by_value_only_asks_for_the_axis`.

## What

The scene is `intent_s6_e_traced_on_a_face::turned_cylinder`: a cylinder placed by values into a box's space and spun about `x` by the free `a`, with a boss drawn on its own top cap tracing its wall (`boss(…, CapEnd::End, …)`).
- At `a = 0.2`, `tangency` proves the boss `by_its_heading` (`a_tilted_cylinders_boss_is_tangent_for_every_tilt`).
- At `a = 0`, the same construction answers `Unproven { replay: Numeric, recourse: ReadTheAxis }`.

The boss is tangent for every `a`, as at `0.2`. So the answer depends on the nominal value, and at `0` it is a miss, not a finding.

This row does not yet know why. Two places read the nominal's exactness, and either would leave the chart constant in the symbols where the cap's normal is a function of them:
- the chart's identity least turn (`mate::value::Chart::charted`, `from.is_exactly(d)`);
- the aligned axis (`mate::turn::aligned`, `Chart::aligned`).

At `a = 0` the cap's normal is the space's `z` in value. Check which branch the replay takes, by rendering `swept_along`'s undischarged form (`geom_core::sym::deferred::render`). If it is a value-only exactness read, it is a decision the replay takes as the build does but does not record. The theorem then holds only where that branch is taken, which is a set of measure zero.

**Repro**: the twin in `a_hole_parallel_to_the_face_by_value_only_asks_for_the_axis`, with `placed(0.0)` in place of `placed(0.2)`.

## Closed

Closed by `doctail/the-least-turn-and-aligned-read-exactness-off-the-nominal-at-sym` (PR 7434), together with this row's own branch. The branch the zero-tilt chart took was the least turn's value-exact identity (`RigidMotion::least_turn_minted`'s `a.is_exactly(b)`). The least turn reads no exactness now, so `S` carries the tilt at `a = 0`.

With a chart's spin set from its frames at `Sym`, the boss on the placed cylinder's own cap proves `by_its_heading` at `a = 0` as at `a = 0.2`. Both are asserted: `intent_s6_e_traced_on_a_face::a_tilted_cylinders_boss_is_tangent_for_every_tilt`, and the twin in `a_hole_parallel_to_the_face_by_value_only_asks_for_the_axis`, on its own zero-tilt document.

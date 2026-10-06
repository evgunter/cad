---
id: a-boolean-ships-a-face-whose-loop-passes-two-vertices-on-one-point
kind: issue
title: A boolean ships a SOUND body whose one face passes two distinct vertices on one point (cube minus prism with a notch corner on a cube edge)
status: parked
opened: 2026-10-05
priority: P0
cost: M
refs: [boolean-declares-no-touching-between-copies-of-one-operand-vertex, a-pinch-no-kept-face-can-cross-refuses]
blocked_on: [a-pinch-no-kept-face-can-cross-refuses]
rides_with: a-pinch-no-kept-face-can-cross-refuses
parent: a-pinch-no-kept-face-can-cross-refuses
---


## What

Found by PR 4038's review r2 (N1), `crates/sweep/examples/r2_pinch_probes.rs`
on `join/pierce-pinch-families-review-r2`. Its `FACE2V` check flags a
built body where one face's loops run through two distinct vertices at
exactly one point. Main ships such bodies as `SOUND` (tiers 2 and 3′,
the certificate, the oracle volume): `outcome` and the census cannot
see it. The same lines are identical on main `8793177b` and on PR 4038's
head, so the class is main's.

Example: `notch307 fib117 edge psi=0.3 pc S` (the 307° notch's top
corner `(2, 1, 1)` on a cube edge, prism ∖ cube): vertices `33v1` and
`34v1` both at `(2, 1, 1)` on one face's outer loop.

Poses (r2's battery tags):
- cube: `notch307 fib113 edge psi=4 cp S`, `notch307 fib105 edge psi=4 cp S`,
  `notch307 fib117 edge psi=0.3 pc S`, `vee300 fib100 edge psi=4 cp S`,
  `vee224bot fib1 edge psi=0.3 cp S`;
- near-tangent: `notch307 nt e1 a5 d±1e-3` and `d±1e-6` `edge psi=4 cp S`,
  `notch307 nt e1 a6 d-1e-3` and `d-1e-6` `edge psi=0.3 pc S`;
- cylinder: `Lbot cyl fib4 psi=0.9 seam cp S`, `notchbot cyl fib15 psi=0.9 seam cp S`.

That breaks the shared-point ruling (PR 3813,
`boolean-declares-no-touching-between-copies-of-one-operand-vertex`):
copies on one point stay apart only where no face meets both, and a
face meeting both is where a weld (`finish::weld_pinches`,
`finish::weld_pierce_copies`) joins them. Every pose is a corner on a
cube edge (the vertex-edge lane), so whichever producer leaves the two
copies is not one of the pierce welds' groups.

## The shape to give

Take `notch307 fib117 edge psi=0.3 pc S`, name which records minted the
two vertices, and decide whether a weld should have joined them (a
face meets both) or the face should have been divided. Add a check that
can go red on a face meeting two vertices at one point to a battery
that reaches these poses.

## Ruling (Ev, PR 4057, 2026-10-05)

A pinch is one vertex per cone: several vertices on one point key, and
no face crosses between cones. See
`work/join/a-pinch-no-kept-face-can-cross-refuses.md`, "The shape to
give". This row is settled by that unit.
Under the ruling, a face meeting two vertices on one point is a right body when each vertex is a manifold cone. The check this row asked for becomes a tier-3 check that every corner is a slice of its own face.

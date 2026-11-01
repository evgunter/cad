---
id: a-ball-cut-by-a-plane-turned-epsilon-escalates-arc-ahead
kind: issue
title: A ball cut by a plane turned a few ε about an off-centre hinge escalates bool_join_arc_ahead, the unturned cut building
status: open
opened: 2026-10-14
priority: P2
cost: M
---

Seen while probing for a door to the sphere-section witness on
`cleave/carrier-cross-one-curve` (PR 5058's fix pass); cause
unmeasured.

`topo::boolean::subtract(ball, slab)` at ε = 1e-9:

- `ball`: carved_sphere_operand.rs's `ball(1.0, 0)` (revolved about
  `y`, seam meridians in `z = 0`);
- `slab`: `brick((−50, 50), (−100, 0), (−50, 50))` rotated so that `ŷ`
  goes to `n0`, then turned `θ` about the line through `0.9·û` along
  `n0 × û`, `û = normalize(n0 × ẑ)`.

At θ = 0 the cut builds. With `n0 = (1, 1, 1)/√3` and θ = 0.9e-9 it
refuses `Escalated { Coincidence(Join, Moot), bool_join_arc_ahead,
margin −1.15e-9 }`. With `n0 = normalize(0.3, 1, 0.5)`, the ball first
spun 1 rad about `n0`, and θ = 3e-9, it refuses the same way with margin
−2.59e-9. The margin grows with θ, so a decision that reads exactly
zero on the unturned cut lands in band when the plane is turned by a
few ε. A single plane cutting a single ball has no near-coincidence
that the turn creates, so first find what that decision compares.

## Slate triage (2026-10-26, CLEAVE)

- Banded P2: a decision reading zero on the unturned cut lands in band a few ε off it: interval and error propagation, P2 by `work/README.md` Priority.
- Priced `M`, the effort the body describes.
- Moved to HONEPOSE (`work/honepose/`) by `git mv`, id and body unchanged: the P2 track opened for in-band refusals at measured poses: a reading that lands in band where the pose is not near a decision. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).

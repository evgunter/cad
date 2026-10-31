---
id: a-revolve-traced-tangent-has-no-axis-read-off-the-face
kind: issue
title: The trace statement's revolve arm has nothing to read: a revolve's axis is typed in the profile plane, and a sphere's centre needs a kind the door does not hold
status: open
opened: 2026-10-30
priority: P3
cost: M
blocked_on: [coaxial-unions-with-a-torus-or-a-sphere-and-a-tangent-cone-refuse]
---

Found while building INTENT stage 6 D (`work/intent/the-door-proves-a-constructed-tangency.md`). D builds the curve tier of `[coincidence.structural]`'s trace statement for an extrude. Ev's motivating scenes are a cone revolved tangent to a sphere or to either branch of a torus, and their revolve arm cannot be built as ratified. Two reads are missing, and the GERM row this is blocked on keeps either scene from building today.

## What rung one reads today (measured by reading the door, on `main` at #6060)

- **The revolve's axis.** `Node::Revolve` writes its axis in the profile's plane: `axis_through` (two lengths) and `axis_heading` (an angle). Its lift is the node's `axis` output. The door keys that output as itself (`coincide/stated.rs` `pose_at`, its `VarDef::Output` arm, via `axis_output`) and relates it to nothing else. Only a mate could put it in one class with F's axis, and a mate relates two spaces, while the traced profile and F share one. So C's cone on a sphere can never satisfy the pose row today, whatever the kernel builds. Its revolve is `fixture::revolve_about(section, (0, 0), along_y(), turn)` on a frame `frame_on` built `Through(edge axis, vertex)` and then spun by values.
- **F's symmetry.** Stage 6 D reads it off F's construction (`coincide.rs` `symmetry`): an extrude's wall is Cylindrical, and a revolve's band is Revolute about its `axis` output (`stated::axis_of`). A sphere is a revolve's band, so it reads as Revolute. Nothing establishes Spherical.

## What would make the revolve arm readable

1. **Constant slots read as a frame role.** A revolve whose slots are exact constants is a role of its profile's frame. With `axis_through = [0, 0]` (`Formula::zero`) and `axis_heading` `0` or `turn/4` (`fixture::along_x`, `along_y`), the `axis` output is the frame's `x` or `y` axis, read through that role. That is `Key { Axis, Pose(frame, Some(X | Y)) }`, a projection of one construction, as `Stated::sense`'s role arm already reads two frames' roles. Any other written value, or any variable in a slot, keeps it apart: "coincides at today's values". This needs:
   - a read of the slots as exact constants;
   - one rung-one equality, the `axis` output equal to that role of the frame.
2. **Constant pose moves folded through the frame.** `Through(F.axis, p)` puts F's axis on the frame's `z`, but a revolve's axis lies in the frame's `xy`. So the frame must be turned so that its `y` (or `x`) is F's axis. A spin of `turn/4` about the frame's own `x` maps `z` to `-y`, and about `y` maps `z` to `x`. That is a constant permutation of roles, known exactly. `stated::frame_z` today follows only `z` through slides, flips and spins about `z`. Generalised, it would follow each role through spins by exact quarter turns about a frame role, and refuse any other angle. Alternatively, a frame vocabulary whose `y` is read off an axis directly (a `Through` variant naming which role the axis takes) would need no fold. That alternative is a design choice for Ev.
3. **F Spherical by construction.** A revolve's band is a sphere when the piece it sweeps is an arc whose centre lies on the revolve's axis. C's `sphere()` gets that from constants: a `Bulge { b: 1 }` semicircle between `(0, -R)` and `(0, R)` (centre at the chord's midpoint, `x = 0`), revolved about the profile's `y` through `[0, 0]`. Reading it needs the arc's centre established on the axis by the same constant-slot read as (1): an arc `Center` written at `[0, 0]`, a full `circle` carrier form centred there, or a bulge of exactly `1` with both ends on the axis. Then the band's family is Spherical about that centre, and "the revolve's axis passes through F's centre" is the revolve's axis meeting that point by the same read.
4. **The torus** (Ev's two branches). The torus's band is Revolute about its own revolve's `axis`. The cone's revolve axis must be one class with it, which is (1) and (2) again. No Spherical is needed.

## Done when

A cone drawn on a sphere's trace and revolved about the frame's axis read as above, and one drawn on a torus's trace likewise, each report their `SurfaceTangent` row `Structural`. The same cone with its axis slot typed `turn/4 + 0` through a variable, or with the frame spun by a non-quarter angle, stays unproven with `Recourse::ReadTheAxis`. This needs the GERM arms. (1)–(3) go to Ev as an `[ev]` PR first, because they make a typed constant count as construction.

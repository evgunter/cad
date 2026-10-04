---
id: a-pierce-with-two-out-runs-at-one-vertex-refuses-every-op
kind: issue
title: A vertex-on-face pierce with two Out runs at one vertex refuses every op; its ring struts are faced by a hard-coded rule
status: open
opened: 2026-10-03
priority: P0
cost: H
---


## What

Found by the sweep of `whole-orbit-fan-end-has-three-spellings`.

**The pose.** Take `join_pierce_strut_facing.rs`'s L-prism and its
`cube_beyond(m)`: the cube's near face lies on the plane through the
reflex top corner `(1, 1, 1)` with normal `m`. Tilt `m` so that the
corner's −z edge also reads Out, e.g. `m = (1, 1.3, 0.7)`,
`(1, 1.3, 0.4)` or `(1.2, 1, 0.3)`. Then `classify_vertex_on_face`
(`boolean/vtxfac.rs`) sees two Out runs at one vertex: the reflex top
sector's bisector alone (a bare strut) and the −z edge alone (a fan).
No declarations are involved.

**What it does.** The undeclared poses above give 18 runs: union,
intersect and subtract, in both operand orders, at three tilts. Every
run refuses. 12 refuse `JoinDesync { "a section vertex's null-edge
copies have not exactly one kept end" }` and 6 refuse
`Euler(SelfLoopEdge)`. Each refusal is loud and typed, and no wrong
body ships. With main's old bare-strut facing, the same 18 runs refused
`JoinDesync` with the words "B senses agree at a matched pair" or
"every chord arc separates a loose scaffolding pair". So the run never
built, under either facing.

**A likely seat.** The pierced side hangs one ring strut per run off
the ring vertex (`classify_vertex_on_face`, step 3, `ring_anchor`). Each
ring strut is faced by a hard-coded rule: `he_minus` faces the run's
start germ. The second strut splices at the first strut's `he_plus`,
whatever the germ directions. This is the third strut-facing spelling.
It is not `insert::strut_faces_first`: no corner edge or entry orders
the struts, because the ring vertex sits in the face's interior. With
one run the facing is free. With two runs, the order of the struts
around the ring vertex has to match the order of the piercing runs.
This is unmeasured, so diagnose it before fixing.

## The shape to give

Add a row that pins the two-run pose in every op and both orders, with
the exact volume (the cube holds the prism's far side whole, as in
`join_pierce_strut_facing.rs`). Then find the first wrong state.

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

## Built

Branch `join/pierce-two-out-runs`.

**The first wrong state was not the ring-strut facing, for the row's
tilts.** With main's facing, the union and the subtract bodies were
already right (tiers 2 and 3′, the certificate and the exact volume
all passed once the refusal was bypassed). Three things broke after
the body was built:

- `finish::discarded`'s `kept_end` read one kept copy per section
  vertex. A vertex that two runs cut keeps two copies on one twin
  section face, so it refused.
- The union kept both runs' copies apart: two vertices at `v`, against
  `boolean_pinch_copies.rs`'s one vertex per point.
- The intersection's zip met the pinch vertex twice on both seams, and
  the second fusion was a self-loop (`SelfLoopEdge`).

**The facing hypothesis was right in part.** Main's hard-coded facing
fails ∪ and ∖ in 171 of 340 two-run poses per order over six corners
(PR 4026's review r1, m2: 70 `SectionLoopMixed`, 101 `JoinDesync`).
On the L corner those are exactly the poses whose +x and +y edges both
read Out, including runs widened by a side face's bisector. The lone-
edge family of the pose sweep (`join_pierce_runs_sweep.rs`) is one of
them. There the walk's facing builds the union and the intersection.

What changed:

- **Ring struts.** With several runs, each ring strut faces its germs
  by the walk about the pierced face's outward normal
  (`insert::strut_order`). The copy's side follows by the sense
  theorem. Where the op keeps both In sides, the copies take In, so
  that the ring's In face passes a copy per run.
- **Discard rows.** Where the twin passes several copies, a stretch's
  ends are the two copies one edge of the twin joins.
- **Seam correspondence.** The guard accepts one pierce's runs.
- **Pierce weld.** `finish::weld_pierce_copies` fuses the copies a
  pierce's runs leave apart after the zips, wherever a face runs
  through two of them. It welds across a ring as a new `Joint::Hole`:
  `mef` + `kev` + `kfmrh`, two holes meeting at the vertex.
  `pinch_site` now returns a hole for a shared ring, rather than
  dividing a face out of it. Copies that no face meets stay apart on
  their one point.

The row: `crates/sweep/tests/join_pierce_strut_facing.rs` (moved from
`topo`, to share `differential::outcome`). The residue is filed:

- `a-pierce-whose-wide-run-pinches-its-intersection-refuses`;
- `a-pierce-whose-difference-pinches-at-two-edge-runs-refuses`;
- `a-reflex-corner-on-a-cube-edge-or-corner-refuses-in-the-vertex-vertex-lane`;
- `ring-struts-of-three-or-more-runs-hang-in-run-order`.

---
id: torus-face-meeting-a-partner-only-in-an-interior-loop-while-crossings-exist-elsewhere
kind: issue
title: A curved face that meets a partner face only in an interior loop, while crossings exist elsewhere, is classified by face-region propagation that cannot see the loop
status: closed
opened: 2026-09-26
refs: [torus-operand-gate-admission]
priority: P0
cost: H
closed: 2026-09-28
---

## What

A face `F` of one operand can meet a face `G` of the other in a closed
loop interior to both faces, touching no edge of either body. When the
operation has crossings elsewhere, the no-crossings fallback's gates
never run, the join cuts nothing along the loop, and face-region
propagation carries each face's side across it: a valid body that is
wrong. Measured instances:

- a half donut and a bracket whose foot cuts an oval off the outer
  equator while its pin crosses the cap (`germ_interior_oval.rs`);
- a dome and a bracket cutting a side cap clear of the dome's seams;
- two cylinder walls meeting in a saddle loop while a pin crosses a cap
  (`cylinder-wall-pair-meeting-in-an-interior-loop-while-crossings-exist-elsewhere`,
  `germ_interior_saddle.rs`).

## Closed by the section certificate (PR 3372)

`crates/topo/src/boolean/section_cert.rs` classifies each undeclared,
box-overlapping face pair's section — torus × plane and torus × sphere
in any pose; torus × coaxial wall or torus, and × a parallel-axis wall;
cylinder × cylinder and × plane; sphere × plane, sphere and cylinder —
and proves each component either absent from `F ∩ G` or not inside both
faces' interiors: unbounded (W1), essential on a face whose chart
describes (W2), a closed-form witness point `Out` of a face (W3), or
the lone component of a pair the reduction recorded an event on (W4).
With no event on the pair, one witness decides; strictly inside both
faces is a certified interior loop and refuses (R-loop). Tangencies
(R-tan), poses and kinds with no arm (R-reach) and undecided witnesses
(R-undec) refuse. It runs on both paths: in place of the stopgap's
`interior_loop_verdict` halves on the crossings path, and in place of
`torus_extent_gate` and `cylinder_extent_gate` on the no-crossings path.
Its premise, sweep completeness, holds with the conic × plane lane
examining every root (PR 3358).

Every measured instance above now refuses as R-loop; the pin-only
bracket, the cube in the donut's hole, and the cylinder pairs the old
gates refused on reach answer their closed forms.

## Still open, each filed

- `interior-loop-cut-in`: R-loop refuses where cutting the loop in
  would answer.
- `nurbs-face-meeting-a-plane-in-an-interior-loop-is-unguarded-on-the-crossings-path`:
  the class's one kind pair outside the certificate's scope.
- `cone-pairs-in-general-pose-have-no-section-arm` and the cone arms on
  `VERBS-CONE`: cones refuse at the operand gate until admitted.
- `radial-hole-through-a-tube-has-no-section-arm`: refuses on reach.
- `coplanar-conic-edge-skips-endpoint-treatment-in-the-sweep`: premise S
  holds for a coplanar conic only through its neighbours.
- `torus-onto-the-subtract-and-intersect-roster`: the ∖/∩ torus roster
  admission, which was parked on this item.

## Measured: the rest of the class (GERM measurement lane, 2026-09-28)

- **Cylinder × cylinder is live on main: P0, filed** as
  `cylinder-wall-pair-meeting-in-an-interior-loop-while-crossings-exist-elsewhere`.
  - A partial 240° cylinder face meets `cyl(1, 2)`'s wall in a saddle
    loop, and a planar pin pierces the top cap.
  - All four ops return valid `Seamed` bodies. ∩ is the pin alone,
    `0.008` against `0.0900944`. Tilted `0.15` rad, the same.
  - Without the pin it refuses at `cylinder_extent_gate`.
- **Sphere × cylinder: nothing escapes the passing clause, because
  nothing reaches it.** Three fixtures were built:
  - the dome × an `r = 0.3` rod along `x` in the base plane (two loops,
    each cut by the dome's rim);
  - a bored plate (`r = 0.5` bore) × an `r = 0.8` ball, two bore circles;
  - the dome × a tilted rod.

  All refuse at `CurvedPierceUnsupported` for every op, before the
  guard: a cylinder line into a sphere face, or a circle against a
  wall, has no pierce door. So the "event + cylinder partner passes"
  clause has no reachable instance in these fixtures. N6's survival is
  consistent with that.
- **Cone (a preview, with `Cone` on both rosters in a scratch patch).**
  The class is live there too; see `VERBS-CONE`.

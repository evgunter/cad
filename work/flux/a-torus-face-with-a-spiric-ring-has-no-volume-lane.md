---
id: a-torus-face-with-a-spiric-ring-has-no-volume-lane
kind: issue
title: mass_properties refuses RingOnCurvedFace on a torus face holding a ring that spiric ovals bound, which the plane x torus boolean now mints
status: open
opened: 2026-10-12
---


Found by GERM's U-T4 (`germ/plane-torus-chord-spiric`, the plane ×
torus chord for the spiric ovals), 2026-10-12.

## Measured

`crates/sweep/tests/germ_plane_torus_spiric.rs` pins the ops that stop
here (letter `v` in its tables). Each builds past the join, and the op's
final validation refuses
`ResultInvalid { errors: [VolumeUncomputable { source: RingOnCurvedFace { face } }] }`
on a torus face of the result that holds a ring bounded by spiric
ovals (and, on B1, B2 and the tie bar, concentric-circle arcs):

- the donut `(2, 0.5)` about `y` against B1, the bar
  `[−0.1, 0.1]² × [−2, 2]` across the hole (∪ both orders, T ∖ P);
  B2, `[−0.05, 0.05] × [0.25, 0.35] × [1, 3]` through the tube (the
  same three); and the bar `[x − w, x + w] × [−w, w] × [−2, 2]`,
  `x = 0.0053`, `w = 0.277`;
- the one-face torus (vertex at 0 and 1) and the torus seamed on
  `y = 0`, the same bars where the ring does not cross the torus's own
  edges, and the box `[−1, 1] × [0.2, 1] × [1.2, 2.8]` (P4) on the
  one-face torus at vertex 0 and π and the seamed torus;
- the corner bar on the fat torus `(1, 0.6)`, half-widths `0.1` and
  `0.3` (B5, B5′), on the fat torus and its one-face spellings at
  vertex 0 and 1.

The raising site is `crates/topo/src/props.rs` `face_flux`'s ring gate:
`Surface::Torus { .. } => !trimmed`, so a torus face with more than one
loop and any trimmed carrier (an ellipse, a spiric) refuses before the
quadrature lane is asked. The spiric quadrature
(`geom_brep::props::spiric`, the torus chart's Green form over each
loop) reads each loop's lifted boundary, so a face whose every loop
lifts closed is in its reach; whether it reads a ring is unmeasured.

## Done when

Those ops return their bodies, held to the truth by the rows that pin
them now (volume against the box oracle, tier 3), or refuse with a
narrower cause.

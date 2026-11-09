---
id: the-face-placement-dispatch-has-four-more-copies
kind: issue
title: four more sites spell 'where does this face hold a point on its carrier' (plane -> contfp, curved -> curved_face_containment) beside face_placement: census placed, fate's cut end and ring vertex, cell_region place
status: open
opened: 2026-11-09
priority: P3
cost: M
---


## Finding

Filed by `onecopy/face-holds`, from the sweep behind
`a-face-holds-a-point-has-no-public-door`. That row's two copies now read
`boolean::contain::face_placement`, the one dispatch. A sweep for the same
shape (a `Surface::Plane` or planar-normal arm calling `contfp`, and an
arm calling `curved_face_containment`) found four more. Each reads a
point it already holds on the face's carrier, so it skips the on-plane
decision that `face_placement` makes:

- `census.rs` `placed` (~1929): the planar arm reads the census's cached
  plane (`planar_face(geo)` → `contain`); the curved arm goes through
  `read_containment`.
- `boolean/fate.rs`, a fate cut's end in a divided face's pieces (~1146):
  the arm is chosen by `face_outward_normal` (sense-folded), and refusals
  map to `contained(.., "a fate cut's end is not placed in its face")`.
- `boolean/fate.rs`, the ring-vertex ladder (~1366): the same dispatch.
  Refusals go by their standing (`ContainError::refusal`, `rung_or_abort`),
  and `StaleFace` goes to `driver_face_stale`.
- `cell_region.rs` `place` (~186): every refusal is dropped with `.ok()`.

Each of the four keeps its own error policy, and that may be right: the
census reports findings, the fate ladder ranks refusals, and a cell
station refusing is no reading. What is copied is the dispatch. The fix
is a carrier-trusting arm of `face_placement` (or the four reading it with
an on-plane predicate, as `rim_wedge` now does), each caller keeping a
one-line decision on its errors. Check first that a fate site's
sense-folded normal and the plane's stored normal give `contfp` the same
reading.

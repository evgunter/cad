---
id: the-face-door-reads-no-spline-arc-of-a-sphere-face
kind: issue
title: The face door and the side reader read no spline arc of a sphere face: their scalar bound holds no spline reader, where point_in_solid's does
status: open
opened: 2026-11-25
priority: P0
cost: H
refs: [point-in-solid-refuses-a-sphere-face-bounded-by-a-spline-chord, point-in-face-and-point-in-solid-read-a-curved-trim-by-different-chains, cross-answers-a-spline-arm-with-a-whole-span-window, point-containments-ray-test-has-no-spline-arm]
---


Filed by INSIDE's `inside/sphere-chord-region` (the fix of
`point-in-solid-refuses-a-sphere-face-bounded-by-a-spline-chord`).

## What

A trimmed sphere face's region (`crates/topo/src/sphere_region.rs`,
`sphere_face_region`) reads a spline arc only where its builder hands it
the scalar's spline reader, `AtRestPolicy::spline_face_crossings`
(`crates/topo/src/props.rs`): the ring composite that crosses a spline
edge with a leg's plane (`region::spline_landings`). Built without one,
it admits circle arcs alone and answers `None` for a face with a spline
arc, as every builder did before.

Two readers build it without one, because their scalar bound is
`T: Decide`, which names no spline reader:

- the face door, `contain::sphere_face_containment` (reached through
  `curved_face_containment`, `face_placement` and tier 3's
  `cycle_side`): a chorded sphere face answers `Trim(None)`, so the
  census's edge-in-face lanes and tier 3's one-region arm read no side
  of it, where `point_in_solid` now does;
- the side reader's aimed rays, `side_reader::cast_aims` (through
  `solid_contain::face_geo`, handed `None`): a selection holding a
  chorded sphere face casts no aimed ray and reads its side off its
  volume.

`point_in_solid` (`solid_contain::point_in_faces`, bound
`AtRestPolicy`) and the sphere cut-in (`boolean::ops::apply_cut_ins`)
hand the reader over.

## Why it matters

D3's one reading per question: the face door and the solid door now
read one chorded face differently (the refs' row is the class). Adding
`AtRestPolicy` to the face door's chain was tried on the branch, the
compiler's errors followed outward: past 140 functions across `census`,
`cover`, `separating` and the boolean, each generic at `T: Decide`
alone, it had not closed, and was backed out.

## Owed

One spline reader every region door can reach at its own bound: a
scalar-level door for a spline span against a plane (the ring
composite, `None` at a dual), so the region needs no capability handed
in; or the face door's chain bound at `AtRestPolicy` with its callers.
Then drop `sphere_face_region`'s `splines` argument.

## 2026-11-25 — re-banded P0; the design is decided (INSIDE orchestrator)

**P0.** PR 7755's review measured the consequence. Every cylinder × sphere
result ∪/∖/∩ a brick touching only the chorded cap refuses
`CurvedPierceUnsupported` (`reduce.rs` maps `Trim(None)` to it), while
the plain ball ∪ the same brick builds. That is a normal verb on normal
geometry, and it is the remainder of the closed P0
`point-in-solid-refuses-a-sphere-face-bounded-by-a-spline-chord`'s
purpose. Re-priced H.

**Decided** (two designers, agreed; `work/inside/log.md`, 2026-11-25).
This replaces the "Owed" section above, whose first option DL5.3 rules
out:
- `curve_face::cross` answers the `SplineComposite` arm itself, at
  `T: Decide`. It decides by `Decide` signs on Bernstein coefficients
  computed in the scalar's own arithmetic: reals at f64 with no pads,
  enclosures at `Interval`.
- Every selection reader (the face door, the side reader,
  `point_in_solid`, the counts) reads it on the value view, so a dual
  takes its base's selection.
- A spline crossing the body stores is a proposal, admitted at the
  storing site under `Certifies`.
- **This unit retires:** `AtRestPolicy::spline_face_crossings` and
  `spline_face_reader`; `Legs::splines`, `Splines`, `SplineCrossings`
  and `spline_landings`; the `splines` argument of `face_geo`,
  `sphere_face_region` and `torus_face_region`; EMBEDWALK's
  `curve_face_placement_reading`; and the `Enclosed` + `Curve3::Nurbs`
  re-entries in `splitting::classify`, `boolean::reduce`,
  `boolean::slit` and `region::landings`.
- **It claims** GERM's `cross-answers-a-spline-arm-with-a-whole-span-window`.
  It builds on the (line, spline patch) arm that
  `point-containments-ray-test-has-no-spline-arm`'s unit puts into `cross`.
- Its passenger, `the-ring-composites-refusals-and-end-readings-are-translated-per-caller`,
  goes with it, since its four translating sites are the re-entries it
  retires.

## A second entry to fold back (EMBEDWALK, PR 7772, 2026-11-25)

The embedding walk's cell door (`cell_region::place`, bound on
certification rights) reaches the face door through
`contain::curved_face_placement_reading(body, face, q, band, splines)`,
a twin of `curved_face_placement` that hands
`props::certified_spline_face_crossings` to the sphere's and the torus's
regions (`torus_face_region` now takes `splines` too). When the face door
reads spline arcs at its own bound, `curved_face_placement_reading` folds
into `curved_face_placement`, and the `splines` parameters of
`curved_face_placement_decided`, `curved_interior_placement`,
`sphere_face_containment` and `torus_face_containment` go with it.

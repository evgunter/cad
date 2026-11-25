---
id: a-moved-copys-pick-span-crosses-its-frame-unconverted
kind: issue
title: A moved copy's pick span is certified on the ray carried into its frame, which is the caller's only to within the carry's rounding
status: open
opened: 2026-10-22
priority: P1
cost: E
---


First written (2026-10-22, split out of
`pickindex-merges-parts-on-a-rounded-t-it-never-converts`) as "the
span crosses the frame unconverted"; `vgeom/project-refuses` found
that premise false and narrowed the row to what is left (2026-11-25).

## Finding

`crates/viewer/src/pickindex.rs`, `PickIndex::pick_for`: a part drawn
under a display map (`display_mapped`, which is
`DisplayView::display_map`: the space's free-move frame composed with
the placement editor's carry, `PreviewedCopy::carry`) is picked by the
caller's ray carried through `map.inverse()`.

**The parameter needs no conversion.** An affine map carries a ray's
parameter with it, `map(o′ + t·d′) = o + t·d`, so the `t`, `t_lo` and
`t_hi` the kernel answers on the carried ray are parameters of the
caller's ray under any affine map, rigid or not. The hit's point is
read off the caller's ray at `t`, as `pick_face` reads every hit's;
`review_gui4_r1::r1_a_rotated_probe_is_drawn_picked_and_reported_in_world`
asserts `point == origin + t·dir` bit for bit.

**What does not carry is the certificate.** The span is certified on
the carried ray AS ROUNDED, which is the caller's only to within the
inverse's rounding — even a translation rounds the carried origin, at
the order of a certificate's own width. That span is then ordered
against spans certified on the caller's ray itself:

- the cross-group merge in `pick_for` (`TSpan::survivors`,
  `answer_of`), where a moved copy's face coincident with an unmoved
  body's can be ordered or tied where the certificate says nothing;
- the edge lane's occlusion probe (`PickIndex::near`): every edge
  hover on a moved copy runs `own_crossing` on the caller's ray, with
  the copy's triangles placed in world, against the face spans
  `front_of` certifies on the carried ray. The two coincide by
  construction (the `gpu.rs` edge-lift doc says so), so a rounding
  either way can read the edge as **occluded by its own face**, and
  the edge does not pick, silently.

## Next step

A witness first: a fuzz hovering moved-copy edge midpoints (and a
coincident moved/unmoved face pair) under rotated and translated
frames, counting picks the unmoved twin answers and the moved copy
does not. If one flips, repair by either shape below; if none can
flip after a fair search, the row drops to P3.

Repair shapes: widen the carried span by a bound on the carry's
rounding, in parameter units; or pick a moved copy against its
triangles placed in world (as `own_crossing` already does), so every
span is certified on the caller's ray.

## History

Parked 2026-10-22 on the stage-3 placement pivot (D′,
`work/intent/a-placement-pins-by-written-values-without-a-mate.md`);
moved from FIT into VGEOM 2026-10-26; released by INTENT 2026-10-26
when D′1–D′3 (PRs 5592, 5613, 5636) merged. Re-read after D′: nothing
picks through world coordinates, so D′ did not overtake it.

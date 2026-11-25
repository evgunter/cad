---
id: a-moved-copys-pick-span-crosses-its-frame-unconverted
kind: issue
title: A moved copy's pick span is certified on the ray carried into its frame, which is the caller's only to within the carry's rounding
status: open
opened: 2026-10-22
priority: P1
cost: E
---


Split out of `pickindex-merges-parts-on-a-rounded-t-it-never-converts`
(its §1) on 2026-10-22, when FIT checked its slate against the D10
hold. The finding is unchanged; the reason it is parked is new.

## Finding

`crates/viewer/src/pickindex.rs`, `PickIndex::pick_for`, the loop over
`DisplayView::moved_placements`: the ray is carried into a moved copy's
display-local space through `frame.affine()`'s inverse, and the hit is
lifted back as

```rust
candidates.push(PickHit {
    point: map.transform_point(hit.point),
    ..hit
});
```

`..hit` carries `t`, `t_lo` and `t_hi` verbatim. They are parameters of
the LOCAL ray (`inverse.transform_vec(ray.dir)`), not of the caller's.
The site argues this is safe because the display layer admits only
rigid probe frames, so `|dir|` is kept and `t` means the same world
distance. The kernel's own statement at `PickHit::t` is *"a hit carried
across a transform converts all three or none"*, and since the
cross-group merge runs `TSpan::survivors` over every group's spans,
these unconverted spans are compared against world ones directly.

Repair shape: convert the span through the same map (a no-op for a
rigid map, the correction for any other), or refuse a non-rigid frame
by name at the door that admits it. The comment at the site cites this
finding by its old path under `work/vgeom/`; the lane that lands the
fix points it here.

## Parked on the D10 hold (2026-10-22)

`moved_placements` is the viewer's free-move over loose copies' spaces.
Stage 3 D′ (`work/intent/a-placement-pins-by-written-values-without-a-mate.md`,
`[ev]` PR 5494) retires loose spaces and moves free-move onto shown
spaces, drawn through one composing door shared with export. That
rewrites the map this site reads, and possibly the site itself, so
fixing it now would fix code D′ is about to replace. When D′ lands,
re-read this site: if the composing door already converts the span, or
picks through world coordinates, the row closes as overtaken.

## Moved (2026-10-26)

FIT folded back into VGEOM (Ev, in chat, 2026-10-26): FIT was cut from
VGEOM on 2026-09-20 to run in parallel, a reason that is gone at FIT's
size. Id and body unchanged.

## Released (INTENT, 2026-10-26)

This row was parked on the stage-3 placement pivot (`a-placement-pins-by-written-values-without-a-mate`). All three of its PRs are merged: D′1 (PR 5592), D′2 (PR 5613) and D′3 (PR 5636, `29110056e8`). Re-check this row against main, then fix, narrow or close it. That is the owning track's call.

## Narrowed (vgeom/project-refuses, 2026-11-25)

Re-read on main after D′. `PickIndex::pick_for` now groups every part
`display_mapped` answers a map for (`DisplayView::display_map`: the
space's free-move frame composed with the placement editor's carry,
`PreviewedCopy::carry`), not `moved_placements`; the crossing is
otherwise the one described above. D′ did not overtake it: nothing
picks through world coordinates and nothing converts the span.

**The parameter needs no conversion, and never did.** The local ray
is the caller's carried through `map.inverse()`, and an affine map
carries a ray's parameter with it: `map(o′ + t·d′) = o + t·d`. So
`t`, `t_lo` and `t_hi` are already parameters of the caller's ray,
whatever the map, rigid or not; the rigid-frame argument the site
leaned on was the wrong reason for a true conclusion. The branch
reads the point off the caller's ray at `t` (`ray.origin + ray.dir *
hit.t`, as `pick_face` reads every hit's) instead of mapping the local
point back, so `PickHit::point` is `origin + t · dir` bit for bit;
`review_gui4_r1::r1_a_rotated_probe_is_drawn_picked_and_reported_in_world`
asserts it, and fails on the mapped point (`y` off in the last bit).

**What is left.** The span is CERTIFIED on the carried ray as rounded,
which is the caller's ray only to within the inverse's rounding, and
the cross-group merge (`TSpan::survivors`, `answer_of`) and the edge
lane's occlusion probe (`PickIndex::near`, `own_crossing` against
`front_of`) order it against spans certified on the caller's ray
itself. A crossing that sits within that rounding of a world one, a
moved copy's face coincident with an unmoved body's, or a moved copy's
edge over its own face, can be ordered or tied where the certificate
says nothing. Repair shapes: widen the carried span by a bound on the
carry's rounding in parameter units, or pick a moved copy against
its triangles placed in world (as `own_crossing` already does) so
every span is the caller's ray's. Not driven: no witness of a flipped
verdict was built.


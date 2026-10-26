---
id: a-moved-copys-pick-span-crosses-its-frame-unconverted
kind: issue
title: A moved copy's pick hit converts its point across the display frame and carries t, t_lo and t_hi over unconverted
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

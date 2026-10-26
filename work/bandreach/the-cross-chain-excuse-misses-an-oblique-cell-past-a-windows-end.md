---
id: the-cross-chain-excuse-misses-an-oblique-cell-past-a-windows-end
kind: issue
title: blend: the reach's cross-chain excuse misses an oblique cell past a link's window end, so a clear sealed island refuses at one pose
status: open
opened: 2026-10-12
priority: P3
cost: M
---


## Finding

The reach meter (`blend/reach.rs`, `band_reach`) clears a cell of
another chain's support only when the cell lies wholly inside the
strip that chain's band replaces (`Reach::replaces`). A straight
link's strip runs over the edge's own extent, its window widened by
`slab = 2 · FaceBoxes::pad(band)` past each end (`straight_reach`).
A box cell of the support that straddles the window's end plane
overruns it by a fraction of the cell's width; over an axis-aligned
edge that is at most the pad, which the slab holds, but over an
oblique edge it can be far more, so the excuse stops firing there.

**Witness.** `blend_per_shell_carry::the_sealed_island_request_builds_at_every_pose`:
the sealed void `[1,3]³` with the island `[1.05,2.95]³`, all 24
edges filleted at `0.25` in one request. At the pose `1.1 about
(1,2,3) through (-0.2, 0.1, 0.4)` it refuses `FaceClearance
{ bounded: true }` on clear geometry; the other five poses build at
the closed form. The refusal is the same on `8cd31ca1d` (before the
one-sided strip of PR 4286).

**Cause, measured.** Widening the excuse's window ends from `slab`
to `chord` (the strip's own radius) builds that pose at the closed
form (`ΔV = 2.1e-14`). That widening is not the fix: past a reflex
end the face goes on where no band replaces it, so the window end
cannot move out by `chord` unscreened.

Conservative, not unsound: nothing wrong is built.

## What would close it

Either read the excuse against the part of the cell on the face
(the support is planar: clip the cell to the support's plane, or to
its own box) rather than the whole cell, or screen the support past
each window end as `screened` screens it across the edge's line and
widen the end where nothing of the face lies there. Then change the
witness row's `1.1` pose to build.


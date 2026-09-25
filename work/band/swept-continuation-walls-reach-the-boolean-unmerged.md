---
id: swept-continuation-walls-reach-the-boolean-unmerged
kind: issue
title: sweep: a declared continuation's two walls share one plane key but reach the boolean unmerged, and the recourse (merge_coplanar_faces) has no door above the kernel
status: open
opened: 2026-09-25
priority: P0
cost: D
---


Measured by `subdivided-profile-side-coplanar-walls-gate`'s rows
(`crates/sweep/tests/band_subdivided_side_walls.rs`).

## What holds

A profile side authored as `line(len)` plus `continue_to` (the declared
straight continuation) extrudes, and revolves full or partial, to two
walls under ONE surface key: `sweep_loop` decides every join's
cosurface verdict up front and `side_surface` hands the continuing
segment `FaceSurface::Shared` (`crates/sweep/src/extrude.rs`; the
revolve twins in `revolve/full.rs` and `revolve/partial.rs`). The body
is tier-3 valid. The structural rung merges the pair on request
(`Body::merge_coplanar_faces`, `crates/topo/src/merge_faces.rs`), and
the merged body unions correctly.

The sweep does not run that merge, so the pair reaches the boolean
unmerged, and `gate_maximal_faces` (`crates/topo/src/boolean/reduce.rs`)
refuses a PLANAR same-key adjacency as `NonMaximalFaces`: at the
extrude's continuation strut, and at the revolve's annulus split
circle. (A same-key CURVED pair — the revolve's split cylinder band —
is the gate's canonical maximal form and passes.) The refusal names
its recourse, `merge_coplanar_faces`.

## The wall

That recourse is a `topo::Body` method and nothing else. No verb in
`crates/verbs/src/verb.rs`, no recipe node in `editor-core`, and
nothing in the `pncad` facade reaches it. An author working through
the recipe layer or the UI who writes a straight continuation can
extrude the result, but cannot use it as a boolean operand at all.

## The fork (needs Ev)

The F7 clause in `docs/DESIGN.md` ("Maximal-faces precondition and
the merge stage") says merging is never silent. `merge_coplanar_faces`
is the explicit opt-in op, and only boolean *outputs* run it as a
documented final stage. Any of the three ways out changes what that
clause decides, or what N4 names:

1. **The sweep runs the structural merge as a documented final stage,
   the way boolean outputs do.** The continuation declared the shared
   carrier, so the key is shared by construction and no coincidence
   is inferred. The cost is naming and handles: the swept wall
   becomes N3's `Merged(Lateral(s), Lateral(s+1))`, the interior
   strut's `LateralEdge` and its `CapVertex` pair lose their entity
   (the full revolve's deleted axis run is the precedent for that
   silence), and `Extruded::side_faces` / `strut_edges` and
   `Revolved::walls` stop being one fresh entity per segment and per
   vertex. `name_swept_topology`'s `rim_between(wall, cap)` then finds
   two rims on a merged wall, so the emitter needs the per-segment
   rims from the builder.
2. **An explicit merge verb at the recipe layer** (`Verb::MergeFaces`
   or similar). F7 stays exactly as written. The cost is that every
   continuation-extrude then needs a manual step before any boolean.
3. **The recipe boolean node merges its operands' structural pairs**
   as a documented pre-stage. The kernel's gate is unchanged, but the
   recipe hides the step that F7 wants to be visible.

Recommendation: (1). What F7 guards against is merging on inferred
(numeric) coincidence, and this pair is structural by the author's own
declaration. The naming rule it needs, N3's `Merged`, is already
ratified.

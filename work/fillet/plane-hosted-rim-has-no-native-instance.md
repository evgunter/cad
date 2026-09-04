---
id: plane-hosted-rim-has-no-native-instance
kind: issue
title: fillet: the one-plane-face closed rim has no native instance — a pole-touching revolve splits every wall, so the shape arises only after a coplanar-merge repair
status: open
opened: 2026-09-04
---


Measured in FILLET-H5's Phase 1, on the branch `fillet/h5-hostless-rim`,
against `docs/FILLET-H5-SPEC.md`'s claim section.

## What the spec claims

> NATIVELY, whenever a revolve's plane annulus does not touch the axis
> but its curved neighbour does (a pole-touching dome or dimple on a
> wider flat top: **the plane is one face from the mint**, the curved
> side is two half-caps, the rim is two arcs).

## What is measured

**False.** A full revolve of a pole-touching profile is the WIRE case
(`crates/sweep/src/revolve/full.rs:313`–`:322`): the on-axis run is
omitted and *the rest of the loop* sweeps in two π-bands. The split is
a property of the BODY, not of the segment — every wall of such a
revolve is two half-faces, whether or not that wall touches the axis.

Witness, built through `test_support::revolved_about_y`: a cylinder of
radius 1 and height 1 carrying a flat top annulus from radius 1 in to
radius 0.5 and a hemispherical dome of radius 0.5 rising from there to
the pole at `(0, 1.5)` — the spec's own "pole-touching dome on a wider
flat top". Four profile segments, census `V=8 E=14 F=8`: **eight faces
for four segments**. The rim at `(0.5, 1.0)` has two arcs, **two**
distinct planar supports, and crossings of valence **4** (two rim arcs
plus one co-surface plane seam plus one co-surface sphere seam). It is
the ordinary seam-split annulus and it **carves today**. Its dimple
twin (the same pocket dipping to `(0, 0.5)`) measures identically.

So the shape the spec is about — ONE plane face hosting every arc, in
that face's own outer cycle, crossings TRIVALENT — arises from a
revolve only after `merge_coplanar_faces`. The item's own framing
(`repaired-pole-rim-serves-no-closed-door`, GitHub issue 1245) was
right: it is a REPAIRED shape.

## The repaired boss and dimple are not substitutes

Repairing the boss/dimple with `merge_coplanar_faces` merges the flat
top's two half-annuli into one face — but that face is an ANNULUS, so
the rim lands in a **ring** of it, not in its outer cycle. It therefore
routes to the LADDER, passes the ladder's gates, and refuses on ring
clearance instead (a separate defect, filed as
`ring-clearance-refuses-a-nested-trim-circle`). Neither the native pair
nor the repaired pair is an instance of the one-plane-face shape.

## Instances that ARE the shape

Every one measured with two arcs, ONE planar support, the rim in that
face's **outer cycle** (length 2), zero rings, crossings of valence 3
(two rim arcs + exactly one co-surface mate seam), refusing
`UnsupportedChain { detail: "a closed chain is not a ring of its plane
support" }`:

| fixture (after `merge_coplanar_faces`) | rim | pair | side |
|---|---|---|---|
| `test_support::lantern` neck | `(1, 0)` | plane×sphere | convex |
| `test_support::lantern` lip | `(0.2, 1.2)` | plane×cone | convex |
| hemisphere on a flat base | `(1, 0)` | plane×sphere | convex |
| `test_support::waisted` base | `(1, 0)` | plane×cone | convex |
| `test_support::waisted` top | `(1, 1)` | plane×cone | convex |
| bowl floor (below) | `(1, 1)` | plane×cone | **concave** |

The bowl is `revolved_about_y` of `(0,0) (1.5,0) (1.5,1.5) (1,1)
(0,1)`: a flat floor at `y = 1` out to radius 1, then a lip rising to
`(1.5, 1.5)` and down the outside. Its floor rim is an inside corner —
the raw (unrepaired) body's carve of it ADDS material, volume delta
`+3.375275670087774e-4` — so **both material sides are reachable in
this shape**, which is what the spec's acceptance needs and what the
boss/dimple pair was there to supply.

## The ask

Re-ratify `docs/FILLET-H5-SPEC.md`: drop the native route from the
claim, and replace the native boss/dimple pair in §Phase 1 and §Rows
with a repaired convex fixture and the bowl. The Phase 2 design is
untouched by this — the routing arm, the `Strut` host foot and the
per-arc host trimlines are the same change either way.

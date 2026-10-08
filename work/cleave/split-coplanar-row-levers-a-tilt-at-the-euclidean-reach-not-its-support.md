---
id: split-coplanar-row-levers-a-tilt-at-the-euclidean-reach-not-its-support
kind: issue
title: the split's coplanarity row levers a face's tilt at its Euclidean reach, not its support across the tilt
status: open
opened: 2026-10-08
priority: P3
cost: M
---

Found by TANG's span-bounded conic reach (PR 4292's differential).

## What

`splitting::rules::apply_rule_a` decides `split_sector_coplanar` on `Margin::levered(|n_face × n_SP|, face_extent)`, the tilt levered at the face's Euclidean reach from the base vertex. A tilt about a hinge line through the vertex moves a point of the face by the tilt times the point's distance from that HINGE, not from the vertex. Every point's distance from the hinge is at most its distance from the vertex, so the margin is sound but loose, by up to the whole reach for a face lying along the hinge.

**Evidence.** `crates/geom-brep/tests/span_reach_differential.rs`, plane sector family (10,000 annular sectors, each tilted about a random line through a corner):
- "tilted" (definitely not coplanar) is served where the face's farthest displacement off the tilted plane is in the band 517 times on head and 2,367 on main;
- every head serving is one main also serves;
- what head gave up is main's whole-turn conic over-statement; what is left is this row's own measure.

## The shape of a fix

Lever the tilt at the face's support across the hinge: the largest `|(x − base)·(n_SP − n_face)|` over the boundary, one edge at a time (`geom_brep::Reach::range_along` takes any direction). On a plane face a linear function peaks on the boundary. That is the displacement itself, so the row escalates only where the face really stands in the band.

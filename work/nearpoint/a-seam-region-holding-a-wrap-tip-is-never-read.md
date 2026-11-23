---
id: a-seam-region-holding-a-wrap-tip-is-never-read
kind: issue
title: A sphere seam region holding a wrap tip is never read: its member's spur turn refuses as a cusp, so the region's side and cover census skip it
status: open
opened: 2026-10-31
priority: P3
cost: M
---


Filed by RESTCURVE's `a-wrap-tip-cap-loses-its-side-datum-when-the-chart-image-drops-the-tip` (PR 6116, from its review), on NEARPOINT's slate because `sphere_region_material_sign`, `sphere_region_cover` and `seam_loops` in `crates/geom-brep/src/props/curved.rs` are NEARPOINT's.

## What

A sphere seam region (`topo::props::read_seam_regions`) parses every member's whole loop with `geom_brep::props::seam_loops`. For a member that holds a wrap tip, that loop walks the wrap edge out to the pole and straight back. `sphere_loop` reads the turn at the tip as a cusp (`props_sphere_loop_cusp`), and `seam_loops` answers `Err(SphereLoop { what: "props_sphere_loop_cusp" })`. `read_seam_regions` then keeps `loops: None`:

- The region's side (`sphere_region_material_sign`) is never read for a region that holds a tip.
- The cover census (`sphere_region_cover`) skips the region, so two seam siblings that overlap around a wrap tip go uncounted.

The review of PR 6116 reproduced this on the small wrap-tip cap: probe `review_probe_region_reader_on_tip_cap` on branch `restcurve/review-quokka-6116` @ 33be8ab0. DESIGN `[topology.tiers]` says no reader reads a turn at a wrap tip, and the region reader is reading one.

Today the side of a one-face tip cap does not depend on it: its own boundary encodes a side from the tip it holds (`boundary_material_sign_holding`), so check 6 never asks the region. The census gap is live for a region of two faces or more that holds a tip, but no door mints a tip yet.

## What a fix owes

Read a member's loop as its chart image when parsing the region, and keep the tips it drops as points the region holds, the same datum the face reader takes. The region reader's "pole on an interior edge" arm (`pole_against_arc` over the members' arcs) would then see a held pole through the datum, not through the spur's arc. Add a row in which two seam siblings overlap around a wrap tip and the cover census refuses them.

---
id: the-trimmed-lanes-image-pieces-are-laid-per-image-not-per-span
kind: issue
title: the trimmed lane lays its General-image pieces per image, not per knot span of the image
status: open
opened: 2026-10-30
---


(QUAD per-span lane, `quad/per-span-round-zero`.)

## What

The trimmed lane's `General` chart images are cut into `IMAGE_INIT_PIECES << round` blocks per image (`bezier_blocks`, `crates/geom-brep/src/props/quad/trimmed.rs`, over `algebra::domain_grid_points(kv, m)`), and its round cost counts `images × pieces` (`OuterRule::round_cost`). That is a per-image grid over the image's whole domain, so an image split in two (at a C⁰ knot of its carrier, which the import door now splits) gets twice the pieces. The same shape the rectangle lanes had per face, one level down.

The chord area under it is per chart knot span already: `chord_polygon_area` cuts on `knot_aligned_cuts`, which is per span.

## Owed

Lay the image's pieces per knot span of the image (`algebra::equal_split_points`) and price the round by them, or say why an image's pieces should not be per span. Not done in the per-span unit: the trimmed lane is not a rectangle lane, and a trimmed face split per span yields tiles of other lanes, so "the same cells whole or tiled" does not hold for it by construction.
